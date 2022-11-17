use cosmwasm_std::{DepsMut, Env, Reply};

use crate::contract::{TemplateApp, TemplateResult};

pub fn example_reply_handler(
    _deps: DepsMut,
    _env: Env,
    _app: TemplateApp,
    _reply: Reply,
) -> TemplateResult {
    // Logic to execute on example reply
    todo!()
}
