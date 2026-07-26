use std::cmp;

use crate::types::{FillEvent, Orderbook, Side, Status};


pub fn try_match(book: &mut Orderbook) -> Vec<FillEvent> {
    let mut fills = Vec::new();

    loop {
        // see the thing is that we are pretty much done with the code
        // just figureout where are we exactly trying to delete a value while we have a muatable
        // reference of that hsit

        // basically if there is no overlap we break

        let Some((&best_bid, _)) = book.bids.last_key_value() else {
            break;
        };
        let Some((&best_ask, _)) = book.asks.first_key_value() else {
            break;
        };
        if best_bid < best_ask {
            break;
        }

        {
            // took mutable reference of best bid pricelevel
            let Some(bid_orders_tuple) = book.bids.get_mut(&best_bid) else {
                break;
            };
            // took mutable reference of best ask pricelevel
            let Some(ask_order_tuple) = book.asks.get_mut(&best_ask) else {
                break;
            };

            // here took mutable reference of vector of asks openorder
            let ask_order_vec = &mut ask_order_tuple.orders;
            // here took mutable reference of vector of bids openorder
            let bid_order_vec = &mut bid_orders_tuple.orders;

            // what all mutations are we doing
            // we are basically telling that listen if we have this order quantity done
            // then update the quantity!
            // get rid of the mut reference
            // and once the quantity is zero then delete me from the vector itself

            {
                let Some(curr_ask_order) = ask_order_vec.first_mut() else {
                    break;
                };
                let Some(curr_bid_order) = bid_order_vec.first_mut() else {
                    break;
                };

                let qty = cmp::min(curr_ask_order.qty, curr_bid_order.qty);

                let new_fill = FillEvent {
                    buyer_id: curr_bid_order.user_id.clone()  ,
                    price: best_ask,
                    qty: qty,
                    seller_id: curr_ask_order.user_id.clone() ,
                };

                fills.push(new_fill);
                book.last_traded_price = best_ask;

                // reduce and pop the values if the quantity is empty

                curr_ask_order.filled_qty += qty;
                curr_ask_order.qty -= qty;
                ask_order_tuple.total_qty -= qty;
                if curr_ask_order.qty == 0 {
                    curr_ask_order.status = Status::Filled;
                } else {
                    curr_ask_order.status = Status::PartiallyFilled;
                }

                curr_bid_order.filled_qty += qty;
                curr_bid_order.qty -= qty;
                bid_orders_tuple.total_qty -= qty;

                if curr_bid_order.qty == 0 {
                    curr_bid_order.status = Status::Filled;
                } else {
                    curr_bid_order.status = Status::PartiallyFilled;
                }
            }

            if ask_order_vec.first().is_some_and(|open_order| open_order.qty == 0) {
                ask_order_vec.remove(0);
            }
            if bid_order_vec.first().is_some_and(|open_order| open_order.qty == 0) {
                bid_order_vec.remove(0);
            }
        }

        // here deleting the price level if it's job is done

        let remove_ask_level = book.asks.get(&best_ask).is_some_and(|level| level.total_qty == 0 );
        let remove_bid_level = book.bids.get(&best_bid).is_some_and(|level|level.total_qty==0);

        if remove_ask_level{
            book.asks.remove(&best_ask);
        }
        if remove_bid_level {
            book.bids.remove(&best_bid);
        }
    }

    fills
}



pub fn market_order_sweep(
    book: &mut Orderbook,
    taker_user_id: String ,
    taker_side: Side,
    qty: i64,
    reference_price: i64,
    slippage_bps: i64,
) -> Vec<FillEvent> {
    let mut fills = Vec::new();

    let mut remaining_qty = qty;

    // basically so what we have to do is basically
    // we just have to see what side the user wants the order
    // if he want the order in short then
    // we will eat the orders in teh buy side and

    //  So basically algorigthm is
    //  if order side is long
    //  so i want to buy
    //  so i will go to the ask side
    //  get the price level for range
    //  and start consuming
    //  my positions will be built
    //  done
    //  else my orders will be short
    //  i will go ahead and
    //  consume the bids

    let max_price_limit = reference_price + (reference_price * slippage_bps / 10_000);
    let min_price_limit = reference_price - (reference_price * slippage_bps / 10_000);
    let taker_user_id_clone = taker_user_id.clone();

    while remaining_qty > 0 {
        // price > adjusted max limit -> break
        // get the range
        // then basically get the first order of that side
        // then generate the fill
        // update the fill
        // update the remain_qty
        if taker_side == Side::Short {
            let Some((&best_bid, _)) = book.bids.last_key_value() else {
                break;
            };
            if best_bid < min_price_limit {
                break;
            }

            let Some(price_level) = book.bids.get_mut(&best_bid) else {
                break;
            };

            // i have got the price level

            let Some(curr_open_order) = price_level.orders.first_mut() else {
                break; // basically this this is empty
            };

            let curr_qty = cmp::min(remaining_qty, curr_open_order.qty);
            if curr_open_order.qty == curr_qty {
                curr_open_order.status = Status::Filled;
            } else {
                curr_open_order.status = Status::PartiallyFilled;
            }

            let curr_fill = FillEvent {
                seller_id: taker_user_id_clone.clone(),
                price: best_bid,
                qty: curr_qty,
                buyer_id: curr_open_order.user_id.clone() ,
            };
            fills.push(curr_fill);

            curr_open_order.qty -= curr_qty;
            curr_open_order.filled_qty += curr_qty;

            remaining_qty -= curr_qty;
            price_level.total_qty -= curr_qty;

            // remove the first one if the order is basically fullfilled
            if price_level.orders.first().unwrap().qty == 0 {
                price_level.orders.remove(0);
            }

            // just simply consume the order
            // update the quantity
            // push the fills
            // delete the openorder if the quantity is empty

            // delete the bids tuple if it's quantity is consumed
            if book.bids.last_key_value().unwrap().1.total_qty == 0 {
                book.bids.remove(&best_bid);
            }

            // now got the mut priceleve
            // now going to

            // now extract the order
            // consume the quantity
            // if it is consumed
            // then check if it is not needed anymore

            // i have to consume more qty
            // consume the asks
        } else {
            let Some((&best_ask, _)) = book.asks.first_key_value() else {
                break;
            };
            if best_ask > max_price_limit {
                break;
            }

            let Some(price_level) = book.asks.get_mut(&best_ask) else {
                break;
            };

            // i have got the price level

            let Some(curr_open_order) = price_level.orders.first_mut() else {
                break; // basically this this is empty
            };

            let curr_qty = cmp::min(remaining_qty, curr_open_order.qty);
            if curr_open_order.qty == curr_qty {
                curr_open_order.status = Status::Filled;
            } else {
                curr_open_order.status = Status::PartiallyFilled;
            }

            let curr_fill = FillEvent {
                buyer_id: taker_user_id_clone.clone() ,
                price: best_ask,
                qty: curr_qty,
                seller_id: curr_open_order.user_id.clone() ,
            };
            fills.push(curr_fill);

            curr_open_order.qty -= curr_qty;
            curr_open_order.filled_qty += curr_qty;

            remaining_qty -= curr_qty;
            price_level.total_qty -= curr_qty;

            // remove the first one if the order is basically fullfilled
            if price_level.orders.first().unwrap().qty == 0 {
                price_level.orders.remove(0);
            }

            // just simply consume the order
            // update the quantity
            // push the fills
            // delete the openorder if the quantity is empty

            // delete the bids tuple if it's quantity is consumed
            if book.asks.first_key_value().unwrap().1.total_qty == 0 {
                book.asks.remove(&best_ask);
            }

            // i have to sell qty
            // consume the bids
        }
    }

    // get the values for a particular price range and try to fill the quantitess

    // if total accumulated quantity is >= req quantity
    // mark it as fullfilled
    // else
    // fill the quantity remaining reject the quantities

    fills
}
