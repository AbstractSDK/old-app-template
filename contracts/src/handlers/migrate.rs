use cosmwasm_std::{DepsMut, Env, Response};

use crate::contract::{TemplateApp, TemplateResult};

use crate::msg::TemplateMigrateMsg;

/// Unused for now but provided here as an example
/// Contract version is migrated automatically
pub fn migrate_handler(
    _deps: DepsMut,
    _env: Env,
    _app: TemplateApp,
    _msg: TemplateMigrateMsg,
) -> TemplateResult {
    Ok(Response::default())
}
