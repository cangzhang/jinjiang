use std::sync::Arc;

use crate::prisma::{list_links, novel, novel_statistics, PrismaClient};
use crate::scrape::get_novel_statistics;
use crate::{create_db_pool, scrape};

pub async fn sync_novel_statistics() -> anyhow::Result<()> {
    let db = create_db_pool().await;

    let rows: Vec<novel::Data> = db.novel().find_many(vec![novel::in_list::equals(true)]).exec().await?;
    for row in rows {
        let novel_id = row.novel_id;
        match get_novel_statistics(novel_id).await {
            Ok(novel) => {
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

pub async fn sync_book_list(db: Arc<PrismaClient>) -> anyhow::Result<()> {
    let list_urls: Vec<list_links::Data> = db.list_links().find_many(vec![]).exec().await?;
    let mut novel_ids = vec![];
    for page_item in list_urls {
        let list = scrape::make_editor_recommended_list(page_item.link).await?;
        for (novel_id, author_id, title) in list {
            novel_ids.push(novel_id);
            let row = db
                .novel()
                .upsert(
                    novel::novel_id::equals(novel_id),
                    novel::create(
                        title.clone(),
                        novel_id,
                        author_id,
                        page_item.name.clone(),
                        vec![
                            novel::in_list::set(true),
                        ],
                    ),
                    vec![
                        novel::title::set(title.clone()),
                        novel::author_id::set(author_id),
                        novel::list_name::set(page_item.name.clone()),
                        novel::in_list::set(true),
                        novel::updated_at::set(Some(chrono::DateTime::from(chrono::Utc::now()))),
                    ],
                )
                .exec()
                .await?;
            println!("{:?}", row);
        }
    }
    let _ = db
        .novel()
        .update_many(
            vec![novel::novel_id::not_in_vec(novel_ids)],
            vec![
                novel::in_list::set(false),
                novel::updated_at::set(Some(chrono::DateTime::from(chrono::Utc::now()))),
            ],
        )
        .exec()
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use dotenvy::dotenv;

    use crate::{create_db_pool, jobs::*};

    #[tokio::test]
    async fn test_sync_book_list() -> anyhow::Result<()> {
        dotenv().ok();

        let db = create_db_pool().await;
        sync_book_list(Arc::new(db)).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_sync_novel_statistics() -> anyhow::Result<()> {
        dotenv().ok();
        sync_novel_statistics().await?;
        Ok(())
    }
}
