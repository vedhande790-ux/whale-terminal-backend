# Whale.TERMINAL — Trades Pipeline

A walkthrough of the trade pipeline in the layered DDD architecture:
what each stage does, why every decision was made, and how data flows
from ingestion to WebSocket broadcast.

This is the companion to ARCHITECTURE_WALKTHROUGH.md and covers the
trade path end to end.

---

## Table of Contents

1. What We Started With (After Markets Extraction)
2. What Is a Trade vs a Whale
3. The Core Lesson: One Concept Per File
4. What Moved and Why
5. The Trade Pipeline: Before
6. The Refactoring: Step by Step
7. Before vs After
8. Why Each Decision Was Made
9. Where Functions Live and Why
10. What Each New File Does
11. What Is Still in main.rs
12. Dependency Direction
13. Common Beginner Questions

---

# THIRD EXTRACTION SESSION — Books, Signals, Auth

14. What We Started With (After Trades Extraction)
15. The Three Extractions
16. The Pattern: What Gets Extracted Each Time
17. Before vs After (All Three Sessions)
18. Common Beginner Questions (Updated)

---

# FOURTH EXTRACTION SESSION — Signal Engine (engine/)

19. What We Started With (After Third Extraction)
20. Why the Signal Engine Was Hard
21. The Architecture: Orchestrator + Pure Detectors
22. What Moved and Why
23. Before vs After (All Four Sessions)
24. The `engine/` Module Walkthrough
25. Common Beginner Questions (Signal Engine)

---

# FIFTH EXTRACTION SESSION — task_data_trades (Trade Ingestion Loop)

26. What We Started With (After Fourth Extraction)
27. Why This Was the Hardest Extraction
28. The Strategy: Helper Functions
29. What Each Helper Does
30. Before vs After (All Five Sessions)
31. Common Beginner Questions (Trade Ingestion)

---

# SIXTH EXTRACTION SESSION — Signal REST Handlers (api/signals_handler.rs)

32. What We Started With (After Fifth Extraction)
33. Why This Extraction Was Easy
34. What Moved
35. Before vs After (All Six Sessions)
36. Common Beginner Questions (REST Handlers)

---

# SEVENTH EXTRACTION SESSION — CLOB Books (task_clob_books)

37. What We Started With (After Sixth Extraction)
38. Why This Extraction Was Medium Difficulty
39. The Strategy: Split Pure Logic from State Mutations
40. What Each Helper Does
41. Before vs After (All Seven Sessions)
42. Common Beginner Questions (CLOB Books)

---

## 1. What We Started With (After Markets Extraction)

After the markets extraction, main.rs was **2,110 lines**. The trades
pipeline was the biggest chunk remaining:

    Lines 132-133   Action enum (BUY, SELL)
    Lines 135-157   Trade struct (20 fields)
    Lines 162-212   TradeWindow struct + impl (6 methods)
    Lines 214-262   WhaleProfile struct + impl (recompute)
    Lines 69-71     shorten_addr() utility
    Lines 83-94     compute_whale_tag() utility
    Lines 1301-1500 task_data_trades() — the 200-line trade ingestion loop
    Lines 1751-1763 h_trades, h_whales, h_stats REST handlers

**The problem was the same as markets:** everything lived in one file. The
Trade struct, whale intelligence logic, HTTP fetching, state updates, signal
firing, and REST handlers were all tangled together.

---

## 2. What Is a Trade vs a Whale

Before we refactor, we need to understand what these two things ARE.

### A Trade is an EVENT

A trade is something that happened at a specific moment in time:

    "At 14:32:05, wallet 0x7f3a bought 500 YES tokens at 67 cents on the
     'Will Trump win?' market. Total size: $335."

Key properties:
- Happened ONCE (point in time)
- Has a direction (BUY or SELL)
- Has a size ($335)
- Has a wallet (who did it)
- Has a market (where it happened)

### A WhaleProfile is an INTELLIGENCE REPORT

A whale profile is built over time from MANY trades:

    "Wallet 0x7f3a has made 47 trades over the past week.
     Total volume: $128,000
     Win rate: 72%
     Average ROI: +8.3%
     Whale score: 81/100
     Tag: ALPHA HUNTER"

Key properties:
- Aggregated over time (not a single event)
- Computed from many trades
- Has scores (whale_score, win_rate, avg_roi)
- Has a classification (ALPHA HUNTER, APEX PREDATOR, etc.)

### Why They Are Different Domain Concepts

| Concept | Trade | WhaleProfile |
|---------|-------|--------------|
| What it is | An event | An intelligence report |
| Time scope | One moment | Many trades over time |
| Has an ID | Yes (tx_hash) | Yes (wallet address) |
| Changes over time | No (immutable once created) | Yes (recomputed on each trade) |
| Where it lives | recent_trades (VecDeque) | whale_profiles (HashMap) |

**A WhaleProfile DEPENDS on trades** (it's built from them), but a Trade
does NOT depend on WhaleProfile. They are separate concepts.

### What About "Is This a Whale Trade?"

Here is where beginners get confused. A trade has a size ($335). Whether
that's a "whale trade" depends on a threshold ($5,000). The classification
"whale" belongs to the whale domain, not the trade domain.

Think of it this way:
- **Trade knows:** "I am $335"
- **Whale domain knows:** "Trades above $5,000 are whale trades"
- **Trade does NOT know:** "I am a whale trade" — that's someone else's opinion

This is why we removed `is_whale` from the Trade struct and moved it to
a function in `domain/whales.rs`.

---

## 3. The Core Lesson: One Concept Per File

The biggest takeaway from this refactoring is:

> **Each domain concept gets its own file.**

Trade is one concept. Whale is another concept. They are related but
independent. Putting them in the same file is like putting "Car" and
"Insurance" in the same class — they're related, but they're not the
same thing.

The rule is simple: **ask "What does this function/struct KNOW about?"**

- If it knows about Polymarket URLs → `domain/markets.rs`
- If it knows about wallet address formatting → `domain/trades.rs`
- If it knows about whale score formulas → `domain/whales.rs`
- If it knows about HTTP endpoints → `ingestion/`
- If it knows about AppState → `main.rs` (infrastructure)

Each function lives where its knowledge belongs.

---

## 4. What Moved and Why

### Domain Layer — Trade (Facts + Rules)

| What | Why |
|------|-----|
| `Action` enum | Core fact: a trade is BUY or SELL |
| `Trade` struct | Core fact: 19 fields defining one trade event |
| `shorten_addr()` | Helper: how trades display wallet addresses |

### Domain Layer — Whale (Facts + Rules)

| What | Why |
|------|-----|
| `WHALE_USD` constant | Business rule: what threshold makes a trade a "whale trade" |
| `is_whale_trade()` | Business rule: classify whether a trade is a whale trade |
| `compute_whale_tag()` | Business rule: classify whale behavior from score |
| `TradeWindow` struct + impl | Business logic: rolling 50-trade window for metrics |
| `WhaleProfile` struct + impl | Business logic: whale score formula |

### Ingestion Layer (Raw Data)

| What | Why |
|------|-----|
| `DATA_TRADES` constant | API endpoint URL |
| `fetch_raw_trades()` | Dumb HTTP fetch, returns `Vec<Value>` |

### Application Layer (Orchestration)

| What | Why |
|------|-----|
| `MIN_TRADE_USD` constant | Business threshold: minimum trade size to process |
| `MAX_TRADES` constant | Config: rolling window size |
| `store_trade()` | Orchestration helper: write to state |

### API Layer (Transport)

| What | Why |
|------|-----|
| `h_trades()` | REST handler: GET /api/trades |
| `h_whales()` | REST handler: GET /api/whales |
| `h_stats()` | REST handler: GET /api/stats |

---

## 5. The Trade Pipeline: Before

Here is the original `task_data_trades` (lines 1301-1500) with each
responsibility marked:

    async fn task_data_trades(state: Arc<AppState>, client: reqwest::Client) {
        loop {
            // === RESPONSIBILITY 1: HTTP fetch (ingestion) ===
            let resp = client.get(DATA_TRADES).send().await;
            let raw: Vec<Value> = resp.json().await;

            for v in raw.into_iter().rev() {
                // === RESPONSIBILITY 2: Deduplication (application) ===
                if !state.seen_hashes.check_insert(...) { continue; }

                // === RESPONSIBILITY 3: Parse raw JSON → Trade fields (domain) ===
                let wallet = v["proxyWallet"]...
                let price = v["price"]...
                let size_usd = shares * price;
                if size_usd < MIN_TRADE_USD { continue; }

                // === RESPONSIBILITY 4: Map to Trade struct (domain) ===
                let action = if v["side"] == "SELL" { SELL } else { BUY };
                let is_whale = size_usd >= WHALE_USD;  // ← whale knowledge in trade code!
                let trade = Trade { id, ts, wallet, ..., is_whale };

                // === RESPONSIBILITY 5: Resolve outcome index (infrastructure) ===
                let (outcome_index, outcome_count) = state.asset_map.get(...);

                // === RESPONSIBILITY 6: Update market price (infrastructure) ===
                state.markets[mi].outcomes[oi].price_cents = price_cents;

                // === RESPONSIBILITY 7: Update whale profile (infrastructure) ===
                let p = state.whale_profiles.entry(wallet).or_insert(...);
                p.total_trades += 1;
                p.recompute();

                // === RESPONSIBILITY 8: Store trade (infrastructure) ===
                state.recent_trades.push_front(trade);

                // === RESPONSIBILITY 9: Fire signal engine (infrastructure) ===
                run_signals_on_trade(&state, &trade);

                // === RESPONSIBILITY 10: Update stats (infrastructure) ===
                state.stats.total_trades_seen += 1;

                // === RESPONSIBILITY 11: Broadcast via WebSocket (infrastructure) ===
                state.tx.send(Ev::Trade(trade));
                state.tx.send(Ev::WhaleUpdate(profile));
                state.tx.send(Ev::WhaleAlert { ... });
            }
        }
    }

**The problems:**
1. 11 responsibilities in one function
2. Whale knowledge (`is_whale`) leaked into trade code
3. You cannot test the Trade struct without running an HTTP server
4. You cannot test the whale score formula without AppState

---

## 6. The Refactoring: Step by Step

### Step 1: Identify the Concepts

Before writing code, we identified three distinct domain concepts:

1. **Trade** — a single trade event (BUY/SELL at a price)
2. **WhaleProfile** — a wallet's intelligence profile (computed over time)
3. **is_whale_trade** — a classification of a trade (whale domain, not trade domain)

### Step 2: Extract Trade (Facts + Rules)

The `Trade` struct and `Action` enum are **domain facts**. They define
what a trade is, regardless of how the system runs.

**What moved to domain/trades.rs:**
- `Action` enum — BUY or SELL
- `Trade` struct — 19 fields defining one trade event (NO `is_whale`)
- `shorten_addr()` — helper for wallet display

**Why `is_whale` was removed from Trade:**

A trade is a fact: "I bought 500 tokens at 67 cents for $335." Whether
that's a "whale trade" is a classification based on a threshold. The trade
doesn't know about thresholds — that's whale knowledge.

Before:
    struct Trade {
        ...
        is_whale: bool,  // ← whale knowledge leaked into trade domain
    }

After:
    struct Trade {
        ...
        // no is_whale — pure trade fact
    }

    // In domain/whales.rs:
    fn is_whale_trade(trade: &Trade, threshold: f64) -> bool {
        trade.size_usd >= threshold
    }

**Why `Trade::from_raw()` was NOT added:**

Unlike `Market::from_gamma()`, the trade mapping logic is deeply coupled
to infrastructure. It needs `state.asset_map` to resolve outcome_index
and `state.seen_hashes` for deduplication. Moving the mapping to domain
would require passing AppState into the domain, violating the rule that
domain depends on nothing.

### Step 3: Extract WhaleProfile (Facts + Rules)

The `WhaleProfile` struct and its methods are **domain facts and rules**.
They define what a whale profile is and how to compute whale scores.

**What moved to domain/whales.rs:**
- `WHALE_USD` constant — the $5,000 threshold
- `is_whale_trade()` — classify whether a trade is a whale trade
- `compute_whale_tag()` — classify whale behavior from score + volumes
- `TradeWindow` struct + 6 methods — rolling 50-trade window for metrics
- `WhaleProfile` struct + `recompute()` — whale score formula

**Why `WHALE_USD` lives in whales, not application:**

Because it's a whale-domain concept. It answers "what makes a trade a
whale trade?" That's whale knowledge, not application configuration.

**Why TradeWindow went with whales:**

TradeWindow is ONLY used by WhaleProfile to compute metrics (win_rate,
avg_roi, consistency_score, top_market, specialization_score). It is a
whale intelligence tool, not a trade tool. If you change how win_rate
is calculated, you only touch `domain/whales.rs`.

**Why WhaleProfile::recompute() lives in domain:**

The whale score formula is a pure business rule:

    whale_score = win_rate × 0.35
                + roi × 0.30 × 3.0
                + consistency × 0.20
                + vol_norm × 0.15

It does not depend on HTTP, WebSocket, or AppState. You can test it with:

    let mut p = WhaleProfile { ... };
    p.recompute();
    assert!(p.whale_score >= 0.0 && p.whale_score <= 100.0);

### Step 4: Extract Ingestion (Raw Data Fetch)

The HTTP transport is an **ingestion concern**. It fetches raw data from
the Data API.

**What moved to ingestion/trades.rs:**
- `DATA_TRADES` constant — API endpoint URL
- `fetch_raw_trades()` — single HTTP GET, returns `Vec<Value>`

**Why ingestion is dumb:**

`fetch_raw_trades()` returns `Vec<serde_json::Value>`. It does not know
what a Trade is. It does not parse, filter, or deduplicate. This makes
it trivial to swap data sources — write a new ingestion function, keep
domain unchanged.

### Step 5: Extract Application (Orchestration)

The application layer holds the **config constants** and a **helper
function** for state writes.

**What moved to application/trade_service.rs:**
- `MIN_TRADE_USD` — $100 minimum trade size
- `MAX_TRADES` — 500 rolling window size
- `store_trade()` — helper to write to `state.recent_trades`

**Why task_data_trades stays in main.rs:**

The orchestration function `task_data_trades` touches `AppState` 8+ times
per trade: reading asset_map, writing markets, writing whale_profiles,
writing recent_trades, writing stats, reading stats, sending 4 WebSocket
events. Extracting it would require passing `Arc<AppState>` to the
application layer and adding a `use crate::AppState` dependency.

### Step 6: Extract API (Transport)

The REST handlers are **transport concerns**. They read state and serialize
to JSON.

**What moved to api/trades_handler.rs:**
- `h_trades()` — GET /api/trades → recent 100 trades as JSON
- `h_whales()` — GET /api/whales → top 50 whale profiles + leaderboards
- `h_stats()` — GET /api/stats → system statistics

### Step 7: Rewire main.rs

After extracting the four layers, we updated main.rs:

**Removed from main.rs:**
- `shorten_addr()` (moved to domain/trades)
- `compute_whale_tag()` (moved to domain/whales)
- `Action`, `Trade` structs (moved to domain/trades)
- `TradeWindow`, `WhaleProfile` structs (moved to domain/whales)
- `h_trades()`, `h_whales()`, `h_stats()` (moved to api)
- `DATA_TRADES`, `WHALE_USD`, `MIN_TRADE_USD`, `MAX_TRADES` constants

**Added to main.rs:**

    use domain::trades::{Trade, Action, shorten_addr};
    use domain::whales::{WhaleProfile, TradeWindow, is_whale_trade, WHALE_USD};

**Changed in main.rs:**

    // Router -- was: get(h_trades)
    .route("/api/trades", get(api::trades_handler::h_trades))
    .route("/api/whales", get(api::trades_handler::h_whales))
    .route("/api/stats",  get(api::trades_handler::h_stats))

    // Constants -- was: const DATA_TRADES = "..."
    client.get(ingestion::trades::DATA_TRADES).send().await

    // Whale check -- was: trade.is_whale
    is_whale_trade(&trade, WHALE_USD)

### Step 8: Verify Compilation

    cargo check
    Finished dev profile [unoptimized + debuginfo] target(s) in 1.38s

Clean build. Zero warnings.

---

## 7. Before vs After

### File Count

|                        | BEFORE (after markets) | AFTER (after trades) |
|------------------------|------------------------|----------------------|
| Total files            | 9                      | 14                   |
| Lines in main.rs       | 2,110                  | 1,759                |
| New files              | --                     | 5                    |

### Domain Layer Split

| BEFORE (all in main.rs)              | AFTER                                    |
|--------------------------------------|------------------------------------------|
| Action, Trade (132-157)              | domain/trades.rs (76 lines)              |
| TradeWindow (162-212)                | domain/whales.rs (115 lines)             |
| WhaleProfile (214-262)               | domain/whales.rs (115 lines)             |
| shorten_addr (69-71)                 | domain/trades.rs (76 lines)              |
| compute_whale_tag (83-94)            | domain/whales.rs (115 lines)             |
| is_whale (inline in task_data_trades)| domain/whales.rs::is_whale_trade()       |
| WHALE_USD constant (64)              | domain/whales.rs (115 lines)             |

### The `is_whale` Journey

This is the most important change to understand:

| Step | What happened |
|------|---------------|
| **Before** | `is_whale: bool` field on Trade struct |
| **Problem** | Whale knowledge leaked into trade domain |
| **Solution** | Remove field, add `is_whale_trade()` function in whale domain |
| **After** | Trade is a pure fact. Whale classification is a function call. |

Before:
    let trade = Trade { ..., is_whale: size_usd >= 5000.0, ... };
    if trade.is_whale { ... }

After:
    let trade = Trade { ..., /* no is_whale */ };
    if is_whale_trade(&trade, WHALE_USD) { ... }

### Responsibility Distribution

| Responsibility              | BEFORE (main.rs)              | AFTER                              |
|-----------------------------|-------------------------------|------------------------------------|
| Action enum                 | main.rs line 132              | domain/trades.rs line 10           |
| Trade struct                | main.rs line 135              | domain/trades.rs line 13           |
| shorten_addr()              | main.rs line 69               | domain/trades.rs line 6            |
| WHALE_USD constant          | main.rs line 64               | domain/whales.rs line 6            |
| is_whale_trade()            | inline in task_data_trades    | domain/whales.rs line 8            |
| compute_whale_tag()         | main.rs line 83               | domain/whales.rs line 14           |
| TradeWindow struct + impl   | main.rs line 162              | domain/whales.rs line 23           |
| WhaleProfile struct + impl  | main.rs line 214              | domain/whales.rs line 75           |
| fetch_raw_trades()          | inline in task_data_trades    | ingestion/trades.rs line 5         |
| DATA_TRADES constant        | main.rs line 53               | ingestion/trades.rs line 1         |
| MIN_TRADE_USD constant      | main.rs line 65               | application/trade_service.rs line 5 |
| MAX_TRADES constant         | main.rs line 62               | application/trade_service.rs line 6 |
| h_trades()                  | main.rs line 1751             | api/trades_handler.rs line 7       |
| h_whales()                  | main.rs line 1754             | api/trades_handler.rs line 11      |
| h_stats()                   | main.rs line 1761             | api/trades_handler.rs line 19      |

### Testability

| Test Case                   | BEFORE                              | AFTER                                |
|-----------------------------|-------------------------------------|--------------------------------------|
| Test Trade struct           | Impossible — coupled to AppState    | cargo test on domain::trades         |
| Test is_whale_trade()       | Impossible — inline in task loop    | cargo test on domain::whales         |
| Test whale score formula    | Impossible — inside WhaleProfile    | cargo test on domain::whales         |
| Test win_rate/avg_roi       | Impossible — inside TradeWindow     | cargo test on domain::whales         |
| Test HTTP fetch             | Impossible — mixed with parsing     | cargo test on ingestion::trades      |
| Test REST handlers          | Impossible — mixed with state       | Mock state, test api::trades_handler |

### Change Isolation

| Change                          | BEFORE (risk)                      | AFTER (risk)                          |
|---------------------------------|------------------------------------|---------------------------------------|
| Data API URL changes            | Edit main.rs (2,110 lines)         | Edit ingestion/trades.rs (11 lines)   |
| Whale threshold changes         | Edit main.rs (2,110 lines)         | Edit domain/whales.rs (115 lines)     |
| Whale score formula changes     | Edit main.rs (2,110 lines)         | Edit domain/whales.rs (115 lines)     |
| Minimum trade size changes      | Edit main.rs (2,110 lines)         | Edit application/trade_service.rs     |
| Add new trade field             | Edit main.rs (2,110 lines)         | Edit domain/trades.rs (76 lines)      |
| Add new trade REST endpoint     | Edit main.rs (2,110 lines)         | Add to api/trades_handler.rs          |
| Swap Data API for another       | Rewrite task_data_trades           | Write new ingestion function          |

---

## 8. Why Each Decision Was Made

### Why Trade and WhaleProfile are separate files

**Decision:** `Trade` lives in `domain/trades.rs`. `WhaleProfile` lives in
`domain/whales.rs`.

**Reason:** They are different domain concepts with different rules:
- Trade = a single event (immutable once created)
- WhaleProfile = an aggregated profile (recomputed on each trade)

If you change how whale scores are calculated, you only touch `whales.rs`.
If you add a field to Trade, you only touch `trades.rs`. They can evolve
independently.

### Why is_whale was removed from Trade

**Decision:** `is_whale` was a field on Trade. Now it's a function in whales.

**Reason:** A trade is a fact: "I bought 500 tokens at 67 cents for $335."
Whether that's a "whale trade" depends on a threshold. The trade doesn't
know about thresholds — that's whale knowledge.

Think of it like a student grade:
- **Student knows:** "I scored 85 on the test"
- **School knows:** "Scores above 90 are A's"
- **Student does NOT know:** "I got an A" — that's the school's classification

Similarly:
- **Trade knows:** "I am $335"
- **Whale domain knows:** "Trades above $5,000 are whale trades"
- **Trade does NOT know:** "I am a whale trade"

### Why WHALE_USD lives in whales, not application

**Decision:** `WHALE_USD` ($5,000) lives in `domain/whales.rs`, not
`application/trade_service.rs`.

**Reason:** It's a whale-domain concept. It answers "what makes a trade
a whale trade?" That's whale knowledge, not application configuration.

`MIN_TRADE_USD` ($100) stays in application because it's a processing
threshold — "should we process this trade at all?" That's orchestration.

### Why domain owns WhaleProfile::recompute()

**Decision:** The whale score formula lives in domain, not in main.rs.

**Reason:** The formula is a business rule. It does not depend on HTTP,
WebSocket, or AppState. If you want to change the weight of win_rate from
0.35 to 0.40, you edit domain/whales.rs. You do not touch main.rs.

### Why ingestion is dumb on purpose

**Decision:** `fetch_raw_trades()` returns `Vec<Value>` — no parsing, no
filtering, no state.

**Reason:** If ingestion knows about Trade, then swapping the Data API
means rewriting parsing logic. With dumb ingestion, you just write a new
fetch function that returns the same raw format.

### Why task_data_trades stays in main.rs

**Decision:** The 200-line trade ingestion loop stays in infrastructure.

**Reason:** It touches `AppState` 8+ times per trade. Extracting it would
require either:
1. Passing `Arc<AppState>` to the application layer (adds dependency)
2. Splitting it into 10+ tiny functions (over-engineering)

---

## 9. Where Functions Live and Why

This is the most important section for learning architecture. Every function
lives somewhere for a reason.

### `polymarket_url` → `domain/markets.rs`

    // domain/markets.rs
    pub fn polymarket_url(event_slug: &str, market_slug: &str) -> String {
        format!("https://polymarket.com/event/{event_slug}#{market_slug}")
    }

**Why:** It's a fact about markets. A Market has a URL. The URL is built
from slugs. That's market knowledge.

Where else could it live?

| Location | Problem |
|----------|---------|
| `ingestion/markets.rs` | Ingestion is dumb — it only fetches raw data |
| `api/markets_handler.rs` | API is transport — it formats output |
| `main.rs` | That's what we're escaping |

**The rule:** If you change the URL format, you edit `domain/markets.rs`.

### `shorten_addr` → `domain/trades.rs`

    // domain/trades.rs
    pub fn shorten_addr(s: &str) -> String {
        if s.len() <= 12 { s.to_string() } else { format!("{}…{}", &s[..6], &s[s.len()-4..]) }
    }

**Why:** It's a fact about how trades display wallets. A Trade has a
`wallet_short` field. The way you compute it is trade knowledge.

**The rule:** If you change how wallet addresses are displayed, you edit
`domain/trades.rs`.

### `is_whale_trade` → `domain/whales.rs`

    // domain/whales.rs
    pub fn is_whale_trade(trade: &Trade, threshold: f64) -> bool {
        trade.size_usd >= threshold
    }

**Why:** It's a whale classification. It answers "is this trade a whale
trade?" That's whale knowledge.

Note: This imports `Trade` from `domain/trades.rs`. That's a **domain →
domain** dependency, which is allowed. The whale module reads Trade.size_usd
to make its classification.

### `compute_whale_tag` → `domain/whales.rs`

    // domain/whales.rs
    pub fn compute_whale_tag(ws: f64, buy_vol: f64, sell_vol: f64, total_vol: f64) -> &'static str {
        match () {
            _ if ws >= 88.0 => "APEX PREDATOR",
            _ if ws >= 74.0 => "ALPHA HUNTER",
            // ...
        }
    }

**Why:** It classifies whale behavior. That's whale knowledge.

### `fetch_raw_trades` → `ingestion/trades.rs`

    // ingestion/trades.rs
    pub async fn fetch_raw_trades(client: &reqwest::Client) -> Vec<serde_json::Value> {
        // just HTTP, no parsing
    }

**Why:** It fetches raw data from an external API. That's ingestion.

### `store_trade` → `application/trade_service.rs`

    // application/trade_service.rs
    pub fn store_trade(state: &AppState, trade: Trade) {
        let mut td = state.recent_trades.lock().unwrap();
        td.push_front(trade);
        if td.len() > MAX_TRADES { td.pop_back(); }
    }

**Why:** It orchestrates a state write. That's application.

### `h_trades` → `api/trades_handler.rs`

    // api/trades_handler.rs
    pub async fn h_trades(State(s): State<Arc<AppState>>) -> Json<serde_json::Value> {
        Json(serde_json::json!({ "trades": s.recent_trades.lock().unwrap()... }))
    }

**Why:** It reads state and serializes to JSON for an HTTP response. That's
transport.

---

## 10. What Each New File Does

### src/domain/trades.rs (76 lines)

Contains:
- `shorten_addr()` — truncate wallet address for display
- `Action` enum — BUY or SELL
- `Trade` struct — 19 fields defining one trade event
- `Trade::from_raw()` — NOT added (mapping stays in main.rs due to AppState coupling)

**Depends on:** `serde`, `chrono`, `domain::markets::polymarket_url`
**Depends on:** nothing external in this crate

### src/domain/whales.rs (115 lines)

Contains:
- `WHALE_USD` — $5,000 whale threshold
- `is_whale_trade()` — classify whether a trade is a whale trade
- `compute_whale_tag()` — classify whale behavior from score + volumes
- `TradeWindow` struct — rolling 50-trade window
  - `push()` — add trade to window
  - `win_rate()` — % of profitable trades
  - `avg_roi()` — average return on investment
  - `consistency_score()` — coefficient of variation
  - `top_market()` — most traded market
  - `specialization_score()` — how focused on one market
- `WhaleProfile` struct — wallet intelligence profile
  - `recompute()` — calculate whale_score, whale_tag from window

**Depends on:** `serde`, `domain::trades::Trade`
**Depends on:** nothing external in this crate

### src/ingestion/trades.rs (11 lines)

Contains:
- `DATA_TRADES` constant — Polymarket Data API endpoint
- `fetch_raw_trades()` — single HTTP GET, returns `Vec<Value>`

**Depends on:** `reqwest` only

### src/application/trade_service.rs (14 lines)

Contains:
- `MIN_TRADE_USD` — $100 minimum trade size
- `MAX_TRADES` — 500 rolling window size
- `store_trade()` — write trade to `state.recent_trades`

**Depends on:** `domain::trades`, `AppState`

### src/api/trades_handler.rs (21 lines)

Contains:
- `h_trades()` — GET /api/trades → recent 100 trades
- `h_whales()` — GET /api/whales → top 50 profiles + leaderboards
- `h_stats()` — GET /api/stats → system statistics

**Depends on:** `axum`, `AppState`

---

## 11. What Is Still in main.rs

The 1,759 lines remaining in main.rs contain these trade-related components:

| Component                    | Lines  | Target File (future)  |
|------------------------------|--------|-----------------------|
| task_data_trades()           | ~200   | application/trade_service.rs (when AppState coupling reduced) |
| run_signals_on_trade()       | ~400   | engine/signals.rs     |
| Signal structs + enums       | ~100   | domain/signals.rs     |
| Book structs (Level, etc.)   | ~80    | domain/books.rs       |
| Book tasks + assembler       | ~100   | ingestion/books.rs    |
| WebSocket tasks              | ~100   | ingestion/ws.rs       |
| Global signal computation    | ~120   | engine/global_signals.rs |
| Leaderboard tasks            | ~60    | ingestion/leaderboard.rs |
| Edge feed computation        | ~100   | engine/edge_feed.rs   |
| Auth handlers                | ~100   | api/auth.rs           |
| AppState + helpers           | ~80    | state/mod.rs          |
| REST handlers (signals, etc.)| ~50    | api/signals_handler.rs |

The pattern is the same for each extraction:
1. Identify the responsibility boundary
2. Move facts to domain
3. Move raw data fetching to ingestion
4. Move orchestration to application
5. Move transport to api
6. Leave infrastructure plumbing in main.rs

---

## 12. Dependency Direction

### The Arrow Rule (Unchanged)

Dependencies point **inward** toward the domain:

    api --> application --> domain <-- ingestion
                            ^
                            |
                       infrastructure

**New trade files follow this rule:**

    // domain/trades.rs — depends on nothing in this crate (only serde, chrono)
    use super::markets::polymarket_url;  // domain → domain (allowed)

    // domain/whales.rs — depends on domain/trades (for Trade struct)
    use super::trades::Trade;  // domain → domain (allowed)

    // ingestion/trades.rs — depends on nothing in this crate
    // (only reqwest)

    // application/trade_service.rs — depends on domain + ingestion
    use crate::domain::trades::Trade;
    use crate::ingestion::trades as data;

    // api/trades_handler.rs — depends on AppState only
    use crate::AppState;

### Domain → Domain Dependencies Are Allowed

`domain/whales.rs` imports `Trade` from `domain/trades.rs`. This is
allowed because both are in the domain layer. The whale module needs to
see Trade.size_usd to classify whale trades.

The rule is: **domain modules can reference each other, but cannot
reference ingestion, application, or infrastructure.**

---

## 13. Common Beginner Questions

### Q: Why wasn't Trade::from_raw() added like Market::from_gamma()?

Because `task_data_trades` needs `state.asset_map` to resolve outcome_index
and `state.seen_hashes` for deduplication. These are infrastructure
concerns. Adding them to domain would require passing `Arc<AppState>` into
the domain, violating the rule that domain depends on nothing.

### Q: Why was is_whale removed from Trade?

Because "is this a whale trade?" is a whale-domain question, not a trade
fact. A trade is: "I bought 500 tokens at 67 cents for $335." Whether
that's a whale trade depends on a threshold. The trade doesn't know about
thresholds.

### Q: Where does the whale threshold live?

In `domain/whales.rs` as `WHALE_USD = 5_000.0`. It's a whale-domain
concept because it answers "what makes a trade a whale trade?"

### Q: Why do MIN_TRADE_USD and MAX_TRADES live in application?

Because they're orchestration thresholds:
- `MIN_TRADE_USD` — "should we process this trade at all?" (application)
- `MAX_TRADES` — "how many trades to keep in the rolling window?" (config)

They're not domain facts or rules — they configure how the pipeline runs.

### Q: Could task_data_trades be extracted next?

Yes, but it requires reducing the AppState coupling. Options:
1. Extract the per-trade state updates into a `TradeProcessor` struct
2. Use dependency injection (pass state as a parameter)
3. Split into small functions that each touch one piece of state

The signal engine (`run_signals_on_trade`) is a cleaner next target.

### Q: What happens if I need to add a new trade field?

Edit `domain/trades.rs` — add the field to the `Trade` struct. The change
propagates automatically:
- `task_data_trades` constructs Trades, so update the construction site
- `api/trades_handler` serializes Trades, so it picks up the new field
- WebSocket clients receive the new field via `Ev::Trade`

### Q: Can I test the whale score formula without running the server?

Yes. `domain/whales.rs` depends on nothing internal (only serde and
domain::trades). You can write:

    #[test]
    fn test_whale_score() {
        let mut p = WhaleProfile {
            total_volume: 100_000.0,
            buy_volume: 80_000.0,
            sell_volume: 20_000.0,
            window: TradeWindow::default(),
            ..Default::default()
        };
        p.window.push(5000.0, 100.0, "market-a");
        p.recompute();
        assert!(p.whale_score > 0.0);
    }

No HTTP server, no AppState, no Tokio runtime needed.

### Q: What is the difference between WHALE_USD and whale_score?

- `WHALE_USD` ($5,000) — binary threshold. A trade is either a whale trade
  or it is not. Used to trigger `Ev::WhaleAlert`.
- `whale_score` (0-100) — continuous score based on win rate, ROI,
  consistency, and volume. Used to rank whales and classify behavior.

### Q: Why did TradeWindow go with whales instead of trades?

Because TradeWindow is ONLY used by WhaleProfile to compute metrics. It
is a whale intelligence tool. If you change how win_rate is calculated,
you only touch `domain/whales.rs`. Trade does not use TradeWindow at all.

### Q: Why does domain/whales.rs import from domain/trades.rs?

Because `is_whale_trade()` needs to read `trade.size_usd`. The whale
module classifies trades, so it needs to see the Trade struct. This is a
domain → domain dependency, which is allowed.

---

# THIRD EXTRACTION SESSION — Books, Signals, Auth

This section covers the third extraction session. We extracted three more
chunks from main.rs:

1. **Books domain structs** → `domain/books.rs`
2. **Signal/stats domain structs** → `domain/signals.rs`
3. **Auth handlers + supabase logic** → `api/auth.rs`

---

## What We Started With (After Trades Extraction)

After the trades extraction, main.rs was **1,946 lines**. Three clean
chunks were sitting there waiting to be extracted:

    Lines 82-112     Level, OutcomeBook, MarketBook structs (book domain)
    Lines 116-337    EdgeSignal, GlobalSignals, Stats, LeaderboardEntry (signal domain)
    Lines 340-518    Supabase license functions + auth handlers
    Lines 1748-1754  EdgeFeedEvent struct (signal domain)

---

## The Three Extractions

### Extraction 1: Books Domain (domain/books.rs)

**What moved:**
- `Level` struct — one price level in the order book
- `OutcomeBook` struct — one outcome's full book (bids, asks, liquidity)
- `MarketBook` struct — a market's complete order book

**Why these are domain facts:**

An order book is a snapshot of market depth. It answers: "What are people
willing to buy and sell, and at what prices?" That's a fact about markets.

Think of it like a restaurant menu:
- **Level** = one item on the menu (price + size)
- **OutcomeBook** = one section of the menu (all items for one outcome)
- **MarketBook** = the complete menu (all outcomes for one market)

The menu doesn't know about the kitchen (HTTP), the waiter (WebSocket),
or the manager (AppState). It just lists what's available.

**Why Level, OutcomeBook, MarketBook are separate from Market:**

Market (in `domain/markets.rs`) defines what a market IS — its name, slug,
outcomes, and probability. MarketBook defines the ORDER BOOK for that market.
They're related but different concepts:

| Concept | What it knows | Changes over time |
|---------|---------------|-------------------|
| Market | name, slug, outcomes, probability | Yes (price changes) |
| MarketBook | bids, asks, liquidity, spread | Yes (book updates every 4s) |

If you change how the menu looks, you edit `domain/markets.rs`.
If you change how the order book is displayed, you edit `domain/books.rs`.

### Extraction 2: Signal/Stats Domain (domain/signals.rs)

**What moved:**
- `SignalKind` enum — 8 signal types (SmartCluster, VelocitySurge, etc.)
- `EdgeSignal` struct — a fired signal with confidence, priority, action
- `EdgeSignal::priority_from_confidence()` — confidence → priority mapping
- `SignalAccuracy` struct — signal performance tracking
- `GlobalSignals` struct — composite market signals (momentum, sentiment, etc.)
- `Stats` struct — system statistics (volume, trade count, whale count)
- `LeaderboardEntry` struct — a ranked whale on the leaderboard
- `EdgeFeedEvent` struct — edge score with direction and strength

**Why these are domain facts:**

Each of these is a fact about the system:

| Struct | What it is | Why it's a fact |
|--------|-----------|-----------------|
| EdgeSignal | A fired signal | "Signal X fired at time T with confidence 84" |
| GlobalSignals | Composite market state | "Momentum=72, sentiment=65, whale_flow=41" |
| Stats | System statistics | "1,247 trades seen, $2.3M volume" |
| LeaderboardEntry | Ranked whale | "Rank #3, address 0x7f3a, PnL +$12,400" |
| EdgeFeedEvent | Edge computation result | "Score=8.2, BULLISH, STRONG, EXECUTE" |

**Why EdgeSignal has a method (priority_from_confidence):**

Because the mapping from confidence to priority is a business rule:

    0-39   → LOW
    40-64  → MEDIUM
    65-84  → HIGH
    85-100 → CRITICAL

This is domain knowledge. If you change the thresholds, you edit
`domain/signals.rs`. You do not touch main.rs.

**Why Stats lives in domain, not application:**

Stats is a fact about what the system has observed. It's not orchestration
(it doesn't decide what to do). It's not transport (it doesn't format
output). It's a pure data structure that holds numbers.

**Why LeaderboardEntry lives in domain, not ingestion:**

The ingestion layer fetches raw leaderboard data from the API. But the
LeaderboardEntry struct defines what a leaderboard entry IS — its fields,
its serialization. That's domain knowledge.

### Extraction 3: Auth Handlers (api/auth.rs)

**What moved:**
- `supabase_base()` — normalize Supabase URL
- `supabase_key()` — read API key from env
- `SupabaseLicense` struct — license record from database
- `url_encode()` — encode strings for URL parameters
- `db_insert_license()` — insert license into Supabase
- `db_lookup_license()` — lookup license from Supabase
- `h_validate()` — GET /validate → check if license is valid
- `h_payment_info()` — GET /api/payment-info → payment instructions
- `h_gen_key()` — GET /admin/gen-key → generate new license key

**Why these are transport (api layer):**

Auth handlers read state and return JSON. They don't define business rules.
They don't fetch raw data. They just connect HTTP requests to the rest
of the system.

Think of it like a hotel front desk:
- **Guest asks:** "Is my room key valid?" → front desk checks the system
- **Guest asks:** "How do I pay?" → front desk hands them a brochure
- **Manager asks:** "Generate a new key" → front desk creates one

The front desk doesn't know how locks work (domain). It doesn't know
how the database stores keys (ingestion). It just handles requests.

**Why supabase functions went with auth:**

The supabase functions (`supabase_base`, `supabase_key`, `db_insert_license`,
`db_lookup_license`) are ONLY used by the auth handlers. They're not general
infrastructure — they're specific to the licensing system.

If you change the database, you edit `api/auth.rs`. You don't touch
main.rs, ingestion, or domain.

**Why url_encode went with auth:**

`url_encode()` is a helper used by `db_lookup_license()` to encode the
license key in the URL. It's not a general utility — it's specific to
the supabase query.

If you need URL encoding elsewhere, you can always move it to a shared
utility later. For now, it lives where it's used.

---

## The Pattern: What Gets Extracted Each Time

After three extractions, a clear pattern emerges:

| Step | Question to ask | Result |
|------|----------------|--------|
| 1 | "Is this a fact about the world?" | → domain/ |
| 2 | "Is this raw data from an API?" | → ingestion/ |
| 3 | "Is this orchestration (connects pieces)?" | → application/ |
| 4 | "Is this an HTTP handler?" | → api/ |
| 5 | "Does this touch AppState 5+ times?" | → stays in main.rs (for now) |

**The key insight:** Most code is either facts (domain) or handlers (api).
Orchestration (application) and raw data (ingestion) are thin layers.
Infrastructure (main.rs) is the ugly glue that holds everything together.

---

## Before vs After (All Three Sessions)

### Line Count Progression

| Stage | main.rs lines | Total files |
|-------|---------------|-------------|
| Original monolith | 2,349 | 1 |
| After markets extraction | 2,110 | 9 |
| After trades extraction | 1,946 | 14 |
| After books + signals + auth | **1,591** | **17** |

### What Each File Contains Now

    domain/
    ├── books.rs      (42 lines)   Level, OutcomeBook, MarketBook, RawBook
    ├── markets.rs    (142 lines)  Market, Outcome, from_gamma, is_live, polymarket_url
    ├── signals.rs    (84 lines)   EdgeSignal, GlobalSignals, Stats, EdgeFeedEvent
    ├── trades.rs     (74 lines)   Action, Trade, shorten_addr
    └── whales.rs     (130 lines)  WhaleProfile, TradeWindow, is_whale_trade, WHALE_USD, LeaderboardEntry

    ingestion/
    ├── markets.rs    (35 lines)   fetch_raw_markets
    └── trades.rs     (12 lines)   fetch_raw_trades

    application/
    ├── market_service.rs  (49 lines)  refresh_markets, task_refresh_markets
    └── trade_service.rs   (16 lines)  store_trade, constants

    api/
    ├── auth.rs             (214 lines)  h_validate, h_payment_info, h_gen_key + supabase
    ├── markets_handler.rs  (44 lines)   h_markets, h_books, h_heatmap, h_scanner
    └── trades_handler.rs   (21 lines)   h_trades, h_whales, h_stats

    engine/
    ├── mod.rs            (161 lines)  orchestrator
    ├── detect.rs         (301 lines)  8 pure detection functions
    └── state.rs          (38 lines)   BookSnapshot, MarketSignalState, SignalDedup

### What main.rs Still Contains (1,591 lines)

The remaining code in main.rs is infrastructure — code that touches
AppState many times and cannot be easily extracted yet:

| Component | Lines | Why it stays |
|-----------|-------|--------------|
| task_data_trades | ~200 | Touches AppState 8+ times per trade |
| WebSocket tasks | ~100 | Manages connections, reads state |
| CLOB book tasks | ~100 | Fetches + updates state |
| Edge feed computation | ~70 | Reads multiple state fields |
| AppState + helpers | ~120 | Core state definition |
| REST handlers (signals, etc.) | ~50 | Reads state, returns JSON |
| main() + router | ~80 | Wiring + startup |
| Other tasks | ~200 | Leaderboard, heartbeat, etc. |

---

## Common Beginner Questions (Updated)

### Q: Why didn't you extract everything at once?

Because architecture is about **boundaries**, not about moving code. Each
extraction establishes one clear boundary:

- Markets extraction: "What is a market?" → `domain/markets.rs`
- Trades extraction: "What is a trade?" → `domain/trades.rs`
- Books extraction: "What is an order book?" → `domain/books.rs`
- Signals extraction: "What is a signal?" → `domain/signals.rs`
- Auth extraction: "How do we handle licenses?" → `api/auth.rs`

If you extract everything at once, you don't learn the boundaries. The
value is in understanding WHY each piece goes where it goes.

### Q: How do I know if something is a "domain fact"?

Ask: "Does this depend on HTTP, WebSocket, or AppState?"

- If NO → it's a domain fact
- If YES → it's infrastructure

Examples:
- `Trade { id: 1, wallet: "0x7f3a", size_usd: 500.0 }` → domain (pure data)
- `state.recent_trades.lock().unwrap()` → infrastructure (touches state)
- `client.get(DATA_TRADES).send().await` → ingestion (HTTP fetch)
- `Json(serde_json::json!({ "trades": ... }))` → api (HTTP response)

### Q: Why are there so many tiny files now?

Because each file has ONE responsibility. The alternative is one big file
with many responsibilities. The tradeoff is:

| Approach | Pros | Cons |
|----------|------|------|
| One big file | Easy to find things | Hard to change one thing without breaking others |
| Many small files | Easy to change one thing | Hard to find things (need to know the structure) |

For a learning project, many small files is better because you can see
exactly where each concept lives.

### Q: What's left to extract from main.rs?

The signal engine was the last "easy" extraction. The remaining code
is infrastructure that genuinely needs AppState:

| Component | Lines | Difficulty |
|-----------|-------|------------|
| task_data_trades | ~200 | Hard — touches AppState 8+ times |
| WebSocket tasks | ~100 | Medium — infrastructure by nature |
| CLOB book tasks | ~100 | Medium — infrastructure by nature |
| Edge feed computation | ~70 | Medium — reads multiple state fields |
| AppState + helpers | ~120 | Hard — core state definition |
| REST handlers | ~50 | Easy — pure transport |
| main() + router | ~80 | Easy — pure wiring |

To extract further, you'd need to restructure AppState itself (dependency
injection, trait objects, or a service locator pattern).

### Q: When should I stop extracting?

When the remaining code in main.rs is all infrastructure that genuinely
needs to touch AppState. The goal isn't to make main.rs zero lines —
it's to make main.rs ONLY contain infrastructure glue.

Currently main.rs has ~1,141 lines of infrastructure. Ideally it would
have ~500-800 lines of pure wiring + state definition. The remaining
code is mostly infrastructure that touches AppState many times.

### Q: How does this help me in real projects?

The pattern scales:

| Project size | Approach |
|--------------|----------|
| Small (< 1K lines) | One file is fine |
| Medium (1K-10K lines) | Extract domain + api |
| Large (10K-100K lines) | Full DDD with all layers |
| Huge (100K+ lines) | Multiple bounded contexts (microservices) |

The skill is knowing WHERE to draw the boundaries. This project teaches
you that by doing it step by step.

---

# FOURTH EXTRACTION SESSION — Signal Engine (engine/)

19. What We Started With (After Third Extraction)
20. Why the Signal Engine Was Hard
21. The Architecture: Orchestrator + Pure Detectors
22. What Moved and Why
23. Before vs After (All Four Sessions)
24. The `engine/` Module Walkthrough
25. Common Beginner Questions (Signal Engine)

---

## 19. What We Started With (After Third Extraction)

After the books, signals, and auth extractions, main.rs was **1,591 lines**.
The biggest remaining chunk was the signal engine — a 420-line function called
`run_signals_on_trade` that runs 8 different detection algorithms after every
trade:

    Lines 333-751    run_signals_on_trade() — 420 lines, 8 signals
    Lines 216-248    BookSnapshot, MarketSignalState, SignalDedup — internal state types
    Lines 206-213    RawBook — raw CLOB book struct

**The problem was different from before.** Previous extractions moved domain
facts (structs, enums, functions). The signal engine is different — it's
BUSINESS LOGIC that needs to read and write state. It's not a pure fact, and
it's not dumb transport. It sits in between.

---

## 20. Why the Signal Engine Was Hard

Previous extractions were straightforward:

| Extraction | What moved | Difficulty |
|------------|-----------|------------|
| Markets | Struct + from_gamma() | Easy — pure data |
| Trades | Struct + helpers | Easy — pure data |
| Whales | Profile + score formula | Easy — pure logic |
| Books | Structs | Easy — pure data |
| Auth | HTTP handlers | Easy — pure transport |

The signal engine is different because each signal function needs to:

1. **READ** from multiple state fields (recent_trades, whale_profiles, markets, raw_books)
2. **WRITE** to per-market mutable state (volume counters, book snapshots, dedup maps)
3. **FIRE** signals through the event system

This is **infrastructure-coupled logic**. It's not pure domain, and it's
not pure transport. It sits in the middle — business logic that depends on
state.

### The Beginner's Dilemma

    "Should I move it to domain? No — it needs state.
     Should I leave it in main.rs? It's 420 lines!
     What do I do?"

The answer: **Create a new layer** — the `engine/` module.

---

## 21. The Architecture: Orchestrator + Pure Detectors

The solution was to split the 420-line function into two parts:

### Part 1: Pure Detection Functions (engine/detect.rs)

Each signal gets its own function that takes **explicit data inputs** — no
AppState, no Mutex, no locks. Just plain data:

    fn detect_whale_print(trade: &Trade, whale_threshold: f64)
        -> Option<SignalOutput>

    fn detect_smart_cluster(
        trade: &Trade,
        profiles: &HashMap<String, WhaleProfile>,
        recent_trades: &VecDeque<Trade>,
        now_ms: i64,
    ) -> Option<SignalOutput>

    fn detect_velocity_surge(
        trade: &Trade,
        mkt_state: &mut MarketSignalState,
        now_ms: i64,
    ) -> Option<SignalOutput>

**What each function receives:**

| Function | Needs | Why |
|----------|-------|-----|
| whale_print | trade, threshold | "Is this trade big?" — pure comparison |
| smart_cluster | trade, profiles, recent_trades | "Are 3+ alpha wallets buying?" — pure data |
| conviction_spike | trade, profiles | "Is this bet 3× the wallet's avg?" — pure math |
| whale_reversal | trade, profiles, prev_action | "Did the wallet flip direction?" — pure comparison |
| velocity_surge | trade, mkt_state | "Is 1-min volume > 4× avg?" — pure math |
| stealth_accum | trade, mkt_state | "Same wallet, 3+ buys, <2¢ drift?" — pure math |
| prob_divergence | trade, recent_trades, prob_change | "Whales buying but price falling?" — pure math |
| liquidity_drain | trade, raw_book_asks, mkt_state | "Ask book thinned >40%?" — pure math |
| momentum_break | trade, prob, volume, mkt_state | "Prob crossed 25/50/75?" — pure comparison |

**The key insight:** Each function is a **pure function** — it takes data
in, returns a signal or None. It doesn't know about AppState, Mutex, or
locks. You can test it with:

    let trade = Trade { size_usd: 50_000.0, ... };
    let result = detect_whale_print(&trade, 5_000.0);
    assert!(result.is_some());
    assert_eq!(result.unwrap().kind, "WHALE_PRINT");

No HTTP server, no WebSocket, no Tokio runtime.

### Part 2: The Orchestrator (engine/mod.rs)

The orchestrator does the "dirty work" — it locks state, extracts the data
each detector needs, calls the detector, and fires the signal:

    pub fn run_signals_on_trade(
        trade: &Trade,
        profiles: &HashMap<String, WhaleProfile>,
        recent_trades: &VecDeque<Trade>,
        markets: &Vec<Market>,
        raw_books: &HashMap<String, RawBook>,
        mkt_signal_state: &mut HashMap<String, MarketSignalState>,
        signal_dedup: &mut SignalDedup,
        wallet_prev_action: &mut HashMap<String, (String, Action)>,
        whale_threshold: f64,
        fire_signal: &dyn Fn(EdgeSignal),
    ) {
        // For each detector:
        //   1. Prepare the data it needs
        //   2. Call the detector
        //   3. Check dedup
        //   4. Fire the signal
    }

**The orchestrator knows about state.** It locks mutexes, reads HashMaps,
and writes to per-market state. But the detectors don't. The orchestrator
is the **only place** that touches infrastructure.

Think of it like a restaurant kitchen:

- **Orchestrator** = the head chef (coordinates, reads orders, assigns tasks)
- **Detectors** = the line cooks (each does one task perfectly, no coordination)

The head chef knows about the kitchen (state). The line cooks only know
about their station (their inputs).

### Part 3: Internal State Types (engine/state.rs)

Some signals need per-market mutable state that doesn't exist in AppState:

    struct BookSnapshot { ask_liq: f64, ts: i64 }
    struct MarketSignalState { vol_1m, vol_60m, accum_buys, book_snaps, ... }
    struct SignalDedup { map: HashMap<String, i64> }

These are **engine-internal** — they don't belong in domain (they're not
facts about the world) and they don't belong in main.rs (they're signal
engine implementation details).

---

## 22. What Moved and Why

### Engine Layer (New)

| What | Why |
|------|-----|
| `engine/mod.rs` (161 lines) | Orchestrator — locks state, calls detectors, fires signals |
| `engine/detect.rs` (301 lines) | 8 pure detection functions — no state dependencies |
| `engine/state.rs` (38 lines) | Internal state types — per-market signal tracking |

### Domain Layer (Updated)

| What | Why |
|------|-----|
| `domain/books.rs` — added `RawBook` | Raw CLOB book struct — needed by both main.rs and engine |

### main.rs (Updated)

| What | Why |
|------|-----|
| Removed `run_signals_on_trade` (~420 lines) | Moved to engine module |
| Removed `BookSnapshot`, `MarketSignalState`, `SignalDedup` (~30 lines) | Moved to engine/state.rs |
| Removed `RawBook` struct (~8 lines) | Moved to domain/books.rs |
| Added thin wrapper (~20 lines) | Locks AppState, calls engine function |

---

## 23. Before vs After (All Four Sessions)

### Line Count Progression

| Stage | main.rs lines | Total files | Engine lines |
|-------|---------------|-------------|--------------|
| Original monolith | 2,349 | 1 | — |
| After markets extraction | 2,110 | 9 | — |
| After trades extraction | 1,946 | 14 | — |
| After books + signals + auth | 1,591 | 17 | — |
| **After signal engine extraction** | **1,141** | **20** | **500** |

**main.rs lost 450 lines.** The signal engine became 500 lines across 3 new files.

### What Each File Contains Now

    domain/
    ├── books.rs      (42 lines)   Level, OutcomeBook, MarketBook, RawBook
    ├── markets.rs    (142 lines)  Market, Outcome, from_gamma, is_live, polymarket_url
    ├── signals.rs    (84 lines)   EdgeSignal, GlobalSignals, Stats, EdgeFeedEvent
    ├── trades.rs     (74 lines)   Action, Trade, shorten_addr
    └── whales.rs     (130 lines)  WhaleProfile, TradeWindow, is_whale_trade, WHALE_USD, LeaderboardEntry

    ingestion/
    ├── markets.rs    (35 lines)   fetch_raw_markets
    └── trades.rs     (12 lines)   fetch_raw_trades

    application/
    ├── market_service.rs  (49 lines)  refresh_markets, task_refresh_markets
    └── trade_service.rs   (16 lines)  store_trade, constants

    api/
    ├── auth.rs             (214 lines)  h_validate, h_payment_info, h_gen_key + supabase
    ├── markets_handler.rs  (44 lines)   h_markets, h_books, h_heatmap, h_scanner
    └── trades_handler.rs   (21 lines)   h_trades, h_whales, h_stats

    engine/                     ← NEW LAYER
    ├── mod.rs            (161 lines)  orchestrator — locks state, calls detectors
    ├── detect.rs         (301 lines)  8 pure detection functions
    └── state.rs          (38 lines)   BookSnapshot, MarketSignalState, SignalDedup

### What main.rs Still Contains (1,141 lines)

| Component | Lines | Why it stays |
|-----------|-------|--------------|
| task_data_trades | ~200 | Touches AppState 8+ times per trade |
| WebSocket tasks | ~100 | Manages connections, reads state |
| CLOB book tasks | ~100 | Fetches + updates state |
| Edge feed computation | ~70 | Reads multiple state fields |
| AppState + helpers | ~120 | Core state definition |
| REST handlers (signals, etc.) | ~50 | Reads state, returns JSON |
| main() + router | ~80 | Wiring + startup |
| Other tasks | ~200 | Leaderboard, heartbeat, etc. |
| Signal engine wrapper | ~20 | Thin wrapper that calls engine module |

---

## 24. The `engine/` Module Walkthrough

### engine/state.rs — Internal State Types

These types track per-market data needed by signal detectors. They don't
exist in AppState because they're implementation details of the engine.

    struct BookSnapshot {
        ask_liq: f64,   // total ask-side liquidity in USD
        ts:      i64,    // when this snapshot was taken
    }

    struct MarketSignalState {
        vol_1m:     f64,    // rolling 1-minute volume
        vol_60m:    f64,    // rolling 60-minute volume
        last_vol_reset_1m: i64,
        last_vol_reset_60m: i64,
        accum_buys: HashMap<String, (u32, f64, f64)>,  // wallet → (count, total_usd, start_price)
        book_snaps: VecDeque<BookSnapshot>,
        last_prob:  f64,
        last_cross_ts: i64,
    }

    struct SignalDedup {
        map: HashMap<String, i64>,  // signal_id → last_fired_ts
    }

**Why these are NOT in domain:**

Domain contains facts about the world (Trade, WhaleProfile, Market).
These types are internal tracking state — they don't represent business
concepts. They're implementation details of how the signal engine works.

**Why these are NOT in main.rs:**

They're specific to the signal engine. Main.rs shouldn't know about
`vol_1m` or `book_snaps`. Keeping them in `engine/state.rs` means the
engine owns its own state.

### engine/detect.rs — 8 Pure Detection Functions

Each function follows the same pattern:

    fn detect_X(trade, context...) -> Option<SignalOutput> {
        // 1. Check if the signal applies
        // 2. If not, return None
        // 3. If yes, compute confidence
        // 4. Return SignalOutput with description, action, edge, color
    }

**SignalOutput** is the return type:

    struct SignalOutput {
        kind: String,           // "WHALE_PRINT", "SMART_CLUSTER", etc.
        confidence: u8,         // 0-100
        description: String,    // human-readable explanation
        action: String,         // what to do ("BUY YES", "ALERT", etc.)
        edge: String,           // why this signal matters
        color: String,          // UI color ("green", "red", "cyan")
        wallet: Option<String>, // who triggered it (if applicable)
    }

**The 8 detectors:**

| # | Signal | What it detects | Inputs |
|---|--------|----------------|--------|
| 1 | WHALE_PRINT | Single large trade | trade, threshold |
| 2 | SMART_CLUSTER | 3+ alpha wallets buying same outcome | trade, profiles, recent_trades |
| 3 | CONVICTION_SPIKE | Trade 3× wallet's own average | trade, profiles |
| 4 | WHALE_REVERSAL | Wallet flips buy→sell or sell→buy | trade, profiles, prev_action |
| 5 | VELOCITY_SURGE | 1-min volume > 4× 60-min avg | trade, mkt_state |
| 6 | STEALTH_ACCUM | Same wallet buys 3+ times, price <2¢ drift | trade, mkt_state |
| 7 | PROB_DIVERGENCE | Whales buying but price falling (or vice versa) | trade, recent_trades, prob_change |
| 8 | LIQUIDITY_DRAIN | Ask book thinned >40% in 5 min | trade, raw_book_asks, mkt_state |
| 9 | MOMENTUM_BREAK | Prob crosses 25/50/75 with volume | trade, prob, volume, mkt_state |

### engine/mod.rs — The Orchestrator

The orchestrator is the only function that knows about state. It:

1. **Resolves market data** — finds the market, gets probability, volume, token_id
2. **For each detector** — prepares the data, calls the detector, checks dedup
3. **Fires signals** — converts SignalOutput to EdgeSignal and calls fire_signal

The orchestrator's signature shows what it needs:

    pub fn run_signals_on_trade(
        trade: &Trade,                          // the trade that triggered this
        profiles: &HashMap<String, WhaleProfile>, // all whale profiles
        recent_trades: &VecDeque<Trade>,        // rolling trade window
        markets: &Vec<Market>,                  // all markets
        raw_books: &HashMap<String, RawBook>,   // raw CLOB books
        mkt_signal_state: &mut HashMap<String, MarketSignalState>,  // per-market mutable state
        signal_dedup: &mut SignalDedup,          // dedup tracking
        wallet_prev_action: &mut HashMap<String, (String, Action)>, // for reversal detection
        whale_threshold: f64,                    // dynamic threshold from AppState
        fire_signal: &dyn Fn(EdgeSignal),        // callback to fire signal
    )

**Why `fire_signal` is a callback:** The engine doesn't know about
WebSocket or AppState. It just calls the callback with an EdgeSignal.
The caller (main.rs wrapper) provides the callback that knows how to
fire the signal through the event system.

### The Wrapper in main.rs

Main.rs now has a thin 20-line wrapper that locks AppState and calls
the engine:

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

**This is infrastructure glue.** It knows about AppState, Mutex, and
locks. But it's only 20 lines — not 420. The business logic is in the
engine module.

---

## 25. Common Beginner Questions (Signal Engine)

### Q: Why create a new `engine/` layer instead of putting it in `application/`?

Because the application layer is for **orchestration** — connecting
ingestion to domain. The signal engine is **business logic** — it
implements detection algorithms. They're different responsibilities:

| Layer | Responsibility | Example |
|-------|---------------|---------|
| application | Connect ingestion → domain | refresh_markets, store_trade |
| engine | Implement business algorithms | detect_whale_print, detect_smart_cluster |

The engine reads domain data (Trade, WhaleProfile) and produces domain
output (EdgeSignal). It's closer to domain than application.

### Q: Why are the detection functions pure (no state)?

Because pure functions are:
1. **Testable** — call with data, assert output
2. **Composable** — combine detectors freely
3. **Readable** — no hidden side effects

If each detector locked AppState, you'd have:
- 8 mutex locks per trade
- Deadlock risk if lock ordering is wrong
- Impossible to test without full AppState

With pure functions, the orchestrator locks once, passes data in, and
gets signals out.

### Q: Why is `fire_signal` a callback?

The engine doesn't know how signals are fired. Maybe they go through
WebSocket. Maybe they go to a database. Maybe they print to stdout.

By using a callback:

    fire_signal: &dyn Fn(EdgeSignal)

The engine says: "Here's a signal. You figure out what to do with it."
The caller provides the implementation.

This is called **dependency injection** — the engine depends on an
abstraction (a function), not a concrete implementation (AppState).

### Q: Why does `detect_velocity_surge` take `&mut MarketSignalState`?

Because velocity surge needs to update volume counters:

    mkt_state.vol_1m += trade.size_usd;
    mkt_state.vol_60m += trade.size_usd;

Some signals need mutable state. That's fine — the state is **local to
the engine**, not global AppState. The engine owns its own mutable data.

### Q: Why did `RawBook` move to `domain/books.rs`?

Because both `main.rs` and `engine/mod.rs` need it. If RawBook stayed
in main.rs, the engine would need `use crate::main::RawBook` — which
creates a circular dependency.

By moving it to domain/books.rs, both modules can import it:

    // engine/mod.rs
    use crate::domain::books::RawBook;

    // main.rs
    use domain::books::RawBook;

Domain is the shared language of the system.

### Q: How do I test a detector?

    #[test]
    fn test_whale_print() {
        let trade = Trade {
            id: "tx1".into(),
            wallet: "0xabc".into(),
            wallet_short: "0xabc…def".into(),
            size_usd: 50_000.0,
            action: Action::BUY,
            price_cents: 67.0,
            outcome_name: "Yes".into(),
            outcome_index: 0,
            outcome_count: 2,
            ..Default::default()
        };

        let result = detect_whale_print(&trade, 5_000.0);
        assert!(result.is_some());
        let sig = result.unwrap();
        assert_eq!(sig.kind, "WHALE_PRINT");
        assert!(sig.confidence >= 80);
    }

No HTTP server. No AppState. No Tokio runtime. Just data in, signal out.

### Q: What's the relationship between `engine/` and `domain/signals.rs`?

| File | What it contains | Role |
|------|-----------------|------|
| domain/signals.rs | EdgeSignal, GlobalSignals, Stats | Domain facts — what a signal IS |
| engine/detect.rs | detect_whale_print, etc. | Business logic — HOW to detect signals |
| engine/mod.rs | run_signals_on_trade | Orchestration — WHEN to run detectors |

Domain defines the data structures. Engine implements the algorithms.
They're separate concerns.

### Q: Why is the orchestrator only 161 lines when the old function was 420?

Because the old function had **duplicated patterns**:

    // Old: each signal did its own dedup check
    if state.signal_dedup.lock().unwrap().should_fire(&sig_id, now, 300_000) {
        state.fire_edge_signal(EdgeSignal { ... });
    }

    // Repeated 8 times with different data

    // New: orchestrator does dedup + fire ONCE for all signals
    for (sig_id, out) in signals_to_fire {
        fire_signal(EdgeSignal { ... });
    }

The old code mixed detection logic with infrastructure (locking, dedup,
firing). The new code separates them — detectors return data, orchestrator
handles infrastructure.

### Q: Can I add a new signal without touching existing code?

Yes! To add a new signal:

1. Add a function to `engine/detect.rs`:

    fn detect_new_signal(trade: &Trade, ...) -> Option<SignalOutput> {
        // your detection logic
    }

2. Add a call in `engine/mod.rs`:

    if let Some(out) = detect_new_signal(trade, ...) {
        let dedup_id = format!("new-signal-{}", slug);
        if signal_dedup.should_fire(&dedup_id, now, 300_000) {
            signals_to_fire.push((dedup_id, out));
        }
    }

That's it. No changes to main.rs, domain, or other detectors.

### Q: What's left to extract from main.rs?

| Component | Lines | Difficulty |
|-----------|-------|------------|
| task_data_trades | ~200 | Hard — touches AppState 8+ times |
| WebSocket tasks | ~100 | Medium — infrastructure by nature |
| CLOB book tasks | ~100 | Medium — infrastructure by nature |
| Edge feed computation | ~70 | Medium — reads multiple state fields |
| AppState + helpers | ~120 | Hard — core state definition |
| REST handlers | ~50 | Easy — pure transport |
| main() + router | ~80 | Easy — pure wiring |

The signal engine was the last "easy" extraction. The remaining code
is infrastructure that genuinely needs AppState. To extract further,
you'd need to restructure AppState itself (dependency injection, trait
objects, or a service locator pattern).

---

## Summary: The Four Extractions

| Session | What | Lines removed | New files |
|---------|------|---------------|-----------|
| 1st | Markets | ~200 | domain/markets.rs, ingestion/markets.rs, application/market_service.rs, api/markets_handler.rs |
| 2nd | Trades + Whales | ~350 | domain/trades.rs, domain/whales.rs, ingestion/trades.rs, application/trade_service.rs, api/trades_handler.rs |
| 3rd | Books + Signals + Auth | ~350 | domain/books.rs, domain/signals.rs, api/auth.rs |
| 4th | Signal Engine | ~450 | engine/mod.rs, engine/detect.rs, engine/state.rs |
| **Total** | | **~1,350** | **17 files** |

**main.rs went from 2,349 lines to 1,141 lines** — a 51% reduction.

The architecture now has 5 layers:

    domain/     — facts about the world (Trade, WhaleProfile, Market, EdgeSignal)
    ingestion/  — raw data from APIs (fetch_raw_markets, fetch_raw_trades)
    application/ — orchestration (refresh_markets, store_trade)
    api/        — HTTP transport (h_markets, h_trades, h_validate)
    engine/     — business algorithms (detect_whale_print, detect_smart_cluster)

Each layer has a single responsibility. Each file has a single concept.
Dependencies point inward toward domain. The system is easy to understand,
easy to change, and easy to test.

---

# FIFTH EXTRACTION SESSION — task_data_trades (Trade Ingestion Loop)

26. What We Started With (After Fourth Extraction)
27. Why This Was the Hardest Extraction
28. The Strategy: Helper Functions
29. What Each Helper Does
30. Before vs After (All Five Sessions)
31. Common Beginner Questions (Trade Ingestion)

---

## 26. What We Started With (After Fourth Extraction)

After the signal engine extraction, main.rs was **1,141 lines**. The
biggest remaining chunk was `task_data_trades` — a 200-line function that
processes every trade from the Polymarket API:

    Lines 357-556    task_data_trades() — 200 lines, 23 state touches

**The problem was different from all previous extractions.** Previous
extractions moved pure functions (domain facts, detection algorithms).
`task_data_trades` is infrastructure glue — it touches AppState 23 times
per trade. You can't extract it cleanly like the signal engine.

---

## 27. Why This Was the Hardest Extraction

Previous extractions had clear boundaries:

| Extraction | What moved | Why it was easy |
|------------|-----------|-----------------|
| Markets | Struct + from_gamma() | Pure data, no state |
| Trades | Struct + helpers | Pure data, no state |
| Whales | Profile + score formula | Pure logic, no state |
| Signal engine | 8 detection functions | Pure functions, take data in |
| task_data_trades | ???. touches AppState 23 times | No clean boundary |

The 23 state touches:

    state.seen_hashes.lock()      — dedup check
    state.asset_map.read()        — resolve outcome index (×2)
    state.markets.read()          — find market (×3)
    state.trade_counter.lock()    — generate trade ID
    state.markets.write()         — update market price
    state.tx.send()               — broadcast events (×4)
    state.whale_profiles.lock()   — update whale profile (×3)
    state.recent_trades.lock()    — store trade (×2)
    state.stats.lock()            — update stats (×3)

If we moved this to application layer, application would need AppState.
That's a dependency violation. If we kept it in main.rs, it stays a
200-line monolith.

**The solution:** Don't extract the function. Extract the PIECES.

---

## 28. The Strategy: Helper Functions

Instead of moving `task_data_trades` to another module, we split it into
5 helper functions that each do ONE job:

    ┌─────────────────────────────────────────────────┐
    │  task_data_trades (main.rs)                     │
    │  (was 200 lines, now 40 lines)                  │
    │                                                 │
    │  loop {                                         │
    │      fetch raw data                             │
    │      for each trade:                            │
    │          dedup check                            │
    │          parse JSON → Trade     ──► Trade::from_raw()
    │          resolve outcome        ──► resolve_outcome()
    │          update market price    ──► update_market_price()
    │          update whale profile   ──► update_whale_profile()
    │          store trade            ──► inline (1 line)
    │          run signal engine      ──► run_signals_on_trade()
    │          update stats           ──► update_stats()
    │          broadcast events       ──► broadcast_trade_events()
    │  }                                             │
    └─────────────────────────────────────────────────┘

**The key insight:** Each helper takes ONLY the state it needs:

    fn resolve_outcome(asset, condition_id, outcome_name, asset_map, markets)
        → returns (outcome_index, outcome_count)

    fn update_market_price(trade, asset, state)
        → updates market, broadcasts PriceUpdate

    fn update_whale_profile(trade, state)
        → returns updated WhaleProfile

    fn update_stats(trade, state)
        → updates stats

    fn broadcast_trade_events(trade, profile, state)
        → broadcasts Trade, WhaleUpdate, WhaleAlert, Stats

**Why this is better than the 200-line monolith:**

| Before | After |
|--------|-------|
| One 200-line function | 5 focused helpers + 1 thin loop |
| 23 state touches scattered | Each helper touches only its state |
| Can't test any piece | Test each helper independently |
| Adding a new event = edit 200-line function | Adding a new event = edit one helper |

---

## 29. What Each Helper Does

### resolve_outcome() — Pure logic, no state

    fn resolve_outcome(
        asset: &str,
        condition_id: &str,
        outcome_name: &str,
        asset_map: &HashMap<String, (usize, usize)>,
        markets: &[Market],
    ) -> (usize, usize)

**What it does:** Given an asset ID and condition ID, figures out which
outcome index and how many outcomes the market has.

**Why it's separate:** The old code had this logic duplicated in two
places (lines 392-411 and 429-438). Now it's one function.

**State touches:** 0 (receives data as parameters)

### update_market_price() — Writes to market state

    fn update_market_price(trade: &Trade, asset: &str, state: &AppState)

**What it does:**
1. Finds the market by asset ID or condition ID
2. Updates the outcome's price_cents and last_trade
3. Computes prob_change_pct (single-tick delta)
4. Classifies signal (BULL/BEAR/HOT/BREAKOUT/NEUTRAL)
5. Updates buy_pressure (symmetric: BUY pushes toward 1.0, SELL toward 0.0)
6. Broadcasts Ev::PriceUpdate

**State touches:** 4 (asset_map read, markets read×2, markets write, tx send)

### update_whale_profile() — Writes to whale profiles

    fn update_whale_profile(trade: &Trade, state: &AppState) -> WhaleProfile

**What it does:**
1. Gets or creates the whale profile for this wallet
2. Updates total_trades, total_volume, buy_volume, sell_volume
3. Updates dominant_action (BUYER or SELLER)
4. Pushes to the rolling window
5. Updates conviction_score
6. Calls recompute() to recalculate whale_score
7. Returns the updated profile

**State touches:** 1 (whale_profiles lock)

### update_stats() — Writes to stats

    fn update_stats(trade: &Trade, state: &AppState)

**What it does:**
1. Increments total_trades_seen
2. Updates biggest_trade if this one is larger
3. Counts whale_count, alpha_wallet_count, active_wallets
4. Computes buy_sell_ratio from recent trades

**State touches:** 3 (stats lock, whale_profiles lock, recent_trades lock)

### broadcast_trade_events() — Sends WebSocket events

    fn broadcast_trade_events(trade: &Trade, profile: &WhaleProfile, state: &AppState)

**What it does:**
1. Broadcasts Ev::Trade (every trade)
2. Broadcasts Ev::WhaleUpdate (every trade's profile)
3. If whale trade: broadcasts Ev::WhaleAlert with tag, score, is_biggest
4. Broadcasts Ev::Stats (updated statistics)

**State touches:** 4 (tx send×4, whale_profiles lock, stats lock)

---

## 30. Before vs After (All Five Sessions)

### Line Count Progression

| Stage | main.rs lines | Total files |
|-------|---------------|-------------|
| Original monolith | 2,349 | 1 |
| After markets extraction | 2,110 | 9 |
| After trades extraction | 1,946 | 14 |
| After books + signals + auth | 1,591 | 17 |
| After signal engine extraction | 1,141 | 20 |
| **After task_data_trades extraction** | **1,144** | **20** |

The line count barely changed because we added 5 helper functions (~150
lines) and removed the old 200-line monolith. But the code is now
**modular** — each helper has one job.

### What main.rs Looks Like Now

    // Helper functions (new)
    fn resolve_outcome(...) -> (usize, usize)           // 20 lines
    fn update_market_price(trade, asset, state)          // 35 lines
    fn update_whale_profile(trade, state) -> WhaleProfile // 35 lines
    fn update_stats(trade, state)                        // 15 lines
    fn broadcast_trade_events(trade, profile, state)     // 30 lines

    // Main loop (rewritten)
    async fn task_data_trades(state, client) {
        loop {
            fetch raw data
            for each trade:
                dedup check
                parse JSON → Trade
                resolve_outcome()
                update_market_price()
                update_whale_profile()
                store trade
                run_signals_on_trade()
                update_stats()
                broadcast_trade_events()
        }
    }

### What Each File Contains Now

    domain/
    ├── books.rs      (42 lines)   Level, OutcomeBook, MarketBook, RawBook
    ├── markets.rs    (142 lines)  Market, Outcome, from_gamma, is_live, polymarket_url
    ├── signals.rs    (84 lines)   EdgeSignal, GlobalSignals, Stats, EdgeFeedEvent
    ├── trades.rs     (74 lines)   Action, Trade, shorten_addr, from_raw
    └── whales.rs     (130 lines)  WhaleProfile, TradeWindow, is_whale_trade, WHALE_USD, LeaderboardEntry

    ingestion/
    ├── markets.rs    (35 lines)   fetch_raw_markets
    └── trades.rs     (12 lines)   fetch_raw_trades

    application/
    ├── market_service.rs  (49 lines)  refresh_markets, task_refresh_markets
    └── trade_service.rs   (16 lines)  store_trade, constants

    api/
    ├── auth.rs             (214 lines)  h_validate, h_payment_info, h_gen_key + supabase
    ├── markets_handler.rs  (44 lines)   h_markets, h_books, h_heatmap, h_scanner
    └── trades_handler.rs   (21 lines)   h_trades, h_whales, h_stats

    engine/
    ├── mod.rs            (161 lines)  orchestrator
    ├── detect.rs         (301 lines)  8 pure detection functions
    └── state.rs          (38 lines)   BookSnapshot, MarketSignalState, SignalDedup

---

## 31. Common Beginner Questions (Trade Ingestion)

### Q: Why didn't you move task_data_trades to application layer?

Because it touches AppState 23 times. Moving it would require passing
Arc<AppState> to the application layer, which adds a dependency from
application → infrastructure. The application layer should be pure
orchestration — it shouldn't know about AppState directly.

### Q: Why helper functions instead of a struct?

A struct would work, but it's over-engineering for this case:

    // Struct approach (over-engineered)
    struct TradeProcessor { state: Arc<AppState> }
    impl TradeProcessor {
        fn update_market_price(&self, trade: &Trade) { ... }
        fn update_whale_profile(&self, trade: &Trade) { ... }
    }

    // Helper function approach (simple)
    fn update_market_price(trade: &Trade, asset: &str, state: &AppState) { ... }

The helper functions are simpler, stateless, and don't need initialization.
A struct would only make sense if we needed to share state between helpers
(we don't).

### Q: Why is resolve_outcome separate from Trade::from_raw?

Because Trade::from_raw sets defaults (outcome_index: 0, outcome_count: 2).
The actual values depend on state (asset_map, markets). We could pass state
to from_raw, but that would add a dependency from domain → infrastructure.

Instead:
1. from_raw() creates Trade with defaults (domain, no state)
2. resolve_outcome() fills in the real values (takes data as params)
3. Main loop sets the fields: trade.outcome_index = oi

This keeps domain pure and pushes state-dependent logic to the caller.

### Q: Why does update_market_price take `asset: &str` as a parameter?

Because the asset ID comes from the raw JSON (v["asset"]), not from the
Trade struct. The Trade struct doesn't have an asset field — it has
condition_id and outcome_index. The asset ID is an intermediate value
used to look up the market.

### Q: Why does update_whale_profile return the profile?

Because broadcast_trade_events needs the updated profile to send
Ev::WhaleUpdate. If update_whale_profile didn't return it, we'd have
to lock whale_profiles again in broadcast_trade_events.

### Q: Can I test these helpers independently?

Yes! Each helper takes explicit parameters:

    #[test]
    fn test_resolve_outcome() {
        let mut asset_map = HashMap::new();
        asset_map.insert("abc".to_string(), (0, 1));
        let markets = vec![Market { outcomes: vec![...], ... }];

        let (oi, cnt) = resolve_outcome("abc", "", "YES", &asset_map, &markets);
        assert_eq!(oi, 0);
        assert_eq!(cnt, 2);
    }

No AppState, no Tokio runtime, no server.

### Q: What's left to extract from main.rs?

| Component | Lines | Difficulty |
|-----------|-------|------------|
| task_clob_books | ~100 | Medium — touches state |
| task_signals | ~100 | Medium — touches state |
| Edge feed computation | ~70 | Medium — reads multiple state fields |
| AppState + helpers | ~120 | Hard — core state definition |
| REST handlers | ~50 | Easy — pure transport |
| main() + router | ~80 | Easy — pure wiring |

The remaining code is infrastructure that genuinely needs AppState.
The biggest wins have been extracted.

---

# SIXTH EXTRACTION SESSION — Signal REST Handlers (api/signals_handler.rs)

32. What We Started With (After Fifth Extraction)
33. Why This Extraction Was Easy
34. What Moved
35. Before vs After (All Six Sessions)
36. Common Beginner Questions (REST Handlers)

---

## 32. What We Started With (After Fifth Extraction)

After the task_data_trades extraction, main.rs was **1,144 lines**. Four
REST handlers were sitting there waiting to be extracted:

    Lines 948-956    h_health() — GET /health → system status
    Lines 957-959    h_signals() — GET /api/signals → global signals
    Lines 960-963    h_edge_signals() — GET /api/edge-signals → edge signals
    Lines 968-973    h_set_threshold() — POST /api/threshold → update whale threshold

**This was the easiest extraction.** Same pattern as h_trades/h_whales
which we already extracted to api/trades_handler.rs.

---

## 33. Why This Extraction Was Easy

REST handlers are pure transport — they read state and return JSON.
No business logic, no mutation, no complex flow.

    handler(state) → read state → format JSON → return

The pattern is identical for every handler:

    async fn h_xxx(State(s): State<Arc<AppState>>) -> Json<Value> {
        let data = s.some_field.lock().unwrap().clone();
        Json(serde_json::json!({ "key": data }))
    }

No state mutations, no event broadcasting, no coordination with other
functions. Just read and return.

---

## 34. What Moved

### New File: api/signals_handler.rs (35 lines)

| Handler | Route | Job |
|---------|-------|-----|
| `h_health()` | GET /health | System status (markets count, trades seen, signals fired) |
| `h_signals()` | GET /api/signals | Global signals (momentum, sentiment, whale_flow) |
| `h_edge_signals()` | GET /api/edge-signals | Edge signals (last 100 fired signals) |
| `h_set_threshold()` | POST /api/set-threshold | Update whale threshold dynamically |

### main.rs Changes

| What | Change |
|------|--------|
| Removed | h_health, h_signals, h_edge_signals, h_set_threshold functions |
| Removed | ThresholdParams struct |
| Removed | Json import (no longer needed) |
| Updated | Router to use api::signals_handler::xxx paths |

### api/mod.rs Changes

Added `pub mod signals_handler;`

---

## 35. Before vs After (All Six Sessions)

### Line Count Progression

| Stage | main.rs lines | Total files |
|-------|---------------|-------------|
| Original monolith | 2,349 | 1 |
| After markets extraction | 2,110 | 9 |
| After trades extraction | 1,946 | 14 |
| After books + signals + auth | 1,591 | 17 |
| After signal engine extraction | 1,141 | 20 |
| After task_data_trades extraction | 1,144 | 20 |
| **After REST handlers extraction** | **1,117** | **21** |

### What Each File Contains Now

    domain/
    ├── books.rs      (42 lines)   Level, OutcomeBook, MarketBook, RawBook
    ├── markets.rs    (142 lines)  Market, Outcome, from_gamma, is_live, polymarket_url
    ├── signals.rs    (84 lines)   EdgeSignal, GlobalSignals, Stats, EdgeFeedEvent
    ├── trades.rs     (74 lines)   Action, Trade, shorten_addr, from_raw
    └── whales.rs     (130 lines)  WhaleProfile, TradeWindow, is_whale_trade, WHALE_USD, LeaderboardEntry

    ingestion/
    ├── markets.rs    (35 lines)   fetch_raw_markets
    └── trades.rs     (12 lines)   fetch_raw_trades

    application/
    ├── market_service.rs  (49 lines)  refresh_markets, task_refresh_markets
    └── trade_service.rs   (16 lines)  store_trade, constants

    api/
    ├── auth.rs             (214 lines)  h_validate, h_payment_info, h_gen_key + supabase
    ├── markets_handler.rs  (44 lines)   h_markets, h_books, h_heatmap, h_scanner
    ├── signals_handler.rs  (35 lines)   h_health, h_signals, h_edge_signals, h_set_threshold
    └── trades_handler.rs   (21 lines)   h_trades, h_whales, h_stats

    engine/
    ├── mod.rs            (161 lines)  orchestrator
    ├── detect.rs         (301 lines)  8 pure detection functions
    └── state.rs          (38 lines)   BookSnapshot, MarketSignalState, SignalDedup

### What main.rs Still Contains (1,117 lines)

| Component | Lines | Why it stays |
|-----------|-------|--------------|
| task_data_trades + helpers | ~200 | Infrastructure, touches AppState 23 times |
| task_clob_books | ~100 | Infrastructure, fetches + updates state |
| task_signals | ~100 | Infrastructure, reads/writes state |
| task_leaderboards | ~30 | Infrastructure, fetches + updates state |
| task_edge_feed + compute_edge_feed | ~120 | Infrastructure, reads multiple state fields |
| task_ws + handle_ws_conn | ~80 | Infrastructure, manages connections |
| task_heartbeat | ~10 | Infrastructure, sends periodic events |
| AppState + helpers | ~120 | Core state definition |
| main() + router | ~80 | Wiring + startup |

---

## 36. Common Beginner Questions (REST Handlers)

### Q: Why were these so easy to extract?

Because they're pure transport — read state, return JSON. No business
logic, no mutations, no coordination. The same pattern as
h_trades/h_whales which we already extracted.

### Q: Why did Json get removed from main.rs imports?

Because main.rs no longer returns Json directly. The handlers that
returned Json are now in api/signals_handler.rs. Main.rs only needs
Json for the router (which uses axum's Json extractor).

### Q: Why didn't ThresholdParams stay in main.rs?

Because ThresholdParams is only used by h_set_threshold, which is now
in api/signals_handler.rs. It's a handler-specific type, not a
global type.

### Q: Can I test these handlers independently?

Yes! Each handler takes State<Arc<AppState>> which you can mock:

    #[tokio::test]
    async fn test_h_health() {
        let state = Arc::new(AppState::new());
        let response = h_health(State(state)).await;
        // assert response contains "healthy"
    }

No server, no database, no WebSocket.

### Q: What's the pattern for extracting REST handlers?

1. Identify handlers in main.rs (functions with State parameter)
2. Create new file in api/ module (e.g., api/signals_handler.rs)
3. Move handlers to new file
4. Add pub mod to api/mod.rs
5. Update router to use new paths (api::signals_handler::h_xxx)
6. Remove old functions from main.rs
7. Verify cargo check

### Q: What's left to extract from main.rs?

| Component | Lines | Difficulty |
|-----------|-------|------------|
| task_clob_books | ~100 | Medium — touches state |
| task_signals | ~100 | Medium — touches state |
| Edge feed computation | ~70 | Medium — reads multiple state fields |
| AppState + helpers | ~120 | Hard — core state definition |
| main() + router | ~80 | Easy — pure wiring |

The remaining code is infrastructure that genuinely needs AppState.

---

# SEVENTH EXTRACTION SESSION — CLOB Books (task_clob_books)

37. What We Started With (After Sixth Extraction)
38. Why This Extraction Was Medium Difficulty
39. The Strategy: Split Pure Logic from State Mutations
40. What Each Helper Does
41. Before vs After (All Seven Sessions)
42. Common Beginner Questions (CLOB Books)

---

## 37. What We Started With (After Sixth Extraction)

After the REST handlers extraction, main.rs was **1,117 lines**. The
CLOB book task was a 55-line function that fetches order books from
the Polymarket CLOB API:

    Lines 564-618    task_clob_books() — 55 lines
    Lines 620-647    fetch_mids_spreads() — 28 lines
    Lines 649-700    assemble_book() — 52 lines

Total: ~135 lines of CLOB book logic.

**The problem:** The main loop mixed pure logic (collecting token IDs,
parsing JSON) with state mutations (storing books, broadcasting events).
Hard to test, hard to understand.

---

## 38. Why This Extraction Was Medium Difficulty

Unlike task_data_trades (23 state touches), task_clob_books has fewer
state touches:

    state.markets.read()     — get markets
    state.raw_books.lock()   — store raw books
    state.asset_map.read()   — resolve token IDs
    state.markets.write()    — update mid/spread
    state.books.lock()       — store assembled books
    state.tx.send()          — broadcast events

But the parsing logic is pure — it doesn't need state. That's the key
insight: **separate pure logic from state mutations.**

---

## 39. The Strategy: Split Pure Logic from State Mutations

    ┌─────────────────────────────────────────────────┐
    │  task_clob_books (main.rs)                      │
    │  (was 55 lines, now 30 lines)                   │
    │                                                 │
    │  loop {                                         │
    │      collect token IDs    ──► collect_token_ids()│
    │      fetch books from API                         │
    │      parse JSON           ──► parse_book_json()  │
    │      store raw books      ──► store_raw_books()  │
    │      fetch mids/spreads   ──► fetch_mids_spreads()│
    │      assemble + broadcast ──► broadcast_books()  │
    │  }                                             │
    └─────────────────────────────────────────────────┘

**Pure functions (no state):**
- collect_token_ids() — takes &[Market], returns Vec<String>
- parse_book_json() — takes &[Value], returns HashMap<String, RawBook>

**State mutations (need AppState):**
- store_raw_books() — writes to state.raw_books
- fetch_mids_spreads() — already existed, reads/writes state
- assemble_book() — already existed, reads state
- broadcast_books() — writes to state.books, sends events

---

## 40. What Each Helper Does

### collect_token_ids() — Pure logic

    fn collect_token_ids(mkts: &[Market]) -> Vec<String>

**What it does:** Collects up to 40 unique token IDs from the top 8
markets by volume. Used to build the API query parameter.

**State touches:** 0 (receives data as parameter)

### parse_book_json() — Pure logic

    fn parse_book_json(json: &[serde_json::Value]) -> HashMap<String, RawBook>

**What it does:** Parses the CLOB API response into RawBook structs.
Each book has bids and asks as (price, size) tuples.

**State touches:** 0 (receives data as parameter)

### store_raw_books() — State mutation

    fn store_raw_books(state: &AppState, books: HashMap<String, RawBook>)

**What it does:** Writes parsed books to state.raw_books.

**State touches:** 1 (raw_books lock)

### broadcast_books() — State mutation

    fn broadcast_books(state: &Arc<AppState>, mkts: &[Market])

**What it does:** For each of the top 8 markets, assembles a MarketBook
from raw data, stores it in state.books, and broadcasts Ev::Book.

**State touches:** 3 (books lock, tx send per market)

---

## 41. Before vs After (All Seven Sessions)

### Line Count Progression

| Stage | main.rs lines | Total files |
|-------|---------------|-------------|
| Original monolith | 2,349 | 1 |
| After markets extraction | 2,110 | 9 |
| After trades extraction | 1,946 | 14 |
| After books + signals + auth | 1,591 | 17 |
| After signal engine extraction | 1,141 | 20 |
| After task_data_trades extraction | 1,144 | 20 |
| After REST handlers extraction | 1,117 | 21 |
| **After CLOB books extraction** | **1,138** | **21** |

The line count increased slightly because we added 4 helper functions
(~60 lines) but the main loop is now cleaner and more readable.

### What main.rs Looks Like Now

    // Pure helpers (new)
    fn collect_token_ids(mkts: &[Market]) -> Vec<String>           // 15 lines
    fn parse_book_json(json: &[Value]) -> HashMap<String, RawBook> // 20 lines
    fn store_raw_books(state, books)                                // 5 lines
    fn broadcast_books(state, mkts)                                 // 8 lines

    // Existing helpers (already separate)
    async fn fetch_mids_spreads(state, client, ids)                 // 28 lines
    fn assemble_book(state, m) -> MarketBook                       // 52 lines

    // Main loop (rewritten)
    async fn task_clob_books(state, client) {
        loop {
            collect token IDs
            fetch books from API
            parse JSON into raw books
            store raw books
            fetch mids + spreads
            assemble and broadcast books
        }
    }

---

## 42. Common Beginner Questions (CLOB Books)

### Q: Why was this easier than task_data_trades?

Because the parsing logic is pure — it doesn't need state. We could
extract collect_token_ids and parse_book_json as standalone functions
that take data in and return data out.

task_data_trades had 23 state touches scattered throughout. Every
block needed AppState. Here, only the storage and broadcast steps
need state.

### Q: Why did we create store_raw_books as a separate function?

It's only 5 lines. But by making it a function:
1. The name explains what it does ("store raw books")
2. It's testable independently
3. The main loop reads like a recipe: "collect, fetch, parse, store, broadcast"

### Q: Why does broadcast_books take &Arc<AppState> instead of &AppState?

Because assemble_book expects &Arc<AppState>. We could change
assemble_book to take &AppState, but it's already working and the
change isn't worth the risk.

### Q: Can I test collect_token_ids independently?

Yes! It takes &[Market] and returns Vec<String>:

    #[test]
    fn test_collect_token_ids() {
        let markets = vec![Market { outcomes: vec![
            Outcome { token_id: "abc".into(), ... },
            Outcome { token_id: "def".into(), ... },
        ], ... }];

        let ids = collect_token_ids(&markets);
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&"abc".to_string()));
    }

No AppState, no HTTP, no Tokio runtime.

### Q: What's the pattern for extracting infrastructure tasks?

1. Identify pure logic (no state) → extract as standalone functions
2. Identify state mutations → keep as helpers with explicit state params
3. Rewrite main loop to call helpers in order
4. Verify compilation

The pure logic is easy to test. The state mutations are easy to
understand. The main loop is easy to read.

### Q: What's left to extract from main.rs?

| Component | Lines | Difficulty |
|-----------|-------|------------|
| task_signals | ~130 | Hard — reads 5+ state fields |
| Edge feed computation | ~80 | Medium — reads multiple state fields |
| task_ws + handle_ws_conn | ~80 | Hard — manages connections |
| task_leaderboards | ~30 | Medium — fetches data |
| AppState + helpers | ~120 | Core state definition |
| main() + router | ~80 | Pure wiring |

---

# FINAL SUMMARY — Refactoring Complete

43. Line Count Progression
44. What Was Extracted (All Sessions)
45. Architecture After Refactoring
46. What Remains in main.rs
47. Key Patterns You Learned
48. When to Stop Refactoring

---

## 43. Line Count Progression

| Stage | main.rs lines | Files | % extracted |
|-------|---------------|-------|-------------|
| Original monolith | 2,349 | 1 | 0% |
| + Markets | 2,110 | 9 | 10% |
| + Trades | 1,946 | 14 | 17% |
| + Books + Signals + Auth | 1,591 | 17 | 32% |
| + Signal Engine | 1,141 | 20 | 51% |
| + task_data_trades | 1,144 | 20 | 51% |
| + REST Handlers | 1,117 | 21 | 52% |
| + CLOB Books | 1,138 | 21 | 52% |
| + Leaderboards + Edge Feed | 1,128 | 21 | 52% |

main.rs: 2,349 → 1,128 lines (**52% extracted**)

---

## 44. What Was Extracted (All Sessions)

| Session | What moved | From | To | Lines moved |
|---------|-----------|------|----|-------------|
| 1 | Market types + logic | main | domain/markets | 128 |
| 1 | Gamma fetcher | main | ingestion/markets | 220 |
| 1 | Market service | main | application/market_service | 30 |
| 1 | Market HTTP handlers | main | api/markets_handler | 44 |
| 2 | Trade types + logic | main | domain/trades | 74 |
| 2 | Whale types + logic | main | domain/whales | 130 |
| 2 | Trade HTTP handlers | main | api/trades_handler | 21 |
| 3 | Books + signals types | main | domain/books, signals | 126 |
| 3 | Auth handlers | main | api/auth | 214 |
| 4 | Signal engine | main | engine/* | 500 |
| 5 | task_data_trades helpers | main | main (refactored) | 60 |
| 6 | REST handlers | main | api/signals_handler | 35 |
| 7 | CLOB book helpers | main | main (refactored) | 100 |
| 8 | Leaderboard helpers | main | main (refactored) | 20 |
| 8 | Edge feed helpers | main | main (refactored) | 40 |

---

## 45. Architecture After Refactoring

```
whale-simple/src/
│
├── domain/
│   ├── books.rs       (42 lines)   Level, OutcomeBook, MarketBook, RawBook
│   ├── markets.rs    (142 lines)   Market, Outcome, from_gamma, is_live, polymarket_url
│   ├── signals.rs     (84 lines)   EdgeSignal, SignalAccuracy, GlobalSignals, Stats, EdgeFeedEvent
│   ├── trades.rs      (74 lines)   Action, Trade, shorten_addr, from_raw
│   └── whales.rs     (130 lines)   WhaleProfile, TradeWindow, is_whale_trade, LeaderboardEntry
│
├── ingestion/
│   ├── trades.rs      (82 lines)   poll_trades, trades_task, parse_user_trades, from_polygon, DATA_TRADES
│   └── markets.rs    (220 lines)   poll_markets, markets_task, fetch_market_details
│
├── application/
│   └── trade_service.rs (148 lines) process_trade_batch, run_book_cycle, from_polygon
│
├── engine/
│   ├── detect.rs     (301 lines)   8 pure detection functions
│   ├── mod.rs        (161 lines)   orchestrator, run_signals_on_trade
│   └── state.rs       (38 lines)   BookSnapshot, MarketSignalState, SignalDedup
│
├── api/
│   ├── auth.rs       (214 lines)   h_validate, h_payment_info, h_gen_key
│   ├── markets_handler.rs (44 lines) h_markets, h_books, h_heatmap, h_scanner
│   ├── signals_handler.rs (35 lines) h_health, h_signals, h_edge_signals, h_set_threshold
│   └── trades_handler.rs (21 lines) h_trades, h_whales, h_stats
│
└── main.rs         (1,128 lines)   infrastructure glue, AppState, task loops, helpers
```

---

## 46. What Remains in main.rs (1,128 lines)

| Component | Lines | Why it stays |
|-----------|-------|-------------|
| task_signals | 130 | Reads 5+ state fields |
| compute_edge_feed | 40 | Reads state, calls pure helper |
| task_edge_feed | 10 | Thin wrapper |
| task_ws + handle_ws_conn | 80 | Manages connections |
| task_leaderboards | 10 | Thin wrapper |
| fetch_lb | 15 | Calls parse helper |
| task_clob_books | 30 | Calls 4 helpers |
| task_data_trades | 30 | Calls 5 helpers |
| task_heartbeat | 10 | Trivial |
| classify_signal | 10 | Pure logic, could extract but trivial |
| AppState | 80 | Core state definition |
| main() + router | 80 | Pure wiring |
| Ev enum + helpers | 120 | Tied to AppState |
| Imports | 40 | Tied to main |

**Why this is the right stopping point:**
- The pure logic is already extracted (engine, domain types)
- The remaining code genuinely needs AppState
- Further extraction would create dependency violations or move trivial code

---

## 47. Key Patterns You Learned

### Pattern 1: Extract Domain Types
Move structs/enums to domain/. They have zero dependencies.

### Pattern 2: Extract Pure Logic
Move functions that don't touch state to domain/ or application/.
They're easy to test.

### Pattern 3: Extract Orchestration
Move multi-step workflows to application/.
They call domain types + ingestion.

### Pattern 4: Extract HTTP Handlers
Move Axum handlers to api/.
They're thin — just parse request, call service, return response.

### Pattern 5: Keep Infrastructure in main.rs
Code that touches AppState stays in main.rs.
Refactor it into helpers for readability.

### Pattern 6: Callback for Dependencies
Engine takes `&dyn Fn(EdgeSignal)` — no AppState import.
main.rs provides the closure that fires events.

---

## 48. When to Stop Refactoring

You stop when:
1. The code is easy to understand (read top-down, follow the flow)
2. The architecture has clear layers (domain → ingestion → application → api)
3. Dependencies point inward (domain depends on nothing)
4. The remaining code genuinely needs infrastructure (AppState)
5. You've spent enough time — diminishing returns

We're at step 5. The architecture is clean. The monolith is organized.
The code compiles with zero warnings.

**Stop here. Ship it.**
