pub mod state;
pub mod detect;

use std::collections::{HashMap, VecDeque};
use chrono::Utc;

use crate::domain::trades::{Trade, Action};
use crate::domain::whales::WhaleProfile;
use crate::domain::signals::EdgeSignal;
use crate::domain::books::RawBook;
use state::{MarketSignalState, SignalDedup};
use detect::*;

const SIGNAL_DEDUP_MS: i64 = 300_000;

pub fn run_signals_on_trade(
    trade: &Trade,
    profiles: &HashMap<String, WhaleProfile>,
    recent_trades: &VecDeque<Trade>,
    markets: &Vec<crate::domain::markets::Market>,
    raw_books: &HashMap<String, RawBook>,
    mkt_signal_state: &mut HashMap<String, MarketSignalState>,
    signal_dedup: &mut SignalDedup,
    wallet_prev_action: &mut HashMap<String, (String, Action)>,
    whale_threshold: f64,
    fire_signal: &dyn Fn(EdgeSignal),
) {
    let now = Utc::now().timestamp_millis();
    let slug = &trade.market_slug;

    // Get per-market mutable state
    let mkt_state = mkt_signal_state.entry(slug.clone()).or_default();

    // Resolve market data
    let (market_prob, market_volume, market_prob_change, token_id) = {
        let mut prob = 50.0;
        let mut vol = 0.0;
        let mut prob_change = 0.0;
        let mut tid = None;
        for m in markets.iter() {
            if m.slug == *slug {
                prob = m.primary_prob();
                vol = m.volume_24h;
                prob_change = m.prob_change_pct;
                // Find token_id for this trade's outcome
                if let Some(o) = m.outcomes.get(trade.outcome_index) {
                    tid = Some(o.token_id.clone());
                }
                break;
            }
        }
        (prob, vol, prob_change, tid)
    };

    // Get raw book asks for liquidity drain detection
    let raw_book_asks = token_id.as_ref().and_then(|tid| raw_books.get(tid)).map(|rb| &rb.asks);

    // Get prev action for whale reversal
    let prev_action = wallet_prev_action.get(&trade.wallet);

    // Build context and run each detector
    let mut signals_to_fire: Vec<(String, SignalOutput)> = Vec::new();

    // Whale Print
    if let Some(out) = detect_whale_print(trade, whale_threshold) {
        let dedup_id = format!("whale-print-{}", trade.id);
        if signal_dedup.should_fire(&dedup_id, now, SIGNAL_DEDUP_MS) {
            signals_to_fire.push((dedup_id, out));
        }
    }

    // Smart Cluster
    if let Some(out) = detect_smart_cluster(trade, profiles, recent_trades, now) {
        let dedup_id = format!("cluster-{}-{}", slug, &trade.outcome_name);
        if signal_dedup.should_fire(&dedup_id, now, SIGNAL_DEDUP_MS) {
            signals_to_fire.push((dedup_id, out));
        }
    }

    // Conviction Spike
    if let Some(out) = detect_conviction_spike(trade, profiles) {
        let dedup_id = format!("conviction-{}", trade.wallet);
        if signal_dedup.should_fire(&dedup_id, now, 120_000) {
            signals_to_fire.push((dedup_id, out));
        }
    }

    // Whale Reversal
    if let Some(out) = detect_whale_reversal(trade, profiles, prev_action) {
        let dedup_id = format!("reversal-{}-{}", trade.wallet, slug);
        if signal_dedup.should_fire(&dedup_id, now, 180_000) {
            signals_to_fire.push((dedup_id, out));
        }
    }
    // Update prev action
    wallet_prev_action.insert(trade.wallet.clone(), (slug.clone(), trade.action.clone()));

    // Velocity Surge
    if let Some(out) = detect_velocity_surge(trade, mkt_state, now) {
        let dedup_id = format!("velocity-{}", slug);
        if signal_dedup.should_fire(&dedup_id, now, 120_000) {
            signals_to_fire.push((dedup_id, out));
        }
    }

    // Stealth Accumulation
    if let Some(out) = detect_stealth_accum(trade, mkt_state) {
        let dedup_id = format!("stealth-{}-{}", trade.wallet, slug);
        if signal_dedup.should_fire(&dedup_id, now, 300_000) {
            signals_to_fire.push((dedup_id, out));
        }
    }

    // Prob Divergence
    if let Some(out) = detect_prob_divergence(trade, recent_trades, market_prob_change, now) {
        let dedup_id = format!("diverge-{}", slug);
        if signal_dedup.should_fire(&dedup_id, now, 300_000) {
            signals_to_fire.push((dedup_id, out));
        }
    }

    // Liquidity Drain
    if let Some(out) = detect_liquidity_drain(trade, raw_book_asks, mkt_state, now) {
        let dedup_id = format!("liquidrain-{}-{}", slug, &trade.outcome_name);
        if signal_dedup.should_fire(&dedup_id, now, 180_000) {
            signals_to_fire.push((dedup_id, out));
        }
    }

    // Momentum Break
    if let Some(out) = detect_momentum_break(trade, market_prob, market_volume, mkt_state, now) {
        let dedup_id = format!("mombreak-{}-{}-{}", slug, market_prob as u32,
            if market_prob >= 50.0 { "UP" } else { "DOWN" });
        if signal_dedup.should_fire(&dedup_id, now, 120_000) {
            signals_to_fire.push((dedup_id, out));
        }
    }

    // Fire all signals
    for (_sig_id, out) in signals_to_fire {
        let title = out.kind.replace('_', " ");
        fire_signal(EdgeSignal {
            id: _sig_id,
            kind: out.kind,
            title,
            description: out.description,
            market: trade.market.clone(),
            market_slug: slug.clone(),
            outcome: trade.outcome_name.clone(),
            price_cents: trade.price_cents,
            confidence: out.confidence,
            priority: EdgeSignal::priority_from_confidence(out.confidence).into(),
            action: out.action,
            edge: out.edge,
            url: trade.url.clone(),
            ts: now,
            color: out.color,
            wallet: out.wallet,
        });
    }
}
