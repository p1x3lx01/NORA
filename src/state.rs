
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
    pub token_info: TokenInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub total_supply: u128,
}

pub const STATE: Item<State> = Item::new("state");
pub const BALANCES: Map<Addr, u128> = Map::new("balances");
