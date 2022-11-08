use std::collections::BTreeMap;
use std::ops::Sub;

use abstract_add_on::state::AddOnState;
use abstract_os::objects::deposit_info::DepositInfo;
use abstract_os::objects::fee::Fee;
use abstract_os::objects::{AssetEntry, ChannelEntry, ContractEntry, UncheckedChannelEntry};
use abstract_os::{EXCHANGE, ICS20};
use abstract_sdk::cw20::query_supply;
use abstract_sdk::{Dependency, Exchange};
use cosmwasm_std::{
    from_binary, to_binary, Addr, Coin, CosmosMsg, Decimal, Decimal256, Deps, DepsMut, Env,
    MessageInfo, Order, Response, StdError, StdResult, Uint128, Uint256, WasmMsg,
};
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};
use cw_asset::{Asset, AssetInfo};

use crate::contract::{TemplateAddOn, TemplateResult};
use crate::error::TemplateError;
use crate::package::state::{Config, CONFIG, COUNTS};

/// Update the application configuration.
pub fn update_config(
    deps: DepsMut,
    msg_info: MessageInfo,
    dapp: TemplateAddOn,
    max_count: Option<Uint128>,
) -> TemplateResult {
    dapp.admin.assert_admin(deps.as_ref(), &msg_info.sender)?;

    let mut config = CONFIG.load(deps.storage)?;

    if let Some(new_max_count) = max_count {
        if new_max_count.gt(&config.max_count) {
            return Err(TemplateError::MaxCountError {
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
    _dapp: TemplateAddOn,
) -> TemplateResult {
    let user = msg_info.sender;
    let max_count = CONFIG.load(deps.storage)?.max_count;

    COUNTS
        .update(deps.storage, &user, |old| match old {
            Some(old) => {
                let new_val = old.checked_add(Uint128::one())?;
                if new_val > max_count {
                    return Err(TemplateError::ExceededMaxCount {});
                };
                Ok(new_val)
            }
            None => Ok(Uint128::one()),
        })?;

    Ok(Response::new().add_attribute("action", "increment"))
}
