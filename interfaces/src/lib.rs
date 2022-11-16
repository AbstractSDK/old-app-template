use boot_core::{Contract, IndexResponse, TxHandler};

use serde::Serialize;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

/// Allow for access to the functions on the underlying Contract instance
pub struct AbstractAddOn<
    Chain: TxHandler,
    ExecuteMsg: Serialize + Debug,
    InitMsg: Serialize + Debug,
    QueryMsg: Serialize + Debug,
    M: Serialize + Debug,
>(Contract<Chain, ExecuteMsg, InitMsg, QueryMsg, M>)
where
    <Chain as TxHandler>::Response: IndexResponse;
impl<
        Chain: TxHandler,
        E: Serialize + Debug,
        I: Serialize + Debug,
        Q: Serialize + Debug,
        M: Serialize + Debug,
    > Deref for AbstractAddOn<Chain, E, I, Q, M>
where
    <Chain as TxHandler>::Response: IndexResponse,
{
    type Target = Contract<Chain, E, I, Q, M>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<
        Chain: TxHandler,
        E: Serialize + Debug,
        I: Serialize + Debug,
        Q: Serialize + Debug,
        M: Serialize + Debug,
    > DerefMut for AbstractAddOn<Chain, E, I, Q, M>
where
    <Chain as TxHandler>::Response: IndexResponse,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<
        Chain: TxHandler,
        E: Serialize + Debug,
        I: Serialize + Debug,
        Q: Serialize + Debug,
        M: Serialize + Debug,
    > AbstractAddOn<Chain, E, I, Q, M>
where
    <Chain as TxHandler>::Response: IndexResponse,
{
}

pub mod template;
