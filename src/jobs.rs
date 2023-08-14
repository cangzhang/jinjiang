use scraper::{Html, Selector};

use crate::prisma::{novel, PrismaClient};
use crate::scrape::get_html;

pub async fn sync_novel_statistics() -> anyhow::Result<()> {
    // let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    // let rows = sqlx::query!("SELECT id, novel_id FROM novels")
    //     .fetch_all(&pool)
    //     .await
    //     .unwrap();

    // for row in rows {
    //     let novel_id = row.novel_id;
    //     match get_novel_detail(novel_id as i32).await {
    //         Ok(novel) => {
    //             println!("{:?}", novel);
    //         }
    //         Err(_) => println!("failed for novel_id: {}", novel_id),
    //     };
    // }

    Ok(())
}

pub async fn sync_editor_recommended_list() -> surf::Result<()> {
    let db = PrismaClient::_builder().build().await.unwrap();

    let url = "https://www.jjwxc.net/channeltopten.php?channelid=118&str=28";
    let html = get_html(url).await?;
    let doc = Html::parse_document(&html);
    let tr_selector =
        Selector::parse(r#"tr[onmouseover]:nth-child(n+2):nth-child(-n+21)"#).unwrap();
    let author_id_selector = Selector::parse(r#"td:nth-child(2) > a"#).unwrap();
    let title_selector = Selector::parse(r#"td:nth-child(3) > a"#).unwrap();

    for (_idx, book_row) in doc.select(&tr_selector).enumerate() {
        let title_td = book_row.select(&title_selector).next().unwrap();
        let title = title_td.text().collect::<String>();
        let novel_id = title_td
            .value()
            .attr("href")
            .unwrap()
            .split('=')
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let author_id = book_row
            .select(&author_id_selector)
            .next()
            .unwrap()
            .value()
            .attr("href")
            .unwrap()
            .split('=')
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let r = db
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
        dbg!(r);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use dotenvy::dotenv;

    use crate::jobs::*;

    #[tokio::test]
    async fn test_sync_editor_recommended_list() -> surf::Result<()> {
        dotenv().ok();
        sync_editor_recommended_list().await?;
        Ok(())
    }
}
