use crate::domain::trades::Trade;
use crate::ingestion::trades as data;
use crate::AppState;

pub const MIN_TRADE_USD: f64 = 100.0;
pub const MAX_TRADES: usize  = 500;

pub async fn fetch_trades(client: &reqwest::Client) -> Vec<serde_json::Value> {
    data::fetch_raw_trades(client).await
}

pub fn store_trade(state: &AppState, trade: Trade) {
    let mut td = state.recent_trades.lock().unwrap();
    td.push_front(trade);
    if td.len() > MAX_TRADES { td.pop_back(); }
}
