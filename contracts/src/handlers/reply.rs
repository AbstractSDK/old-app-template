use cosmwasm_std::{DepsMut, Env, Reply};

use crate::contract::{ {{app_contract}},  {{app_result}}};

pub fn example_reply_handler(
    _deps: DepsMut,
    _env: Env,
    _app:  {{app_contract}},
    _reply: Reply,
) ->  {{app_result}} {
    // Logic to execute on example reply
    todo!()
}
