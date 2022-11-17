use abstract_app::export_endpoints;
use abstract_app::AppContract;
use abstract_sdk::os::EXCHANGE;
use cosmwasm_std::Response;

use crate::error::TemplateError;
use crate::handlers::{self};
use crate::msg::{
    TemplateExecuteMsg, TemplateInstantiateMsg, TemplateMigrateMsg, TemplateQueryMsg,
};

// As an add-on writer, the only changes necessary to this file are with the handlers and API dependencies on the `TEMPLATE_ADDON` const.
pub type TemplateApp = AppContract<
    TemplateError,
    TemplateExecuteMsg,
    TemplateInstantiateMsg,
    TemplateQueryMsg,
    TemplateMigrateMsg,
>;

pub type TemplateResult = Result<Response, TemplateError>;

/// The namespace for the addon, like "abstract" -> "abstract:template"
pub const ADDON_NAMESPACE: &str = "template_namespace";
/// The name of the addon, excluding the namespace
pub const ADDON_NAME: &str = "template_addon_name";
/// The initial version of the addon, which will use the package version if not altered
const ADDON_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Expected replies
pub const EXAMPLE_REPLY_ID: u64 = 1;

/// Used as the foundation for building your addon.
/// All entrypoints are executed through this const (`instantiate`, `query`, `execute`, `migrate`)
/// The `dependencies` are Abstract API dependencies in the format: Vec(`namespace:contract_name`)
const ADDON: TemplateApp = TemplateApp::new(ADDON_NAME, ADDON_VERSION)
    .with_instantiate(handlers::instantiate_handler)
    .with_query(handlers::query_handler)
    .with_execute(handlers::execute_handler)
    .with_migrate(handlers::migrate_handler)
    .with_replies(&[(EXAMPLE_REPLY_ID, handlers::example_reply_handler)])
    .with_dependencies(&[EXCHANGE]);

// don't export endpoints when imported as library
#[cfg(not(feature = "library"))]
// Export the endpoints for this contract
export_endpoints!(ADDON, TemplateApp);
