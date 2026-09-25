use std::collections::{HashMap, VecDeque};
use serde::{Deserialize, Serialize};

use super::trades::Trade;

pub const WHALE_USD: f64 = 5_000.0;

pub fn is_whale_trade(trade: &Trade, threshold: f64) -> bool {
    trade.size_usd >= threshold
}

pub fn compute_whale_tag(ws: f64, buy_vol: f64, sell_vol: f64, total_vol: f64) -> &'static str {
    match () {
        _ if ws >= 88.0                             => "APEX PREDATOR",
        _ if ws >= 74.0                             => "ALPHA HUNTER",
        _ if total_vol >= 200_000.0                 => "MARKET MAKER",
        _ if ws >= 60.0                             => "SMART GRINDER",
        _ if buy_vol >= sell_vol * 2.5              => "ACCUMULATOR",
        _ if sell_vol >= buy_vol * 2.5              => "DISTRIBUTOR",
        _ if ws < 35.0 && total_vol >= 20_000.0     => "DEGEN GAMBLER",
        _                                           => "TREND FOLLOWER",
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub rank:     usize,
    pub address:  String,
    pub username: Option<String>,
    pub pnl:      f64,
    pub volume:   f64,
    pub period:   String,
}

#[derive(Debug, Default, Clone)]
pub struct TradeWindow {
    sizes:   VecDeque<f64>,
    pnls:    VecDeque<f64>,
    markets: HashMap<String, u32>,
}

impl TradeWindow {
    pub fn push(&mut self, size: f64, pnl: f64, slug: &str) {
        self.sizes.push_back(size);
        self.pnls.push_back(pnl);
        *self.markets.entry(slug.to_string()).or_default() += 1;
        if self.sizes.len() > 50 { self.sizes.pop_front(); self.pnls.pop_front(); }
    }

    pub fn win_rate(&self) -> f64 {
        let wins = self.pnls.iter().filter(|&&p| p > 0.0).count();
        if self.pnls.is_empty() { return 50.0; }
        wins as f64 / self.pnls.len() as f64 * 100.0
    }

    pub fn avg_roi(&self) -> f64 {
        if self.pnls.is_empty() { return 0.0; }
        let sum: f64 = self.pnls.iter().sum();
        let sz: f64  = self.sizes.iter().sum();
        if sz > 0.0 { (sum / sz * 100.0).clamp(-100.0, 100.0) } else { 0.0 }
    }

    pub fn consistency_score(&self) -> f64 {
        if self.sizes.len() < 2 { return 50.0; }
        let avg = self.sizes.iter().sum::<f64>() / self.sizes.len() as f64;
        if avg <= 0.0 { return 50.0; }
        let var = self.sizes.iter().map(|&s| (s - avg).powi(2)).sum::<f64>() / self.sizes.len() as f64;
        let cv  = var.sqrt() / avg;
        ((1.0 - cv.min(2.0) / 2.0) * 100.0).clamp(0.0, 100.0)
    }

    pub fn top_market(&self) -> String {
        self.markets.iter().max_by_key(|(_,&v)| v).map(|(k,_)| k.clone()).unwrap_or_default()
    }

    pub fn specialization_score(&self) -> f64 {
        if self.markets.is_empty() { return 0.0; }
        let total: u32 = self.markets.values().sum();
        let max: u32   = *self.markets.values().max().unwrap_or(&0);
        max as f64 / total.max(1) as f64 * 100.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhaleProfile {
    pub wallet:              String,
    pub wallet_short:        String,
    pub pseudonym:           Option<String>,
    pub total_trades:        u32,
    pub total_volume:        f64,
    pub buy_volume:          f64,
    pub sell_volume:         f64,
    pub dominant_action:     String,
    pub favourite_market:    String,
    pub favourite_outcome:   String,
    pub whale_score:         f64,
    pub win_rate:            f64,
    pub avg_roi:             f64,
    pub consistency:         f64,
    pub specialization:      f64,
    pub conviction_score:    u8,
    pub whale_tag:           String,
    pub last_seen:           String,
    pub pnl_proxy:           f64,
    pub active_bets:         u32,
    #[serde(skip)]
    pub window:              TradeWindow,
}

impl WhaleProfile {
    pub fn recompute(&mut self) {
        self.win_rate     = self.window.win_rate();
        self.avg_roi      = self.window.avg_roi();
        self.consistency  = self.window.consistency_score();
        self.specialization = self.window.specialization_score();
        self.favourite_market = self.window.top_market();

        let vol_norm = (self.total_volume / 500_000.0 * 100.0).min(100.0);

        self.whale_score = (self.win_rate * 0.35)
            + (self.avg_roi.max(0.0) * 0.30 * 3.0).min(30.0)
            + (self.consistency    * 0.20)
            + (vol_norm            * 0.15);
        self.whale_score = self.whale_score.clamp(0.0, 100.0);

        self.whale_tag = compute_whale_tag(
            self.whale_score, self.buy_volume, self.sell_volume, self.total_volume
        ).to_string();
    }
}
