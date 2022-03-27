use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {

    pub owner: Addr,
}

pub const BOARD1: Item<[i32;100]> = Item::new("Board1");
pub const BOARD2: Item<[i32;100]> = Item::new("Board2");