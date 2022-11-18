use abstract_os::app::{BaseExecuteMsg, BaseQueryMsg};
use abstract_os::base;

use crate::AbstractApp;
use boot_core::{BootError, Contract, IndexResponse, TxHandler, TxResponse};
use cosmwasm_std::Coin;
use serde::de::DeserializeOwned;
use serde::Serialize;
use template_app::contract::MODULE_NAME;
use template_app::msg::{
     {{app_execute_msg}},  {{app_instantiate_msg}},  {{app_migrate_msg}},  {{app_query_msg}},
};

/// Contract wrapper for deploying with BOOT
/// @TODO don't wrap using base here, but in the abstract-boot layer
pub type  {{app_contract}}<Chain> = AbstractApp<
    Chain,
    base::ExecuteMsg<BaseExecuteMsg,  {{app_execute_msg}}>,
    base::InstantiateMsg< {{app_instantiate_msg}}>,
    base::QueryMsg<BaseQueryMsg,  {{app_query_msg}}>,
    base::MigrateMsg< {{app_migrate_msg}}>,
>;

impl<Chain: TxHandler + Clone>  {{app_contract}}<Chain>
where
    TxResponse<Chain>: IndexResponse,
{
    pub fn new(name: &str, chain: &Chain) -> Self {
        Self(
            Contract::new(name, chain).with_wasm_path(MODULE_NAME),
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

    /// Temporary helper to query the app explicitly
    pub fn query_app<T: Serialize + DeserializeOwned>(
        &self,
        query_msg:  {{app_query_msg}},
    ) -> Result<T, BootError> {
        self.query(&base::QueryMsg::App(query_msg))
    }

    /// Temporary helper to query the app base explicitly
    pub fn query_base<T: Serialize + DeserializeOwned>(
        &self,
        query_msg: BaseQueryMsg,
    ) -> Result<T, BootError> {
        self.query(&base::QueryMsg::Base(query_msg))
    }

    /// Temporary helper to execute the app explicitly
    pub fn execute_app(
        &self,
        execute_msg:  {{app_execute_msg}},
        coins: Option<&[Coin]>,
    ) -> Result<TxResponse<Chain>, BootError> {
        self.execute(&base::ExecuteMsg::App(execute_msg), coins)
    }

    /// Temporary helper to execute the app base explicitly
    pub fn execute_base(
        &self,
        execute_msg: BaseExecuteMsg,
        coins: Option<&[Coin]>,
    ) -> Result<TxResponse<Chain>, BootError> {
        self.execute(&base::ExecuteMsg::Base(execute_msg), coins)
    }
}
