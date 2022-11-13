use cosmwasm_std::{DepsMut, Env, Response};

use crate::contract::{ {{addon_contract}},  {{addon_result}}};

use crate::msg:: {{addon_migrate_msg}};

/// Unused for now but provided here as an example
/// Contract version is migrated automatically
pub fn migrate_handler(
    _deps: DepsMut,
    _env: Env,
    _add_on:  {{addon_contract}},
    _msg:  {{addon_migrate_msg}},
) ->  {{addon_result}} {
    Ok(Response::default())
}
