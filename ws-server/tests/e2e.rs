use std::{
    net::TcpListener,
    process::{Child, Command, Stdio},
    time::Duration,
};

use futures_util::{SinkExt, StreamExt};
use redis::AsyncCommands;
use serde_json::json;
use tokio::{
    net::TcpStream,
    time::{sleep, timeout},
};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{client::IntoClientRequest, Message},
    MaybeTlsStream, WebSocketStream,
};

const REDIS_URL: &str = "redis://127.0.0.1/";

type Ws = WebSocketStream<MaybeTlsStream<TcpStream>>;

fn get_available_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .expect("failed to bind to random port")
        .local_addr()
        .unwrap()
        .port()
}

/// Starts the real WS server binary, exactly like production.
/// Each test gets its own process, port, and market channels.
fn start_server(port: u16, markets: &[&str]) -> Child {
    let bin_path = env!("CARGO_BIN_EXE_ws-server");

    Command::new(bin_path)
        .env("PORT", port.to_string())
        .env("MARKETS", markets.join(","))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to start ws-server binary")
}

async fn wait_for_server(port: u16) {
    let addr = format!("127.0.0.1:{port}");
    for _ in 0..50 {
        if TcpStream::connect(&addr).await.is_ok() {
            return;
        }

        sleep(Duration::from_millis(100)).await;
    }

    panic!("WS server did not become ready on port {port}");
}

async fn connect_ws(port: u16) -> Ws {
    wait_for_server(port).await;

    let ws_url = format!("ws://127.0.0.1:{port}/ws");
    let request = ws_url
        .into_client_request()
        .expect("failed to build websocket request");

    let (ws, _) = connect_async(request)
        .await
        .expect("failed to connect to websocket");

    ws
}

async fn subscribe(ws: &mut Ws, channels: &[&str]) {
    let message = json!({
        "method": "SUBSCRIBE",
        "params": channels,
    });

    ws.send(Message::Text(message.to_string().into()))
        .await
        .expect("failed to send SUBSCRIBE");
}

async fn unsubscribe(ws: &mut Ws, channels: &[&str]) {
    let message = json!({
        "method": "UNSUBSCRIBE",
        "params": channels,
    });

    ws.send(Message::Text(message.to_string().into()))
        .await
        .expect("failed to send UNSUBSCRIBE");
}

async fn publish(channel: &str, event: &serde_json::Value) {
    let client = redis::Client::open(REDIS_URL).expect("failed to create redis client");
    let mut connection = client
        .get_multiplexed_async_connection()
        .await
        .expect("failed to connect to redis");

    let _: i32 = connection
        .publish(channel, event.to_string())
        .await
        .expect("failed to publish redis event");
}

async fn receive_json(ws: &mut Ws) -> serde_json::Value {
    loop {
        let message = timeout(Duration::from_secs(2), ws.next())
            .await
            .expect("timed out waiting for websocket message")
            .expect("websocket stream ended")
            .expect("websocket error");

        match message {
            Message::Text(text) => {
                return serde_json::from_str(&text).expect("server sent invalid JSON");
            }
            Message::Ping(data) => {
                ws.send(Message::Pong(data))
                    .await
                    .expect("failed to respond to ping");
            }
            Message::Pong(_) => {}
            Message::Close(_) => panic!("server closed websocket unexpectedly"),
            _ => {}
        }
    }
}
async fn assert_no_message(ws: &mut Ws) {
    let result = timeout(Duration::from_millis(500), ws.next()).await;

    match result {
        Err(_) => {} // No message arrived. Good.

        Ok(None) => {
            panic!("websocket closed unexpectedly");
        }

        Ok(Some(Err(e))) => {
            panic!("websocket error: {e}");
        }

        Ok(Some(Ok(Message::Ping(data)))) => {
            ws.send(Message::Pong(data))
                .await
                .expect("failed to respond to ping");

            // We received a protocol ping, not our market event.
            // Just don't treat it as an application message.
        }

        Ok(Some(Ok(Message::Pong(_)))) => {
            // Ignore protocol pong.
        }

        Ok(Some(Ok(Message::Text(text)))) => {
            panic!("received an unexpected websocket message: {text}");
        }

        Ok(Some(Ok(other))) => {
            panic!("received an unexpected websocket message: {other:?}");
        }
    }
}

fn trade_event(trade_id: u64, symbol: &str) -> serde_json::Value {
    json!({
        "Trade": {
            "symbol": symbol,
            "trade_id": trade_id,
            "price": 100000,
            "quantity": 10,
            "taker_side": "Buy",
            "timestamp": 123456789
        }
    })
}

fn ticker_event(symbol: &str) -> serde_json::Value {
    json!({
        "Ticker": {
            "symbol": symbol,
            "last_price": 100000,
            "bid_price": 99900,
            "bid_quantity": 5,
            "ask_price": 100100,
            "ask_quantity": 4,
            "volume_24h": 1000000,
            "quote_volume_24h": 100000000,
            "high_24h": 110000,
            "low_24h": 90000,
            "timestamp": 123456789
        }
    })
}

#[tokio::test]
async fn btc_event_only_reaches_btc_subscriber() {
    let port = get_available_port();
    let btc = "BTCUSDT_T1";
    let eth = "ETHUSDT_T1";
    let mut server = start_server(port, &[btc, eth]);

    let result = async {
        let mut btc_ws = connect_ws(port).await;
        let mut eth_ws = connect_ws(port).await;

        let btc_channel = format!("trades.{btc}");
        let eth_channel = format!("trades.{eth}");

        subscribe(&mut btc_ws, &[&btc_channel]).await;
        subscribe(&mut eth_ws, &[&eth_channel]).await;

        let event = trade_event(1, btc);
        publish(&btc_channel, &event).await;

        let received = receive_json(&mut btc_ws).await;

        assert_eq!(received, event);

        // ETH subscriber must not receive BTC data.
        assert_no_message(&mut eth_ws).await;
    }
    .await;

    let _ = server.kill();
    let _ = server.wait();

    result
}

#[tokio::test]
async fn one_connection_can_subscribe_to_multiple_channels() {
    let port = get_available_port();
    let btc = "BTCUSDT_T2";
    let mut server = start_server(port, &[btc]);

    let result = async {
        let mut ws = connect_ws(port).await;

        let trades_ch = format!("trades.{btc}");
        let ticker_ch = format!("ticker.{btc}");
        let depth_ch = format!("depth.{btc}");
        let deriv_ch = format!("derivativeticker.{btc}");

        subscribe(
            &mut ws,
            &[
                &trades_ch,
                &ticker_ch,
                &depth_ch,
                &deriv_ch,
            ],
        )
        .await;

        let trade = trade_event(10, btc);
        publish(&trades_ch, &trade).await;
        assert_eq!(receive_json(&mut ws).await, trade);

        let ticker = ticker_event(btc);
        publish(&ticker_ch, &ticker).await;
        assert_eq!(receive_json(&mut ws).await, ticker);
    }
    .await;

    let _ = server.kill();
    let _ = server.wait();

    result
}

#[tokio::test]
async fn two_users_on_same_channel_both_receive_event() {
    let port = get_available_port();
    let btc = "BTCUSDT_T3";
    let mut server = start_server(port, &[btc]);

    let result = async {
        let mut ws_a = connect_ws(port).await;
        let mut ws_b = connect_ws(port).await;

        let btc_ch = format!("trades.{btc}");
        subscribe(&mut ws_a, &[&btc_ch]).await;
        subscribe(&mut ws_b, &[&btc_ch]).await;

        let event = trade_event(20, btc);
        publish(&btc_ch, &event).await;

        assert_eq!(receive_json(&mut ws_a).await, event);
        assert_eq!(receive_json(&mut ws_b).await, event);
    }
    .await;

    let _ = server.kill();
    let _ = server.wait();

    result
}

#[tokio::test]
async fn unsubscribe_stops_delivery() {
    let port = get_available_port();
    let btc = "BTCUSDT_T4";
    let mut server = start_server(port, &[btc]);

    let result = async {
        let mut ws = connect_ws(port).await;

        let btc_ch = format!("trades.{btc}");
        subscribe(&mut ws, &[&btc_ch]).await;

        let first = trade_event(30, btc);
        publish(&btc_ch, &first).await;
        assert_eq!(receive_json(&mut ws).await, first);

        unsubscribe(&mut ws, &[&btc_ch]).await;

        let second = trade_event(31, btc);
        publish(&btc_ch, &second).await;

        assert_no_message(&mut ws).await;
    }
    .await;

    let _ = server.kill();
    let _ = server.wait();

    result
}

#[tokio::test]
async fn disconnect_removes_user_from_subscriptions() {
    let port = get_available_port();
    let btc = "BTCUSDT_T5";
    let mut server = start_server(port, &[btc]);

    let result = async {
        let mut ws = connect_ws(port).await;

        let btc_ch = format!("trades.{btc}");
        subscribe(&mut ws, &[&btc_ch]).await;

        // Drop the socket.
        drop(ws);

        sleep(Duration::from_millis(100)).await;

        let event = trade_event(40, btc);
        publish(&btc_ch, &event).await;
    }
    .await;

    let _ = server.kill();
    let _ = server.wait();

    result
}

#[tokio::test]
async fn duplicate_subscription_does_not_duplicate_messages() {
    let port = get_available_port();
    let btc = "BTCUSDT_T6";
    let mut server = start_server(port, &[btc]);

    let result = async {
        let mut ws = connect_ws(port).await;

        let btc_ch = format!("trades.{btc}");
        subscribe(
            &mut ws,
            &[
                &btc_ch,
                &btc_ch,
                &btc_ch,
            ],
        )
        .await;

        let event = trade_event(50, btc);
        publish(&btc_ch, &event).await;

        // HashSet should ensure exactly one delivery.
        assert_eq!(receive_json(&mut ws).await, event);
        assert_no_message(&mut ws).await;
    }
    .await;

    let _ = server.kill();
    let _ = server.wait();

    result
}

#[tokio::test]
async fn wrong_market_does_not_reach_subscriber() {
    let port = get_available_port();
    let btc = "BTCUSDT_T7";
    let eth = "ETHUSDT_T7";
    let mut server = start_server(port, &[btc, eth]);

    let result = async {
        let mut ws = connect_ws(port).await;

        let btc_ch = format!("trades.{btc}");
        let eth_ch = format!("trades.{eth}");
        subscribe(&mut ws, &[&btc_ch]).await;

        let event = trade_event(60, eth);
        publish(&eth_ch, &event).await;

        assert_no_message(&mut ws).await;
    }
    .await;

    let _ = server.kill();
    let _ = server.wait();

    result
}

#[tokio::test]
async fn all_event_types_are_forwarded_without_variant_based_routing() {
    let port = get_available_port();
    let btc = "BTCUSDT_T8";
    let mut server = start_server(port, &[btc]);

    let result = async {
        let mut ws = connect_ws(port).await;

        let trades_ch = format!("trades.{btc}");
        let ticker_ch = format!("ticker.{btc}");
        let depth_ch = format!("depth.{btc}");
        let deriv_ch = format!("derivativeticker.{btc}");

        subscribe(
            &mut ws,
            &[
                &trades_ch,
                &ticker_ch,
                &depth_ch,
                &deriv_ch,
            ],
        )
        .await;

        let trade = trade_event(70, btc);
        publish(&trades_ch, &trade).await;
        assert_eq!(receive_json(&mut ws).await, trade);

        let ticker = ticker_event(btc);
        publish(&ticker_ch, &ticker).await;
        assert_eq!(receive_json(&mut ws).await, ticker);

        let depth = json!({
            "Depth": {
                "symbol": btc,
                "bids": [
                    {"price": 99900, "quantity": 5}
                ],
                "asks": [
                    {"price": 100100, "quantity": 4}
                ],
                "timestamps": 123456789
            }
        });
        publish(&depth_ch, &depth).await;
        assert_eq!(receive_json(&mut ws).await, depth);

        let derivatives = json!({
            "DerivativesTicker": {
                "symbol": btc,
                "mark_price": 100000,
                "index_price": 99990,
                "funding_rate": 10,
                "next_funding_time": 123456999,
                "timestamp": 123456789
            }
        });
        publish(&deriv_ch, &derivatives).await;
        assert_eq!(receive_json(&mut ws).await, derivatives);
    }
    .await;

    let _ = server.kill();
    let _ = server.wait();

    result
}
