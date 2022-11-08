use abstract_add_on::AddOnContract;
use abstract_sdk::abstract_os::add_on::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use abstract_sdk::{
    ExecuteEndpoint, InstantiateEndpoint, MigrateEndpoint, QueryEndpoint, ReplyEndpoint, EXCHANGE,
};
use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};

use crate::error::TemplateError;
use crate::handlers::{self};

use crate::msg::{
    TemplateExecuteMsg, TemplateInstantiateMsg, TemplateMigrateMsg, TemplateQueryMsg,
};

// As an add-on writer, the only changes necessary to this file are with the handlers and API dependencies on the `TEMPLATE_ADDON` const.
pub type TemplateAddOn<'a> = AddOnContract<
    TemplateError,
    TemplateExecuteMsg,
    TemplateInstantiateMsg,
    TemplateQueryMsg,
    TemplateMigrateMsg,
>;

pub type TemplateResult = Result<Response, TemplateError>;
pub const CONTRACT_NAME: &str = "template";

const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Used as the foundation for building your addon. All entrypoints are executed through this const.
/// `instantiate`, `query`, `execute`, `migrate`: entrypoint handlers
/// `dependencies`:  Vec(`namespace:contract_name`) of Abstract API dependencies
const TEMPLATE_ADDON: TemplateAddOn = TemplateAddOn::new(CONTRACT_NAME, CONTRACT_VERSION)
    .with_instantiate(handlers::instantiate_handler)
    .with_query(handlers::query_handler)
    .with_execute(handlers::execute_handler)
    .with_migrate(handlers::migrate_handler)
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
