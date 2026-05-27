use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use crate::models::{balance::Balance, position::Positions};

// userid -> balance
pub static BALANCE: LazyLock<Mutex<HashMap<i64, Balance>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

// userid -> Position[]
pub static POSITION: LazyLock<Mutex<HashMap<i64, Positions>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
