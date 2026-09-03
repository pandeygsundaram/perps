// enum
// thoseThreeEvents

pub enum MarketEvents {
    Trade(TradeEvent),
    Ticker(TickerEvent),
    Depth(DepthEvent),
    DerivativesTicker(DerivativesTickerEvent),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Price(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Quantity(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rate(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timestamp(pub u64);


pub enum Side {
    Buy,
    Sell,
}

pub struct TradeEvent {
    pub symbol: String,
    pub trade_id: u64,

    pub price: Price,
    pub quantity: Quantity,

    pub taker_side: Side,
    
    pub timestamp: Timestamp,
}

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

pub struct DepthEvent {
    pub symbol: String,

    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,

    pub timestamps: u64,
}


pub struct DerivativesTickerEvent {
    pub symbol: String,

    pub mark_price: Price,
    pub index_price: Price,

    // total vol on left side
    // total vol on right side
    // that can be calculated from the ticker event thingy! 

    pub funding_rate: Rate,
    pub next_funding_time: Timestamp,

    pub timestamp: Timestamp ,
}

pub struct PriceLevel {
    pub price: Price,
    pub quantity: Quantity,
}
