# Whale.TERMINAL — Architecture Walkthrough

A walkthrough of the layered DDD architecture: what each layer owns,
why every decision was made, and how the pieces fit.

---

## Table of Contents

1. What We Started With
2. What Is DDD (and Why Should You Care)
3. The Four Layers
4. The Refactoring: Step by Step
5. Before vs After
6. Why Each Decision Was Made
7. Dependency Direction
8. What Each File Does Now
9. What Is Still in main.rs
10. How to Read the New Code
11. Common Beginner Questions
12. The Full Original main.rs Map

---

## 1. What We Started With

### The Original Monolith

Before the refactor, **every single line of code** lived in one file:

    src/main.rs -- 2,327 lines

That is it. One file. Everything.

### What Was Inside

Here is a map of what lived where inside that single file:

    Lines 1-20      Header comment (8 signals, whale score formula)
    Lines 21-65     Imports, constants (API URLs, limits, thresholds)
    Lines 67-94     Utility functions (shorten_addr, classify_signal, compute_whale_tag)
    Lines 96-262    Struct definitions (Level, OutcomeBook, MarketBook, Action, Trade,
                    TradeWindow, WhaleProfile)
    Lines 264-430   Signal types (SignalKind, EdgeSignal, Ev enum, EdgeFeedEvent)
    Lines 432-487   Stats, GlobalSignals, LeaderboardEntry
    Lines 488-662   Supabase license client, RawBook, BookSnapshot,
                    MarketSignalState, SignalDedup
    Lines 664-695   AppState struct (shared state container)
    Lines 697-870   REST handlers (h_validate, h_payment_info, h_gen_key)
    Lines 874-1290  Signal engine (run_signals_on_trade -- the 400-line monster)
    Lines 1290-1500 Task: live trade ingestion (task_data_trades)
    Lines 1504-1560 Task: CLOB order books (task_clob_books)
    Lines 1562-1589 Mid/spread fetching (fetch_mids_spreads)
    Lines 1591-1645 Book assembler (assemble_book)
    Lines 1647-1673 Leaderboard tasks
    Lines 1675-1804 Global signal computation (task_signals)
    Lines 1806-1879 WebSocket tasks (task_ws, handle_ws_msg)
    Lines 1881-1889 Heartbeat task
    Lines 1890-1980 REST handlers (h_health, h_trades, h_whales, h_stats, etc.)
    Lines 1980-2022 WebSocket handler + connection manager
    Lines 2026-2110 main() function + router setup

### The Problem

Everything depended on everything:

    main.rs
      structs depend on nothing (good)
      handlers depend on structs + AppState + helpers
      tasks depend on structs + AppState + handlers + helpers
      signal engine depends on structs + AppState + tasks + helpers
      AppState depends on ALL structs
      main() depends on ALL tasks + ALL handlers + ALL helpers

**One file means one reason to change for everything.** If the Gamma API URL
changes, you edit the same file as when the signal engine logic changes. If
you want to add a new field to Market, you scroll through 2300 lines to find
it. If you want to test the HTTP fetch, you cannot -- it is tangled with
parsing, filtering, state writing, and logging.

---

## 2. What Is DDD (and Why Should You Care)

### DDD = Domain-Driven Design

DDD is an architecture pattern that says: **organize your code around the
business domain, not around technical concerns.**

In a typical beginner project, you organize by technical type:

    src/
      models/       (all structs)
      handlers/     (all HTTP handlers)
      services/     (all business logic)
      utils/        (all helpers)

DDD says: **organize by what the code is about, not what it does technically.**

    src/
      domain/       (what are the facts and rules?)
      ingestion/    (where does data come from?)
      application/  (how do we orchestrate the work?)
      api/          (how do we send data to clients?)

### Why DDD for This Project

Whale.TERMINAL has a clear business domain: **Polymarket prediction markets**.
The core concepts are:

- **Markets** -- things being traded (have outcomes, prices, volumes)
- **Trades** -- actions taken by wallets (buy/sell, size, price)
- **Whales** -- wallets with large volume
- **Signals** -- patterns detected in trade flow

These are domain concepts. They should be organized by what they *mean*, not
by whether they are HTTP handlers or database queries.

### The Key Insight

> If you change the business rule for what makes a market "live", you should
> only touch the file that defines markets. You should not have to scroll past
> HTTP handlers, WebSocket code, and signal algorithms to find it.

DDD makes this possible by giving each concept its own home.
---

## 3. The Four Layers

### The Layer Diagram

    +--------------------------------------------------------------+
    |                        api                                    |
    |  "What do we send to clients?"                               |
    |  REST handlers, WebSocket message builders                   |
    |  Depends on: application, domain                             |
    +--------------------------------------------------------------+
    |                    application                                |
    |  "How do we orchestrate the work?"                           |
    |  Calls ingestion -> domain -> state                          |
    |  Depends on: ingestion, domain                               |
    +--------------------------------------------------------------+
    |                      domain                                   |
    |  "What are the facts and rules?"                             |
    |  Structs, business logic, mapping                            |
    |  Depends on: nothing in this crate (only serde, chrono)      |
    +--------------------------------------------------------------+
    |                    ingestion                                  |
    |  "Where does data come from?"                                |
    |  HTTP fetches, WebSocket connections, file reads             |
    |  Depends on: nothing in this crate (only reqwest, tokio)     |
    +--------------------------------------------------------------+
    |                  infrastructure                               |
    |  "How does the system run?"                                  |
    |  AppState, task scheduling, signal engine, WS broadcasting   |
    |  Depends on: everything                                      |
    +--------------------------------------------------------------+

### Layer 1: Domain (Facts + Rules)

**What goes here:** Structs, enums, and business logic that is true
*regardless* of how the system runs.

**Example:** A Market has a condition_id and outcomes whether you fetch it
from Gamma, store it in a database, or print it on paper. That is a fact
about markets.

**Example:** The rule "a market is live if it is not expired, not resolved,
and has volume" is a business rule. It does not depend on HTTP or WebSockets.

**File:** src/domain/markets.rs (142 lines)

The domain layer contains:
- Market struct -- the core fact
- Outcome struct -- one tradable side
- Market::from_gamma() -- maps raw JSON to Market (DTO to domain)
- Market::is_live() -- business rule for filtering
- Market::primary_prob() -- first outcome's price
- polymarket_url() -- URL builder
- infer_category() -- classifies market into Crypto/Politics/etc
- parse_str_arr(), parse_f64_arr(), 	runc() -- private parsing helpers

**Key rule:** Domain depends on **nothing** in this crate. It only uses
external crates like serde and chrono. This means you can test it in isolation.

### Layer 2: Ingestion (Raw Data from Endpoints)

**What goes here:** Code that fetches data from external APIs, reads files,
or connects to WebSockets. The ingestion layer is *dumb on purpose* -- it
does not know what a Market is.

**Example:** etch_raw_markets() fetches 30 pages from the Gamma API and
returns Vec<serde_json::Value>. It does not parse, filter, or store anything.

**File:** src/ingestion/markets.rs (35 lines)

The ingestion layer contains:
- etch_raw_markets() -- paginated HTTP fetch, returns raw JSON
- GAMMA_API constant -- base URL
- GAMMA_API_TOP_VOLUME constant -- volume-sorted URL

**Key rule:** Ingestion returns raw data (Vec<Value>). It does not know
what a Market is. This makes it trivial to swap data sources.

### Layer 3: Application (Orchestration)

**What goes here:** Code that coordinates the work. It calls ingestion to
get data, passes it through domain to validate/transform, and writes results
to state.

**Example:** efresh_markets() calls etch_raw_markets(), then maps each
raw JSON to a Market using Market::from_gamma(), then filters using
Market::is_live(), then writes to state.

**File:** src/application/market_service.rs (49 lines)

The application layer contains:
- efresh_markets() -- the orchestration pipeline
- 	ask_refresh_markets() -- 300s loop calling refresh_markets

**Key rule:** Application orchestrates but does not compute. It does not
contain business rules -- it calls domain functions that contain them.

### Layer 4: API (Transport)

**What goes here:** HTTP handlers, WebSocket message builders. The api layer
reads from state and serializes to JSON.

**Example:** h_markets() reads the market list from state and returns it
as JSON.

**File:** src/api/markets_handler.rs (44 lines)

The api layer contains:
- h_markets() -- GET /api/markets
- h_books() -- GET /api/books
- h_heatmap() -- GET /api/heatmap
- h_scanner() -- GET /api/scanner

**Key rule:** API handlers do not know how data was fetched or processed.
They just read state and format output.

### Why Not Just Two Layers?

You *could* do domain/ + main.rs. But then:
- Ingestion code (HTTP, pagination, rate limits) mingles with domain logic
- REST handlers mix transport concerns with business logic
- You cannot test the domain without spinning up an HTTP server

Four layers gives you **one reason to change per file**:
- Change the Gamma API URL? Edit ingestion/markets.rs
- Change the filtering rule? Edit domain/markets.rs
- Change the JSON format? Edit api/markets_handler.rs
- Change the refresh interval? Edit application/market_service.rs
---

## 4. The Refactoring: Step by Step

We chose load_markets as the first function to refactor because:
1. It was the largest single function (~120 lines)
2. It had 7 distinct responsibilities tangled together
3. It is called on boot and every 5 minutes -- critical path
4. It touches every layer (HTTP, parsing, filtering, state, logging)

### Step 1: Identify the Responsibilities

Here is the original load_markets() with each responsibility marked:

    async fn load_markets(state: &Arc<AppState>, client: &reqwest::Client) {

        // === RESPONSIBILITY 1: HTTP transport + pagination ===
        // Two for-loops fetching 5 + 25 pages from Gamma API
        let mut raw: Vec<serde_json::Value> = vec![];
        for page in 0..5usize {
            let url = format!("{}&offset={}", GAMMA_API, page * 100);
            let text = client.get(&url).send().await?.text().await?;
            raw.extend(serde_json::from_str(&text)?);
            tokio::time::sleep(Duration::from_millis(300)).await;
        }
        // Second pass: volume-sorted pages (25 more pages)
        for page in 0..25usize { /* ... same pattern ... */ }

        // === RESPONSIBILITY 2: Deduplication ===
        let mut seen = HashSet::new();
        raw.retain(|v| seen.insert(v["conditionId"]...));

        // === RESPONSIBILITY 3: JSON -> Market mapping ===
        let mut markets: Vec<Market> = vec![];
        for (i, v) in raw.iter().enumerate() {
            let condition_id = v["conditionId"]...;
            let question = v["question"]...;
            let outcomes = parse_str_arr(&v["clobTokenIds"])...;
            // 40 lines of field extraction + Outcome construction
            markets.push(Market { id: i, condition_id, slug, ... });
        }

        // === RESPONSIBILITY 4: Business filtering ===
        markets.retain(|m| {
            let date_ok = m.end_date > now;
            let not_resolved = !m.outcomes.any(|o| o.price >= 99.0);
            let has_volume = m.volume_24h > 10.0;
            date_ok && not_resolved && has_volume
        });

        // === RESPONSIBILITY 5: Index building ===
        let mut amap = HashMap::new();
        for (mi, m) in markets.iter().enumerate() {
            for (oi, o) in m.outcomes.iter().enumerate() {
                amap.insert(o.token_id.clone(), (mi, oi));
            }
        }

        // === RESPONSIBILITY 6: State writing ===
        *state.markets.write().unwrap() = markets;
        *state.asset_map.write().unwrap() = amap;
        state.stats.open_markets = count;
        state.stats.total_volume_24h = vol;

        // === RESPONSIBILITY 7: Logging ===
        println!("  {} markets loaded", markets.len());
    }

**The problem:** All 7 responsibilities are in one function. You cannot test
the HTTP fetch without also testing the parsing. You cannot test the business
filter without also testing the HTTP fetch. Changing the API URL touches the
same file as changing the filter rule.

### Step 2: Extract Domain (Facts + Rules)

The mapping logic (responsibility 3) and filtering logic (responsibility 4)
are **domain concerns**. They define what a Market is and what makes it "live".

**What moved to domain/markets.rs:**
- Market struct definition
- Outcome struct definition
- Market::from_gamma() -- extracted from the 40-line inline mapping block
- Market::is_live() -- extracted from the 15-line inline filtering block
- parse_str_arr(), parse_f64_arr(), trunc() -- private helpers
- polymarket_url() -- public URL builder
- infer_category() -- public category classifier

**Why from_gamma() lives in domain:**

The domain knows how to construct itself from raw data. The mapping logic
(JSON field to Market field) is a fact about what a Market looks like. If the
Gamma API changes its field names, you edit domain/markets.rs. You do not
touch ingestion or application.

**Why is_live() lives in domain:**

The rule "a market is live if not expired, not resolved, and has volume" is a
business rule. It does not depend on HTTP or state. You can test it with a
simple unit test: create a Market, call is_live(), check the result.

### Step 3: Extract Ingestion (Raw Data Fetch)

The HTTP transport (responsibility 1) is an **ingestion concern**. It fetches
raw data from the outside world.

**What moved to ingestion/markets.rs:**
- GAMMA_API constant
- GAMMA_API_TOP_VOLUME constant
- The two pagination for-loops become fetch_raw_markets()

**Why ingestion is dumb:**

etch_raw_markets() returns Vec<serde_json::Value>. It does not know what
a Market is. It does not parse, filter, or store anything. This makes it
trivial to swap data sources -- write a new ingestion module, keep domain
unchanged.

### Step 4: Extract Application (Orchestration)

The orchestration logic (responsibilities 2, 5, 6, 7) is an **application
concern**. It coordinates the pipeline.

**What moved to application/market_service.rs:**
- Deduplication logic
- Index building (asset_map)
- State writing (3 locks)
- Logging
- The 300s refresh loop becomes task_refresh_markets()

**Why application orchestrates:**

The efresh_markets() function reads like a recipe:
1. Fetch raw data
2. Deduplicate
3. Map to Market structs
4. Filter by business rules
5. Publish to state

Each step is one line. You can read the entire flow in 30 seconds.

### Step 5: Extract API (Transport)

The REST handlers are **transport concerns**. They read state and serialize
to JSON.

**What moved to api/markets_handler.rs:**
- h_markets()
- h_books()
- h_heatmap()
- h_scanner()

**Why API handlers are thin:**

These functions do not know how data was fetched or processed. They just read
from state and format output. This makes them trivial to test and change.

### Step 6: Rewire main.rs

After extracting the four layers, we updated main.rs:

**Removed from main.rs:**
- parse_str_arr() (moved to domain)
- parse_f64_arr() (moved to domain)
- trunc() (moved to domain)
- polymarket_url() (moved to domain)
- infer_category() (moved to domain)
- task_load_markets() (moved to application)
- h_markets(), h_books(), h_heatmap(), h_scanner() (moved to api)
- GAMMA_API constant (moved to ingestion)

**Added to main.rs:**

    mod domain;
    mod ingestion;
    mod application;
    mod api;

    use domain::markets::{polymarket_url, Market};

**Changed in main.rs:**

    // Boot spawn -- was: load_markets(&s, &c).await;
    application::market_service::refresh_markets(&s, &c).await;

    // Task spawn -- was: tokio::spawn(task_load_markets(...));
    tokio::spawn(application::market_service::task_refresh_markets(...));

    // Router -- was: get(h_markets)
    .route("/api/markets", get(api::markets_handler::h_markets))
    .route("/api/books",   get(api::markets_handler::h_books))
    .route("/api/heatmap", get(api::markets_handler::h_heatmap))
    .route("/api/scanner", get(api::markets_handler::h_scanner))

### Step 7: Verify Compilation

    cargo check
    Finished dev profile [unoptimized + debuginfo] target(s) in 3.64s

Clean build. No errors.
---

## 5. Before vs After

### File Count

|                        | BEFORE | AFTER  |
|------------------------|--------|--------|
| Total files            | 1      | 9      |
| Lines in main.rs       | 2,327  | 1,906  |
| New files              | --     | 8      |

### Responsibility Distribution

| Responsibility              | BEFORE                          | AFTER                             |
|-----------------------------|---------------------------------|-----------------------------------|
| Market struct definition    | main.rs (somewhere)             | domain/markets.rs line 4          |
| Outcome struct definition   | main.rs (somewhere)             | domain/markets.rs line 80         |
| Market::from_gamma()        | main.rs (inline, 40 lines)      | domain/markets.rs line 28         |
| Market::is_live()           | main.rs (inline, 15 lines)      | domain/markets.rs line 67         |
| parse_str_arr()             | main.rs (somewhere)             | domain/markets.rs line 106        |
| parse_f64_arr()             | main.rs (somewhere)             | domain/markets.rs line 114        |
| trunc()                     | main.rs (somewhere)             | domain/markets.rs line 129        |
| polymarket_url()            | main.rs (somewhere)             | domain/markets.rs line 90         |
| infer_category()            | main.rs (somewhere)             | domain/markets.rs line 134        |
| fetch_raw_markets()         | main.rs (inline, 30 lines)      | ingestion/markets.rs line 6       |
| refresh_markets()           | main.rs (as load_markets)       | application/market_service.rs:9   |
| task_refresh_markets()      | main.rs (as task_load_markets)  | application/market_service.rs:43  |
| h_markets()                 | main.rs (somewhere)             | api/markets_handler.rs line 7     |
| h_books()                   | main.rs (somewhere)             | api/markets_handler.rs line 12    |
| h_heatmap()                 | main.rs (somewhere)             | api/markets_handler.rs line 17    |
| h_scanner()                 | main.rs (somewhere)             | api/markets_handler.rs line 33    |

### Testability

| Test Case               | BEFORE                              | AFTER                                |
|-------------------------|-------------------------------------|--------------------------------------|
| Test HTTP fetch         | Impossible -- mixed with parsing    | cargo test on ingestion::markets     |
| Test is_live() rule     | Impossible -- inside load_markets   | cargo test on domain::markets        |
| Test mapping logic      | Impossible -- inside load_markets   | cargo test on domain::markets        |
| Test orchestrator       | Impossible -- mixed with HTTP       | Mock ingestion + state               |
| Test REST handler       | Impossible -- mixed with state      | Mock state, test h_markets           |

### Change Isolation

| Change                          | BEFORE (risk)                 | AFTER (risk)                         |
|---------------------------------|-------------------------------|--------------------------------------|
| Gamma API URL changes           | Edit main.rs (2,300 lines)    | Edit ingestion/markets.rs (35 lines) |
| Filter rule changes             | Edit main.rs (2,300 lines)    | Edit domain/markets.rs (142 lines)   |
| JSON format changes             | Edit main.rs (2,300 lines)    | Edit domain/markets.rs (37 lines)    |
| Add new REST endpoint           | Edit main.rs (2,300 lines)    | Add to api/markets_handler.rs        |
| Swap Gamma for another API      | Rewrite load_markets          | Write new ingestion module           |
---

## 6. Why Each Decision Was Made

### Why domain owns from_gamma()

**Decision:** The mapping logic (JSON to Market) lives in domain, not ingestion.

**Reason:** The domain knows what a valid Market looks like. If the Gamma API
changes its field names (e.g., conditionId to condition_id), you edit
domain/markets.rs. Ingestion does not need to change because it only returns
raw JSON.

**Beginner analogy:** Imagine you are a chef. The domain is your recipe book.
The ingestion is your grocery shopper. The shopper brings raw ingredients
(raw JSON). The recipe book (domain) knows how to turn ingredients into a
dish (Market). If the grocery store changes its packaging, the shopper does
not care -- they just bring the bags. The recipe book handles the rest.

### Why ingestion is dumb on purpose

**Decision:** etch_raw_markets() returns Vec<Value> -- no parsing, no
filtering, no state.

**Reason:** If ingestion knows about Market, then swapping the data source
means rewriting parsing logic. With dumb ingestion, you just write a new
fetch function that returns the same raw format.

**Beginner analogy:** A grocery shopper does not cook. They bring bags of
ingredients. If you switch stores, you get a new shopper. The recipe book
does not change.

### Why application orchestrates

**Decision:** efresh_markets() is the only function that touches state.
It coordinates the full pipeline.

**Reason:** Orchestration is a single concern. If you spread it across files,
you cannot see the full flow at a glance. Putting it in one function means
you read 49 lines and understand everything.

**Beginner analogy:** The head chef (application) tells the shopper
(ingestion) what to buy, then follows the recipe (domain) to cook, then
plates the dish (state). The head chef does not cook itself -- they follow
the recipe. They do not shop themselves -- they send the shopper.

### Why API handlers are thin

**Decision:** REST handlers just read state and serialize to JSON.

**Reason:** If API handlers contain business logic, changing the JSON format
requires understanding the business rules. With thin handlers, you can
change the output format without touching the domain.

**Beginner analogy:** The waiter (API) takes the plate from the kitchen
(state) and brings it to the customer. The waiter does not cook. The waiter
does not shop. The waiter just delivers.

### Why AppState stays in main.rs (for now)

**Decision:** AppState, the signal engine, and task scheduling remain in
main.rs.

**Reason:** AppState is the "glue" that ties everything together. It is
infrastructure. Extracting it would require a more complex dependency
injection setup. For now, it is fine where it is. The next phase will extract
it to state/mod.rs.

### Why we did not move everything at once

**Decision:** We extracted the markets pipeline first. Everything else stays
in main.rs.

**Reason:** Big-bang refactors are dangerous. If you move everything at once
and something breaks, you do not know which move caused the bug. By extracting
one pipeline at a time, each extraction is independently compilable and
testable. You can stop at any point and the system still works.

---

## 7. Dependency Direction

### The Arrow Rule

Dependencies should point **inward** toward the domain:

    api --> application --> domain <-- ingestion
                            ^
                            |
                       infrastructure

- api depends on application and domain
- application depends on ingestion and domain
- ingestion depends on NOTHING in this crate
- domain depends on NOTHING in this crate

### What This Means in Practice

    // domain/markets.rs -- depends on nothing in this crate
    use serde::{Deserialize, Serialize};
    // No use of crate::AppState, crate::ingestion, etc.

    // ingestion/markets.rs -- depends on nothing in this crate
    use std::time::Duration;
    // No use of crate::domain, crate::AppState, etc.

    // application/market_service.rs -- depends on domain + ingestion
    use crate::domain::markets::Market;
    use crate::ingestion::markets as gamma;
    use crate::AppState;

    // api/markets_handler.rs -- depends on AppState only
    use crate::AppState;

### Why Direction Matters

If domain depended on ingestion, you could not test domain without making
HTTP calls. If ingestion depended on domain, you could not swap data sources
without rewriting domain logic. By keeping both independent, each layer is
testable in isolation.
---

## 8. What Each File Does Now

### src/domain/mod.rs (1 line)

    pub mod markets;

Just declares the markets module.

### src/domain/markets.rs (142 lines)

Contains:
- Market struct -- 16 fields defining a prediction market
- Outcome struct -- 6 fields defining one tradable side
- Market::from_gamma() -- maps raw JSON Value to Market (37 lines)
- Market::is_live() -- business rule: not expired, not resolved, has volume
- Market::primary_prob() -- returns first outcome's price
- polymarket_url() -- builds Polymarket URL from slugs
- infer_category() -- classifies question text into Crypto/Politics/etc
- parse_str_arr() -- parses JSON string-or-array into Vec<String>
- parse_f64_arr() -- parses JSON string-or-array into Vec<f64>
- 	runc() -- truncates string to n chars with ellipsis

### src/ingestion/mod.rs (1 line)

    pub mod markets;

### src/ingestion/markets.rs (35 lines)

Contains:
- etch_raw_markets() -- fetches 5 + 25 pages from Gamma API
- GAMMA_API constant -- base URL for default sort
- GAMMA_API_TOP_VOLUME constant -- base URL for volume sort

The function returns Vec<serde_json::Value> -- raw JSON, no parsing.

### src/application/mod.rs (1 line)

    pub mod market_service;

### src/application/market_service.rs (49 lines)

Contains:
- efresh_markets() -- the full pipeline: fetch -> dedupe -> map -> filter -> publish
- 	ask_refresh_markets() -- 300s interval loop

### src/api/mod.rs (1 line)

    pub mod markets_handler;

### src/api/markets_handler.rs (44 lines)

Contains:
- h_markets() -- GET /api/markets -> full market list as JSON
- h_books() -- GET /api/books -> order books as JSON
- h_heatmap() -- GET /api/heatmap -> heatmap cells as JSON
- h_scanner() -- GET /api/scanner -> top 30 movers as JSON

### src/main.rs (1,906 lines)

Still contains everything else:
- AppState struct (shared state container)
- Ev enum (WebSocket events)
- All other structs (Trade, WhaleProfile, MarketBook, etc.)
- Signal engine (run_signals_on_trade -- 400 lines)
- Book assembler (assemble_book)
- Live trade ingestion (task_data_trades)
- CLOB book polling (task_clob_books)
- Mid/spread fetching (fetch_mids_spreads)
- Global signal computation (task_signals)
- WebSocket tasks (task_ws, handle_ws_msg)
- Leaderboard tasks
- Edge feed computation
- All remaining REST handlers
- Auth handlers (validate, payment, gen-key)
- main() function + router setup

---

## 9. What Is Still in main.rs

The 1,906 lines remaining in main.rs contain these components, listed in
order of extraction priority:

| Component                    | Lines  | Target File                  |
|------------------------------|--------|------------------------------|
| Trade, WhaleProfile,         | ~150   | domain/trades.rs             |
| TradeWindow                  |        |                              |
| MarketBook, RawBook,         | ~80    | domain/books.rs              |
| OutcomeBook, Level           |        |                              |
| EdgeSignal, GlobalSignals,   | ~100   | domain/signals.rs            |
| Stats                        |        |                              |
| run_signals_on_trade         | ~400   | engine/signals.rs            |
| (8 signals)                  |        |                              |
| assemble_book                | ~60    | engine/books.rs              |
| task_data_trades             | ~150   | ingestion/trades.rs          |
| task_clob_books              | ~60    | ingestion/books.rs           |
| fetch_mids_spreads           | ~30    | ingestion/books.rs           |
| task_ws + handle_ws_msg      | ~100   | ingestion/ws.rs              |
| task_signals                 | ~120   | engine/global_signals.rs     |
| REST handlers (h_health,     | ~100   | api/trades_handler.rs        |
| h_trades, h_whales, etc.)    |        | api/signals_handler.rs       |
| WebSocket handler            | ~100   | api/ws_handler.rs            |
| Auth (validate, payment,     | ~100   | api/auth.rs                  |
| gen-key)                     |        |                              |
| AppState + helpers           | ~80    | state/mod.rs                 |

The pattern is the same for each extraction:
1. Identify the responsibility boundary
2. Move facts to domain
3. Move raw data fetching to ingestion
4. Move orchestration to application
5. Move transport to api
6. Leave infrastructure plumbing in main.rs
---

## 10. How to Read the New Code

### Start with the Domain

Read src/domain/markets.rs first. This is the "vocabulary" of the system.
It tells you what a Market is, what an Outcome is, and what rules govern them.

### Then Read the Pipeline

Read src/application/market_service.rs. This is the "recipe". It tells you
how data flows through the system: fetch -> dedupe -> map -> filter -> publish.

### Then Read the Ingestion

Read src/ingestion/markets.rs. This tells you where data comes from. It is
simple -- just HTTP pagination.

### Then Read the API

Read src/api/markets_handler.rs. This tells you how data reaches clients.
It is simple -- just state reads + JSON serialization.

### Finally Read main.rs

Read src/main.rs last. It is the "glue" -- AppState, task scheduling, and
everything that has not been extracted yet. Focus on the boot sequence
(main function) and the router setup.

### The Dependency Flow

    main() boot
      |
      +-> application::market_service::refresh_markets()
            |
            +-> ingestion::markets::fetch_raw_markets()  (HTTP)
            |
            +-> Market::from_gamma()                      (domain mapping)
            |
            +-> Market::is_live()                         (domain filtering)
            |
            +-> state.markets.write()                     (infrastructure)

---

## 11. Common Beginner Questions

### Q: Why not just use modules without the DDD pattern?

You could. But without clear layer boundaries, modules tend to become
dumping grounds. "utils" grows forever. "services" gets everything that
does not fit elsewhere. DDD gives each file a clear purpose and clear rules
about what it can depend on.

### Q: Is this overkill for a small project?

It depends. For a 500-line project, probably yes. For a 2300-line project
that is actively growing, no. The key is: each extraction is independently
valuable. You do not have to do all 4 layers at once. You can start with
just domain + ingestion and leave the rest in main.rs.

### Q: Why not use dependency injection (DI)?

DI frameworks in Rust are complex and usually unnecessary. The pattern we
use is simpler: pass &Arc<AppState> and &reqwest::Client as function
arguments. This is "explicit dependency injection" -- you can see exactly
what each function needs by looking at its signature.

### Q: Why is the domain layer so small (142 lines)?

Because the domain layer only contains facts and rules. It does not contain
HTTP code, state management, or orchestration. Its small size is a feature --
it means the core business logic is focused and easy to understand.

### Q: Why does ingestion return Vec<Value> instead of Vec<Market>?

Because ingestion is dumb on purpose. If ingestion returned Vec<Market>, it
would need to know about the Market type, which means it depends on domain.
If you later want to swap Gamma for a different API that returns a different
JSON format, you would need to change the ingestion code AND the domain
mapping code. With Vec<Value>, ingestion is completely independent.

### Q: How do I know which layer a function belongs to?

Ask these questions:
1. Does it define a fact or business rule? -> domain
2. Does it fetch raw data from an external source? -> ingestion
3. Does it coordinate multiple steps into a pipeline? -> application
4. Does it serialize data for an HTTP response? -> api
5. Is it glue code that ties everything together? -> infrastructure (main.rs)

### Q: What is the difference between application and infrastructure?

Application orchestrates business workflows (fetch -> transform -> publish).
Infrastructure is the plumbing that makes the system run (AppState, task
scheduling, WebSocket broadcasting, router setup). Application knows about
the business domain. Infrastructure knows about everything.

### Q: Can I test the domain without running the server?

Yes! That is the whole point. Since domain/markets.rs depends on nothing in
this crate, you can write:

    #[test]
    fn test_is_live() {
        let m = Market { /* ... fields ... */ };
        assert!(m.is_live(chrono::Utc::now()));
    }

No HTTP server, no AppState, no Tokio runtime needed.

### Q: What happens if I break something during extraction?

Each extraction is independently compilable. If you break something, cargo
check will tell you exactly which file has the error. You do not need to
revert everything -- just fix the one file you changed.

### Q: How do I know when to stop extracting?

Stop when:
- main.rs is under 500 lines
- Each file has one clear responsibility
- You can explain what each file does in one sentence
- cargo check passes

---

## 12. The Full Original main.rs Map

Here is the complete map of what the original 2,327-line main.rs contained,
so you can see exactly what was where:

    Line    What
    ----    ----
    1-20    Header comment (8 signals, whale score formula)
    21      #![allow(dead_code)]
    22-48   use axum::..., use chrono::..., use serde::..., etc.
    49      use domain::markets::{polymarket_url, Market}; (ADDED)
    51-58   API endpoint constants (DATA_TRADES, DATA_LB, CLOB_BOOKS, etc.)
    60-65   Config constants (BROADCAST_CAP, MAX_TRADES, TRIAL_SECS, etc.)
    67-71   fn shorten_addr() -- truncate wallet address
    73-80   fn classify_signal() -- BULL/BEAR/HOT/BREAKOUT/NEUTRAL
    83-94   fn compute_whale_tag() -- classify whale behavior
    99-104  struct Level -- order book price level
    106-120 struct OutcomeBook -- one side of the book
    122-130 struct MarketBook -- full book for a market
    132-133 enum Action { BUY, SELL }
    135-157 struct Trade -- one trade event
    162-212 struct TradeWindow -- rolling 50-trade window
    214-261 struct WhaleProfile + impl (recompute, win_rate, etc.)
    264-276 enum SignalKind -- 8 signal types
    278-296 struct EdgeSignal -- one fired signal
    298-365 fn compute_edge_feed() -- edge signal computation
    367-376 async fn task_edge_feed() -- 2s loop
    378-387 impl EdgeSignal::priority_from_confidence()
    391-430 enum Ev -- WebSocket event variants
    432-438 struct SignalAccuracy
    440-459 struct GlobalSignals -- market-wide signal summary
    461-476 struct Stats -- system statistics
    478-487 struct LeaderboardEntry
    488-604 Supabase license client (db_insert_license, db_lookup_license)
    607-614 struct RawBook -- raw order book
    616-621 struct BookSnapshot -- for liquidity drain detection
    623-645 struct MarketSignalState -- per-market signal scratch state
    647-662 struct SignalDedup -- signal deduplication
    664-695 struct AppState -- shared state container
    697-870 Auth handlers (h_validate, h_payment_info, h_gen_key)
    874-1290 fn run_signals_on_trade() -- 8-signal engine (416 lines)
    1290-1500 async fn task_data_trades() -- live trade ingestion
    1504-1560 async fn task_clob_books() -- CLOB book polling
    1562-1589 async fn fetch_mids_spreads() -- mid/spread fetching
    1591-1645 fn assemble_book() -- raw books to MarketBook
    1647-1673 async fn task_leaderboards() + fetch_lb()
    1675-1804 async fn task_signals() -- 30s global signal computation
    1806-1879 async fn task_ws() + handle_ws_msg() -- CLOB WebSocket
    1881-1889 async fn task_heartbeat() -- 30s heartbeat
    1890-1912 REST handlers (h_health, h_trades, h_whales, h_stats, h_signals, h_edge_signals)
    1915-1930 h_set_threshold -- POST /api/set-threshold
    1935-1978 async fn ws_handler() + handle_ws_conn() -- WebSocket
    2026-2110 async fn main() -- boot, spawns, router, listener

The functions that were **removed** (moved to new files):
- parse_str_arr, parse_f64_arr, trunc -> domain/markets.rs
- polymarket_url, infer_category -> domain/markets.rs
- GAMMA_API constant -> ingestion/markets.rs
- task_load_markets -> application/market_service.rs (as task_refresh_markets)
- h_markets, h_books, h_heatmap, h_scanner -> api/markets_handler.rs

The functions that were **added** in main.rs:
- mod domain; mod ingestion; mod application; mod api;
- use domain::markets::{polymarket_url, Market};
- Updated boot spawn to call application::market_service::refresh_markets
- Updated task spawn to call application::market_service::task_refresh_markets
- Updated router to call api::markets_handler::h_markets etc.
