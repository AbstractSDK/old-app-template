use abstract_os::add_on::{BaseExecuteMsg, BaseQueryMsg};
use abstract_os::middleware;

use crate::AbstractAddOn;
use boot_core::{BootError, Contract, IndexResponse, TxHandler, TxResponse};
use cosmwasm_std::Coin;
use serde::de::DeserializeOwned;
use serde::Serialize;
use template_addon::contract::ADDON_NAME;
use template_addon::msg::{
    TemplateExecuteMsg, TemplateInstantiateMsg, TemplateMigrateMsg, TemplateQueryMsg,
};

/// Contract wrapper for deploying with BOOT
/// @TODO don't wrap using middleware here, but in the boot-abstract layer
pub type TemplateAddOn<Chain> = AbstractAddOn<
    Chain,
    middleware::ExecuteMsg<BaseExecuteMsg, TemplateExecuteMsg>,
    middleware::InstantiateMsg<TemplateInstantiateMsg>,
    middleware::QueryMsg<BaseQueryMsg, TemplateQueryMsg>,
    middleware::MigrateMsg<TemplateMigrateMsg>,
>;

impl<Chain: TxHandler + Clone> TemplateAddOn<Chain>
where
    TxResponse<Chain>: IndexResponse,
{
    pub fn new(name: &str, chain: &Chain) -> Self {
        Self(
            Contract::new(name, chain).with_wasm_path(ADDON_NAME),
            // Uncomment to deploy and use contracts with mock implementations
            // .with_mock(Box::new(
            //     ContractWrapper::new_with_empty(
            //         ::contract::execute,
            //         ::contract::instantiate,
            //         ::contract::query,
            //     ),
            // ))
        )
    }

    /// Temporary helper to query the addon explicitly
    pub fn query_addon<T: Serialize + DeserializeOwned>(
        &self,
        query_msg: TemplateQueryMsg,
    ) -> Result<T, BootError> {
        self.query(&middleware::QueryMsg::App(query_msg))
    }

    /// Temporary helper to query the addon base explicitly
    pub fn query_base<T: Serialize + DeserializeOwned>(
        &self,
        query_msg: BaseQueryMsg,
    ) -> Result<T, BootError> {
        self.query(&middleware::QueryMsg::Base(query_msg))
    }

    /// Temporary helper to execute the addon explicitly
    pub fn execute_addon(
        &self,
        execute_msg: TemplateExecuteMsg,
        coins: Option<&[Coin]>,
    ) -> Result<TxResponse<Chain>, BootError> {
        self.execute(&middleware::ExecuteMsg::App(execute_msg), coins)
    }

    /// Temporary helper to execute the addon base explicitly
    pub fn execute_base(
        &self,
        execute_msg: BaseExecuteMsg,
        coins: Option<&[Coin]>,
    ) -> Result<TxResponse<Chain>, BootError> {
        self.execute(&middleware::ExecuteMsg::Base(execute_msg), coins)
    }
}
