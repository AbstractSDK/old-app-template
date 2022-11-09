use crate::contract::TemplateAddOn;
use crate::msg::{ConfigResponse, TemplateQueryMsg, UserCountResponse};
use crate::state::{CONFIG, COUNTS};
use cosmwasm_std::{to_binary, Addr, Binary, Deps, Env, StdResult, Uint128};

pub fn query_handler(
    deps: Deps,
    env: Env,
    _add_on: &TemplateAddOn,
    msg: TemplateQueryMsg,
) -> StdResult<Binary> {
    match msg {
        TemplateQueryMsg::Config {} => to_binary(&query_config(deps, env)?),
        TemplateQueryMsg::UserCounts { users: _ } => {
            todo!()
            // to_binary(&queries::query_user_counts(deps, env, users)?)
        }
        TemplateQueryMsg::UserCountList {
            page_token: _,
            page_size: _,
        } => todo!(), // } => to_binary(&queries::query_user_count_list(deps, env)?),
        TemplateQueryMsg::UserCount { user } => to_binary(&query_count(deps, env, user)?),
    }
}

pub fn query_config(deps: Deps, _env: Env) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;

    Ok(ConfigResponse {
        max_count: config.max_count,
    })
}

pub fn query_count(deps: Deps, _env: Env, user: Addr) -> StdResult<UserCountResponse> {
    let count = COUNTS.may_load(deps.storage, &user)?;
    let count = match count {
        Some(count) => count,
        None => Uint128::zero(),
    };

    Ok(UserCountResponse { user, count })
}
