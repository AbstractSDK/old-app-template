use cosmwasm_std::{DepsMut, Env, Response};

use crate::contract::{ {{app_contract}},  {{app_result}}};

use crate::msg:: {{app_migrate_msg}};

/// Unused for now but provided here as an example
/// Contract version is migrated automatically
pub fn migrate_handler(
    _deps: DepsMut,
    _env: Env,
    _app:  {{app_contract}},
    _msg:  {{app_migrate_msg}},
) ->  {{app_result}} {
    Ok(Response::default())
}
