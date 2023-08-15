use axum::{http::StatusCode, Json, extract::Path};

use crate::scrape;

pub async fn novel_statistics(Path(novel_id): Path<i32>) -> (StatusCode, Json<scrape::NovelStatistic>) {
    let novel = scrape::get_novel_statistics(novel_id).await.unwrap();
    (StatusCode::OK, Json(novel))
}

pub async fn novel_clicks(Path(novel_id): Path<i32>) -> (StatusCode, Json<(i32, i32)>) {
    let data = scrape::get_chapter_clicks(novel_id).await.unwrap();
    (StatusCode::OK, Json(data))
}