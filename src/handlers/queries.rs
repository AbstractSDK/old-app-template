use crate::package::state::{CONFIG, COUNTS};
use crate::package::{ConfigResponse, UserCountResponse};
use abstract_os::objects::AssetEntry;
use cosmwasm_std::{to_binary, Addr, Deps, Env, Order, StdError, StdResult, Uint128};

pub fn query_config(deps: Deps, env: Env) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;

    Ok(ConfigResponse {
        max_count: config.max_count,
    })
}

//     let asset_weights: Vec<(AssetEntry, WeightedAsset)> = ASSET_WEIGHTS
//         .range(deps.storage, None, None, Order::Ascending)
//         .collect::<Result<Vec<(AssetEntry, WeightedAsset)>, StdError>>()?;
pub fn query_count(deps: Deps, _env: Env, user: Addr) -> StdResult<UserCountResponse> {
    let count = COUNTS.may_load(deps.storage, &user)?;
    let count = match count {
        Some(count) => count,
        None => Uint128::zero(),
    };

    Ok(UserCountResponse { user, count })
}
