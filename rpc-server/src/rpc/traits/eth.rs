use crate::rpc::*;
use jsonrpc_core::{Result, BoxFuture};
use jsonrpc_derive::rpc;
use ethereum_types::{H64, H160, H256, U64, U256};


/// eth rpc interface.
#[rpc(server)]
pub trait RpcMethod {
    /// RPC Metadata
    type Metadata;

    /// Returns an object with data about the sync status or false. (wtf?)
    #[rpc(name = "syncing")]
    fn eth_syncing(&self) -> Result<SyncStatus>;


    /// Returns true if client is actively mining new blocks.
    #[rpc(name = "mining")]
    fn is_mining(&self) -> Result<bool>;
}
