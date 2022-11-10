use cosmwasm_std::{DepsMut, Env, Reply};

use crate::contract::{TemplateAddOn, TemplateResult};

pub fn example_reply_handler(
    _deps: DepsMut,
    _env: Env,
    _add_on: TemplateAddOn,
    _reply: Reply,
) -> TemplateResult {
    // Logic to execute on example reply
    todo!()
}
