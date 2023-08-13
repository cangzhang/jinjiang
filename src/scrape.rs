use std::collections::HashMap;

use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Novel {
    pub id: u64,
    pub novel_id: u64,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct NovelDetail {
    pub novel_id: u64,
    pub title: String,
    pub author: u64,
    pub reviews: u64,
    pub collected: u64,
    pub first_chapter_clicks: u64,
    pub last_chapter_clicks: u64,
}

pub fn to_u64(s: &str) -> u64 {
    s.parse::<u64>().unwrap()
}

pub async fn get_novel_detail(novel_id: u64) -> surf::Result<NovelDetail> {
    let novel_url = format!("https://www.jjwxc.net/onebook.php?novelid={novel_id}");
    let (html_resp, clicks_resp) = futures::join!(surf::get(novel_url), get_chapter_clicks(novel_id));
    let body = if let Ok(mut resp) = html_resp {
        resp.body_bytes().await?
    } else {
        return Err(html_resp.unwrap_err());
    };
    let clicks = if let Ok(c) = clicks_resp {
        c
    } else {
        return Err(clicks_resp.unwrap_err());
    };

    let html = String::from_utf8_lossy(&body);
    let doc = Html::parse_document(&html);

    let review_count_selector = Selector::parse(r#"[itemProp="reviewCount"]"#).unwrap();
    let collected_selector = Selector::parse(r#"[itemProp="collectedCount"]"#).unwrap();
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

    Ok(NovelDetail {
        novel_id,
        reviews: to_u64(&reviews),
        collected: to_u64(&collected),
        first_chapter_clicks: clicks.0,
        last_chapter_clicks: clicks.1,
        ..Default::default()
    })
}

pub async fn get_chapter_clicks(novel_id: u64) -> surf::Result<(u64, u64)> {
    let clicks_url = format!("https://s8-static.jjwxc.net/getnovelclick.php?novelid={novel_id}&jsonpcallback=novelclick");
    let body = surf::get(clicks_url).recv_string().await?;
    let click_map_str = body.replace("novelclick(", "").replace(')', "");
    let click_map = serde_json::from_str::<HashMap<String, String>>(&click_map_str).unwrap();

    let mut sorted: Vec<(&String, &String)> = click_map.iter().collect();
    sorted.sort_by_key(|a| a.0.parse::<u64>().unwrap());
    let len = sorted.len();
    let first = to_u64(sorted[0].1);
    let last = to_u64(sorted[len - 1].1);

    Ok((first, last))
}
