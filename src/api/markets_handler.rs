use axum::extract::State;
use axum::response::Json;
use std::sync::Arc;

use crate::AppState;

pub async fn h_markets(State(s): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let mkts = s.markets.read().unwrap().clone();
    Json(serde_json::json!({ "markets": mkts, "total": mkts.len() }))
}

pub async fn h_books(State(s): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let books: Vec<_> = s.books.lock().unwrap().values().cloned().collect();
    Json(serde_json::json!({ "books": books }))
}

pub async fn h_heatmap(State(s): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let mkts = s.markets.read().unwrap().clone();
    let avg_vol = mkts.iter().map(|m| m.volume_24h).sum::<f64>() / mkts.len().max(1) as f64;
    let cells: Vec<_> = mkts.iter().map(|m| serde_json::json!({
        "id": m.id, "short_name": m.short_name, "question": m.question,
        "prob": m.primary_prob(), "change": m.prob_change_pct,
        "volume": m.volume_24h, "signal": m.signal,
        "is_hot": m.volume_24h > avg_vol * 1.8, "category": m.category,
        "url": m.url, "buy_pressure": m.buy_pressure,
        "outcomes": m.outcomes.iter().map(|o| serde_json::json!({
            "name": o.name, "price": o.price_cents, "mid": o.mid_cents, "spread": o.spread
        })).collect::<Vec<_>>(),
    })).collect();
    Json(serde_json::json!({ "cells": cells }))
}

pub async fn h_scanner(State(s): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let mut mkts = s.markets.read().unwrap().clone();
    mkts.sort_by(|a,b| b.prob_change_pct.abs().partial_cmp(&a.prob_change_pct.abs()).unwrap_or(std::cmp::Ordering::Equal));
    let rows: Vec<_> = mkts.iter().take(30).map(|m| serde_json::json!({
        "id": m.id, "question": m.question, "signal": m.signal,
        "prob": m.primary_prob(), "change": m.prob_change_pct,
        "volume": m.volume_24h, "category": m.category, "url": m.url,
        "buy_pressure": m.buy_pressure,
        "outcomes": m.outcomes.iter().map(|o| serde_json::json!({ "name": o.name, "price": o.price_cents, "mid": o.mid_cents })).collect::<Vec<_>>(),
    })).collect();
    Json(serde_json::json!({ "rows": rows }))
}
