use std::env;

use scraper::{Html, Selector};
use sqlx::SqlitePool;

use crate::scrape::{get_novel_detail, get_html, Novel};

pub async fn sync_novel_statistics() -> anyhow::Result<()> {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    // let rows = sqlx::query!("SELECT id, novel_id FROM novels")
    //     .fetch_all(&pool)
    //     .await
    //     .unwrap();

    // for row in rows {
    //     let novel_id = row.novel_id;
    //     match get_novel_detail(novel_id as u64).await {
    //         Ok(novel) => {
    //             println!("{:?}", novel);
    //         }
    //         Err(_) => println!("failed for novel_id: {}", novel_id),
    //     };
    // }

    Ok(())
}

pub async fn sync_editor_recommended_list() -> surf::Result<()> {
    let url = "https://www.jjwxc.net/channeltopten.php?channelid=118&str=28";
    let html = get_html(url).await?;
    let doc = Html::parse_document(&html);
    
    let tr_selector = Selector::parse(r#"tr[onmouseover]:nth-child(n+2):nth-child(-n+21)"#).unwrap();
    let author_id_selector = Selector::parse(r#"td:nth-child(2) > a"#).unwrap();
    let title_selector = Selector::parse(r#"td:nth-child(3) > a"#).unwrap();

    let mut novels = vec![];
    for book_row in doc.select(&tr_selector) {
        let title_td = book_row.select(&title_selector).next().unwrap();
        let title = title_td.text()
            .collect::<String>();
        let novel_id = title_td.value().attr("href").unwrap().split('=').last().unwrap().parse::<u64>().unwrap();
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
            .parse::<u64>()
            .unwrap();

        novels.push(Novel {
            title,
            author_id,
            novel_id,
            ..Default::default()
        });

        db::insert_novel(&pool, &novels).await?;
    }
    dbg!(novels);

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