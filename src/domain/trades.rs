use serde::{Deserialize, Serialize};
use chrono::Utc;

use super::markets::polymarket_url;

pub fn shorten_addr(s: &str) -> String {
    if s.len() <= 12 { s.to_string() } else { format!("{}…{}", &s[..6], &s[s.len()-4..]) }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action { BUY, SELL }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub id:             u64,
    pub ts:             i64,
    pub time:           String,
    pub wallet:         String,
    pub wallet_short:   String,
    pub pseudonym:      Option<String>,
    pub market:         String,
    pub market_slug:    String,
    pub event_slug:     String,
    pub condition_id:   String,
    pub outcome_name:   String,
    pub outcome_index:  usize,
    pub outcome_count:  usize,
    pub action:         Action,
    pub price_cents:    f64,
    pub size_usd:       f64,
    pub implied_shares: f64,
    pub tx_hash:        String,
    pub url:            String,
}

impl Trade {
    pub fn from_raw(v: &serde_json::Value, id: u64) -> Option<Trade> {
        let tx_hash = v["transactionHash"].as_str().unwrap_or("").to_string();
        let wallet = v["proxyWallet"].as_str().unwrap_or("").to_string();
        if wallet.is_empty() { return None; }

        let price  = v["price"].as_f64().unwrap_or(0.0);
        let shares = v["size"].as_f64().unwrap_or(0.0);
        let size_usd = (shares * price).max(0.0);

        let action = if v["side"].as_str().map(|s| s.to_uppercase()).as_deref() == Some("SELL") {
            Action::SELL } else { Action::BUY };
        let price_cents   = (price * 100.0).clamp(0.0, 100.0);
        let implied_shares = if price > 0.0 { size_usd / price } else { 0.0 };

        let outcome_name = v["outcome"].as_str().unwrap_or("YES").to_string();
        let market_title = v["title"].as_str().unwrap_or("?").to_string();
        let market_slug  = v["slug"].as_str().unwrap_or("").to_string();
        let event_slug   = v["eventSlug"].as_str().unwrap_or("").to_string();
        let condition_id = v["conditionId"].as_str().unwrap_or("").to_string();
        let pseudonym    = v["pseudonym"].as_str().filter(|s| !s.is_empty() && *s != "null").map(String::from);
        let ts_secs      = v["timestamp"].as_i64().unwrap_or_else(|| Utc::now().timestamp());
        let url = polymarket_url(&event_slug, &market_slug);

        Some(Trade {
            id, ts: ts_secs * 1000,
            time: Utc::now().format("%H:%M:%S").to_string(),
            wallet: wallet.clone(), wallet_short: shorten_addr(&wallet),
            pseudonym,
            market: market_title, market_slug,
            event_slug, condition_id,
            outcome_name,
            outcome_index: 0, outcome_count: 2,
            action,
            price_cents, size_usd, implied_shares,
            tx_hash, url,
        })
    }
}
