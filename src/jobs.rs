use std::env;

use sqlx::SqlitePool;

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
                println!("{:?}", novel);
            }
            Err(_) => println!("failed for novel_id: {}", novel_id),
        };
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;

    use crate::jobs::sync_novel_details;

    #[tokio::test]
    async fn sync_novels() -> anyhow::Result<()> {
        dotenv().ok();
        let _ = sync_novel_details().await?;
        Ok(())
    }
}
