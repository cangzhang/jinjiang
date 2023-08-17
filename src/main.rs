pub mod errors;
pub mod jobs;
#[allow(warnings, unused)]
pub mod prisma;
pub mod route_fn;
pub mod scrape;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Extension, Json, Router,
};
use dotenvy::dotenv;
use prisma::PrismaClient;
use prisma_client_rust::{
    prisma_errors::query_engine::{RecordNotFound, UniqueKeyViolation},
    QueryError,
};
use std::{net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let db = create_db_pool().await;
    let db_guard = Arc::new(db);
    let db_guard2 = db_guard.clone();

    let app = Router::new()
        .nest(
            "/api",
            Router::new().nest(
                "/novel/:novel_id",
                Router::new()
                    .route("/statistics", get(route_fn::novel_statistics))
                    .route("/detail", get(route_fn::novel_detail)),
            ),
        )
        .layer(Extension(db_guard));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3300));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

pub async fn create_db_pool() -> PrismaClient {
    PrismaClient::_builder().build().await.unwrap()
}

pub enum AppError {
    PrismaError(QueryError),
    NotFound,
}

pub type Database = Extension<Arc<PrismaClient>>;
pub type AppResult<T> = Result<T, AppError>;
pub type AppJsonResult<T> = AppResult<Json<T>>;

impl From<QueryError> for AppError {
    fn from(error: QueryError) -> Self {
        match error {
            e if e.is_prisma_error::<RecordNotFound>() => AppError::NotFound,
            e => AppError::PrismaError(e),
        }
    }
}

// This centralizes all different errors from our app in one place
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::PrismaError(error) if error.is_prisma_error::<UniqueKeyViolation>() => {
                StatusCode::CONFLICT
            }
            AppError::PrismaError(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound => StatusCode::NOT_FOUND,
        };

        status.into_response()
    }
}
