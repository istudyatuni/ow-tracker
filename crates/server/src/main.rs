use std::sync::LazyLock;

use axum::{
    Json, Router,
    extract::{Query, State},
    http::{Method, StatusCode},
    routing::post,
};
use tokio::net::TcpListener;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing::{error, info};
use uuid::Uuid;

use common::saves;
use ow_tracker_server::*;
use store::Store;

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
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_target(false)
            .with_max_level(tracing::Level::DEBUG)
            .finish(),
    )?;

    let db_path = std::env::var("DB_PATH")
        .inspect_err(|e| {
            use std::env::VarError;
            match e {
                VarError::NotPresent => info!("DB_PATH is not set, using {DB_PATH}"),
                VarError::NotUnicode(_) => {
                    error!("DB_PATH constains non-utf value, using {DB_PATH}")
                }
            }
        })
        .ok()
        .unwrap_or(DB_PATH.to_string());

    let cors = CorsLayer::new()
        .allow_origin([ALLOW_ORIGIN.parse().unwrap()])
        .allow_methods([Method::GET]);
    let app = Router::new()
        .route(
            "/api/register",
            post(register).put(update_register).get(get_register),
        )
        .layer(cors)
        .layer(TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default()))
        .with_state(Store::new(db_path)?);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", *SERVER_PORT)).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn register(
    store: State<Store>,
    Json(args): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, StatusCode> {
    let id = Uuid::new_v4();
    if let Err(e) = store.0.save_register(id, args.save) {
        error!("failed to save register: {e}");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(Json(RegisterResponse { id }))
}

async fn update_register(
    store: State<Store>,
    Json(args): Json<UpdateRegisterRequest>,
) -> Result<(), ResponseError<&'static str>> {
    if !saves::is_valid_number_of_keys(&args.save) {
        return Err(ResponseError::StatusMessage((
            StatusCode::BAD_REQUEST,
            "invalid encoded save",
        )));
    }
    let current_save = match store.0.get_register(args.id) {
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
    if let Err(e) = store.0.save_register(args.id, args.save) {
        error!("failed to update register: {e}");
        return Err(ResponseError::Status(StatusCode::INTERNAL_SERVER_ERROR));
    }

    Ok(())
}

async fn get_register(
    store: State<Store>,
    Query(args): Query<GetRegisterRequest>,
) -> Result<Json<GetRegisterResponse>, StatusCode> {
    let id = args.id;
    let save = match store.0.get_register(id) {
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
