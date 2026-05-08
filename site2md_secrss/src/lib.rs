use wreq::Client;
use wreq_util::Emulation;

/// 调用流程：
///
/// 第一次从 HTML 读取，此后都是 JSON.
///
/// 1. 请求 index.html, 解析 `data-lastPublishedAt`
/// 2. 此后
pub async fn fetch_index() -> String {
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

pub fn parse_index() {}
