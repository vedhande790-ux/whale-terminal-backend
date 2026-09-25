use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub id:              usize,
    pub condition_id:    String,
    pub slug:            String,
    pub event_slug:      String,
    pub question:        String,
    pub short_name:      String,
    pub category:        &'static str,
    pub outcomes:        Vec<Outcome>,
    pub volume_24h:      f64,
    pub volume_total:    f64,
    pub liquidity:       f64,
    pub prob_change_pct: f64,
    pub signal:          String,
    pub end_date:        Option<String>,
    pub url:             String,
    pub buy_pressure:    f64,
}

impl Market {
    pub fn primary_prob(&self) -> f64 {
        self.outcomes.first().map(|o| o.price_cents).unwrap_or(50.0)
    }

    pub fn from_gamma(v: &serde_json::Value, id: usize) -> Option<Market> {
        let condition_id = match v["conditionId"].as_str().or_else(|| v["condition_id"].as_str()) {
            Some(s) => s.to_string(),
            None => return None,
        };
        let question = v["question"].as_str().unwrap_or("?").to_string();
        let slug = v["slug"].as_str().unwrap_or("").to_string();
        let event_slug = v["events"].as_array()
            .and_then(|a| a.first()).and_then(|e| e["slug"].as_str())
            .or_else(|| v["eventSlug"].as_str()).unwrap_or(&slug).to_string();
        let volume_24h = v["volume24hr"].as_str().and_then(|s| s.parse().ok())
            .or_else(|| v["volume24hr"].as_f64()).unwrap_or(0.0);
        let volume_total = v["volume"].as_str().and_then(|s| s.parse().ok())
            .or_else(|| v["volume"].as_f64()).unwrap_or(0.0);
        let liquidity = v["liquidity"].as_str().and_then(|s| s.parse().ok())
            .or_else(|| v["liquidity"].as_f64()).unwrap_or(0.0);
        let end_date = v["endDate"].as_str().map(String::from);
        let token_ids = { let a = parse_str_arr(&v["clobTokenIds"]); if !a.is_empty(){a}else{parse_str_arr(&v["clob_token_ids"])} };
        if token_ids.is_empty() { return None; }
        let names  = parse_str_arr(&v["outcomes"]);
        let prices = parse_f64_arr(&v["outcomePrices"]);
        let outcomes: Vec<Outcome> = token_ids.iter().enumerate().map(|(idx, tid)| {
            let name  = names.get(idx).cloned().unwrap_or_else(|| if idx==0{"YES".into()}else{"NO".into()});
            let price = prices.get(idx).copied().unwrap_or(0.5) * 100.0;
            Outcome { token_id: tid.clone(), name, price_cents: price, mid_cents: price, spread: 0.0, last_trade: price }
        }).collect();
        let url = polymarket_url(&event_slug, &slug);
        Some(Market {
            id, condition_id, slug, event_slug,
            question: question.clone(),
            short_name: trunc(&question, 20),
            category: infer_category(&question),
            outcomes, volume_24h, volume_total, liquidity,
            prob_change_pct: 0.0, signal: "NEUTRAL".into(),
            end_date, url,
            buy_pressure: 0.5,
        })
    }

    pub fn is_live(&self, now: chrono::DateTime<chrono::Utc>) -> bool {
        let date_ok = match &self.end_date {
            Some(d) => chrono::DateTime::parse_from_rfc3339(d)
                .map(|dt| dt > now)
                .unwrap_or(true),
            None => true,
        };
        let not_resolved = !self.outcomes.iter().any(|o| o.price_cents >= 99.0);
        let has_volume = self.volume_24h > 10.0;
        date_ok && not_resolved && has_volume
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outcome {
    pub token_id:    String,
    pub name:        String,
    pub price_cents: f64,
    pub mid_cents:   f64,
    pub spread:      f64,
    pub last_trade:  f64,
}

pub fn polymarket_url(event_slug: &str, market_slug: &str) -> String {
    let event_slug = event_slug.trim();
    let market_slug = market_slug.trim();
    if !event_slug.is_empty() && event_slug != "undefined" {
        if !market_slug.is_empty() && market_slug != "undefined" {
            format!("https://polymarket.com/event/{event_slug}#{market_slug}")
        } else {
            format!("https://polymarket.com/event/{event_slug}")
        }
    } else if !market_slug.is_empty() && market_slug != "undefined" {
        format!("https://polymarket.com/event/{market_slug}")
    } else {
        "https://polymarket.com".into()
    }
}

fn parse_str_arr(v: &serde_json::Value) -> Vec<String> {
    match v {
        serde_json::Value::Array(a) => a.iter().filter_map(|x| x.as_str().map(String::from)).collect(),
        serde_json::Value::String(s) => serde_json::from_str::<Vec<String>>(s).unwrap_or_default(),
        _ => vec![],
    }
}

fn parse_f64_arr(v: &serde_json::Value) -> Vec<f64> {
    match v {
        serde_json::Value::Array(a) => a.iter().filter_map(|x| {
            x.as_str().and_then(|s| s.parse().ok()).or_else(|| x.as_f64())
        }).collect(),
        serde_json::Value::String(s) => {
            serde_json::from_str::<Vec<String>>(s).ok()
                .map(|v| v.iter().filter_map(|x| x.parse().ok()).collect())
                .or_else(|| serde_json::from_str(s).ok())
                .unwrap_or_default()
        }
        _ => vec![],
    }
}

fn trunc(s: &str, n: usize) -> String {
    let c: Vec<char> = s.chars().collect();
    if c.len() <= n { s.to_string() } else { format!("{}…", c[..n].iter().collect::<String>()) }
}

pub fn infer_category(q: &str) -> &'static str {
    let l = q.to_lowercase();
    if ["bitcoin","btc","eth","crypto","solana","doge"].iter().any(|k| l.contains(k)) { return "Crypto"; }
    if ["election","president","trump","biden","harris","vote","congress"].iter().any(|k| l.contains(k)) { return "Politics"; }
    if ["nba","nfl","nhl","mlb","soccer","champion","world cup","super bowl"].iter().any(|k| l.contains(k)) { return "Sports"; }
    if ["fed","rate","gdp","recession","inflation","interest"].iter().any(|k| l.contains(k)) { return "Finance"; }
    if ["ai","gpt","openai","apple","google","spacex","tesla"].iter().any(|k| l.contains(k)) { return "Tech"; }
    "Other"
}
