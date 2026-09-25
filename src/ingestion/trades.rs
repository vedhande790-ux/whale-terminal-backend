pub const DATA_TRADES: &str = "https://data-api.polymarket.com/trades?limit=100&takerOnly=false";

pub async fn fetch_raw_trades(client: &reqwest::Client) -> Vec<serde_json::Value> {
    let resp = match client.get(DATA_TRADES).send().await {
        Ok(r) => r,
        Err(e) => { eprintln!("Trades fetch: {e}"); return vec![]; }
    };
    match resp.json().await {
        Ok(v) => v,
        Err(e) => { eprintln!("Trades parse: {e}"); vec![] }
    }
}
