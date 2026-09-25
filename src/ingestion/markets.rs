use std::time::Duration;

const GAMMA_API: &str = "https://gamma-api.polymarket.com/markets?active=true&closed=false&archived=false&limit=100";
const GAMMA_API_TOP_VOLUME: &str = "https://gamma-api.polymarket.com/markets?active=true&closed=false&archived=false&limit=100&order=volume24hr&ascending=false";

pub async fn fetch_raw_markets(client: &reqwest::Client) -> Vec<serde_json::Value> {
    let mut raw: Vec<serde_json::Value> = vec![];
    for page in 0..5usize {
        let offset = page * 100;
        let url = format!("{}&offset={}", GAMMA_API, offset);
        let text = match client.get(&url).send().await {
            Ok(r) => r.text().await.unwrap_or_default(),
            Err(e) => { eprintln!("Gamma page {page}: {e}"); break; }
        };
        let page_raw: Vec<serde_json::Value> = serde_json::from_str(&text).unwrap_or_default();
        let page_len = page_raw.len();
        raw.extend(page_raw);
        if page_len < 100 { break; }
        tokio::time::sleep(Duration::from_millis(300)).await;
    }
    for page in 0..25usize {
        let offset = page * 100;
        let url = format!("{}&offset={}", GAMMA_API_TOP_VOLUME, offset);
        let text = match client.get(&url).send().await {
            Ok(r) => r.text().await.unwrap_or_default(),
            Err(e) => { eprintln!("Gamma vol page {page}: {e}"); break; }
        };
        let page_raw: Vec<serde_json::Value> = serde_json::from_str(&text).unwrap_or_default();
        let page_len = page_raw.len();
        raw.extend(page_raw);
        if page_len < 100 { break; }
        tokio::time::sleep(Duration::from_millis(150)).await;
    }
    raw
}
