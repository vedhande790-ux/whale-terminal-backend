use axum::extract::State;
use axum::response::Json;
use std::sync::Arc;

use crate::AppState;

pub async fn h_trades(State(s): State<Arc<AppState>>) -> Json<serde_json::Value> {
    Json(serde_json::json!({ "trades": s.recent_trades.lock().unwrap().iter().take(100).cloned().collect::<Vec<_>>() }))
}

pub async fn h_whales(State(s): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let mut p: Vec<_> = s.whale_profiles.lock().unwrap().values().cloned().collect();
    p.sort_by(|a,b| b.whale_score.partial_cmp(&a.whale_score).unwrap_or(std::cmp::Ordering::Equal));
    let lb_m = s.leaderboard_month.lock().unwrap().clone();
    let lb_a = s.leaderboard_all.lock().unwrap().clone();
    Json(serde_json::json!({ "profiles": p.iter().take(50).collect::<Vec<_>>(), "leaderboard_month": lb_m, "leaderboard_all": lb_a }))
}

pub async fn h_stats(State(s): State<Arc<AppState>>) -> Json<serde_json::Value> {
    Json(serde_json::json!({ "stats": s.stats.lock().unwrap().clone() }))
}
