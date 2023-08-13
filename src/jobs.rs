use std::env;

use sqlx::SqlitePool;
use tracing::{info, error};

use crate::scrape::get_novel_detail;

pub async fn sync_novel_details() -> anyhow::Result<()> {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    let rows = sqlx::query!("SELECT id, novel_id FROM novels")
        .fetch_all(&pool)
        .await
        .unwrap();

    for row in rows {
        let novel_id = row.novel_id.unwrap();
        match get_novel_detail(novel_id as u64).await {
            Ok(novel) => {
                info!("{:?}", novel);
            }
            Err(_) => error!("failed for novel_id: {}", novel_id),
        };
    }

    Ok(())
}
