use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeSignal {
    pub id:           String,
    pub kind:         String,
    pub title:        String,
    pub description:  String,
    pub market:       String,
    pub market_slug:  String,
    pub outcome:      String,
    pub price_cents:  f64,
    pub confidence:   u8,
    pub priority:     String,
    pub action:       String,
    pub edge:         String,
    pub url:          String,
    pub ts:           i64,
    pub color:        String,
    pub wallet:       Option<String>,
}

impl EdgeSignal {
    pub fn priority_from_confidence(c: u8) -> &'static str {
        match c {
            0..=39  => "LOW",
            40..=64 => "MEDIUM",
            65..=84 => "HIGH",
            _       => "CRITICAL",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalAccuracy {
    pub signal_kind:    String,
    pub total_fired:    u32,
    pub confirmed_pct:  f64,
    pub avg_conf:       f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalSignals {
    pub momentum:           u8,
    pub volume:             u8,
    pub sentiment:          u8,
    pub whale_flow:         u8,
    pub whale_bias:         f64,
    pub composite:          u8,
    pub recommendation:     String,
    pub rec_probability:    u8,
    pub rec_detail:         String,
    pub rec_signal_type:    String,
    pub confidence_score:   u8,
    pub signal_accuracy:    Vec<SignalAccuracy>,
    pub top_market:         String,
    pub hottest_outcome:    String,
    pub whale_follow_yield: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub total_volume_24h:   f64,
    pub total_volume_ever:  f64,
    pub active_wallets:     usize,
    pub open_markets:       usize,
    pub total_trades_seen:  u64,
    pub biggest_trade:      f64,
    pub buy_sell_ratio:     f64,
    pub whale_count:        usize,
    pub signals_fired_today: u32,
    pub alpha_wallet_count:  usize,
    pub whale_follow_yield: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeFeedEvent {
    pub edge_score: f64,
    pub direction:  String,
    pub strength:   String,
    pub reasons:    Vec<String>,
    pub execution:  String,
    pub ts:         i64,
}
