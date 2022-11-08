//! # Add-On Template
//!
//! `your_namespace::template` is an add-on which allows users to ...
//!
//! ## Description
//! This contract uses the proxy's value calculation configuration to get the value of the assets held in the proxy and the relative value of the deposit asset.
//! It then mints LP tokens that are claimable for an equal portion of the proxy assets at a later date.
//!
//! ---
//! **WARNING:** This mint/burn mechanism can be mis-used by flash-loan attacks if the assets contained are of low-liquidity compared to the etf's size.
//!
//! ## Creation
//! The etf contract can be added on an OS by calling [`ExecuteMsg::CreateModule`](crate::manager::ExecuteMsg::CreateModule) on the manager of the os.
//! ```ignore
//! let etf_init_msg = InstantiateMsg{
//!                deposit_asset: "juno".to_string(),
//!                base: BaseInstantiateMsg{memory_address: "juno1...".to_string()},
//!                fee: Decimal::percent(10),
//!                provider_addr: "juno1...".to_string(),
//!                token_code_id: 3,
//!                etf_lp_token_name: Some("demo_etf".to_string()),
//!                etf_lp_token_symbol: Some("DEMO".to_string()),
//!        };
//! let create_module_msg = ExecuteMsg::CreateModule {
//!                 module: Module {
//!                     info: ModuleInfo {
//!                         name: ETF.into(),
//!                         version: None,
//!                     },
//!                     kind: crate::core::modules::ModuleKind::External,
//!                 },
//!                 init_msg: Some(to_binary(&etf_init_msg).unwrap()),
//!        };
//! // Call create_module_msg on manager
//! ```
//!
//! ## Migration
//! Migrating this contract is done by calling `ExecuteMsg::Upgrade` on [`crate::manager`] with `crate::TEMPLATE` as module.

use cosmwasm_std::{Addr, Uint128};

/// Migrate msg
#[cosmwasm_schema::cw_serde]
pub struct TemplateMigrateMsg {}

/// Init msg
#[cosmwasm_schema::cw_serde]
pub struct TemplateInstantiateMsg {
    /// The initial value for max_count
    pub max_count: Uint128,
    /// Initial user counts
    pub initial_counts: Option<Vec<(String, Uint128)>>,
}

#[cosmwasm_schema::cw_serde]
pub enum TemplateExecuteMsg {
    /// Update the configuration for this contract
    UpdateConfig { max_count: Option<Uint128> },
    /// Add a count of 1 to the calling user
    Increment {},
}

#[cosmwasm_schema::cw_serde]
pub enum TemplateQueryMsg {
    /// Returns [`ConfigResponse`]
    Config {},
    /// Returns the counts of the users
    /// Returns [`UserCountsResponse`]
    UserCountList {
        page_token: Option<Addr>,
        page_size: Option<u8>,
    },
    UserCounts {
        users: Vec<Addr>,
    },
    /// Return the calling user's count if any
    UserCount {
        user: Addr,
    },
}

// #### RESPONSES ####

#[cosmwasm_schema::cw_serde]
pub struct UserCountResponse {
    pub user: Addr,
    pub count: Uint128,
}

#[cosmwasm_schema::cw_serde]
pub struct UserCountsResponse {
    pub counts: Vec<(Addr, Uint128)>,
}

#[cosmwasm_schema::cw_serde]
pub struct ConfigResponse {
    pub max_count: Uint128,
}
