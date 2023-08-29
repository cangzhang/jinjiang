use std::collections::HashMap;

use anyhow::bail;
use encoding_rs::GB18030;
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};
use regex::Regex;
use lazy_static::lazy_static;

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
    match s.parse::<i32>() {
        Ok(i) => i,
        Err(_) => {
            dbg!(s);
            0
        },
    }
}

pub fn extract_num(text: &String) -> i32 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?m)\d+").unwrap();
    }

    let m = RE.find_iter(text)
        // try to parse the string matches as i64 (inferred from fn type signature)
        // and filter out the matches that can't be parsed (e.g. if there are too many digits to store in an i64).
        .filter_map(|digits| digits.as_str().parse::<i32>().ok())
        // collect the results in to a Vec<i64> (inferred from fn type signature)
        .collect::<Vec<i32>>();
    m.get(0).unwrap_or(&0).clone()
}

pub async fn get_novel_statistics(novel_id: i32) -> anyhow::Result<NovelStatistic> {
    let novel_url = format!("https://m.jjwxc.net/book2/{novel_id}");
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
    let collected_selector = Selector::parse(r#"[itemProp="collectedCount"]"#).unwrap();
    let review_count_selector = Selector::parse(r#"[href^="/review/"]"#).unwrap();
    let rewards_selector = Selector::parse(r#"[href^="/nutrition/sendnutrition"]"#).unwrap();

    let reviews = get_element_content(doc.select(&review_count_selector).next());
    let collected = get_element_content(doc.select(&collected_selector).next());
    let rewards = get_element_content(doc.select(&rewards_selector).next());

    Ok(NovelStatistic {
        novel_id,
        first_chapter_clicks: clicks.0,
        last_chapter_clicks: clicks.1,
        reviews: extract_num(&reviews),
        collected: extract_num(&collected),
        rewards: extract_num(&rewards),
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
    let bytes = reqwest::get(url).await?.bytes().await?;
    let (cow, _, _) = GB18030.decode(&bytes);
    Ok(cow.into_owned())
}

pub async fn make_editor_recommended_list(
    list_url: String,
) -> anyhow::Result<Vec<(i32, i32, String)>> {
    // let url = "https://www.jjwxc.net/channeltopten.php?channelid=118&str=28";
    let url = format!("{list_url}&_={}", cuid::cuid2());
    let html = match get_html(&url).await {
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

fn get_element_content(el: Option<ElementRef>) -> String {
    match el {
        Some(e) => e.text().collect::<String>(),
        _ => "0".to_string(),
    }
}
