use crate::{engine::state::POSITION, models::position::{Position, PositionSide, Positions}};

pub fn update_position(user_id:i64, market : &str, side: PositionSide , qty: i64, price: i64){
    let mut positions = POSITION.lock().unwrap();
    let user_positions = positions.entry(user_id).or_insert(Positions { positions: vec![] });

    if let Some(pos) = user_positions.positions.iter_mut().find(|p| p.market == market && p.side == side){
        let total_cost = pos.average_price * pos.qty + price * qty;
        pos.qty += qty;
        pos.average_price = total_cost/pos.qty;
        println!("updated position: user={} market={} side={:?} qty={} avg_price={}",
              user_id, market, pos.side, pos.qty, pos.average_price);
        
    }else {
        user_positions.positions.push(Position {
            market : market.to_string(),
            side: side.clone(),
            qty,
            margin: 0,
            average_price : price,
            liquidation_price : 0,
            unrealised_pnl : 0
        });
        println!("new position: user={} market={} side={:?} qty={} price={}",
              user_id, market, side, qty, price);
    }

}