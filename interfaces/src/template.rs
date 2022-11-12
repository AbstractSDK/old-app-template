use abstract_os::add_on::{BaseExecuteMsg, BaseQueryMsg};
use abstract_os::middleware;
use boot_abstract::AbstractOS;
use std::{cmp::min, env, fs::File};

use cw_asset::AssetInfoUnchecked;

use serde_json::from_reader;

use crate::AbstractAddOn;
use boot_core::{BootError, Contract, Daemon, IndexResponse, TxHandler, TxResponse};
use serde::de::DeserializeOwned;
use serde::Serialize;
use template_addon::msg::{
    {{addon_execute_msg}}, {{addon_instantiate_msg}}, {{addon_migrate_msg}}, {{addon_query_msg}},
};
use cosmwasm_std::Coin;
use template_addon::contract::ADDON_NAME;

/// Contract wrapper for deploying with BOOT
/// @TODO don't wrap using middleware here, but in the boot-abstract layer
pub type {{addon_contract}}<Chain> = AbstractAddOn<
    Chain,
    middleware::ExecuteMsg<BaseExecuteMsg, {{addon_execute_msg}}>,
    middleware::InstantiateMsg<{{addon_instantiate_msg}}>,
    middleware::QueryMsg<BaseQueryMsg, {{addon_query_msg}}>,
    middleware::MigrateMsg<{{addon_migrate_msg}}>,
>;

impl<Chain: TxHandler + Clone> {{addon_contract}}<Chain>
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
    pub fn query_addon<T: Serialize + DeserializeOwned>(&self, query_msg: {{addon_query_msg}}) -> Result<T, BootError> {
        self.query(&middleware::QueryMsg::App(query_msg))
    }

    /// Temporary helper to query the addon base explicitly
    pub fn query_base<T: Serialize + DeserializeOwned>(&self, query_msg: BaseQueryMsg) -> Result<T, BootError> {
        self.query(&middleware::QueryMsg::Base(query_msg))
    }

    /// Temporary helper to execute the addon explicitly
    pub fn execute_addon(&self, execute_msg: {{addon_execute_msg}}, coins: Option<&[Coin]>) -> Result<TxResponse<Chain>, BootError> {
        self.execute(&middleware::ExecuteMsg::App(execute_msg), coins)
    }

    /// Temporary helper to execute the addon base explicitly
    pub fn execute_base(&self, execute_msg: BaseExecuteMsg, coins: Option<&[Coin]>) -> Result<TxResponse<Chain>, BootError> {
        self.execute(&middleware::ExecuteMsg::Base(execute_msg), coins)
    }
}

