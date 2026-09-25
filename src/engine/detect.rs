use std::collections::{HashMap, VecDeque};
use crate::domain::trades::{Trade, Action};
use crate::domain::whales::{WhaleProfile, is_whale_trade, WHALE_USD};
use super::state::MarketSignalState;

pub struct SignalOutput {
    pub kind: String,
    pub confidence: u8,
    pub description: String,
    pub action: String,
    pub edge: String,
    pub color: String,
    pub wallet: Option<String>,
}

pub fn detect_whale_print(trade: &Trade, whale_threshold: f64) -> Option<SignalOutput> {
    if !is_whale_trade(trade, whale_threshold) { return None; }

    let outcome_label = if trade.outcome_count > 2 {
        format!("{} (option {}/{})", trade.outcome_name, trade.outcome_index + 1, trade.outcome_count)
    } else {
        trade.outcome_name.clone()
    };

    let conf = if trade.size_usd >= whale_threshold * 3.0 { 91 }
               else if trade.size_usd >= whale_threshold * 2.0 { 84 }
               else { 76 };

    Some(SignalOutput {
        kind: "WHALE_PRINT".into(),
        confidence: conf,
        description: format!("{} {} ${:.1}K on {} at {:.1}c.",
            trade.wallet_short,
            if trade.action == Action::BUY { "bought" } else { "sold" },
            trade.size_usd / 1000.0, outcome_label, trade.price_cents),
        action: format!("{} {}", if trade.action == Action::BUY { "BUY" } else { "SELL" }, outcome_label),
        edge: "Oversized whale prints are immediate directional information, especially in fragmented Polymarket sub-markets.".into(),
        color: if trade.action == Action::BUY { "green" } else { "red" }.into(),
        wallet: Some(trade.wallet_short.clone()),
    })
}

pub fn detect_smart_cluster(
    trade: &Trade,
    profiles: &HashMap<String, WhaleProfile>,
    recent_trades: &VecDeque<Trade>,
    now_ms: i64,
) -> Option<SignalOutput> {
    let slug = &trade.market_slug;
    let cutoff = now_ms - 15 * 60 * 1000;
    let mut alpha_wallets = std::collections::HashSet::new();
    let mut cluster_outcome = String::new();

    for t in recent_trades.iter() {
        if t.market_slug != *slug { continue; }
        if t.ts < cutoff { continue; }
        if t.action != Action::BUY { continue; }
        if let Some(p) = profiles.get(&t.wallet) {
            if p.whale_score >= 70.0 {
                alpha_wallets.insert(t.wallet.clone());
                cluster_outcome = t.outcome_name.clone();
            }
        }
    }

    if alpha_wallets.len() < 3 { return None; }

    let conf = (55 + (alpha_wallets.len().min(10) as u8 - 3) * 5).min(95);
    Some(SignalOutput {
        kind: "SMART_CLUSTER".into(),
        confidence: conf,
        description: format!("{} alpha wallets (score ≥70) entered {} in the last 15 min.",
            alpha_wallets.len(), cluster_outcome),
        action: format!("ALERT: High activity detected on {cluster_outcome}"),
        edge: "Coordinated smart-money entry historically precedes 15-25% prob moves.".into(),
        color: "cyan".into(),
        wallet: None,
    })
}

pub fn detect_conviction_spike(
    trade: &Trade,
    profiles: &HashMap<String, WhaleProfile>,
) -> Option<SignalOutput> {
    let profile = profiles.get(&trade.wallet)?;
    let avg = if profile.total_trades > 1 {
        profile.total_volume / profile.total_trades as f64
    } else { trade.size_usd };

    if trade.size_usd < avg * 3.0 || trade.size_usd < 2_000.0 { return None; }

    let conf = (60 + ((trade.size_usd / avg) as u8).min(30)).min(92);
    Some(SignalOutput {
        kind: "CONVICTION_SPIKE".into(),
        confidence: conf,
        description: format!("{} bet ${:.0}K — {:.1}× their own avg ${:.0}K. Wallet score: {:.0}.",
            profile.wallet_short, trade.size_usd/1000.0,
            trade.size_usd/avg, avg/1000.0, profile.whale_score),
        action: format!("ALERT: Large {} detected on {}",
            if trade.action==Action::BUY{"buy"}else{"sell"}, trade.outcome_name),
        edge: "Oversize bet vs personal baseline = strong directional conviction.".into(),
        color: "yellow".into(),
        wallet: Some(profile.wallet_short.clone()),
    })
}

pub fn detect_whale_reversal(
    trade: &Trade,
    profiles: &HashMap<String, WhaleProfile>,
    prev_action: Option<&(String, Action)>,
) -> Option<SignalOutput> {
    let (prev_slug, prev_action) = prev_action?;
    if prev_slug != &trade.market_slug || *prev_action == trade.action { return None; }

    let score = profiles.get(&trade.wallet).map(|p| p.whale_score).unwrap_or(0.0);
    if score < 65.0 { return None; }

    let conf = (score as u8).min(88);
    Some(SignalOutput {
        kind: "WHALE_REVERSAL".into(),
        confidence: conf,
        description: format!("{} (score {:.0}) flipped from {} → {} on {}.",
            trade.wallet_short, score,
            if *prev_action==Action::BUY{"BUY"}else{"SELL"},
            if trade.action==Action::BUY{"BUY"}else{"SELL"},
            trade.outcome_name),
        action: format!("ALERT: Position change detected on {}", trade.outcome_name),
        edge: "Smart money position flips reveal new information about fair value.".into(),
        color: "orange".into(),
        wallet: Some(trade.wallet_short.clone()),
    })
}

pub fn detect_velocity_surge(
    trade: &Trade,
    mkt_state: &mut MarketSignalState,
    now_ms: i64,
) -> Option<SignalOutput> {
    let min_ms = 60_000i64;
    let hr_ms  = 3_600_000i64;

    if now_ms - mkt_state.last_vol_reset_1m > min_ms {
        mkt_state.vol_1m = 0.0;
        mkt_state.last_vol_reset_1m = now_ms;
    }
    if now_ms - mkt_state.last_vol_reset_60m > hr_ms {
        mkt_state.vol_60m = 0.0;
        mkt_state.last_vol_reset_60m = now_ms;
    }
    mkt_state.vol_1m  += trade.size_usd;
    mkt_state.vol_60m += trade.size_usd;

    let avg_1m_baseline = mkt_state.vol_60m / 60.0;
    if mkt_state.vol_1m <= avg_1m_baseline * 4.0 || mkt_state.vol_1m <= 1_500.0 || avg_1m_baseline <= 0.0 {
        return None;
    }

    let ratio = mkt_state.vol_1m / avg_1m_baseline;
    let vol_1m_snap = mkt_state.vol_1m;
    let conf = (55 + (ratio as u8).min(30)).min(88);

    Some(SignalOutput {
        kind: "VELOCITY_SURGE".into(),
        confidence: conf,
        description: format!("{:.1}× volume spike vs 60-min avg. ${:.0}K vol in last 60s.",
            ratio, vol_1m_snap / 1000.0),
        action: "ALERT: High volume spike detected".into(),
        edge: "Activity spikes precede prob moves 70% of the time on Polymarket.".into(),
        color: "yellow".into(),
        wallet: None,
    })
}

pub fn detect_stealth_accum(
    trade: &Trade,
    mkt_state: &mut MarketSignalState,
) -> Option<SignalOutput> {
    if trade.action != Action::BUY { return None; }

    let entry = mkt_state.accum_buys.entry(trade.wallet.clone()).or_insert((0, 0.0, trade.price_cents));
    entry.0 += 1;
    entry.1 += trade.size_usd;
    let count = entry.0;
    let total_usd = entry.1;
    let start_price = entry.2;
    let price_drift = (trade.price_cents - start_price).abs();

    if count < 3 || price_drift >= 2.0 || total_usd < 3_000.0 { return None; }

    let conf = (60 + count.min(10) as u8 * 3).min(90);
    Some(SignalOutput {
        kind: "STEALTH_ACCUM".into(),
        confidence: conf,
        description: format!("{} bought {} {} times for ${:.0}K total. Price moved only {:.1}¢.",
            trade.wallet_short, trade.outcome_name, count, total_usd/1000.0, price_drift),
        action: format!("ALERT: Accumulation pattern on {}", trade.outcome_name),
        edge: "Pattern indicates patient accumulation before price movement.".into(),
        color: "green".into(),
        wallet: Some(trade.wallet_short.clone()),
    })
}

pub fn detect_prob_divergence(
    trade: &Trade,
    recent_trades: &VecDeque<Trade>,
    market_prob_change: f64,
    now_ms: i64,
) -> Option<SignalOutput> {
    let slug = &trade.market_slug;
    let cutoff = now_ms - 10 * 60 * 1000;
    let (buy_v, sell_v): (f64, f64) = recent_trades.iter()
        .filter(|t| t.market_slug == *slug && t.ts >= cutoff && is_whale_trade(t, WHALE_USD))
        .fold((0.0, 0.0), |(b,s),t| match t.action {
            Action::BUY  => (b + t.size_usd, s),
            Action::SELL => (b, s + t.size_usd),
        });

    let total = buy_v + sell_v;
    if total < 5_000.0 { return None; }

    let buy_frac = buy_v / total;
    let divergence = (buy_frac > 0.65 && market_prob_change < -3.0)
                  || (buy_frac < 0.35 && market_prob_change > 3.0);

    if !divergence { return None; }

    Some(SignalOutput {
        kind: "PROB_DIVERGENCE".into(),
        confidence: 68,
        description: format!("Whales {:.0}% buying but price moved {:.1}¢ {}. Reversion likely.",
            buy_frac*100.0, market_prob_change.abs(),
            if market_prob_change < 0.0 {"DOWN"}else{"UP"}),
        action: "ALERT: Price/flow divergence detected".into(),
        edge: "Flow/price divergence = mean-reversion edge, ~65% win rate historically.".into(),
        color: "purple".into(),
        wallet: None,
    })
}

pub fn detect_liquidity_drain(
    trade: &Trade,
    raw_book_asks: Option<&Vec<(f64, f64)>>,
    mkt_state: &mut MarketSignalState,
    now_ms: i64,
) -> Option<SignalOutput> {
    let asks = raw_book_asks?;
    let ask_liq: f64 = asks.iter().map(|(p,s)| s*(p/100.0)).sum();

    mkt_state.book_snaps.push_back(super::state::BookSnapshot { ask_liq, ts: now_ms });
    while mkt_state.book_snaps.len() > 20 { mkt_state.book_snaps.pop_front(); }

    let five_min_ago = now_ms - 300_000;
    let old = mkt_state.book_snaps.iter().find(|s| s.ts <= five_min_ago)?;
    let drain_pct = if old.ask_liq > 0.0 { (old.ask_liq - ask_liq) / old.ask_liq } else { 0.0 };

    if drain_pct < 0.40 || ask_liq >= 5_000.0 { return None; }

    let conf = (60 + (drain_pct * 40.0) as u8).min(85);
    Some(SignalOutput {
        kind: "LIQUIDITY_DRAIN".into(),
        confidence: conf,
        description: format!("Ask-side book on {} thinned by {:.0}% in 5 min. ${:.0}K ask liq remaining.",
            trade.outcome_name, drain_pct*100.0, ask_liq/1000.0),
        action: format!("ALERT: Low liquidity on {}", trade.outcome_name),
        edge: "Low liquidity creates potential for high price impact.".into(),
        color: "red".into(),
        wallet: None,
    })
}

pub fn detect_momentum_break(
    trade: &Trade,
    market_prob: f64,
    market_volume: f64,
    mkt_state: &mut MarketSignalState,
    now_ms: i64,
) -> Option<SignalOutput> {
    let key_levels = [25.0_f64, 50.0, 75.0];
    let prev = mkt_state.last_prob;
    mkt_state.last_prob = market_prob;

    for &lvl in &key_levels {
        let crossed = (prev < lvl && market_prob >= lvl) || (prev > lvl && market_prob <= lvl);
        if crossed && (now_ms - mkt_state.last_cross_ts) > 60_000 {
            mkt_state.last_cross_ts = now_ms;
            let direction = if market_prob >= lvl { "UP" } else { "DOWN" };

            return Some(SignalOutput {
                kind: "MOMENTUM_BREAK".into(),
                confidence: 72,
                description: format!("Probability crossed {:.0}¢ {} with ${:.0}K volume.",
                    lvl, direction, market_volume / 1000.0),
                action: format!("ALERT: Key level break on {}", trade.outcome_name),
                edge: "Key level crosses with volume = trend continuation ~68% of time on Polymarket.".into(),
                color: "cyan".into(),
                wallet: None,
            });
        }
    }
    None
}
