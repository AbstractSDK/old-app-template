use cosmwasm_std::{DepsMut, Env, Reply};

use crate::contract::{TemplateResult, TemplateAddOn};

pub fn example_reply_handler(deps: DepsMut, env: Env, add_on: TemplateAddOn,reply: Reply) -> TemplateResult {
    // Logic to execute on example reply
    todo!()
}