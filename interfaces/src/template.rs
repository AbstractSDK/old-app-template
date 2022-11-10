use std::{cmp::min, env, fs::File};

use cw_asset::AssetInfoUnchecked;

use serde_json::from_reader;

use crate::AbstractAddOn;
use boot_core::{BootError, Contract, Daemon, IndexResponse, TxHandler, TxResponse};
use template_addon::msg::{
    TemplateExecuteMsg, TemplateInstantiateMsg, TemplateMigrateMsg, TemplateQueryMsg,
};

// TODO: rename
pub type TemplateAddOn<Chain> = AbstractAddOn<
    Chain,
    TemplateExecuteMsg,
    TemplateInstantiateMsg,
    TemplateQueryMsg,
    TemplateMigrateMsg,
>;

impl<Chain: TxHandler + Clone> TemplateAddOn<Chain>
where
    TxResponse<Chain>: IndexResponse,
{
    pub fn new(name: &str, chain: &Chain) -> Self {
        Self(
            Contract::new(name, chain).with_wasm_path("template"),
            // .with_mock(Box::new(
            //     ContractWrapper::new_with_empty(
            //         ::contract::execute,
            //         ::contract::instantiate,
            //         ::contract::query,
            //     ),
            // ))
        )
    }
}

impl TemplateAddOn<Daemon> {
    // Add any methods you want to call with your addon here for easier use during deployment
    pub fn helper_example(&self) -> Result<(), BootError> {
        Ok(())
    }
}
