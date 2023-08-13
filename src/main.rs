pub mod errors;
pub mod route_fn;
pub mod scrape;

use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().nest(
        "/api",
        Router::new().nest(
            "/novel/:novel_id",
            Router::new()
                .route("/detail", get(route_fn::novel_detail))
                .route("/clicks", get(route_fn::novel_clicks)),
        ),
    );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3300));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
