#![allow(unused, dead_code)]

use scraper::{Html, Selector};

fn main() {
    let sel_post = Selector::parse("div.post").unwrap();
    let sel_title = Selector::parse("h2.post-title").unwrap();
    let sel_time = Selector::parse("time").unwrap();
    let sel_link = Selector::parse("a").unwrap();

    let html = include_str!("../html/nick.dev.html");
    let html = Html::parse_document(html);
    for post in html.select(&sel_post) {
        let title = post
            .select(&sel_title)
            .next()
            .unwrap()
            .text()
            .collect::<String>();
        let time = post
            .select(&sel_time)
            .next()
            .unwrap()
            .value()
            .attr("datetime")
            .unwrap();
        let link = post
            .select(&sel_link)
            .next()
            .unwrap()
            .value()
            .attr("href")
            .unwrap();
        println!("{title}\n{time}\n{link}\n");
    }
}
