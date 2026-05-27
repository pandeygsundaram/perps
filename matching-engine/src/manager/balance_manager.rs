// function to add balance in the user

use shared_types::commands::add_balance::AddBalanceCommand;
use crate::{engine::state::BALANCE, models::balance::Balance};

pub async fn process_add_balance (
    cmd: AddBalanceCommand,
    conn : &mut redis::aio::MultiplexedConnection
){
    let mut balances = BALANCE.lock().unwrap();

    let balance = balances.entry(cmd.user_id).or_insert(
        Balance{
            available:0,
            locked:0
        }
    );
    balance.available += cmd.amount;
    println!("Updated balance for {}" , cmd.user_id);


    
}