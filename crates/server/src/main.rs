use std::{convert::Infallible, sync::LazyLock, time::Duration};

use axum::{
    Json, Router,
    extract::{Query, State},
    http::{Method, StatusCode},
    response::{Redirect, Sse, sse},
    routing::{get, patch, post},
};
use futures_util::{SinkExt, Stream};
use iced_futures::stream;
use tokio::{net::TcpListener, sync::broadcast::error::TryRecvError};
use tower_http::{
    cors::CorsLayer,
    limit::RequestBodyLimitLayer,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing::{Level, debug, error, info, trace};
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
const WEB_ORIGIN: &str = dotenvy_macro::dotenv!("WEB_ORIGIN");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    common::logger::Builder::new()
        .with_crate_name(env!("CARGO_CRATE_NAME"))
        .with_crate_level(Level::TRACE)
        // .with_target("tower_http", Level::DEBUG)
        .init();

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
        .allow_origin([WEB_ORIGIN.parse().unwrap()])
        .allow_methods([Method::GET]);
    let app = Router::new()
        .route("/", get(async || Redirect::to(WEB_ORIGIN)))
        .nest(
            "/api",
            Router::new()
                .route("/auth", post(auth))
                .route(
                    "/register",
                    post(register).put(update_register).get(get_register),
                )
                .route("/register/fact", patch(update_register_fact))
                .route("/registers", get(list_registers))
                .route("/watch", get(watch_updates)),
        )
        .layer(cors)
        .layer(TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default()))
        .layer(RequestBodyLimitLayer::new(1024))
        .with_state(Store::new(db_path)?);

    let url = format!("0.0.0.0:{}", *SERVER_PORT);
    debug!("http listen on {url}");
    let listener = TcpListener::bind(url).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn auth(
    State(store): State<Store>,
    Json(args): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, ResponseError<&'static str>> {
    let id = Uuid::new_v4();
    if let Err(e) = store.save_user(id, args.name) {
        error!("failed to save user: {e}");
        return Err(ResponseError::Status(StatusCode::INTERNAL_SERVER_ERROR));
    }

    Ok(Json(AuthResponse { key: id }))
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
    if let Err(e) = store.save_register(id, args.key, args.save) {
        error!("failed to save register: {e}");
        return Err(ResponseError::Status(StatusCode::INTERNAL_SERVER_ERROR));
    }

    Ok(Json(RegisterResponse { id }))
}

/// Update registered save
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

    if current_save.user != args.key {
        return Err(ResponseError::Status(StatusCode::UNAUTHORIZED));
    }
    if current_save.save == args.save {
        return Err(ResponseError::Status(StatusCode::NOT_MODIFIED));
    }
    if !saves::is_allowed_to_override(&current_save.save, &args.save) {
        return Err(ResponseError::StatusMessage((
            StatusCode::BAD_REQUEST,
            "can't remove keys from save",
        )));
    }

    if let Err(e) = store.save_register(args.id, args.key, args.save) {
        error!("failed to update register: {e}");
        return Err(ResponseError::Status(StatusCode::INTERNAL_SERVER_ERROR));
    }

    watch
        .send(args.id)
        .await
        .expect("should be able to post watch update");

    Ok(())
}

/// Enable fact in registered save
#[cfg(debug_assertions)]
async fn update_register_fact(
    State(store): State<Store>,
    State(watch): State<Watches>,
    Json(args): Json<UpdateRegisterFactRequest>,
) -> Result<(), ResponseError<&'static str>> {
    if args.num >= saves::KEYS_COUNT {
        return Err(ResponseError::StatusMessage((
            StatusCode::BAD_REQUEST,
            "num is too big",
        )));
    }

    let current_save = match store.get_register(args.id) {
        Ok(None) => return Err(ResponseError::Status(StatusCode::NOT_FOUND)),
        Ok(Some(save)) => save,
        Err(e) => {
            error!("failed to get register when updating by num: {e}");
            return Err(ResponseError::Status(StatusCode::INTERNAL_SERVER_ERROR));
        }
    };

    if current_save.user != args.key {
        return Err(ResponseError::Status(StatusCode::UNAUTHORIZED));
    }
    if saves::has_bool_enabled(&current_save.save, args.num) {
        return Err(ResponseError::Status(StatusCode::NOT_MODIFIED));
    }
    if let Err(e) = store.save_register(
        args.id,
        args.key,
        saves::enable_bool(&current_save.save, args.num),
    ) {
        error!("failed to update register by num: {e}");
        return Err(ResponseError::Status(StatusCode::INTERNAL_SERVER_ERROR));
    }

    watch
        .send(args.id)
        .await
        .expect("should be able to post watch update");

    Ok(())
}

#[cfg(not(debug_assertions))]
async fn update_register_fact() -> StatusCode {
    StatusCode::NOT_FOUND
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
async fn list_registers() -> StatusCode {
    StatusCode::NOT_FOUND
}

async fn watch_updates(
    State(mut watch): State<Watches>,
    Query(args): Query<GetRegisterRequest>,
) -> Sse<impl Stream<Item = Result<sse::Event, Infallible>>> {
    let stream = stream::channel(100, async move |mut output| {
        trace!("starting watch channel");
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            let id = match watch.rx.try_recv() {
                Ok(id) if args.id != id => continue,
                Ok(id) => id,
                Err(TryRecvError::Empty) => continue,
                Err(_) => break,
            };
            trace!("sending watch update for {id}");
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
