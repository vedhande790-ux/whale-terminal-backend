use axum::{extract::{State, Json}, response::IntoResponse};
use chrono::Utc;
use serde::Deserialize;
use std::sync::Arc;

use crate::AppState;

pub async fn h_health(State(s): State<Arc<AppState>>) -> impl IntoResponse {
    let stats = s.stats.lock().unwrap().clone();
    Json(serde_json::json!({
        "status": "healthy", "version": "4.0.0", "source": "polymarket-live",
        "markets": s.markets.read().unwrap().len(),
        "trades_seen": stats.total_trades_seen, "alpha_wallets": stats.alpha_wallet_count,
        "signals_today": stats.signals_fired_today, "ts": Utc::now().timestamp_millis(),
    }))
}

pub async fn h_signals(State(s): State<Arc<AppState>>) -> impl IntoResponse {
    Json(serde_json::json!({ "signals": s.signals.lock().unwrap().clone() }))
}

pub async fn h_edge_signals(State(s): State<Arc<AppState>>) -> impl IntoResponse {
    let sigs: Vec<_> = s.edge_signals.lock().unwrap().iter().cloned().collect();
    Json(serde_json::json!({ "signals": sigs }))
}

#[derive(Deserialize)]
pub struct ThresholdParams { pub threshold: f64 }

pub async fn h_set_threshold(State(s): State<Arc<AppState>>, Json(p): Json<ThresholdParams>) -> impl IntoResponse {
    let threshold = p.threshold.max(100.0);
    *s.whale_threshold.lock().unwrap() = threshold;
    *s.alert_min_size.lock().unwrap() = threshold;
    Json(serde_json::json!({ "ok": true, "threshold": threshold }))
}
