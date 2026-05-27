# Perpetual Exchange — Backend

A distributed perpetual futures exchange backend built in Rust. Designed as a modular,
event-driven system where the API server and matching engine are completely separate
services communicating asynchronously through Redis.

## Tech Stack

- **Axum** — HTTP API server
- **Tokio** — async runtime
- **Redis Streams** — async command queue (API → Engine)
- **Redis PubSub** — ACK channel (Engine → API)
- **PostgreSQL + SQLx** — persistent storage
- **Rust Workspaces** — monorepo with shared types

---

## Architecture

```
Client
  ↓ HTTP
API Server
  ↓ XADD engine:commands
Redis Stream
  ↓ XREAD
Matching Engine
  ↓ PUBLISH engine:acks
Redis PubSub
  ↓ SUBSCRIBE
API Server
  ↓ HTTP Response
Client

Matching Engine also:
  ↓ XADD engine:events
DB Writer Service (reads fills, balance changes, positions → persists to Postgres)
```

### Matching Engine Internals

```
Main Thread
  ├── owns global BALANCE map (user_id → Balance)
  ├── owns global POSITION map (user_id → Positions)
  ├── reads Redis stream in a loop
  ├── validates commands (margin check etc)
  └── dispatches to market workers via mpsc channels

Market Worker (one per market, own thread)
  ├── owns its Orderbook (bids BTreeMap, asks BTreeMap)
  ├── receives OrderCommands from main thread
  ├── runs matching loop on each new order
  └── updates POSITION map on fills
```

---

## Workspace Structure

```
perp-rust/
├── api-server/         — Axum HTTP server, auth, routes, handlers
├── matching-engine/    — Core matching engine, orderbook, position management
└── libs/
    ├── shared-types/   — Command and event types shared across services
    └── redis-client/   — Shared Redis connection utility
```

---

## Order Types

- **Limit Order** — specify exact price, sits in orderbook until matched
- **Market Order** — executes immediately at best available price, specify slippage tolerance

## Order Sides

- **LONG** — buying, goes into bids side of orderbook
- **SHORT** — selling, goes into asks side of orderbook

---

## TODO

---

### Phase 1 — Limit Order End to End (in progress)

Goal: place a limit order, it sits in the orderbook, two opposing orders match, both users get positions, API server gets a response.

- [x] **matching-engine** — Redis stream consumer loop reading from `engine:commands`
- [x] **matching-engine** — AddBalance: lock BALANCE map, insert/update available balance
- [x] **matching-engine** — CreateOrder: margin check, lock funds, send to market worker via mpsc
- [x] **matching-engine** — Publish ACK to `engine:acks` pubsub after each command
- [x] **matching-engine** — Publish event to `engine:events` stream after order placed
- [ ] **shared-types** — Clean up `EngineCommand` — finalise variants: `AddBalance`, `CreateOrder`, `CancelOrder`, `ClosePosition`, `AddMarket`
- [ ] **matching-engine** — Clean up `OrderCommand` — remove read queries (`GetBalance`, `GetPosition` etc), those are answered by main thread directly
- [ ] **matching-engine** — Market worker: insert order into `orderbook.bids` (LONG) or `orderbook.asks` (SHORT), update `total_qty` on the price level
- [ ] **matching-engine** — Market worker: `try_match()` loop — highest bid vs lowest ask, if `bid_price >= ask_price` fill `min(bid_remaining, ask_remaining)`, update `filled_qty`, remove fully filled orders, remove empty price levels
- [ ] **matching-engine** — Market worker: on fill call `update_position` for both buyer and seller — create new position or average into existing one
- [ ] **matching-engine** — Market worker: after each fill XADD fill event to `engine:events` with buyer_id, seller_id, market, price, qty, timestamp
- [ ] **shared-types** — Add `request_id: String` to every command so engine can echo it back in ACK
- [ ] **matching-engine** — Echo `request_id` in ACK publish so API server knows which request to unblock
- [ ] **api-server** — Loopback utility: generate UUID `request_id`, attach to command, XADD to `engine:commands`, store `oneshot::Sender` in global map, await receiver with 10s timeout
- [ ] **api-server** — ACK consumer background task: subscribe to `engine:acks`, parse `request_id`, fire the waiting `oneshot::Sender`
- [ ] **api-server** — Wire `POST /orders` to loopback, return order_id or error
- [ ] **api-server** — Wire `POST /balance/add` to loopback, return updated balance

---

### Phase 2 — Order Lifecycle

Goal: orders can be cancelled, positions closed, partial fills tracked correctly.

- [ ] **matching-engine** — CancelOrder: remove from orderbook bids/asks, unlock margin (locked -= margin, available += margin), publish cancel event
- [ ] **matching-engine** — Track `OrderStatus` transitions: `OPEN` → `PARTIALLY_FILLED` → `FILLED` / `CANCELLED`, include in fill events
- [ ] **matching-engine** — ClosePosition: validate user has open position, send opposing order to worker at current market price to close, release margin
- [ ] **api-server** — Wire `DELETE /orders/:id` to CancelOrder command via loopback
- [ ] **api-server** — Wire `POST /positions/close` to ClosePosition command via loopback

---

### Phase 3 — Market Orders

Goal: user can place a market order that executes immediately at best available price.

- [ ] **shared-types** — Update `CreateOrderCommand`: add `order_type: OrderType` (LIMIT/MARKET), make `price` optional (`Option<i64>`), add `slippage_bps: i64` (e.g. 50 = 0.5% max slippage)
- [ ] **matching-engine** — Market worker: if `order_type == MARKET`, skip orderbook insertion, immediately sweep best available price levels until qty filled or slippage exceeded
- [ ] **matching-engine** — If slippage exceeded before full fill: reject remaining qty, unlock proportional margin, publish partial fill event

---

### Phase 4 — Typed Responses and Events

Goal: engine speaks typed structs back to API server, not raw strings.

- [ ] **shared-types** — Add `EngineResponse` enum: `OrderPlaced { order_id }`, `OrderCancelled { order_id }`, `BalanceUpdated { available, locked }`, `PositionOpened { market, side, qty, avg_price }`, `Error { code, message }`
- [ ] **shared-types** — Add `EngineEvent` enum: `Fill { buyer_id, seller_id, market, price, qty, timestamp }`, `BalanceChanged { user_id, delta, new_available }`, `PositionChanged { user_id, market, side, qty, avg_price }`, `Liquidation { user_id, market, qty, price }`
- [ ] **matching-engine** — Replace all raw `format!()` JSON strings in publisher with proper `serde_json::to_string(&EngineEvent::...)` calls
- [ ] **api-server** — Deserialize ACK payload into `EngineResponse` and return typed HTTP responses

---

### Phase 5 — DB Writer Service

Goal: everything persisted to Postgres so state survives restarts.

- [ ] **db-writer** — New crate in workspace, connects to Postgres via SQLx and Redis
- [ ] **db-writer** — Read from `engine:events` using a consumer group so no event is ever processed twice
- [ ] **db-writer** — On `Fill` event: insert into `fills` table
- [ ] **db-writer** — On `BalanceChanged` event: upsert into `balances` table
- [ ] **db-writer** — On `PositionChanged` event: upsert into `positions` table
- [ ] **db-writer** — Use Redis stream message ID as idempotency key to handle duplicate events on restart

---

### Phase 6 — Unrealised PnL and Liquidation

Goal: positions show live PnL, underwater positions get force closed.

- [ ] **matching-engine** — After every fill recalculate `unrealised_pnl` for all users with open positions in that market: LONG = `(index_price - avg_entry) * qty`, SHORT = `(avg_entry - index_price) * qty`
- [ ] **matching-engine** — Liquidation task: tokio task running every N seconds, iterates POSITION map, checks if `(available + locked + unrealised_pnl) < maintenance_margin` (maintenance margin = 5% of notional)
- [ ] **matching-engine** — If underwater: force close at market price, zero out position, publish `Liquidation` event to `engine:events`
- [ ] **api-server** — Position response includes `unrealised_pnl` field
