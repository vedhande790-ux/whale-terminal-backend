# Whale.TERMINAL — Architecture

## Layers

```
┌─────────────────────────────────────────────────┐
│                   api                            │  Transport: REST + WS handlers
│  markets_handler.rs  trades_handler.rs           │  Serializes domain types to JSON
├─────────────────────────────────────────────────┤
│               application                        │  Orchestration: coordinates
│  market_service.rs   trade_service.rs            │  ingestion → domain → state
├─────────────────────────────────────────────────┤
│                 domain                           │  Facts + business rules
│  markets.rs  trades.rs  whales.rs                │  Market, Trade, WhaleProfile
├─────────────────────────────────────────────────┤
│               ingestion                          │  Raw data from external endpoints
│  markets.rs          trades.rs                   │  HTTP fetch, pagination, rate limits
├─────────────────────────────────────────────────┤
│               infrastructure                     │  State, WS, tasks, signal engine
│  main.rs                                         │  AppState, task loops, signal engine
└─────────────────────────────────────────────────┘
```

## File Structure

```
src/
├── main.rs                          1759 lines   wiring, state, tasks, signal engine
├── domain/
│   ├── mod.rs                         3 lines
│   ├── markets.rs                   132 lines   Market, Outcome, from_gamma, is_live
│   ├── trades.rs                     67 lines   Action, Trade, from_raw, shorten_addr
│   └── whales.rs                    107 lines   WhaleProfile, TradeWindow, compute_whale_tag
├── ingestion/
│   ├── mod.rs                         2 lines
│   ├── markets.rs                    33 lines   fetch_raw_markets (HTTP only)
│   └── trades.rs                     11 lines   fetch_raw_trades (HTTP only)
├── application/
│   ├── mod.rs                         2 lines
│   ├── market_service.rs             40 lines   refresh_markets, task_refresh_markets
│   └── trade_service.rs              14 lines   store_trade, constants
└── api/
    ├── mod.rs                         2 lines
    ├── markets_handler.rs            39 lines   h_markets, h_books, h_heatmap, h_scanner
    └── trades_handler.rs             17 lines   h_trades, h_whales, h_stats
```

## Layer Responsibilities

### domain — Facts + Rules
**File:** `src/domain/markets.rs`

| Symbol | Type | Purpose |
|--------|------|---------|
| `Market` | struct | Core fact: id, condition_id, slug, outcomes, volumes, signal, url |
| `Outcome` | struct | One tradable side: token_id, name, price_cents, mid, spread |
| `Market::from_gamma()` | fn | Maps raw JSON Value → Market (DTO → domain) |
| `Market::is_live()` | fn | Business rule: not expired, not resolved, has volume |
| `Market::primary_prob()` | fn | First outcome's price_cents |
| `polymarket_url()` | fn | Builds Polymarket URL from slugs |
| `infer_category()` | fn | Classifies market into Crypto/Politics/Sports/etc |
| `parse_str_arr()` | fn | Parses JSON string-or-array into Vec<String> |
| `parse_f64_arr()` | fn | Parses JSON string-or-array into Vec<f64> |
| `trunc()` | fn | Truncates string to n chars |

**Depends on:** `serde`, `serde_json`, `chrono`
**Depends on:** nothing in this crate

**File:** `src/domain/trades.rs`

| Symbol | Type | Purpose |
|--------|------|---------|
| `Action` | enum | BUY or SELL |
| `Trade` | struct | Core fact: 20 fields defining one trade event |
| `TradeWindow` | struct | Rolling 50-trade window for metric calculation |
| `TradeWindow::push()` | fn | Add trade to window |
| `TradeWindow::win_rate()` | fn | % of profitable trades |
| `TradeWindow::avg_roi()` | fn | Average return on investment |
| `TradeWindow::consistency_score()` | fn | Coefficient of variation |
| `TradeWindow::top_market()` | fn | Most traded market |
| `TradeWindow::specialization_score()` | fn | How focused on one market |
| `WhaleProfile` | struct | Wallet intelligence profile |
| `WhaleProfile::recompute()` | fn | Calculate whale_score, whale_tag from window |
| `shorten_addr()` | fn | Truncate wallet address for display |
| `compute_whale_tag()` | fn | Classify whale behavior from score + volumes |

**Depends on:** `serde`, `chrono`, `domain::markets::polymarket_url`
**Depends on:** nothing external in this crate

**File:** `src/domain/whales.rs`

| Symbol | Type | Purpose |
|--------|------|---------|
| `compute_whale_tag()` | fn | Classify whale behavior from score + volumes |
| `TradeWindow` | struct | Rolling 50-trade window for metric calculation |
| `TradeWindow::push()` | fn | Add trade to window |
| `TradeWindow::win_rate()` | fn | % of profitable trades |
| `TradeWindow::avg_roi()` | fn | Average return on investment |
| `TradeWindow::consistency_score()` | fn | Coefficient of variation |
| `TradeWindow::top_market()` | fn | Most traded market |
| `TradeWindow::specialization_score()` | fn | How focused on one market |
| `WhaleProfile` | struct | Wallet intelligence profile |
| `WhaleProfile::recompute()` | fn | Calculate whale_score, whale_tag from window |

**Depends on:** `serde`
**Depends on:** nothing in this crate

### ingestion — Raw Data from Endpoints
**File:** `src/ingestion/markets.rs`

| Symbol | Type | Purpose |
|--------|------|---------|
| `fetch_raw_markets()` | async fn | Fetches 5+25 pages from Gamma API, returns raw JSON values |

**Depends on:** `reqwest`, `serde_json`, `tokio`
**Depends on:** nothing in this crate

**File:** `src/ingestion/trades.rs`

| Symbol | Type | Purpose |
|--------|------|---------|
| `DATA_TRADES` | const | Polymarket Data API endpoint URL |
| `fetch_raw_trades()` | async fn | Single HTTP GET, returns raw JSON values |

**Depends on:** `reqwest` only
**Depends on:** nothing in this crate

### application — Orchestration
**File:** `src/application/market_service.rs`

| Symbol | Type | Purpose |
|--------|------|---------|
| `refresh_markets()` | async fn | Full pipeline: fetch → dedupe → map → filter → publish to state |
| `task_refresh_markets()` | async fn | 300s loop calling refresh_markets |

**Depends on:** `domain::markets`, `ingestion::markets`, `crate::AppState`

**File:** `src/application/trade_service.rs`

| Symbol | Type | Purpose |
|--------|------|---------|
| `WHALE_USD` | const | $5,000 whale threshold |
| `MIN_TRADE_USD` | const | $100 minimum trade size |
| `MAX_TRADES` | const | 500 rolling window size |
| `store_trade()` | fn | Write trade to `state.recent_trades` |

**Depends on:** `domain::trades`, `crate::AppState`

### api — Transport
**File:** `src/api/markets_handler.rs`

| Symbol | Type | Purpose |
|--------|------|---------|
| `h_markets()` | handler | GET /api/markets → full market list |
| `h_books()` | handler | GET /api/books → order books |
| `h_heatmap()` | handler | GET /api/heatmap → heatmap cells |
| `h_scanner()` | handler | GET /api/scanner → top movers |

**Depends on:** `axum`, `crate::AppState`

**File:** `src/api/trades_handler.rs`

| Symbol | Type | Purpose |
|--------|------|---------|
| `h_trades()` | handler | GET /api/trades → recent 100 trades |
| `h_whales()` | handler | GET /api/whales → top 50 whale profiles + leaderboards |
| `h_stats()` | handler | GET /api/stats → system statistics |

**Depends on:** `axum`, `crate::AppState`

### infrastructure — State, Tasks, Signal Engine
**File:** `src/main.rs` (1759 lines, next extraction target)

Still contains:
- `AppState` struct (shared state container)
- `Ev` enum (WebSocket events)
- `MarketBook`, `OutcomeBook`, `Level`, `RawBook` (book types)
- `MarketSignalState`, `SignalDedup` (per-market signal scratch state)
- `EdgeSignal`, `GlobalSignals`, `Stats` (signal/stats types)
- `run_signals_on_trade()` (8-signal engine)
- `assemble_book()` (raw books → MarketBook)
- `task_data_trades()` (live trade ingestion — touches AppState heavily)
- `task_clob_books()` (CLOB book polling)
- `fetch_mids_spreads()` (CLOB mid/spread polling)
- `task_ws()` + `handle_ws_msg()` (CLOB WebSocket)
- `task_signals()` (30s global signal computation)
- `task_leaderboards()` + `fetch_lb()` (leaderboard polling)
- `task_edge_feed()` + `compute_edge_feed()` (edge signal publisher)
- REST handlers: h_health, h_signals, h_edge_signals, h_set_threshold
- WebSocket: ws_handler, handle_ws_conn
- Auth: h_validate, h_payment_info, h_gen_key

## Data Flow: Markets Pipeline

```
Gamma API (external)
       │
       ▼
┌──────────────────────┐
│ ingestion::markets   │  fetch_raw_markets() → Vec<Value>
│   HTTP GET ×30 pages │  pagination, rate limits
│   dedupe raw         │  error swallow
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│ application::        │  refresh_markets()
│ market_service       │  dedupe by conditionId
│                      │  Market::from_gamma() ×N
│                      │  Market::is_live() filter
│                      │  build asset_map
│                      │  write state.markets + stats
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│ AppState.markets     │  RwLock<Vec<Market>>
│ AppState.asset_map   │  RwLock<HashMap<token_id, (mi, oi)>>
│ AppState.stats       │  open_markets, total_volume_24h
└──────────┬───────────┘
           │
     ┌─────┼──────────────────┐
     ▼     ▼                  ▼
  REST    WS               Tasks
  /api    Snapshot          Signal engine
  /ws     to frontend       Books, Trades
```

## Data Flow: Live Updates

```
Data API (trades)           CLOB API (books/mids/spreads)
       │                              │
       ▼                              ▼
task_data_trades              task_clob_books
  reads asset_map               reads markets (top 8)
  reads markets                 writes raw_books
  writes Market fields          writes books
  sends Ev::PriceUpdate         sends Ev::Book
       │                              │
       ▼                              ▼
CLOB WebSocket ──────────► handle_ws_msg
                             writes raw_books
                             writes Market.price_cents
                             sends Ev::Book
                                     │
                                     ▼
                              broadcast::Sender<Ev>
                                     │
                          ┌──────────┼──────────┐
                          ▼          ▼          ▼
                     WS clients  REST polls  Signal engine
```

## Key Design Decisions

1. **Domain owns its mapping** — `Market::from_gamma()` lives in domain, not ingestion. The domain knows how to construct itself from raw data.

2. **Ingestion is dumb** — `fetch_raw_markets()` returns `Vec<Value>`. No parsing, no filtering, no state. Just HTTP + pagination.

3. **Application orchestrates** — `refresh_markets()` is the only function that touches state. It coordinates ingestion → domain → state writes.

4. **AppState stays in main.rs** — shared state container for now. Will be extracted to `state/` in a future pass.

5. **Signal engine stays in main.rs** — reads Market fields directly. Will move to `application/` or `engine/` next.

6. **Trade domain owns structs + business logic** — `Trade`, `WhaleProfile`, `TradeWindow` and their methods live in domain. The state-coupled mapping stays in main.rs because it needs `asset_map` and `seen_hashes`.

## Migration Status

| Component | Status | Location |
|-----------|--------|----------|
| Market, Outcome | Done | domain/markets.rs |
| Market::from_gamma, is_live | Done | domain/markets.rs |
| fetch_raw_markets | Done | ingestion/markets.rs |
| refresh_markets | Done | application/market_service.rs |
| REST market handlers | Done | api/markets_handler.rs |
| Action, Trade | Done | domain/trades.rs |
| shorten_addr | Done | domain/trades.rs |
| WhaleProfile, TradeWindow | Done | domain/whales.rs |
| compute_whale_tag | Done | domain/whales.rs |
| fetch_raw_trades | Done | ingestion/trades.rs |
| store_trade, constants | Done | application/trade_service.rs |
| REST trade handlers | Done | api/trades_handler.rs |
| MarketBook, RawBook | TODO | → domain/books.rs |
| run_signals_on_trade | TODO | → engine/signals.rs |
| task_data_trades | TODO | → application/trade_service.rs (when AppState coupling reduced) |
| task_clob_books | TODO | → ingestion/books.rs |
| task_ws | TODO | → ingestion/ws.rs |
| task_signals | TODO | → engine/global_signals.rs |
| REST handlers (signals) | TODO | → api/signals_handler.rs |
| AppState | TODO | → state/mod.rs |
