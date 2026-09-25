// ═══════════════════════════════════════════════════════════════════════════════
//  WHALE.TERMINAL v4.0  —  Decision Engine for Polymarket
//
//  Signal Engine (8 signals):
//    1. SMART CLUSTER     — 3+ alpha wallets enter same market in 15 min
//    2. VELOCITY SURGE    — 1-min volume 4x vs 60-min baseline
//    3. STEALTH ACCUM     — Large whale repeated buys at stable price
//    4. LIQUIDITY DRAIN   — Order book thinning on ask side (imminent move)
//    5. PROB DIVERGENCE   — Price moving against whale flow (reversion edge)
//    6. WHALE REVERSAL    — Top wallet flips from prior position
//    7. CONVICTION SPIKE  — Single wallet size 3x+ their own avg
//    8. MOMENTUM BREAK    — Prob crosses key level with volume confirmation
//
//  Wallet Intelligence:
//    - Whale Score formula: win_rate×0.35 + roi×0.30 + consistency×0.20 + vol×0.15
//    - Rolling 30-trade window for all metrics
//    - Market specialization tracking
//
//  cargo run  →  http://localhost:8080
// ═══════════════════════════════════════════════════════════════════════════════
#![allow(dead_code)]

mod domain;
mod ingestion;
mod application;
mod api;
mod engine;

use axum::{
    extract::{
        ws::{Message as WsMsg, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use chrono::Utc;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::{Arc, Mutex, RwLock},
    time::Duration,
};
use tokio::sync::broadcast;
use tokio_tungstenite::{connect_async, tungstenite::Message as TungMsg};
use tower_http::cors::{Any, CorsLayer};
use domain::books::{Level, OutcomeBook, MarketBook};
use domain::markets::Market;
use domain::signals::{EdgeSignal, SignalAccuracy, GlobalSignals, Stats, EdgeFeedEvent};
use domain::trades::{Trade, Action, shorten_addr};
use domain::whales::{WhaleProfile, TradeWindow, is_whale_trade, WHALE_USD, LeaderboardEntry};
use application::trade_service::{MIN_TRADE_USD, MAX_TRADES};

// ─── API endpoints ─────────────────────────────────────────────────────────────
const DATA_LB: &str    = "https://data-api.polymarket.com/v1/leaderboard?limit=25&timePeriod=MONTH";
const DATA_LB_ALL: &str= "https://data-api.polymarket.com/v1/leaderboard?limit=25&timePeriod=ALL";
const CLOB_BOOKS: &str = "https://clob.polymarket.com/books";
const CLOB_MID: &str   = "https://clob.polymarket.com/midpoint";
const CLOB_SPREAD: &str= "https://clob.polymarket.com/spread";
const CLOB_WS: &str    = "wss://ws-subscriptions-clob.polymarket.com/ws/market";

const BROADCAST_CAP: usize = 4096;
const TRIAL_SECS: u64      = 300;
const SIGNAL_DEDUP_MS: i64 = 300_000; // 5 min dedup window per signal id

// ─── Utility functions ─────────────────────────────────────────────────────────

fn classify_signal(change: f64, vol: f64, avg_vol: f64) -> &'static str {
    match () {
        _ if vol > avg_vol * 2.5 && change.abs() > 3.0 => "BREAKOUT",
        _ if vol > avg_vol * 1.8                        => "HOT",
        _ if change > 1.5                               => "BULL",
        _ if change < -1.5                              => "BEAR",
        _                                               => "NEUTRAL",
    }
}

// ─── Core models ───────────────────────────────────────────────────────────────


// ─── Edge Feed Computation (Lightweight Confluence) ─────────────────────────
fn compute_edge_score(buy_recent: usize, sell_recent: usize, liquidity_present: bool, composite: f64) -> (f64, String, String, Vec<String>, String) {
    let mut score = 0.0f64;
    let dir = if buy_recent > sell_recent { "BULLISH" } else if sell_recent > buy_recent { "BEARISH" } else { "NEUTRAL" };
    if buy_recent > sell_recent { score += 4.0; } else if sell_recent > buy_recent { score -= 4.0; }
    if liquidity_present { score += 1.5; }
    if composite > 60.0 { score += 1.5; }
    if score > 10.0 { score = 10.0; }

    let strength = match score as i32 {
        0..=3 => "WEAK",
        4..=5 => "MODERATE",
        6..=7 => "STRONG",
        _ => "VERY STRONG",
    }.to_string();

    let mut reasons = Vec::new();
    if buy_recent > sell_recent { reasons.push("Whale accumulation".to_string()); } else if sell_recent > buy_recent { reasons.push("Whale distribution".to_string()); }
    if liquidity_present { reasons.push("Liquidity event".to_string()); }
    if composite > 60.0 { reasons.push("Positive sentiment".to_string()); }

    let execution = if score >= 8.0 { "EXECUTE" } else if score >= 6.0 { "PREPARE" } else { "WAIT" };
    (score, dir.to_string(), strength, reasons, execution.to_string())
}

fn compute_edge_feed(state: &AppState) -> Option<EdgeFeedEvent> {
    let now_ms = Utc::now().timestamp_millis();

    let trades: Vec<Trade> = state.recent_trades.lock().unwrap().iter().cloned().collect();
    let mut buy_recent = 0usize;
    let mut sell_recent = 0usize;
    for t in trades.iter().rev().take(60) {
        if (now_ms - t.ts) < 60_000 && is_whale_trade(t, WHALE_USD) {
            match t.action {
                Action::BUY => buy_recent += 1,
                Action::SELL => sell_recent += 1,
            }
        }
    }

    let liquidity_present = state.edge_signals.lock().unwrap().iter().rev().take(20)
        .any(|e| e.priority == "HIGH" || e.priority == "CRITICAL");

    let composite = state.signals.lock().unwrap().composite as f64;

    let (score, dir, strength, reasons, execution) = compute_edge_score(buy_recent, sell_recent, liquidity_present, composite);
    if score.abs() < 6.0 { return None; }

    Some(EdgeFeedEvent { edge_score: score, direction: dir, strength, reasons, execution, ts: now_ms })
}

// ─── Edge Feed Publisher ───────────────────────────────────────────────────
async fn task_edge_feed(state: Arc<AppState>) {
    let mut interval = tokio::time::interval(Duration::from_secs(2));
    loop {
        interval.tick().await;
        if let Some(ev) = compute_edge_feed(&state) {
            let _ = state.tx.send(Ev::EdgeFeed(ev));
        }
    }
}

// ─── WS Events ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Ev {
    Trade(Trade),
    Book(MarketBook),
    PriceUpdate {
        condition_id:  String,
        market_id:     usize,
        outcome_idx:   usize,
        price_cents:   f64,
        mid_cents:     f64,
        spread:        f64,
        prob_change:   f64,
        volume_24h:    f64,
        signal:        String,
    },
    Signals(GlobalSignals),
    Stats(Stats),
    WhaleAlert {
        wallet:        String,
        wallet_short:  String,
        pseudonym:     Option<String>,
        market:        String,
        outcome:       String,
        action:        Action,
        size_usd:      f64,
        price_cents:   f64,
        url:           String,
        tag:           String,
        whale_score:   f64,
        is_biggest:    bool,
        is_reversal:   bool,
    },
    WhaleUpdate(WhaleProfile),
    EdgeSignal(EdgeSignal),
    EdgeFeed(EdgeFeedEvent),
    LeaderboardUpdate(Vec<LeaderboardEntry>),
    Heartbeat { ts: i64, trial_remaining_secs: i64 },
    TrialExpired,
}

use domain::books::RawBook;

// ─── App State ─────────────────────────────────────────────────────────────────

pub struct AppState {
    pub tx:                broadcast::Sender<Ev>,
    pub http_client:       reqwest::Client,
    pub markets:           RwLock<Vec<Market>>,
    pub recent_trades:     Mutex<VecDeque<Trade>>,
    pub books:             Mutex<HashMap<String, MarketBook>>,
    raw_books:             Mutex<HashMap<String, RawBook>>,
    pub whale_profiles:    Mutex<HashMap<String, WhaleProfile>>,
    pub leaderboard_month: Mutex<Vec<LeaderboardEntry>>,
    pub leaderboard_all:   Mutex<Vec<LeaderboardEntry>>,
    pub signals:           Mutex<GlobalSignals>,
    pub stats:             Mutex<Stats>,
    pub edge_signals:      Mutex<VecDeque<EdgeSignal>>,
    pub trade_counter:     Mutex<u64>,
    seen_hashes:           Mutex<SeenSet>,
    pub(crate) asset_map:             RwLock<HashMap<String, (usize, usize)>>,
    mkt_signal_state:      Mutex<HashMap<String, engine::state::MarketSignalState>>,
    signal_dedup:          Mutex<engine::state::SignalDedup>,
    // Per-wallet prev action for reversal detection
    wallet_prev_action:    Mutex<HashMap<String, (String, Action)>>, // wallet → (market_slug, action)
    // Signal accuracy tracking: kind → (total_fired, sum_conf)
    signal_accuracy_map:   Mutex<HashMap<String, (u32, u64)>>,
    // Alert thresholds (configurable via WS)
    pub alert_min_size:    Mutex<f64>,
    pub alert_whale_count: Mutex<u32>,
    pub alert_window_secs: Mutex<u64>,
    pub alert_sound:       Mutex<bool>,
    pub whale_threshold:  Mutex<f64>,
    // Trial tracking: device_id → first_seen unix timestamp (secs)
    trial_registry:       Mutex<HashMap<String, i64>>,
}


#[derive(Default)]
struct SeenSet { set: HashSet<String>, q: VecDeque<String> }
impl SeenSet {
    fn check_insert(&mut self, k: String) -> bool {
        if self.set.contains(&k) { return false; }
        self.set.insert(k.clone()); self.q.push_back(k);
        while self.q.len() > 3000 { if let Some(o) = self.q.pop_front() { self.set.remove(&o); } }
        true
    }
}

impl AppState {
    fn new(tx: broadcast::Sender<Ev>) -> Self {
        Self {
            tx,
            http_client:       reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap(),
            markets:           RwLock::new(vec![]),
            recent_trades:     Mutex::new(VecDeque::new()),
            books:             Mutex::new(HashMap::new()),
            raw_books:         Mutex::new(HashMap::new()),
            whale_profiles:    Mutex::new(HashMap::new()),
            leaderboard_month: Mutex::new(vec![]),
            leaderboard_all:   Mutex::new(vec![]),
            signals:           Mutex::new(GlobalSignals {
                momentum: 50, volume: 0, sentiment: 50, whale_flow: 0,
                whale_bias: 0.0, composite: 50,
                recommendation: "AWAITING DATA".into(),
                rec_probability: 0,
                rec_detail: String::new(),
                rec_signal_type: String::new(),
                confidence_score: 0,
                signal_accuracy: vec![],
                top_market: "—".into(), hottest_outcome: "—".into(),
                whale_follow_yield: 0.0,
            }),
            stats:             Mutex::new(Stats {
                total_volume_24h: 0.0, total_volume_ever: 0.0,
                active_wallets: 0, open_markets: 0,
                total_trades_seen: 0, biggest_trade: 0.0,
                buy_sell_ratio: 1.0, whale_count: 0,
                signals_fired_today: 0, alpha_wallet_count: 0,
                whale_follow_yield: 0.0,
            }),
            edge_signals:      Mutex::new(VecDeque::new()),
            trade_counter:     Mutex::new(0),
            seen_hashes:       Mutex::new(SeenSet::default()),
            asset_map:         RwLock::new(HashMap::new()),
            mkt_signal_state:  Mutex::new(HashMap::new()),
            signal_dedup:      Mutex::new(engine::state::SignalDedup::default()),
            wallet_prev_action: Mutex::new(HashMap::new()),
            signal_accuracy_map: Mutex::new(HashMap::new()),
            alert_min_size:    Mutex::new(5_000.0),
            alert_whale_count: Mutex::new(2),
            alert_window_secs: Mutex::new(90),
            alert_sound:       Mutex::new(false),
            whale_threshold:  Mutex::new(5_000.0),
            trial_registry:   Mutex::new(HashMap::new()),
        }
    }

    fn fire_edge_signal(&self, sig: EdgeSignal) {
        self.stats.lock().unwrap().signals_fired_today += 1;
        // Track accuracy stats per signal kind
        {
            let mut acc = self.signal_accuracy_map.lock().unwrap();
            let entry = acc.entry(sig.kind.clone()).or_insert((0, 0));
            entry.0 += 1;
            entry.1 += sig.confidence as u64;
        }
        let mut q = self.edge_signals.lock().unwrap();
        q.push_front(sig.clone());
        if q.len() > 100 { q.pop_back(); }
        let _ = self.tx.send(Ev::EdgeSignal(sig));
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SIGNAL ENGINE — 8 signals, called after every trade ingested
// ═══════════════════════════════════════════════════════════════════════════════

fn run_signals_on_trade(state: &Arc<AppState>, trade: &Trade) {
    let whale_threshold = *state.whale_threshold.lock().unwrap();
    let profiles = state.whale_profiles.lock().unwrap();
    let recent_trades = state.recent_trades.lock().unwrap();
    let markets = state.markets.read().unwrap();
    let raw_books = state.raw_books.lock().unwrap();
    let mut mkt_signal_state = state.mkt_signal_state.lock().unwrap();
    let mut signal_dedup = state.signal_dedup.lock().unwrap();
    let mut wallet_prev_action = state.wallet_prev_action.lock().unwrap();

    engine::run_signals_on_trade(
        trade,
        &profiles,
        &recent_trades,
        &markets,
        &raw_books,
        &mut mkt_signal_state,
        &mut signal_dedup,
        &mut wallet_prev_action,
        whale_threshold,
        &|sig| state.fire_edge_signal(sig),
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
//  BACKGROUND TASKS
// ═══════════════════════════════════════════════════════════════════════════════


// ─── Task: Real trades from Data API ──────────────────────────────────────────

fn resolve_outcome(
    asset: &str,
    condition_id: &str,
    outcome_name: &str,
    asset_map: &std::collections::HashMap<String, (usize, usize)>,
    markets: &[Market],
) -> (usize, usize) {
    if let Some(&(mi, oi)) = asset_map.get(asset) {
        let cnt = markets.get(mi).map(|m| m.outcomes.len()).unwrap_or(2);
        return (oi, cnt);
    }
    if !condition_id.is_empty() {
        if let Some(mi) = markets.iter().position(|m| m.condition_id == condition_id) {
            return (0, markets[mi].outcomes.len());
        }
    }
    let idx = match outcome_name.to_uppercase().as_str() {
        "YES" => 0, "NO" => 1, _ => 0,
    };
    (idx, 2)
}

fn update_market_price(
    trade: &Trade,
    asset: &str,
    state: &AppState,
) {
    let mkt_id_opt = {
        let amap = state.asset_map.read().unwrap();
        amap.get(asset).copied().or_else(|| {
            if !trade.condition_id.is_empty() {
                state.markets.read().unwrap().iter()
                    .position(|m| m.condition_id == trade.condition_id)
                    .map(|mi| (mi, 0))
            } else { None }
        })
    };

    if let Some((mi, oi)) = mkt_id_opt {
        let old_price = state.markets.read().unwrap().get(mi)
            .and_then(|m| m.outcomes.get(oi)).map(|o| o.price_cents).unwrap_or(50.0);
        let mut mkts = state.markets.write().unwrap();
        if let Some(m) = mkts.get_mut(mi) {
            if let Some(o) = m.outcomes.get_mut(oi) {
                o.price_cents = trade.price_cents;
                o.last_trade = trade.price_cents;
            }
            m.prob_change_pct = trade.price_cents - old_price;
            let avg_vol = mkts.iter().map(|x| x.volume_24h).sum::<f64>() / mkts.len().max(1) as f64;
            if let Some(m2) = mkts.get_mut(mi) {
                m2.signal = classify_signal(m2.prob_change_pct, m2.volume_24h, avg_vol).into();
                let weight = (trade.size_usd / 50_000.0).clamp(0.005, 0.05);
                if trade.action == Action::BUY {
                    m2.buy_pressure = (m2.buy_pressure * (1.0 - weight) + weight).min(1.0);
                } else {
                    m2.buy_pressure = (m2.buy_pressure * (1.0 - weight)).max(0.0);
                }
            }
            let (sig, vol) = if mi < mkts.len() {
                (mkts[mi].signal.clone(), mkts[mi].volume_24h)
            } else { ("NEUTRAL".into(), 0.0) };
            let _ = state.tx.send(Ev::PriceUpdate {
                condition_id: trade.condition_id.clone(), market_id: mi, outcome_idx: oi,
                price_cents: trade.price_cents, mid_cents: trade.price_cents, spread: 0.0,
                prob_change: trade.price_cents - old_price, volume_24h: vol, signal: sig,
            });
        }
    }
}

fn update_whale_profile(
    trade: &Trade,
    state: &AppState,
) -> WhaleProfile {
    let mut profiles = state.whale_profiles.lock().unwrap();
    let pnl_this_trade = 0.0_f64;
    let p = profiles.entry(trade.wallet.clone()).or_insert_with(|| WhaleProfile {
        wallet: trade.wallet.clone(), wallet_short: shorten_addr(&trade.wallet),
        pseudonym: trade.pseudonym.clone(),
        total_trades: 0, total_volume: 0.0,
        buy_volume: 0.0, sell_volume: 0.0,
        dominant_action: "BUYER".into(),
        favourite_market: trade.market.clone(),
        favourite_outcome: trade.outcome_name.clone(),
        whale_score: 50.0, win_rate: 50.0, avg_roi: 0.0,
        consistency: 50.0, specialization: 0.0,
        conviction_score: 0, whale_tag: "NEW PLAYER".into(),
        last_seen: trade.time.clone(), pnl_proxy: 0.0, active_bets: 0,
        window: TradeWindow::default(),
    });
    p.total_trades += 1;
    p.total_volume += trade.size_usd;
    match trade.action {
        Action::BUY => p.buy_volume += trade.size_usd,
        Action::SELL => p.sell_volume += trade.size_usd,
    }
    p.dominant_action = if p.buy_volume >= p.sell_volume { "BUYER".into() } else { "SELLER".into() };
    p.favourite_outcome = trade.outcome_name.clone();
    p.last_seen = trade.time.clone();
    p.pseudonym = trade.pseudonym.clone().or_else(|| p.pseudonym.clone());
    p.pnl_proxy += pnl_this_trade;
    p.window.push(trade.size_usd, pnl_this_trade, &trade.market_slug);
    p.conviction_score = ((p.conviction_score as f64 * 0.87) + (trade.size_usd / 800.0).min(13.0)) as u8;
    p.recompute();
    p.clone()
}

fn update_stats(
    trade: &Trade,
    state: &AppState,
) {
    let mut s = state.stats.lock().unwrap();
    s.total_trades_seen += 1;
    if trade.size_usd > s.biggest_trade { s.biggest_trade = trade.size_usd; }
    let profiles = state.whale_profiles.lock().unwrap();
    s.whale_count = profiles.values().filter(|p| p.total_volume >= WHALE_USD).count();
    s.alpha_wallet_count = profiles.values().filter(|p| p.whale_score >= 70.0).count();
    s.active_wallets = profiles.len();
    drop(profiles);
    let trades = state.recent_trades.lock().unwrap();
    let (bv, sv) = trades.iter().fold((0.0_f64, 0.0_f64), |(b, s), t| match t.action {
        Action::BUY => (b + t.size_usd, s),
        Action::SELL => (b, s + t.size_usd),
    });
    s.buy_sell_ratio = if sv > 0.0 { bv / sv } else { 1.0 };
}

fn broadcast_trade_events(
    trade: &Trade,
    profile: &WhaleProfile,
    state: &AppState,
) {
    let _ = state.tx.send(Ev::Trade(trade.clone()));
    let _ = state.tx.send(Ev::WhaleUpdate(profile.clone()));

    if is_whale_trade(trade, WHALE_USD) {
        let (tag, ws, is_rev, biggest) = {
            let profiles = state.whale_profiles.lock().unwrap();
            let tg = profiles.get(&trade.wallet).map(|p| p.whale_tag.clone()).unwrap_or_else(|| "WHALE".into());
            let ws = profiles.get(&trade.wallet).map(|p| p.whale_score).unwrap_or(50.0);
            let big = state.stats.lock().unwrap().biggest_trade == trade.size_usd;
            (tg, ws, false, big)
        };
        let _ = state.tx.send(Ev::WhaleAlert {
            wallet: trade.wallet.clone(), wallet_short: shorten_addr(&trade.wallet),
            pseudonym: trade.pseudonym.clone(), market: trade.market.clone(),
            outcome: trade.outcome_name.clone(), action: trade.action.clone(),
            size_usd: trade.size_usd, price_cents: trade.price_cents,
            url: trade.url.clone(), tag, whale_score: ws,
            is_biggest: biggest, is_reversal: is_rev,
        });
    }

    let s = state.stats.lock().unwrap().clone();
    let _ = state.tx.send(Ev::Stats(s));
}

async fn task_data_trades(state: Arc<AppState>, client: reqwest::Client) {
    let mut iv = tokio::time::interval(Duration::from_secs(2));
    loop {
        iv.tick().await;
        let resp = match client.get(ingestion::trades::DATA_TRADES).send().await { Ok(r)=>r, Err(_)=>continue };
        let raw: Vec<serde_json::Value> = match resp.json().await { Ok(v)=>v, Err(_)=>continue };

        for v in raw.into_iter().rev() {
            // ── Dedup ────────────────────────────────────────────────────────
            let tx_hash = v["transactionHash"].as_str().unwrap_or("").to_string();
            let asset   = v["asset"].as_str().unwrap_or("").to_string();
            if !state.seen_hashes.lock().unwrap().check_insert(format!("{tx_hash}:{asset}")) { continue; }

            // ── Parse raw JSON → Trade struct ────────────────────────────────
            let trade_id = { let mut c = state.trade_counter.lock().unwrap(); *c += 1; *c };
            let mut trade = match Trade::from_raw(&v, trade_id) {
                Some(t) => t, None => continue,
            };
            if trade.size_usd < MIN_TRADE_USD { continue; }

            // ── Resolve outcome index ────────────────────────────────────────
            let (outcome_index, outcome_count) = {
                let amap = state.asset_map.read().unwrap();
                let mkts = state.markets.read().unwrap();
                resolve_outcome(&asset, &trade.condition_id, &trade.outcome_name, &amap, &mkts)
            };
            trade.outcome_index = outcome_index;
            trade.outcome_count = outcome_count;

            // ── Update market live price ─────────────────────────────────────
            update_market_price(&trade, &asset, &state);

            // ── Update whale profile ─────────────────────────────────────────
            let profile = update_whale_profile(&trade, &state);

            // ── Store trade ──────────────────────────────────────────────────
            { let mut td = state.recent_trades.lock().unwrap(); td.push_front(trade.clone()); if td.len()>MAX_TRADES{td.pop_back();} }

            // ── Signal engine ────────────────────────────────────────────────
            run_signals_on_trade(&state, &trade);

            // ── Update stats + broadcast events ──────────────────────────────
            update_stats(&trade, &state);
            broadcast_trade_events(&trade, &profile, &state);
        }
    }
}

// ─── Task: CLOB order books ────────────────────────────────────────────────────

fn collect_token_ids(mkts: &[Market]) -> Vec<String> {
    let mut token_ids: Vec<String> = vec![];
    let mut seen: HashSet<String> = HashSet::new();
    for m in mkts.iter().take(8) {
        for o in &m.outcomes {
            if seen.insert(o.token_id.clone()) { token_ids.push(o.token_id.clone()); }
            if token_ids.len() >= 40 { break; }
        }
        if token_ids.len() >= 40 { break; }
    }
    token_ids
}

fn parse_book_json(json: &[serde_json::Value]) -> HashMap<String, RawBook> {
    let now = Utc::now().timestamp_millis();
    let mut books = HashMap::new();
    for v in json {
        let tid = v["asset_id"].as_str().or_else(|| v["token_id"].as_str()).unwrap_or("").to_string();
        if tid.is_empty() { continue; }
        let bids = v["bids"].as_array().map(|a| a.iter().filter_map(|l| {
            let p = l["price"].as_str()?.parse::<f64>().ok()? * 100.0;
            let s = l["size"].as_str()?.parse::<f64>().ok()?;
            Some((p,s))
        }).take(15).collect()).unwrap_or_default();
        let asks = v["asks"].as_array().map(|a| a.iter().filter_map(|l| {
            let p = l["price"].as_str()?.parse::<f64>().ok()? * 100.0;
            let s = l["size"].as_str()?.parse::<f64>().ok()?;
            Some((p,s))
        }).take(15).collect()).unwrap_or_default();
        books.insert(tid, RawBook { bids, asks, ts: now });
    }
    books
}

fn store_raw_books(state: &AppState, books: HashMap<String, RawBook>) {
    let mut raw = state.raw_books.lock().unwrap();
    for (tid, book) in books {
        raw.insert(tid, book);
    }
}

fn broadcast_books(state: &Arc<AppState>, mkts: &[Market]) {
    for m in mkts.iter().take(8) {
        let book = assemble_book(state, m);
        state.books.lock().unwrap().insert(m.condition_id.clone(), book.clone());
        let _ = state.tx.send(Ev::Book(book));
    }
}

async fn task_clob_books(state: Arc<AppState>, client: reqwest::Client) {
    let mut iv = tokio::time::interval(Duration::from_secs(4));
    loop {
        iv.tick().await;
        let mut mkts = state.markets.read().unwrap().clone();
        if mkts.is_empty() { continue; }
        mkts.sort_by(|a,b| b.volume_24h.partial_cmp(&a.volume_24h).unwrap_or(std::cmp::Ordering::Equal));

        let token_ids = collect_token_ids(&mkts);
        if token_ids.is_empty() { continue; }
        let ids_param = token_ids.join(",");

        // Fetch books
        if let Ok(resp) = client.get(format!("{CLOB_BOOKS}?token_ids={ids_param}")).send().await {
            if let Ok(json) = resp.json::<Vec<serde_json::Value>>().await {
                let books = parse_book_json(&json);
                store_raw_books(&state, books);
            }
        }

        // Fetch mids + spreads
        let _ = fetch_mids_spreads(&state, &client, &ids_param).await;

        // Rebuild and broadcast books
        broadcast_books(&state, &mkts);
    }
}

async fn fetch_mids_spreads(state: &Arc<AppState>, client: &reqwest::Client, ids: &str) {
    if let Ok(r) = client.get(format!("{CLOB_MID}?token_id={ids}")).send().await {
        if let Ok(json) = r.json::<Vec<serde_json::Value>>().await {
            let amap = state.asset_map.read().unwrap();
            let mut mkts = state.markets.write().unwrap();
            for item in json {
                let tid = item["asset_id"].as_str().unwrap_or("").to_string();
                let mid = item["mid"].as_str().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0) * 100.0;
                if let Some(&(mi,oi)) = amap.get(&tid) {
                    if let Some(m) = mkts.get_mut(mi) { if let Some(o) = m.outcomes.get_mut(oi) { o.mid_cents = mid; } }
                }
            }
        }
    }
    if let Ok(r) = client.get(format!("{CLOB_SPREAD}?token_id={ids}")).send().await {
        if let Ok(json) = r.json::<Vec<serde_json::Value>>().await {
            let amap = state.asset_map.read().unwrap();
            let mut mkts = state.markets.write().unwrap();
            for item in json {
                let tid    = item["asset_id"].as_str().unwrap_or("").to_string();
                let spread = item["spread"].as_str().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0) * 100.0;
                if let Some(&(mi,oi)) = amap.get(&tid) {
                    if let Some(m) = mkts.get_mut(mi) { if let Some(o) = m.outcomes.get_mut(oi) { o.spread = spread; } }
                }
            }
        }
    }
}

fn assemble_book(state: &Arc<AppState>, m: &Market) -> MarketBook {
    let raw = state.raw_books.lock().unwrap();
    let mut outcome_books = vec![];
    let mut total_liq = 0.0_f64;
    let mut max_bid = 0.0_f64;
    let mut dominant = String::new();
    let ts = Utc::now().timestamp_millis();

    for o in &m.outcomes {
        let rb = raw.get(&o.token_id);
        let current = o.price_cents;
        let mid = o.mid_cents;
        let spread = o.spread;

        let (bids, bid_liq) = if let Some(b) = rb {
            let liq: f64 = b.bids.iter().map(|(p,s)| s*(p/100.0)).sum();
            let max_s = b.bids.iter().map(|(_,s)| *s).fold(1.0_f64, f64::max);
            let lvls = b.bids.iter().take(8).map(|(p,s)| Level {
                price: *p, size: s*(p/100.0), fill_pct: ((s/max_s)*100.0).min(100.0) as u8
            }).collect();
            (lvls, liq)
        } else { (vec![], 0.0) }; // No real book → empty, never fake

        let (asks, ask_liq) = if let Some(b) = rb {
            let liq: f64 = b.asks.iter().map(|(p,s)| s*(p/100.0)).sum();
            let max_s = b.asks.iter().map(|(_,s)| *s).fold(1.0_f64, f64::max);
            let lvls = b.asks.iter().take(8).map(|(p,s)| Level {
                price: *p, size: s*(p/100.0), fill_pct: ((s/max_s)*100.0).min(100.0) as u8
            }).collect();
            (lvls, liq)
        } else { (vec![], 0.0) }; // No real book → empty, never fake

        let mut bids = bids; bids.sort_by(|a,b| b.price.partial_cmp(&a.price).unwrap_or(std::cmp::Ordering::Equal));
        let mut asks = asks; asks.sort_by(|a,b| a.price.partial_cmp(&b.price).unwrap_or(std::cmp::Ordering::Equal));

        let best_bid = bids.first().map(|l| l.price).unwrap_or(0.0);
        let best_ask = asks.first().map(|l| l.price).unwrap_or(0.0);
        let imbalance = if bid_liq + ask_liq > 0.0 { (bid_liq - ask_liq) / (bid_liq + ask_liq) } else { 0.0 };

        total_liq += bid_liq + ask_liq;
        if bid_liq > max_bid { max_bid = bid_liq; dominant = o.name.clone(); }

        outcome_books.push(OutcomeBook {
            outcome_name: o.name.clone(), token_id: o.token_id.clone(),
            current_price: current, mid, spread, bids, asks,
            best_bid, best_ask,
            bid_liquidity: bid_liq, ask_liquidity: ask_liq, imbalance,
        });
    }

    MarketBook { market_id: m.id, condition_id: m.condition_id.clone(), outcome_books, total_liquidity: total_liq, dominant_side: dominant, ts }
}


// ─── Task: Leaderboards ────────────────────────────────────────────────────────

async fn task_leaderboards(state: Arc<AppState>, client: reqwest::Client) {
    let mut iv = tokio::time::interval(Duration::from_secs(300));
    loop {
        iv.tick().await;
        fetch_lb(&state, &client, DATA_LB, "MONTH").await;
        tokio::time::sleep(Duration::from_secs(2)).await;
        fetch_lb(&state, &client, DATA_LB_ALL, "ALL").await;
    }
}

fn parse_leaderboard_json(data: &[serde_json::Value], period: &str) -> Vec<LeaderboardEntry> {
    data.iter().enumerate().map(|(i,v)| LeaderboardEntry {
        rank: i+1,
        address: v["proxyWallet"].as_str().or_else(||v["user"].as_str()).unwrap_or("").to_string(),
        username: v["username"].as_str().filter(|s|!s.is_empty()).map(String::from),
        pnl: v["pnl"].as_f64().unwrap_or(0.0),
        volume: v["volume"].as_f64().unwrap_or(0.0),
        period: period.to_string(),
    }).collect()
}

fn store_leaderboard(state: &AppState, period: &str, entries: Vec<LeaderboardEntry>) {
    if period == "MONTH" { *state.leaderboard_month.lock().unwrap() = entries.clone(); }
    else                 { *state.leaderboard_all.lock().unwrap()   = entries.clone(); }
    let _ = state.tx.send(Ev::LeaderboardUpdate(entries));
}

async fn fetch_lb(state: &Arc<AppState>, client: &reqwest::Client, url: &str, period: &str) {
    let Ok(resp) = client.get(url).send().await else { return };
    let Ok(data) = resp.json::<Vec<serde_json::Value>>().await else { return };
    let entries = parse_leaderboard_json(&data, period);
    store_leaderboard(state, period, entries);
}

// ─── Task: Global signals ──────────────────────────────────────────────────────

async fn task_signals(state: Arc<AppState>) {
    let mut iv = tokio::time::interval(Duration::from_secs(30));
    loop {
        iv.tick().await;
        let mkts     = state.markets.read().unwrap();
        let trades   = state.recent_trades.lock().unwrap();
        let profiles = state.whale_profiles.lock().unwrap();
        if mkts.is_empty() { continue; }

        let n = mkts.len() as f64;
        // Use 5m-windowed price change, not single-tick delta
        let now_ms = Utc::now().timestamp_millis();
        {
            let mut mss = state.mkt_signal_state.lock().unwrap();
            for m in mkts.iter() {
                let ms = mss.entry(m.slug.clone()).or_default();
                let price = m.primary_prob();
                if ms.price_5m_ts == 0 || (now_ms - ms.price_5m_ts) >= 300_000 {
                    ms.price_5m_ago = price;
                    ms.price_5m_ts  = now_ms;
                }
                if ms.price_15m_ts == 0 || (now_ms - ms.price_15m_ts) >= 900_000 {
                    ms.price_15m_ago = price;
                    ms.price_15m_ts  = now_ms;
                }
            }
        }
        let bullish = {
            let mss = state.mkt_signal_state.lock().unwrap();
            mkts.iter().filter(|m| {
                mss.get(&m.slug).map(|ms| m.primary_prob() > ms.price_5m_ago).unwrap_or(false)
            }).count() as f64
        };
        let momentum = (bullish / n * 100.0) as u8;
        let total_vol = mkts.iter().map(|m| m.volume_24h).sum::<f64>();
        let volume = (total_vol / 5_000_000.0 * 100.0).min(100.0) as u8;
        let sentiment = if momentum > 50 { 55 + (momentum-50)/2 } else { 45u8.saturating_sub((50-momentum)/2) };
        let whale_flow = if profiles.is_empty() { 0 } else {
            (profiles.values().filter(|p| p.conviction_score > 7).count() as f64 / profiles.len() as f64 * 100.0) as u8
        };
        let (bv, sv) = trades.iter().filter(|t| is_whale_trade(t, WHALE_USD))
            .fold((0.0_f64,0.0_f64), |(b,s),t| match t.action {
                Action::BUY  => (b+t.size_usd, s), Action::SELL => (b, s+t.size_usd) });
        let whale_bias = if bv+sv > 0.0 { (bv-sv)/(bv+sv) } else { 0.0 };
        let composite = ((momentum as u32 + volume as u32 + sentiment as u32 + whale_flow as u32) / 4) as u8;

        // ── Enhanced recommendation with detail ───────────────────────────────
        let rec_probability = composite;
        // Recommendation MUST derive ONLY from composite. No overrides.
        let (recommendation, rec_detail, rec_signal_type) = {
            let base = match composite {
                0..=20  => "EXTREME BEAR",
                21..=35 => "STRONG BEAR",
                36..=45 => "BEARISH",
                46..=55 => "NEUTRAL",
                56..=65 => "BULLISH",
                66..=80 => "STRONG BULL",
                _       => "EXTREME BULL",
            };
            (
                base.to_string(),
                format!("Composite score {}/100 (momentum={} volume={} sentiment={} whale_flow={})",
                    composite, momentum, volume, sentiment, whale_flow),
                "composite".to_string(),
            )
        };

        let confidence_score = {
            let base = composite;
            let bonus = if whale_flow > 60 { 10u8 } else { 0 };
            base.saturating_add(bonus).min(100)
        };

        // ── Signal accuracy from tracker ──────────────────────────────────────
        let signal_accuracy: Vec<SignalAccuracy> = {
            let acc = state.signal_accuracy_map.lock().unwrap();
            acc.iter().map(|(kind, (total, sum_conf))| SignalAccuracy {
                signal_kind: kind.clone(),
                total_fired: *total,
                // Proxy: estimate confirmed% as conf/100 with some regression-to-mean
                confirmed_pct: if *total > 0 {
                    let avg_conf = *sum_conf as f64 / *total as f64;
                    (avg_conf * 0.80 + 10.0).min(95.0)
                } else { 0.0 },
                avg_conf: if *total > 0 { *sum_conf as f64 / *total as f64 } else { 0.0 },
            }).collect()
        };

        // Projections removed

        // ── Whale follow yield (average ROI of top 10 whales) ─────────────────
        let whale_follow_yield: f64 = {
            let mut top: Vec<f64> = profiles.values()
                .filter(|p| p.whale_score >= 70.0)
                .map(|p| p.avg_roi)
                .collect();
            top.sort_by(|a,b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
            if top.is_empty() { 0.0 } else {
                top.iter().take(10).sum::<f64>() / top.len().min(10) as f64
            }
        };

        let top_mkt = mkts.iter().max_by(|a,b| a.volume_24h.partial_cmp(&b.volume_24h).unwrap_or(std::cmp::Ordering::Equal))
            .map(|m| m.short_name.clone()).unwrap_or_else(|| "—".into());
        let hottest = trades.iter().take(30).filter(|t| is_whale_trade(t, WHALE_USD))
            .max_by(|a,b| a.size_usd.partial_cmp(&b.size_usd).unwrap_or(std::cmp::Ordering::Equal))
            .map(|t| t.outcome_name.clone()).unwrap_or_else(|| "—".into());

        drop(mkts); drop(trades); drop(profiles);

        // Update stats with performance data
        {
            let mut s = state.stats.lock().unwrap();
            s.whale_follow_yield = whale_follow_yield;
            // signals_7d / signals_30d removed — fabricated data
        }

        let sig = GlobalSignals {
            momentum, volume, sentiment, whale_flow, whale_bias, composite,
            recommendation, rec_probability, rec_detail, rec_signal_type,
            confidence_score, signal_accuracy,
            top_market: top_mkt, hottest_outcome: hottest,
            whale_follow_yield,
        };
        *state.signals.lock().unwrap() = sig.clone();
        let _ = state.tx.send(Ev::Signals(sig));
    }
}

// ─── Task: CLOB WebSocket ─────────────────────────────────────────────────────

async fn task_ws(state: Arc<AppState>) {
    let retry = Duration::from_secs(5);
    loop {
        loop { if !state.markets.read().unwrap().is_empty() { break; } tokio::time::sleep(Duration::from_secs(2)).await; }

        let token_ids: Vec<String> = state.markets.read().unwrap().iter()
            .flat_map(|m| m.outcomes.iter().map(|o| o.token_id.clone())).collect();

        println!("🔌  WS connecting ({} tokens)…", token_ids.len());
        let (ws, _) = match connect_async(CLOB_WS).await { Ok(w)=>w, Err(e)=>{ eprintln!("WS: {e}"); tokio::time::sleep(retry).await; continue; } };
        let (mut write, mut read) = ws.split();
        for chunk in token_ids.chunks(100) {
    let _ = write.send(TungMsg::Text(serde_json::json!({ "assets_ids": chunk, "type": "market" }).to_string())).await;
}
        println!("✅  WS subscribed");

        let mut ping = tokio::time::interval(Duration::from_secs(10));
        loop {
            tokio::select! {
                _ = ping.tick() => { if write.send(TungMsg::Ping(vec![])).await.is_err() { break; } }
                msg = read.next() => {
                    match msg {
                        Some(Ok(TungMsg::Text(t)))   => handle_ws_msg(&state, &t),
                        Some(Ok(TungMsg::Ping(d)))   => { let _ = write.send(TungMsg::Pong(d)).await; }
                        Some(Ok(TungMsg::Pong(_)))   => {}
                        _ => { eprintln!("⚠️  WS drop"); break; }
                    }
                }
            }
        }
        tokio::time::sleep(retry).await;
    }
}

fn handle_ws_msg(state: &Arc<AppState>, text: &str) {
    let vals: Vec<serde_json::Value> = if text.starts_with('[') {
        serde_json::from_str(text).unwrap_or_default()
    } else { serde_json::from_str::<serde_json::Value>(text).map(|v| vec![v]).unwrap_or_default() };

    for v in vals {
        let et  = v["event_type"].as_str().unwrap_or("");
        let tid = v["asset_id"].as_str().unwrap_or("").to_string();
        if tid.is_empty() { continue; }

        if et == "book" {
            let bids: Vec<(f64,f64)> = v["bids"].as_array().map(|a| a.iter().filter_map(|l| {
                let p = l["price"].as_str()?.parse::<f64>().ok()? * 100.0;
                let s = l["size"].as_str()?.parse::<f64>().ok()?;
                Some((p,s))
            }).take(15).collect()).unwrap_or_default();
            let asks: Vec<(f64,f64)> = v["asks"].as_array().map(|a| a.iter().filter_map(|l| {
                let p = l["price"].as_str()?.parse::<f64>().ok()? * 100.0;
                let s = l["size"].as_str()?.parse::<f64>().ok()?;
                Some((p,s))
            }).take(15).collect()).unwrap_or_default();
            let ts = Utc::now().timestamp_millis();
            state.raw_books.lock().unwrap().insert(tid.clone(), RawBook { bids, asks, ts });
            let amap = state.asset_map.read().unwrap();
            if let Some(&(mi,_)) = amap.get(&tid) {
                let mkt = state.markets.read().unwrap().get(mi).cloned();
                if let Some(m) = mkt { let book = assemble_book(state, &m); state.books.lock().unwrap().insert(m.condition_id.clone(), book.clone()); let _ = state.tx.send(Ev::Book(book)); }
            }
        } else if et == "price_change" || et == "last_trade_price" {
            let price = v["price"].as_str().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0) * 100.0;
            let amap  = state.asset_map.read().unwrap();
            if let Some(&(mi,oi)) = amap.get(&tid) {
                let mut mkts = state.markets.write().unwrap();
                if let Some(m) = mkts.get_mut(mi) { if let Some(o) = m.outcomes.get_mut(oi) { o.price_cents = price; o.last_trade = price; } }
            }
        }
    }
}

// ─── Task: Heartbeat ──────────────────────────────────────────────────────────

async fn task_heartbeat(tx: broadcast::Sender<Ev>) {
    let mut iv = tokio::time::interval(Duration::from_secs(1));
    loop { iv.tick().await; let _ = tx.send(Ev::Heartbeat { ts: Utc::now().timestamp_millis(), trial_remaining_secs: 0 }); }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  REST HANDLERS
// ═══════════════════════════════════════════════════════════════════════════════

// ─── WebSocket handler ─────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct WsQ { min_size: Option<f64>, whales_only: Option<bool>, device_id: Option<String> }

async fn ws_handler(ws: WebSocketUpgrade, State(s): State<Arc<AppState>>, Query(p): Query<WsQ>) -> impl IntoResponse {
    // Debug: track new WebSocket connections
    println!("WS: new connection request received");
    ws.on_upgrade(move |socket| handle_ws_conn(socket, s, p))
}

async fn handle_ws_conn(socket: WebSocket, state: Arc<AppState>, p: WsQ) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx  = state.tx.subscribe();
    let min_sz  = p.min_size.unwrap_or(0.0);
    let whales  = p.whales_only.unwrap_or(false);

    // ── Trial registry: tie trial to device_id, survives refreshes ──
    let trial_start_secs: i64 = {
        let dev = p.device_id.clone().unwrap_or_else(|| "unknown".to_string());
        let mut reg = state.trial_registry.lock().unwrap();
        let now_s = Utc::now().timestamp();
        *reg.entry(dev).or_insert(now_s)  // first seen → locked in forever
    };
    // remaining = TRIAL_SECS - (now - first_seen). Never resets on reconnect.
    let trial_elapsed = move || Utc::now().timestamp() - trial_start_secs;

    // Full snapshot on connect
    let snap = {
        let trades   = state.recent_trades.lock().unwrap().iter().take(50).cloned().collect::<Vec<_>>();
        let markets  = state.markets.read().unwrap().clone();
        let mut profiles: Vec<_> = state.whale_profiles.lock().unwrap().values().cloned().collect();
        profiles.sort_by(|a,b| b.whale_score.partial_cmp(&a.whale_score).unwrap_or(std::cmp::Ordering::Equal));
        profiles.truncate(30);
        let lb_m   = state.leaderboard_month.lock().unwrap().clone();
        let lb_a   = state.leaderboard_all.lock().unwrap().clone();
        let stats  = state.stats.lock().unwrap().clone();
        let sigs   = state.signals.lock().unwrap().clone();
        let books  = state.books.lock().unwrap().values().take(8).cloned().collect::<Vec<_>>();
        let edge   = state.edge_signals.lock().unwrap().iter().take(20).cloned().collect::<Vec<_>>();
    let rem    = (TRIAL_SECS as i64 - trial_elapsed()).max(0);
    println!("WS: prepared Snapshot with trades={}, markets={}, whale_profiles= {}", trades.len(), markets.len(), profiles.len());
    serde_json::json!({
            "type": "Snapshot",
            "data": {
                "trades": trades, "markets": markets,
                "whale_profiles": profiles, "leaderboard_month": lb_m, "leaderboard_all": lb_a,
                "stats": stats, "signals": sigs, "order_books": books, "edge_signals": edge,
                "trial_remaining_secs": rem, "trial_total_secs": TRIAL_SECS,
            }
        })
    };
    if sender.send(WsMsg::Text(snap.to_string())).await.is_err() { return; }
    println!("WS: Snapshot sent to client");

    let send = tokio::spawn(async move {
        loop {
            let rem = (TRIAL_SECS as i64 - trial_elapsed()).max(0);
            if rem <= 0 {
                let _ = sender.send(WsMsg::Text(serde_json::to_string(&Ev::TrialExpired).unwrap())).await;
                break;
            }
            match rx.recv().await {
                Ok(ev) => {
                    if let Ev::Trade(ref t) = ev {
                        if whales && !is_whale_trade(t, WHALE_USD) { continue; }
                        if t.size_usd < min_sz { continue; }
                    }
                    let json = if let Ev::Heartbeat { ts, .. } = &ev {
                        serde_json::json!({ "type":"Heartbeat","data":{"ts":ts,"trial_remaining_secs":rem} }).to_string()
                    } else { serde_json::to_string(&ev).unwrap_or_default() };
                    if sender.send(WsMsg::Text(json)).await.is_err() { break; }
                }
                Err(broadcast::error::RecvError::Closed)     => break,
                Err(broadcast::error::RecvError::Lagged(n))  => eprintln!("WS lagged {n}"),
            }
        }
    });

    while let Some(Ok(m)) = receiver.next().await { if matches!(m, WsMsg::Close(_)) { break; } }
    send.abort();
}



// ─── Main ─────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  🐋  WHALE.TERMINAL v4.0  —  Poly Market                 ║");
    println!("║                                                               ║");
    println!("║  8 Signals: Cluster · Velocity · StealthAccum · LiqDrain     ║");
    println!("║             Divergence · Reversal · Conviction · MomBreak    ║");
    println!("║  Whale Score: WR×35% + ROI×30% + Consistency×20% + Vol×15%  ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");

    let (tx, _) = broadcast::channel::<Ev>(BROADCAST_CAP);
    let state   = Arc::new(AppState::new(tx.clone()));

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .user_agent("WhaleTerminal/4.0")
        .build()
        .unwrap();

    // Boot: load markets immediately before spawning tasks
    // Boot: load markets in background so server starts immediately
{
    let s = state.clone(); let c = client.clone();
    tokio::spawn(async move { application::market_service::refresh_markets(&s, &c).await; });
}

    // Fetch leaderboards once on boot
    {
        let s = state.clone(); let c = client.clone();
        tokio::spawn(async move {
            fetch_lb(&s, &c, DATA_LB, "MONTH").await;
            fetch_lb(&s, &c, DATA_LB_ALL, "ALL").await;
        });
    }

    // Spawn background tasks
    tokio::spawn(application::market_service::task_refresh_markets(state.clone(), client.clone()));
    tokio::spawn(task_data_trades(state.clone(), client.clone()));
    tokio::spawn(task_clob_books(state.clone(), client.clone()));
    tokio::spawn(task_leaderboards(state.clone(), client.clone()));
    tokio::spawn(task_signals(state.clone()));
    tokio::spawn(task_ws(state.clone()));
    // Edge Feed: emit high-signal edge events for the dashboard
    tokio::spawn(task_edge_feed(state.clone()));
    tokio::spawn(task_heartbeat(tx));

    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);
    let app  = Router::new()
        .route("/health",            get(api::signals_handler::h_health))
        .route_service("/", tower_http::services::ServeFile::new("index.html"))
        .route("/api/trades",        get(api::trades_handler::h_trades))
        .route("/api/markets",       get(api::markets_handler::h_markets))
        .route("/api/books",         get(api::markets_handler::h_books))
        .route("/api/whales",        get(api::trades_handler::h_whales))
        .route("/api/stats",         get(api::trades_handler::h_stats))
        .route("/api/signals",       get(api::signals_handler::h_signals))
        .route("/api/edge-signals",  get(api::signals_handler::h_edge_signals))
        .route("/api/heatmap",       get(api::markets_handler::h_heatmap))
        .route("/api/scanner",       get(api::markets_handler::h_scanner))
        .route("/api/set-threshold", post(api::signals_handler::h_set_threshold))
        .route("/validate",          get(api::auth::h_validate))
        .route("/api/payment-info",  get(api::auth::h_payment_info))
        .route("/admin/gen-key",     get(api::auth::h_gen_key))
        .route("/ws",                get(ws_handler))
        .layer(cors)
        .with_state(state);

    let base: u16 = std::env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(8080);
    let (listener, port) = {
        let mut l = None; let mut chosen = base;
        for i in 0..20u16 {
            chosen = base.saturating_add(i);
            if let Ok(x) = tokio::net::TcpListener::bind(format!("0.0.0.0:{chosen}")).await { l = Some(x); break; }
        }
        (l.unwrap_or_else(|| { eprintln!("No free port"); std::process::exit(1) }), chosen)
    };

    println!("\n  ✅  http://localhost:{port}");
    println!("  📡  ws://localhost:{port}/ws");
    println!("  🔔  /api/edge-signals  — live signal feed\n");

    axum::serve(listener, app).await.unwrap();
}