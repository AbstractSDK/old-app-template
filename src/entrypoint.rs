use crate::contract;
use crate::contract::{TemplateAddOn, TemplateResult, CONTRACT_NAME};
use crate::package::{
    TemplateExecuteMsg, TemplateInstantiateMsg, TemplateMigrateMsg, TemplateQueryMsg,
};
use abstract_os::add_on::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use abstract_os::EXCHANGE;
use abstract_sdk::{
    ExecuteEndpoint, InstantiateEndpoint, MigrateEndpoint, QueryEndpoint, ReplyEndpoint,
};
use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Reply, StdResult};

// As an add-on writer, the only changes necessary to this file are with the handlers and API dependencies on the `TEMPLATE_ADDON` const.

const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Used as the foundation for building your addon. All entrypoints are executed through this const.
/// `instantiate`, `query`, `execute`, `migrate`: entrypoint handlers
/// `dependencies`:  Vec(`namespace:contract_name`) of Abstract API dependencies
const TEMPLATE_ADDON: TemplateAddOn = TemplateAddOn::new(CONTRACT_NAME, CONTRACT_VERSION)
    .with_instantiate(contract::instantiate_handler)
    .with_query(contract::query_handler)
    .with_execute(contract::execute_handler)
    .with_migrate(contract::migrate_handler)
    .with_dependencies(&[EXCHANGE]);

/// Instantiate entrypoint
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg<TemplateInstantiateMsg>,
) -> TemplateResult {
    TEMPLATE_ADDON.instantiate(deps, env, info, msg)
}

/// Execute entrypoint
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg<TemplateExecuteMsg>,
) -> TemplateResult {
    TEMPLATE_ADDON.execute(deps, env, info, msg)
}

/// Query entrypoint
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg<TemplateQueryMsg>) -> StdResult<Binary> {
    TEMPLATE_ADDON.query(deps, env, msg)
}

/// Migrate entrypoint
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg<TemplateMigrateMsg>) -> TemplateResult {
    TEMPLATE_ADDON.migrate(deps, env, msg)
}

// Reply entrypoint
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> TemplateResult {
    TEMPLATE_ADDON.reply(deps, env, msg)
}
