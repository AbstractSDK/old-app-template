use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};

use crate::contract::{ {{addon_contract}},  {{addon_result}}};
use crate::error:: {{addon_error}};
use crate::msg:: {{addon_execute_msg}};
use crate::state::{CONFIG, COUNTS};

/// Handle the ` {{addon_execute_msg}}`s sent to this add-on.
pub fn execute_handler(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    add_on:  {{addon_contract}},
    msg:  {{addon_execute_msg}},
) ->  {{addon_result}} {
    match msg {
         {{addon_execute_msg}}::UpdateConfig { max_count } => {
            update_config(deps, info, add_on, max_count)
        }
         {{addon_execute_msg}}::Increment {} => increment_sender(deps, info, add_on),
    }
}

/// Update the application configuration.
pub fn update_config(
    deps: DepsMut,
    msg_info: MessageInfo,
    dapp:  {{addon_contract}},
    max_count: Option<Uint128>,
) ->  {{addon_result}} {
    dapp.admin.assert_admin(deps.as_ref(), &msg_info.sender)?;

    let mut config = CONFIG.load(deps.storage)?;

    if let Some(new_max_count) = max_count {
        if new_max_count.gt(&config.max_count) {
            return Err( {{addon_error}}::MaxCountError {
                msg: "Max count must be greater than previous setting".into(),
            });
        }

        config.max_count = new_max_count;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

/// Increments the sending address' count by 1
pub fn increment_sender(
    deps: DepsMut,
    msg_info: MessageInfo,
    _dapp:  {{addon_contract}},
) ->  {{addon_result}} {
    let user = msg_info.sender;
    let max_count = CONFIG.load(deps.storage)?.max_count;

    COUNTS.update(deps.storage, &user, |old| match old {
        Some(old) => {
            let new_val = old.checked_add(Uint128::one())?;
            if new_val > max_count {
                return Err( {{addon_error}}::ExceededMaxCount {});
            };
            Ok(new_val)
        }
        None => Ok(Uint128::one()),
    })?;

    Ok(Response::new().add_attribute("action", "increment"))
}
