use std::sync::Arc;

use crate::prisma::{novel, novel_statistics, PrismaClient};
use crate::scrape::get_novel_statistics;
use crate::{create_db_pool, scrape};

pub async fn sync_novel_statistics() -> anyhow::Result<()> {
    let db = create_db_pool().await;

    let rows: Vec<novel::Data> = db.novel().find_many(vec![]).exec().await?;
    for row in rows {
        let novel_id = row.novel_id;
        match get_novel_statistics(novel_id).await {
            Ok(novel) => {
                dbg!("got statistics for novel_id: {}", novel_id);

                let s = db
                    .novel_statistics()
                    .create(
                        novel_id,
                        vec![
                            novel_statistics::first_chapter_clicks::set(novel.first_chapter_clicks),
                            novel_statistics::last_chapter_clicks::set(novel.last_chapter_clicks),
                            novel_statistics::reviews::set(novel.reviews),
                            novel_statistics::collected::set(novel.collected),
                            novel_statistics::rewards::set(novel.rewards),
                        ],
                    )
                    .exec()
                    .await?;
                println!("{:?}", s);
            }
            Err(e) => println!("failed for novel_id: {}, {:?}", novel_id, e),
        };
    }

    Ok(())
}

pub async fn sync_editor_recommended_list(db: Arc<PrismaClient>) -> anyhow::Result<()> {
    let list = scrape::make_editor_recommended_list().await?;
    for (novel_id, title, author_id) in list {
        let row = db
            .novel()
            .upsert(
                novel::novel_id::equals(novel_id),
                novel::create(title.clone(), novel_id, author_id, vec![]),
                vec![
                    novel::title::set(title.clone()),
                    novel::author_id::set(author_id),
                    novel::updated_at::set(Some(chrono::DateTime::from(chrono::Utc::now()))),
                ],
            )
            .exec()
            .await?;
        println!("{:?}", row);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use dotenvy::dotenv;

    use crate::{create_db_pool, jobs::*};

    #[tokio::test]
    async fn test_sync_editor_recommended_list() -> surf::Result<()> {
        dotenv().ok();

        let db = create_db_pool().await;
        sync_editor_recommended_list(Arc::new(db)).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_sync_novel_statistics() -> surf::Result<()> {
        dotenv().ok();
        sync_novel_statistics().await?;
        Ok(())
    }
}
