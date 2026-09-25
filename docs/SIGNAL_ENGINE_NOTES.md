# Signal Engine — Handwritten Notes

## The Problem

Old main.rs had 420 lines of signal code. Each signal locked mutexes
inside itself. Hard to test, hard to change, hard to understand.

## The Solution

Split into 3 parts:

1. Wrapper (main.rs) — locks data, passes to engine
2. Orchestrator (engine/mod.rs) — coordinates detectors
3. Detectors (engine/detect.rs) — pure logic, no locks

---

## The Flow (write this on paper)

```
Trade arrives
    │
    ▼
task_data_trades (main.rs)
    │
    │  calls: run_signals_on_trade(&state, &trade)
    ▼
┌─────────────────────────────────────────────────┐
│  FUNCTION 1: main.rs:326 (wrapper)              │
│                                                 │
│  fn run_signals_on_trade(state, trade) {        │
│      profiles = state.whale_profiles.lock()     │
│      recent_trades = state.recent_trades.lock() │
│      markets = state.markets.read()             │
│      raw_books = state.raw_books.lock()         │
│      mkt_signal_state = ...lock()               │
│      signal_dedup = ...lock()                   │
│      wallet_prev_action = ...lock()             │
│                                                 │
│      engine::run_signals_on_trade(              │
│          trade,          ◄── data to process    │
│          &profiles,      ◄── already locked     │
│          &recent_trades, ◄── already locked     │
│          &markets,       ◄── already locked     │
│          &raw_books,     ◄── already locked     │
│          &mut mkt_state, ◄── engine updates     │
│          &mut dedup,     ◄── engine updates     │
│          &mut prev_act,  ◄── engine updates     │
│          threshold,      ◄── f64 value          │
│          &callback       ◄── how to fire signal │
│      );                                         │
│  }                                              │
└──────────────────────┬──────────────────────────┘
                       │
                       │ engine:: prefix means
                       │ "go to engine module"
                       ▼
┌─────────────────────────────────────────────────┐
│  FUNCTION 2: engine/mod.rs:16 (orchestrator)    │
│                                                 │
│  pub fn run_signals_on_trade(                   │
│      trade, profiles, recent_trades, ...        │
│  ) {                                            │
│      // Call 8 detectors                        │
│      detect_whale_print(trade, threshold)       │
│      detect_smart_cluster(trade, profiles, ...) │
│      detect_conviction_spike(trade, profiles)   │
│      detect_whale_reversal(...)                 │
│      detect_velocity_surge(...)                 │
│      detect_stealth_accum(...)                  │
│      detect_prob_divergence(...)                │
│      detect_liquidity_drain(...)                │
│      detect_momentum_break(...)                 │
│                                                 │
│      // Fire signals that returned Some(...)    │
│      for signal in signals_to_fire {            │
│          fire_signal(signal);                   │
│      }                                          │
│  }                                              │
└──────────────────────┬──────────────────────────┘
                       │
                       │ each detector is called
                       ▼
┌─────────────────────────────────────────────────┐
│  FUNCTION 3: engine/detect.rs (pure logic)      │
│                                                 │
│  pub fn detect_smart_cluster(                   │
│      trade: &Trade,                             │
│      profiles: &HashMap,                        │
│      recent_trades: &VecDeque,                  │
│      now_ms: i64,                               │
│  ) -> Option<SignalOutput> {                    │
│      // Pure logic — no locks, no state         │
│      for t in recent_trades.iter() {            │
│          if profiles.get(t.wallet) score >= 70 {│
│              alpha_wallets.insert(t.wallet);    │
│          }                                      │
│      }                                          │
│      if alpha_wallets >= 3 {                    │
│          Some(SignalOutput { kind: "SMART_CLUSTER", ... })
│      } else {                                   │
│          None                                   │
│      }                                          │
│  }                                              │
└─────────────────────────────────────────────────┘
```

---

## Key Insight: Two Functions, Same Name

```
main.rs:     fn run_signals_on_trade(...)     ← wrapper
engine/mod.rs: pub fn run_signals_on_trade(...) ← implementation
```

When main.rs says `engine::run_signals_on_trade(...)`:
- The `engine::` prefix means "look in engine module"
- Without it, Rust would call main.rs version = infinite loop

---

## Why Locks Are in Wrapper, Not Detectors

OLD (bad):
```
signal 1: lock profiles, lock trades, detect, fire, unlock
signal 2: lock profiles, detect, fire, unlock
signal 3: lock trades, lock state, detect, fire, unlock
...
8 signals × 3 locks each = 24 lock/unlock operations
```

NEW (good):
```
wrapper: lock everything once (8 locks)
detector 1: receive &profiles, detect, return signal
detector 2: receive &profiles, detect, return signal
...
8 detectors × 0 locks = 0 lock/unlock operations
wrapper: unlock everything once (8 unlocks)
```

---

## The Callback

```rust
&|sig| state.fire_edge_signal(sig)
```

This is a function the engine can call. When engine fires a signal,
it calls this callback. The callback knows about AppState (it captures
`state` from the wrapper). The engine doesn't know about AppState.

Think of it like:
- Engine: "Here's a signal, fire it"
- Callback: "OK, I'll broadcast it via WebSocket"
- Engine doesn't know HOW it's fired, just THAT it's fired

---

## Testing

Old code: impossible to test without AppState
New code: test detectors with plain data

```rust
#[test]
fn test_smart_cluster() {
    let trade = Trade { ... };
    let profiles = HashMap::new(); // fake data
    let recent_trades = VecDeque::new(); // fake data
    
    let result = detect_smart_cluster(&trade, &profiles, &recent_trades, now);
    assert!(result.is_some()); // or assert!(result.is_none());
}
```

No server. No locks. Just data in, signal out.

---

## Summary (one sentence each)

1. Wrapper locks data
2. Wrapper passes data to engine
3. Engine calls 8 detectors
4. Each detector runs pure logic
5. Detector returns signal or None
6. Engine collects signals
7. Engine fires via callback
8. Callback broadcasts to WebSocket
