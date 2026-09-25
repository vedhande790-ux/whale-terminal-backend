use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

use crate::domain::markets::Market;
use crate::ingestion::markets as gamma;
use crate::AppState;

pub async fn refresh_markets(state: &Arc<AppState>, client: &reqwest::Client) {
    println!("📊  Reloading markets from Gamma API…");
    let mut raw = gamma::fetch_raw_markets(client).await;

    let mut seen = HashSet::new();
    raw.retain(|v| seen.insert(v["conditionId"].as_str().unwrap_or("").to_string()));
    if raw.is_empty() { eprintln!("Gamma empty"); return; }

    let mut markets: Vec<Market> = raw.iter().enumerate()
        .filter_map(|(i, v)| Market::from_gamma(v, i))
        .collect();

    let now = chrono::Utc::now();
    markets.retain(|m| m.is_live(now));

    if markets.is_empty() { eprintln!("No markets parsed"); return; }
    println!("✅  {} markets loaded", markets.len());

    let mut amap = HashMap::new();
    for (mi, m) in markets.iter().enumerate() {
        for (oi, o) in m.outcomes.iter().enumerate() {
            amap.insert(o.token_id.clone(), (mi, oi));
        }
    }

    let count = markets.len();
    let vol: f64 = markets.iter().map(|m| m.volume_24h).sum();
    *state.markets.write().unwrap() = markets;
    *state.asset_map.write().unwrap() = amap;
    let mut s = state.stats.lock().unwrap();
    s.open_markets = count;
    s.total_volume_24h = vol;
}

pub async fn task_refresh_markets(state: Arc<AppState>, client: reqwest::Client) {
    let mut iv = tokio::time::interval(Duration::from_secs(300));
    loop {
        iv.tick().await;
        refresh_markets(&state, &client).await;
    }
}
