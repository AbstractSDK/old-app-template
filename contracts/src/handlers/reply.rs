use cosmwasm_std::{DepsMut, Env, Reply};

use crate::contract::{ {{addon_contract}},  {{addon_result}}};

pub fn example_reply_handler(
    _deps: DepsMut,
    _env: Env,
    _add_on:  {{addon_contract}},
    _reply: Reply,
) ->  {{addon_result}} {
    // Logic to execute on example reply
    todo!()
}
