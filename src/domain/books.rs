use serde::{Deserialize, Serialize};

/// Raw CLOB book — price-size tuples, not yet modeled as Levels
#[derive(Debug, Clone, Default)]
pub struct RawBook {
    pub bids: Vec<(f64, f64)>,
    pub asks: Vec<(f64, f64)>,
    pub ts:   i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level {
    pub price:    f64,
    pub size:     f64,
    pub fill_pct: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutcomeBook {
    pub outcome_name:  String,
    pub token_id:      String,
    pub current_price: f64,
    pub mid:           f64,
    pub spread:        f64,
    pub bids:          Vec<Level>,
    pub asks:          Vec<Level>,
    pub best_bid:      f64,
    pub best_ask:      f64,
    pub bid_liquidity: f64,
    pub ask_liquidity: f64,
    pub imbalance:     f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketBook {
    pub market_id:       usize,
    pub condition_id:    String,
    pub outcome_books:   Vec<OutcomeBook>,
    pub total_liquidity: f64,
    pub dominant_side:   String,
    pub ts:              i64,
}
