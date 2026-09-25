# Whale-Terminal-Backend

Live Polymarket whale-tracking engine in Rust — ingests real trades and order books,
detects smart-money flow with an 8-signal detection engine, and streams it to a
terminal dashboard over WebSocket + REST.

![Whale Terminal UI](docs/screenshot.png)

> Save the UI screenshot as `docs/screenshot.png` to render the image above.

## What it does

- **Live ingestion** — polls Polymarket's Data API for trades (~2s), fetches CLOB
  order books (~4s), and subscribes to the CLOB WebSocket for real-time
  `book` / `price_change` events.
- **8-signal detection engine** — every ingested trade runs through Smart Cluster,
  Velocity Surge, Stealth Accumulation, Liquidity Drain, Probability Divergence,
  Whale Reversal, Conviction Spike, and Momentum Break, with per-signal dedup and
  accuracy tracking.
- **Wallet intelligence** — per-wallet profiles with rolling 30-trade windows;
  Whale Score = `win_rate×0.35 + roi×0.30 + consistency×0.20 + volume×0.15`.
- **Edge feed** — lightweight confluence scorer (`EXECUTE` / `PREPARE` / `WAIT`)
  fusing whale flow, liquidity events, and composite sentiment.
- **Streaming + REST** — one broadcast channel fans out to WebSocket clients
  (`Snapshot` on connect, then live events); REST endpoints serve the same state.

## Architecture

Five layers, dependencies point inward (`api → application → domain`):

```
src/
├── domain/        # Pure types: books, markets, signals, trades, whales
├── ingestion/     # Polymarket Data API + CLOB pollers (markets, trades)
├── engine/        # Signal detection (detect) + per-market state (state)
├── application/   # Use-cases: market_service, trade_service
├── api/           # HTTP + WS surface: auth, markets/trades/signals handlers
└── main.rs        # AppState, broadcast fan-out, background tasks, router
```

State is in-memory (`RwLock`/`Mutex` over markets, trades, books, profiles;
`tokio::broadcast` channel, cap 4096). No database, no Docker — clone and run.

## Requirements

- Rust (stable) — https://rustup.rs
- Internet access — the backend streams live Polymarket APIs

## Run

```powershell
cargo run
```

First build takes a few minutes; after that it starts in seconds. Then open:

- **Dashboard:** served by the companion frontend repo, pointed at this API
- **WebSocket:** ws://localhost:8080/ws

Port defaults to `8080` (`PORT` env overrides; auto-scans `8080–8099` if busy).
Stop with `Ctrl+C`.

## API

| Endpoint | Description |
|---|---|
| `GET /health` | Liveness check |
| `GET /api/markets` | Tracked markets with live prices |
| `GET /api/books` | Assembled order books (top 8 levels/side) |
| `GET /api/trades` | Recent trades (newest first) |
| `GET /api/whales` | Whale profiles + leaderboard |
| `GET /api/stats` | Global stats (volume, wallets, biggest trade) |
| `GET /api/signals` | Composite signals snapshot |
| `GET /api/edge-signals` | Fired edge signals (latest 20) |
| `GET /api/heatmap` | Market heatmap data |
| `GET /api/scanner` | Scanner view |
| `POST /api/set-threshold` | Tune alert thresholds live |
| `GET /ws` | WebSocket: `Snapshot` on connect, then `Trade`, `Book`, `WhaleAlert`, `EdgeSignal`, `Signals`, `Stats`, `Heartbeat` events |

Quick check:

```powershell
curl http://localhost:8080/health
curl http://localhost:8080/api/stats
```

## Tech stack

`axum` (HTTP + WebSocket) · `tokio` (async runtime, broadcast fan-out) ·
`tower-http` (CORS, static file) · `reqwest` + `tokio-tungstenite` (Polymarket
REST + WS clients) · `serde` / `serde_json` · `chrono` · `rand` · `uuid`

## Docs

Design notes live in [`docs/`](docs/): `architecture.md`,
`ARCHITECTURE_WALKTHROUGH.md`, `TRADES_PIPELINE.md`, `SIGNAL_ENGINE_NOTES.md`.

## Author

Built by [vedhande790-ux](https://github.com/vedhande790-ux). MIT licensed —
see [LICENSE](LICENSE).
