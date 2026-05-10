use std::sync::LazyLock;

use scraper::{Html, Selector};
use wreq::Client;
use wreq_util::Emulation;

// TODO: design the interface
// TODO: from either url or article id
pub async fn fetch_article(url: &str) -> String {
    let client = Client::builder()
        .emulation(Emulation::Chrome147)
        .build()
        .unwrap();
    let response = client.get(url).send().await.unwrap();
    response.text().await.unwrap()
}

pub fn parse_article(html: &str) -> String {
    static SUMMARY: LazyLock<Selector> = LazyLock::new(|| Selector::parse(".summary").unwrap());
    static ARTICLE_P: LazyLock<Selector> =
        LazyLock::new(|| Selector::parse(".article-body p").unwrap());

    let html = Html::parse_document(html);

    // TODO: markdown builder
    let mut markdown = String::new();

    let summary: String = html
        .select(&SUMMARY)
        .next()
        .unwrap()
        .text()
        .map(str::trim)
        .collect();
    markdown.push_str("> ");
    markdown.push_str(&summary);
    markdown.push('\n');

    let mut ps = html.select(&ARTICLE_P).peekable();
    while let Some(p) = ps.next() {
        let text: String = p.text().map(str::trim).collect();
        if !text.is_empty() {
            markdown.push('\n');
            if ps.peek().is_none() {
                markdown.push_str("> ");
            }
            markdown.push_str(&text);
            markdown.push('\n');
        }
    }

    markdown
}

#[cfg(test)]
mod tests {
    use crate::decompress_brotli;

    use super::*;

    #[test]
    fn test_parse_index_html() {
        let bytes = include_bytes!("../samples/articles/90141/index.html.br");
        let html = decompress_brotli(bytes);
        let parsed = parse_article(&html);
        insta::assert_snapshot!(parsed);
    }
}
