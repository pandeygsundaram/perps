#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    pub market: String,
    pub side: PositionSide,
    pub qty: i64,
    pub margin: i64,
    pub average_price: i64,
    pub liquidation_price: i64,
    pub unrealised_pnl: i64,
}


#[derive(Debug, PartialEq, Clone)]
pub enum PositionSide {
    LONG,
    SHORT,
}

pub struct Positions {
    pub positions: Vec<Position>,
}