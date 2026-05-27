use shared_types::commands::create_order::OrderSide;
use tokio::sync::mpsc::Receiver;
use crate::{engine::commands::OrderCommand, models::{order::{Order, OrderStatus}, orderbook::{OpenOrder, Orderbook, PriceLevel}}};

pub struct MarketWorker {
    pub market: String,
    pub orderbook: Orderbook,
    pub receiver: Receiver<OrderCommand>,
}

impl MarketWorker {
    pub async fn run(mut self) {
        while let Some(cmd) = self.receiver.recv().await {
            match cmd {
                OrderCommand::CreateOrder { order_id , user_id , side, price, qty, margin } => {
                    let order = Order {
                        user_id,
                        filled_qty : 0,
                        qty : qty,
                        order_id,
                        side : side.clone(),
                        margin,
                        status: OrderStatus::OPEN,
                        created_at : chrono::Utc::now(),
                        market:self.market.clone(),
                        price

                    };
                    self.add_order_to_book(order);
                    println!("got order: market={} price={} qty={} side={:?}", self.market, price, qty, side);
                }
                OrderCommand::CancelOrder { .. } => {}
                _ => {}
            }
        }
    }

    pub fn add_order_to_book( &mut self , order: Order){
        let new_open_order = OpenOrder {
            user_id : order.user_id,
            filled_qty : 0,
            qty : order.qty,
            order_id : order.order_id,
            created_at: order.created_at

        };

        let book_side = if order.side == OrderSide::LONG{
            &mut self.orderbook.bids
        } else {
             &mut self.orderbook.asks
        };

        let price_level = book_side.entry(order.price).or_insert(PriceLevel { total_qty: 0, orders: Vec::new() });
        price_level.total_qty+=  order.qty;
        price_level.orders.push(new_open_order);

    }
    pub fn try_match(&mut self) {
        loop {
            // if there is any fill going to happen then the position is going to 
            // get updated
            // and secondly the thing is that ummmm
            // so what is happening is basically we are goinng to do some cool thiny
            let best_ask = self.orderbook.asks.first_key_value();
            let best_bid = self.orderbook.bids.last_key_value();
            
            let prices = match (best_bid , best_ask) {
                (Some((bid_price, bid_level) ) , Some((ask_price , ask_level))) => {
                    if bid>= ask_price {
                        Some((
                            *bid_price,
                            *ask_price,
                            bid_level.orders[0].user_id,
                            ask_level.orders[0].user_id,
                            bid_level.orders[0].qty - bid_level.orders[0].filled_qty,
                            ask_level.orders[0].qty - ask_level.orders[0].filled_qty,

                        ))
                    } else {
                        None
                    }
                    }
                }
                _ =>None,
                    
            }       
            
        }
    }




}



// fn add_order_to_book
// fn try_match u
// fn publish fills events