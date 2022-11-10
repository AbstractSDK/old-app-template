use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

/// BaseState is initialized in contract
/// State stores LP token address
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Config {
    /// The maximum a user's count is allowed to be
    pub max_count: Uint128,
}

pub const CONFIG: Item<Config> = Item::new("config");

/// Mapping of the user to their respective count
pub const COUNTS: Map<&Addr, Uint128> = Map::new("counts");
