use std::{convert::Infallible, sync::LazyLock, time::Duration};

use axum::{
    Json, Router,
    extract::{Query, State},
    http::{Method, StatusCode},
    response::{Sse, sse},
    routing::{get, post},
};
use futures_util::{SinkExt, Stream};
use iced_futures::stream;
use tokio::net::TcpListener;
use tower_http::{
    cors::CorsLayer,
    limit::RequestBodyLimitLayer,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing::{debug, error, info};
use uuid::Uuid;

use common::saves;
use common::server_models::*;
use store::{Store, Watches};

use response::ResponseError;

mod response;
mod store;

static SERVER_PORT: LazyLock<u16> = LazyLock::new(|| {
    dotenvy_macro::dotenv!("SERVER_PORT")
        .parse()
        .expect("server port should be a valid number")
});
const DB_PATH: &str = dotenvy_macro::dotenv!("DB_PATH");
const ALLOW_ORIGIN: &str = dotenvy_macro::dotenv!("ALLOW_ORIGIN");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    common::logger::init_logging(env!("CARGO_CRATE_NAME"));

    let db_path = std::env::var("DB_PATH")
        .inspect_err(|e| {
            use std::env::VarError;
            match e {
                VarError::NotPresent => info!("DB_PATH is not set, using {DB_PATH}"),
                VarError::NotUnicode(value) => {
                    error!("DB_PATH constains non-utf value: \"{value:?}\", using {DB_PATH}")
                }
            }
        })
        .ok()
        .unwrap_or(DB_PATH.to_string());

    let cors = CorsLayer::new()
        .allow_origin([ALLOW_ORIGIN.parse().unwrap()])
        .allow_methods([Method::GET]);
    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                .route(
                    "/register",
                    post(register).put(update_register).get(get_register),
                )
                .route("/registers", get(list_registers))
                .route("/watch", get(watch_updates)),
        )
        .layer(cors)
        .layer(TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default()))
        .layer(RequestBodyLimitLayer::new(1024))
        .with_state(Store::new(db_path)?);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", *SERVER_PORT)).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn register(
    State(store): State<Store>,
    Json(args): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, ResponseError<&'static str>> {
    if !saves::is_valid_number_of_keys(&args.save) {
        return Err(ResponseError::StatusMessage((
            StatusCode::BAD_REQUEST,
            "invalid encoded save",
        )));
    }

    let id = Uuid::new_v4();
    if let Err(e) = store.save_register(id, args.save) {
        error!("failed to save register: {e}");
        return Err(ResponseError::Status(StatusCode::INTERNAL_SERVER_ERROR));
    }

    Ok(Json(RegisterResponse { id }))
}

async fn update_register(
    State(store): State<Store>,
    State(watch): State<Watches>,
    Json(args): Json<UpdateRegisterRequest>,
) -> Result<(), ResponseError<&'static str>> {
    if !saves::is_valid_number_of_keys(&args.save) {
        return Err(ResponseError::StatusMessage((
            StatusCode::BAD_REQUEST,
            "invalid encoded save",
        )));
    }

    let current_save = match store.get_register(args.id) {
        Ok(None) => return Err(ResponseError::Status(StatusCode::NOT_FOUND)),
        Ok(Some(save)) => save,
        Err(e) => {
            error!("failed to get register when updating: {e}");
            return Err(ResponseError::Status(StatusCode::INTERNAL_SERVER_ERROR));
        }
    };

    if current_save.save == args.save {
        return Err(ResponseError::Status(StatusCode::NOT_MODIFIED));
    }
    if !saves::is_allowed_to_override(&current_save.save, &args.save) {
        return Err(ResponseError::StatusMessage((
            StatusCode::BAD_REQUEST,
            "can't remove keys from save",
        )));
    }

    if let Err(e) = store.save_register(args.id, args.save) {
        error!("failed to update register: {e}");
        return Err(ResponseError::Status(StatusCode::INTERNAL_SERVER_ERROR));
    }

    watch
        .send(args.id)
        .await
        .expect("should be able to post watch update");

    Ok(())
}

async fn get_register(
    State(store): State<Store>,
    Query(args): Query<GetRegisterRequest>,
) -> Result<Json<GetRegisterResponse>, StatusCode> {
    let id = args.id;
    let save = match store.get_register(id) {
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Ok(Some(save)) => save,
        Err(e) => {
            error!("failed to get register: {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(Json(GetRegisterResponse {
        id,
        save: save.save,
        updated: save.updated,
    }))
}

#[cfg(debug_assertions)]
async fn list_registers(
    State(store): State<Store>,
) -> Result<Json<GetRegistersResponse>, StatusCode> {
    let registers = match store.list_registers() {
        Ok(registers) => registers,
        Err(e) => {
            error!("failed to get register: {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(Json(GetRegistersResponse {
        count: registers.len(),
        registers: registers
            .into_iter()
            .map(|(id, r)| GetRegisterResponse {
                id,
                save: r.save,
                updated: r.updated,
            })
            .collect(),
    }))
}

#[cfg(not(debug_assertions))]
async fn list_registers(store: State<Store>) -> StatusCode {
    StatusCode::NOT_FOUND
}

async fn watch_updates(
    State(mut watch): State<Watches>,
    Query(args): Query<GetRegisterRequest>,
) -> Sse<impl Stream<Item = Result<sse::Event, Infallible>>> {
    let stream = stream::channel(100, async move |mut output| {
        while let Ok(id) = watch.rx.recv().await {
            if args.id != id {
                continue;
            }

            debug!("sending watch update for {id}");
            output
                .send(Ok(sse::Event::default().data("save-updated")))
                .await
                .unwrap();
        }
    });

    Sse::new(stream).keep_alive(
        sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keeped alive"),
    )
}
