//! # Add-On Template
//!
//! `your_namespace::template` is an add-on which allows users to ...
//!
//! ## Creation
//! The contract can be added on an OS by calling [`ExecuteMsg::CreateModule`](crate::manager::ExecuteMsg::CreateModule) on the manager of the os.
//! ```ignore
//! let template_init_msg = InstantiateMsg:: {{addon_instantiate_msg}}{
//!               /// The initial value for max_count
//!               pub max_count: Uint128,
//!               /// Initial user counts
//!               pub initial_counts: Option<Vec<(String, Uint128)>>,
//!           };
//!
//! let create_module_msg = ExecuteMsg::CreateModule {
//!                 module: Module {
//!                     info: ModuleInfo {
//!                         name: TEMPLATE.into(),
//!                         version: None,
//!                     },
//!                     kind: crate::core::modules::ModuleKind::External,
//!                 },
//!                 init_msg: Some(to_binary(&template_init_msg).unwrap()),
//!        };
//! // Call create_module_msg on manager
//! ```
//!
//! ## Migration
//! Migrating this contract is done by calling `ExecuteMsg::Upgrade` on [`crate::manager`] with `crate::TEMPLATE` as module.

use cosmwasm_std::{Addr, Uint128};

/// Migrate msg
#[cosmwasm_schema::cw_serde]
pub struct  {{addon_migrate_msg}} {}

/// Init msg
#[cosmwasm_schema::cw_serde]
pub struct  {{addon_instantiate_msg}} {
    /// The initial value for max_count
    pub max_count: Uint128,
    /// Initial user counts
    pub initial_counts: Option<Vec<(String, Uint128)>>,
}

#[cosmwasm_schema::cw_serde]
pub enum  {{addon_execute_msg}} {
    /// Update the configuration for this contract
    UpdateConfig { max_count: Option<Uint128> },
    /// Add a count of 1 to the calling user
    Increment {},
}

#[cosmwasm_schema::cw_serde]
pub enum  {{addon_query_msg}} {
    /// Returns [`ConfigResponse`]
    Config {},
    /// Returns the counts of the users
    /// Returns [`UserCountsResponse`]
    UserCountList {
        page_token: Option<String>,
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
