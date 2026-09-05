use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MarketEvents {
    Trade(TradeEvent),
    Ticker(TickerEvent),
    Depth(DepthEvent),
    DerivativesTicker(DerivativesTickerEvent),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct Price(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct Quantity(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct Rate(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct Timestamp(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TradeEvent {
    pub symbol: String,
    pub trade_id: u64,

    pub price: Price,
    pub quantity: Quantity,

    pub taker_side: Side,

    pub timestamp: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TickerEvent {
    pub symbol: String,

    pub last_price: Price,

    pub bid_price: Price,
    pub bid_quantity: Quantity,

    pub ask_price: Price,
    pub ask_quantity: Quantity,

    pub volume_24h: Quantity,
    pub quote_volume_24h: Quantity,

    pub high_24h: Price,
    pub low_24h: Price,

    pub timestamp: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DepthEvent {
    pub symbol: String,

    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,

    pub timestamps: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DerivativesTickerEvent {
    pub symbol: String,

    pub mark_price: Price,
    pub index_price: Price,

    // total vol on left side
    // total vol on right side
    // that can be calculated from the ticker event thingy!
    pub funding_rate: Rate,
    pub next_funding_time: Timestamp,

    pub timestamp: Timestamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct PriceLevel {
    pub price: Price,
    pub quantity: Quantity,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct ClientMessage {
    pub method: WsMethod,
    pub params: Vec<String>,
}

#[derive(Serialize ,Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum WsMethod {
    Subscribe,
    Unsubscribe,
}
