use site2md_secrss::fetch_index;

#[tokio::main]
async fn main() {
    let out = fetch_index().await;
    println!("{}", out);
}
