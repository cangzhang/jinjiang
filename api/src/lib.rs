pub mod errors;
pub mod jobs;
pub mod route_fn;
pub mod scrape;

use axum::{routing::get, Extension, Router};
use dotenvy::dotenv;
use std::{
    env,
    net::SocketAddr, thread, time,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let pool = db::create_pool(&env::var("DATABASE_URL")?);

    tokio::spawn(async move {
        loop {
            dbg!("[sync_novel_details] started");
            let _ = jobs::sync_novel_statistics().await;
            thread::sleep(time::Duration::from_secs(60 * 30));
        }
    });

    let app = Router::new()
        .nest(
            "/api",
            Router::new().nest(
                "/novel/:novel_id",
                Router::new()
                    .route("/detail", get(route_fn::novel_statistics))
                    .route("/clicks", get(route_fn::novel_clicks)),
            ),
        )
        .layer(Extension(pool.clone()));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3300));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
