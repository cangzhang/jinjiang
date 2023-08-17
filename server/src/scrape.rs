use std::collections::HashMap;

use anyhow::bail;
use encoding::{all::GBK, Encoding, DecoderTrap};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Novel {
    pub id: i32,
    pub novel_id: i32,
    pub title: String,
    pub author_id: i32,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct NovelStatistic {
    pub id: i32,
    pub novel_id: i32,
    pub reviews: i32,
    pub collected: i32,
    pub first_chapter_clicks: i32,
    pub last_chapter_clicks: i32,
    pub rewards: i32,
    pub updated_at: String,
}

pub fn to_i32(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}

pub async fn get_novel_statistics(novel_id: i32) -> anyhow::Result<NovelStatistic> {
    let novel_url = format!("https://www.jjwxc.net/onebook.php?novelid={novel_id}");
    let (html_body, clicks_resp) =
        futures::join!(get_html(&novel_url), get_chapter_clicks(novel_id));
    let html = if let Ok(b) = html_body {
        b
    } else {
        return Err(html_body.unwrap_err());
    };
    let clicks = if let Ok(c) = clicks_resp {
        c
    } else {
        return Err(clicks_resp.unwrap_err());
    };

    let doc = Html::parse_document(&html);
    let review_count_selector = Selector::parse(r#"[itemProp="reviewCount"]"#).unwrap();
    let collected_selector = Selector::parse(r#"[itemProp="collectedCount"]"#).unwrap();
    let rewards_selector = Selector::parse(r#"[itemProp="collectedCount"] + span"#).unwrap();
    let reviews = doc
        .select(&review_count_selector)
        .next()
        .unwrap()
        .text()
        .collect::<String>();
    let collected = doc
        .select(&collected_selector)
        .next()
        .unwrap()
        .text()
        .collect::<String>();
    let rewards = doc
        .select(&rewards_selector)
        .next()
        .unwrap()
        .text()
        .collect::<String>();

    let title_selector = Selector::parse(r#"[itemProp="articleSection"]"#).unwrap();
    let _title = doc
        .select(&title_selector)
        .next()
        .unwrap()
        .text()
        .collect::<String>();

    Ok(NovelStatistic {
        novel_id,
        reviews: to_i32(&reviews),
        collected: to_i32(&collected),
        first_chapter_clicks: clicks.0,
        last_chapter_clicks: clicks.1,
        rewards: to_i32(&rewards),
        ..Default::default()
    })
}

pub async fn get_chapter_clicks(novel_id: i32) -> anyhow::Result<(i32, i32)> {
    let clicks_url = format!(
        "https://s8-static.jjwxc.net/getnovelclick.php?novelid={novel_id}&jsonpcallback=novelclick"
    );
    let body = reqwest::get(clicks_url).await?.text().await?;
    let click_map_str = body.replace("novelclick(", "").replace(')', "");
    if click_map_str.eq("[]") {
        bail!("clicks is empty")
    }

    let click_map = serde_json::from_str::<HashMap<String, String>>(&click_map_str).unwrap();
    let mut sorted: Vec<(&String, &String)> = click_map.iter().collect();
    sorted.sort_by_key(|a| a.0.parse::<i32>().unwrap());
    let len = sorted.len();
    let first = to_i32(sorted[0].1);
    let last = to_i32(sorted[len - 1].1);

    Ok((first, last))
}

pub async fn get_html(url: &str) -> anyhow::Result<String> {
    let buf = reqwest::get(url).await?.bytes().await?;
    let utf8_str = GBK.decode(&buf, DecoderTrap::Ignore).unwrap();
    Ok(utf8_str)
}

pub async fn make_editor_recommended_list(
    list_url: String,
) -> anyhow::Result<Vec<(i32, i32, String)>> {
    // let url = "https://www.jjwxc.net/channeltopten.php?channelid=118&str=28";
    let html = match get_html(&list_url).await {
        Ok(h) => h,
        Err(_) => {
            bail!("fetch {list_url} failed")
        }
    };
    let doc = Html::parse_document(&html);
    let tr_selector =
        Selector::parse(r#"tr[onmouseover]:nth-child(n+2):nth-child(-n+21)"#).unwrap();
    let author_id_selector = Selector::parse(r#"td:nth-child(2) > a"#).unwrap();
    let title_selector = Selector::parse(r#"td:nth-child(3) > a"#).unwrap();

    let mut novels = vec![];
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

        novels.push((novel_id, author_id, title));
    }

    Ok(novels)
}
