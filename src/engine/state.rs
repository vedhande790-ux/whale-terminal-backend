use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
pub struct BookSnapshot {
    pub ask_liq: f64,
    pub ts:      i64,
}

#[derive(Debug, Default)]
pub struct MarketSignalState {
    pub vol_1m:     f64,
    pub vol_60m:    f64,
    pub last_vol_reset_1m: i64,
    pub last_vol_reset_60m: i64,
    pub accum_buys: HashMap<String, (u32, f64, f64)>,
    pub book_snaps: VecDeque<BookSnapshot>,
    pub last_prob:  f64,
    pub last_cross_ts: i64,
    pub price_5m_ago:  f64,
    pub price_15m_ago: f64,
    pub price_5m_ts:   i64,
    pub price_15m_ts:  i64,
}

#[derive(Default)]
pub struct SignalDedup {
    map: HashMap<String, i64>,
}

impl SignalDedup {
    pub fn should_fire(&mut self, id: &str, now_ms: i64, window_ms: i64) -> bool {
        if let Some(&last) = self.map.get(id) {
            if now_ms - last < window_ms { return false; }
        }
        self.map.insert(id.to_string(), now_ms);
        true
    }
}
