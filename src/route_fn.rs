use axum::{http::StatusCode, Json, extract::Path};

use crate::scrape::{Novel, self};

pub async fn novel_detail(Path(novel_id): Path<u64>) -> (StatusCode, Json<Novel>) {
    let novel = scrape::get_novel_detail(novel_id).await.unwrap();
    (StatusCode::OK, Json(novel))
}

pub async fn novel_clicks(Path(novel_id): Path<u64>) -> (StatusCode, Json<(u64, u64)>) {
    let data = scrape::get_chapter_clicks(novel_id).await.unwrap();
    (StatusCode::OK, Json(data))
}