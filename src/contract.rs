use std::vec;

use abstract_add_on::AddOnContract;
use abstract_os::add_on::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use abstract_os::objects::fee::Fee;
use abstract_os::objects::AssetEntry;
use abstract_os::EXCHANGE;
use abstract_sdk::{ExecuteEndpoint, InstantiateEndpoint, MigrateEndpoint, QueryEndpoint};
use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Order, Reply, ReplyOn,
    Response, StdError, StdResult, SubMsg, WasmMsg,
};
use cw2::{get_contract_version, set_contract_version};
use cw20::{Balance, Cw20ReceiveMsg, MinterResponse};
use protobuf::Message;
use semver::Version;

use crate::error::TemplateError;
use crate::handlers::{commands, queries};
use crate::package::state::{Config, CONFIG, COUNTS};
use crate::package::{
    TemplateExecuteMsg, TemplateInstantiateMsg, TemplateMigrateMsg, TemplateQueryMsg,
};

pub type TemplateAddOn<'a> = AddOnContract<
    TemplateError,
    TemplateExecuteMsg,
    TemplateInstantiateMsg,
    TemplateQueryMsg,
    TemplateMigrateMsg,
>;

pub type TemplateResult = Result<Response, TemplateError>;
pub const CONTRACT_NAME: &str = "template";

/// Initial instantiation of the contract
pub fn instantiate_handler(
    mut deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _add_on: TemplateAddOn,
    msg: TemplateInstantiateMsg,
) -> TemplateResult {
    // Initial config
    let config: Config = Config {
        max_count: msg.max_count,
    };

    CONFIG.save(deps.storage, &config)?;

    if let Some(initial_counts) = msg.initial_counts {
        for (addr, count) in initial_counts {
            let addr = deps.api.addr_validate(addr.as_str())?;

            COUNTS.save(deps.storage, &addr, &count)?;
        }
    }

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("contract", CONTRACT_NAME))
}

pub fn query_handler(
    deps: Deps,
    env: Env,
    _add_on: &TemplateAddOn,
    msg: TemplateQueryMsg,
) -> StdResult<Binary> {
    match msg {
        TemplateQueryMsg::Config {} => to_binary(&queries::query_config(deps, env)?),
        TemplateQueryMsg::UserCounts { users } => {
            todo!()
            // to_binary(&queries::query_user_counts(deps, env, users)?)
        }
        TemplateQueryMsg::UserCountList {
            page_token,
            page_size,
        } => todo!(), // } => to_binary(&queries::query_user_count_list(deps, env)?),
        TemplateQueryMsg::UserCount { user } => to_binary(&queries::query_count(deps, env, user)?),
    }
}

/// Handle the `TemplateExecuteMsg`s sent to this add-on.
pub fn execute_handler(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    add_on: TemplateAddOn,
    msg: TemplateExecuteMsg,
) -> TemplateResult {
    match msg {
        TemplateExecuteMsg::UpdateConfig { max_count } => {
            commands::update_config(deps, info, add_on, max_count)
        }
        TemplateExecuteMsg::Increment {} => {
            commands::increment_sender(deps, info, add_on)
        }
    }
}

/// Unused for now but provided here as an example
/// Contract version is migrated automatically
pub fn migrate_handler(
    _deps: DepsMut,
    _env: Env,
    _add_on: TemplateAddOn,
    _msg: TemplateMigrateMsg,
) -> TemplateResult {
    Ok(Response::default())
}
