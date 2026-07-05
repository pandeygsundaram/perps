// ============================================================
// ENGINE COMMAND SEEDER
// Pushes a realistic batch of commands to the engine:commands
// Redis stream — covers every command type the engine handles.
//
// Run: cargo run --bin seeder
//
// What gets pushed (in order):
//   1.  AddMarket      — spin up BTC and ETH markets
//   2.  AddBalance     — fund 4 users
//   3.  CreateOrder    — limit long  (Alice buys BTC)
//   4.  CreateOrder    — limit short (Bob  sells BTC)
//   5.  CreateOrder    — limit long  (Carol buys ETH)
//   6.  CreateOrder    — market buy  (Dave  sweeps asks)
//   7.  CreateOrder    — market sell (Eve   sweeps bids)
//   8.  CreateOrder    — reduce-only (Alice partially closes her long)
//   9.  CreateOrder    — post-only   (Bob places maker-only order)
//   10. CreateOrder    — cross-margin long (Frank opens with cross)
//   11. CancelOrder    — Bob cancels his resting limit
//   12. AddTrigger     — Alice sets stop-loss at 90
//   13. AddTrigger     — Alice sets take-profit at 120
//   14. CancelTrigger  — Alice cancels the take-profit
//   15. ClosePosition  — Carol closes her ETH long
//   16. AddBalance     — top-up for Grace (stress test user)
//   17-26. CreateOrder — 10 rapid limit orders at various prices
// ============================================================

use redis::{AsyncCommands, Client};
use serde_json::json;
use uuid::Uuid;

// ---- helpers ----

fn req_id() -> String {
    Uuid::new_v4().to_string()
}

// Push one command to engine:commands stream.
// Each entry has two fields: "type" and "payload" (JSON string).
// The engine reads type to dispatch, then deserialises payload.
async fn push(conn: &mut redis::aio::MultiplexedConnection, cmd_type: &str, payload: serde_json::Value) {
    let payload_str = payload.to_string();
    let _: String = conn
        .xadd(
            "engine:commands",
            "*",  // auto-generate stream ID
            &[("type", cmd_type), ("payload", &payload_str)],
        )
        .await
        .expect("xadd failed");
    println!("  → pushed {} | {}", cmd_type, payload_str);
}

#[tokio::main]
async fn main() {
    let client = Client::open("redis://127.0.0.1/").expect("redis connect failed");
    let mut conn = client
        .get_multiplexed_async_connection()
        .await
        .expect("get connection failed");

    // flush previous seeder data so we start clean
    let _: () = redis::cmd("DEL")
        .arg("engine:commands")
        .query_async(&mut conn)
        .await
        .unwrap();
    println!("cleared engine:commands\n");

    println!("=== SEEDING ENGINE COMMANDS ===\n");

    // ----------------------------------------------------------
    // 1-2. Spin up markets
    // ----------------------------------------------------------
    println!("-- markets --");

    push(&mut conn, "AddMarket", json!({
        "request_id":           req_id(),
        "market_id":            "BTC",
        "name":                 "Bitcoin",
        "initial_price":        100000,
        "maker_fee_rate":       0.0002,
        "taker_fee_rate":       0.0005,
        "maintenance_margin":   0.05,
        "initial_margin":       0.10,
        "tick_size":            10,
        "lot_size":             1,
        "max_leverage":         100,
        "price_band_pct":       0.10
    })).await;

    push(&mut conn, "AddMarket", json!({
        "request_id":           req_id(),
        "market_id":            "ETH",
        "name":                 "Ethereum",
        "initial_price":        3000,
        "maker_fee_rate":       0.0002,
        "taker_fee_rate":       0.0005,
        "maintenance_margin":   0.05,
        "initial_margin":       0.10,
        "tick_size":            1,
        "lot_size":             1,
        "max_leverage":         50,
        "price_band_pct":       0.10
    })).await;

    // ----------------------------------------------------------
    // 3-7. Fund users
    // ----------------------------------------------------------
    println!("\n-- balances --");

    for (user_id, amount) in [(1, 50000), (2, 30000), (3, 20000), (4, 10000), (5, 15000), (6, 25000)] {
        push(&mut conn, "AddBalance", json!({
            "request_id": req_id(),
            "user_id":    user_id,
            "amount":     amount
        })).await;
    }

    // ----------------------------------------------------------
    // 8. Limit LONG — Alice buys 5 BTC @ 99000
    // ----------------------------------------------------------
    println!("\n-- limit orders --");

    push(&mut conn, "CreateOrder", json!({
        "request_id":   req_id(),
        "user_id":      1,
        "market":       "BTC",
        "side":         "LONG",
        "order_type":   "LIMIT",
        "price":        99000,
        "qty":          5,
        "leverage":     10,
        "slippage_bps": 0,
        "reduce_only":  false,
        "post_only":    false,
        "margin_mode":  "ISOLATED"
    })).await;

    // 9. Limit SHORT — Bob sells 3 BTC @ 101000
    push(&mut conn, "CreateOrder", json!({
        "request_id":   req_id(),
        "user_id":      2,
        "market":       "BTC",
        "side":         "SHORT",
        "order_type":   "LIMIT",
        "price":        101000,
        "qty":          3,
        "leverage":     5,
        "slippage_bps": 0,
        "reduce_only":  false,
        "post_only":    false,
        "margin_mode":  "ISOLATED"
    })).await;

    // 10. Limit LONG — Carol buys 10 ETH @ 2950
    push(&mut conn, "CreateOrder", json!({
        "request_id":   req_id(),
        "user_id":      3,
        "market":       "ETH",
        "side":         "LONG",
        "order_type":   "LIMIT",
        "price":        2950,
        "qty":          10,
        "leverage":     20,
        "slippage_bps": 0,
        "reduce_only":  false,
        "post_only":    false,
        "margin_mode":  "ISOLATED"
    })).await;

    // 11. Limit LONG — Dave buys 2 BTC @ 100000 (will cross with Bob's ask)
    push(&mut conn, "CreateOrder", json!({
        "request_id":   req_id(),
        "user_id":      4,
        "market":       "BTC",
        "side":         "LONG",
        "order_type":   "LIMIT",
        "price":        100000,
        "qty":          2,
        "leverage":     10,
        "slippage_bps": 0,
        "reduce_only":  false,
        "post_only":    false,
        "margin_mode":  "ISOLATED"
    })).await;

    // ----------------------------------------------------------
    // 12-13. Market orders
    // ----------------------------------------------------------
    println!("\n-- market orders --");

    // Market BUY — Eve sweeps asks, 2% slippage tolerance
    push(&mut conn, "CreateOrder", json!({
        "request_id":   req_id(),
        "user_id":      5,
        "market":       "BTC",
        "side":         "LONG",
        "order_type":   "MARKET",
        "price":        0,
        "qty":          2,
        "leverage":     10,
        "slippage_bps": 200,
        "reduce_only":  false,
        "post_only":    false,
        "margin_mode":  "ISOLATED"
    })).await;

    // Market SELL — Frank sweeps bids, 1.5% slippage tolerance
    push(&mut conn, "CreateOrder", json!({
        "request_id":   req_id(),
        "user_id":      6,
        "market":       "ETH",
        "side":         "SHORT",
        "order_type":   "MARKET",
        "price":        0,
        "qty":          5,
        "leverage":     15,
        "slippage_bps": 150,
        "reduce_only":  false,
        "post_only":    false,
        "margin_mode":  "ISOLATED"
    })).await;

    // ----------------------------------------------------------
    // 14. Reduce-only — Alice partially closes her BTC LONG
    //     (engine must reject this if she has no position)
    // ----------------------------------------------------------
    println!("\n-- reduce-only --");

    push(&mut conn, "CreateOrder", json!({
        "request_id":   req_id(),
        "user_id":      1,
        "market":       "BTC",
        "side":         "SHORT",       // opposite side = closing
        "order_type":   "LIMIT",
        "price":        100500,
        "qty":          2,             // close 2 of her 5
        "leverage":     10,
        "slippage_bps": 0,
        "reduce_only":  true,          // KEY: won't flip to short if no long
        "post_only":    false,
        "margin_mode":  "ISOLATED"
    })).await;

    // ----------------------------------------------------------
    // 15. Post-only — Bob places a maker-only bid
    //     (engine rejects if it would immediately match)
    // ----------------------------------------------------------
    println!("\n-- post-only --");

    push(&mut conn, "CreateOrder", json!({
        "request_id":   req_id(),
        "user_id":      2,
        "market":       "BTC",
        "side":         "LONG",
        "order_type":   "LIMIT",
        "price":        98000,         // well below ask — will rest in book as maker
        "qty":          1,
        "leverage":     5,
        "slippage_bps": 0,
        "reduce_only":  false,
        "post_only":    true,          // KEY: reject if would match (taker)
        "margin_mode":  "ISOLATED"
    })).await;

    // ----------------------------------------------------------
    // 16. Cross-margin — Grace opens BTC long with cross mode
    //     (her entire balance backs this position, not just locked margin)
    // ----------------------------------------------------------
    println!("\n-- cross-margin --");

    push(&mut conn, "CreateOrder", json!({
        "request_id":   req_id(),
        "user_id":      6,
        "market":       "BTC",
        "side":         "LONG",
        "order_type":   "LIMIT",
        "price":        99500,
        "qty":          3,
        "leverage":     20,
        "slippage_bps": 0,
        "reduce_only":  false,
        "post_only":    false,
        "margin_mode":  "CROSS"        // KEY: full balance as margin buffer
    })).await;

    // ----------------------------------------------------------
    // 17. CancelOrder — Bob cancels his resting SHORT @ 101000
    //     (order_id 2 from earlier — engine unlocks his margin)
    // ----------------------------------------------------------
    println!("\n-- cancel order --");

    push(&mut conn, "CancelOrder", json!({
        "request_id": req_id(),
        "user_id":    2,
        "order_id":   2,               // Bob's SHORT order from step 9
        "market":     "BTC",
        "price":      101000,          // needed to find the price level in book
        "side":       "SHORT"
    })).await;

    // ----------------------------------------------------------
    // 18-19. Stop-loss and take-profit for Alice's BTC LONG
    // ----------------------------------------------------------
    println!("\n-- triggers (SL / TP) --");

    // Stop-loss: close if BTC drops to 90000
    push(&mut conn, "AddTrigger", json!({
        "request_id":    req_id(),
        "user_id":       1,
        "market":        "BTC",
        "trigger_type":  "STOP_LOSS",
        "trigger_price": 90000,        // fires when mark_price <= 90000
        "qty":           5,            // close the full position
        "position_side": "LONG"
    })).await;

    // Take-profit: close if BTC rises to 110000
    push(&mut conn, "AddTrigger", json!({
        "request_id":    req_id(),
        "user_id":       1,
        "market":        "BTC",
        "trigger_type":  "TAKE_PROFIT",
        "trigger_price": 110000,       // fires when mark_price >= 110000
        "qty":           5,
        "position_side": "LONG"
    })).await;

    // 20. Cancel the take-profit (Alice changes her mind)
    push(&mut conn, "CancelTrigger", json!({
        "request_id": req_id(),
        "user_id":    1,
        "order_id":   2               // TP trigger order_id
    })).await;

    // ----------------------------------------------------------
    // 21. ClosePosition — Carol closes her entire ETH long
    //     (engine places a market sell for her full position qty)
    // ----------------------------------------------------------
    println!("\n-- close position --");

    push(&mut conn, "ClosePosition", json!({
        "request_id":   req_id(),
        "user_id":      3,
        "market":       "ETH",
        "slippage_bps": 300            // 3% slippage tolerance for market close
    })).await;

    // ----------------------------------------------------------
    // 22-31. Rapid-fire limit orders at various price levels
    //        (stress test the orderbook depth)
    // ----------------------------------------------------------
    println!("\n-- orderbook depth stress --");

    let depth_orders = vec![
        // (user, side, price, qty, leverage)
        (1, "LONG",  98000, 1, 10),
        (2, "LONG",  97000, 2,  5),
        (3, "LONG",  96000, 3, 10),
        (4, "LONG",  95000, 1, 20),
        (5, "LONG",  94000, 2, 10),
        (1, "SHORT", 102000, 1, 10),
        (2, "SHORT", 103000, 2,  5),
        (3, "SHORT", 104000, 3, 10),
        (4, "SHORT", 105000, 1, 20),
        (5, "SHORT", 106000, 2, 10),
    ];

    for (user_id, side, price, qty, leverage) in depth_orders {
        push(&mut conn, "CreateOrder", json!({
            "request_id":   req_id(),
            "user_id":      user_id,
            "market":       "BTC",
            "side":         side,
            "order_type":   "LIMIT",
            "price":        price,
            "qty":          qty,
            "leverage":     leverage,
            "slippage_bps": 0,
            "reduce_only":  false,
            "post_only":    false,
            "margin_mode":  "ISOLATED"
        })).await;
    }

    // ----------------------------------------------------------
    // Done — print stream length
    // ----------------------------------------------------------
    let len: i64 = conn.xlen("engine:commands").await.unwrap();
    println!("\n=== DONE — {} commands in engine:commands ===", len);
    println!("Read them: redis-cli XRANGE engine:commands - +");
}
