use std::sync::LazyLock;

use clap::{Args, Subcommand};
use scraper::{Html, Selector};
use serde::Serialize;
use wreq::Client;
use wreq_util::Emulation;

pub mod article;

/// 安全内参
#[derive(Debug, Args)]
pub struct Secrss {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    RawIndexHtml,
    RawIndexJson,
}

/// 调用流程：
///
/// 第一次从 HTML 读取，此后都是 JSON.
///
/// 1. 请求 index.html, 解析 `data-lastPublishedAt`
/// 2. 此后
pub async fn fetch_index_html() -> String {
    let client = Client::builder()
        .emulation(Emulation::Chrome147)
        .build()
        .unwrap();
    let response = client.get(URL).send().await.unwrap();
    response.text().await.unwrap()
}

fn _url() -> &'static str {
    "https://www.secrss.com/api/articles?lastPublishedAt=2026-04-30%2023%3A15%3A34&referer=web"
}

const URL: &str = "https://www.secrss.com/";

#[derive(Debug, Serialize)]
pub struct Entry {
    title: String,
    href: String,
    tag: String,
    author: String,
    /// Date actually.
    time: String,
}

pub fn parse_index_html(html: &str) -> Vec<Entry> {
    static LIST_ITEM: LazyLock<Selector> = LazyLock::new(|| Selector::parse(".list-item").unwrap());
    static H2_A: LazyLock<Selector> = LazyLock::new(|| Selector::parse("h2.title a").unwrap());
    static TAG: LazyLock<Selector> = LazyLock::new(|| Selector::parse(".tag").unwrap());
    static AUTHOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse(".author").unwrap());
    static TIME: LazyLock<Selector> = LazyLock::new(|| Selector::parse(".time").unwrap());

    let mut entries = vec![];

    let html = Html::parse_document(html);
    for item in html.select(&LIST_ITEM) {
        let Some(a) = item.select(&H2_A).next() else {
            continue;
        };
        let title = a.attr("title").unwrap();
        let href = a.attr("href").unwrap();

        let tag = item
            .select(&TAG)
            .next()
            .unwrap()
            .text()
            .map(str::trim)
            .collect();
        let author = item
            .select(&AUTHOR)
            .next()
            .unwrap()
            .text()
            .map(str::trim)
            .collect();
        let time = item
            .select(&TIME)
            .next()
            .unwrap()
            .text()
            .map(str::trim)
            .collect();

        entries.push(Entry {
            title: title.to_string(),
            href: href.to_string(),
            tag,
            author,
            time,
        });
    }

    entries
}

#[cfg(test)]
fn decompress_brotli(bytes: &[u8]) -> String {
    use brotli::Decompressor;
    use std::io::Read;

    let mut decompressor = Decompressor::new(bytes, 4096);

    let mut html = String::new();
    decompressor.read_to_string(&mut html).unwrap();

    html
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_index_html() {
        let bytes = include_bytes!("../samples/index.html.br");
        let html = decompress_brotli(bytes);
        let parsed = parse_index_html(&html);
        insta::assert_yaml_snapshot!(parsed);
    }
}
