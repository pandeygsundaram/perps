// function to add balance in the user

use shared_types::commands:: create_order::CreateOrderCommand;
use crate::{engine::state::BALANCE,};

pub enum OrderError {
    InsufficientBalance,
    UserNotFound,
}

pub fn process_add_order (
    cmd: CreateOrderCommand,
) -> Result<(), OrderError>{

    // first it will take the hold of the balances lock and then see if the 
    let mut balances = BALANCE.lock().unwrap();

    let balance = balances.get_mut(&cmd.user_id);
    match balance {
        Some(balance)=>{

            // deduct the money for the user trade 

            if balance.available >= cmd.margin {
                balance.locked += cmd.margin;
                balance.available -= cmd.margin; 
                Ok(())
            }else {
                Err(OrderError::InsufficientBalance)
                // publish in the pub sub that insufficient balance to place trade

            }

            // pass the order the relevant thread from here now!
            // basically that thread needs to add the order in the orderbook
            // and another process is supposed to read the balances and keep on matching the orders and shit!!
            // also the same process is supposed to push the events in the redis stream and
            // update the pnl for the users / unrealised profit or liquidate them

            // basically one thread is supposed to keep on iterating the users to liquidate them !

        }
        None => {
            Err(OrderError::UserNotFound)
        }        
    }
}