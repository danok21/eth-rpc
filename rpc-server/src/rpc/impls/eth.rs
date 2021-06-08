
use crate::rpc::*;

use jsonrpc_core::{Result, BoxFuture};
use jsonrpc_core::futures::future;

use ethereum_types::{H64, H160, H256, U64, U256};
//extern crate hex_literal;
// use  hex_literal::hex;
use serde::{Serialize, Serializer};
use crate::rpc::types::sync::{SyncStatus, SyncInfo};


pub struct BloomClient {}

impl BloomClient {
    pub fn new() -> Self {
        BloomClient {}
    }
}

impl RpcMethod for BloomClient {
    type Metadata = Metadata;
    fn eth_syncing(&self) -> Result<SyncStatus> {
        let info = SyncInfo {
            /// Current block
            current_block: U256::from(133),
            /// Highest block seen so far
            highest_block: U256::from(134),
            /// Starting block
            starting_block: U256::from(0x432),
        };
        Ok(SyncStatus::Info(info))
    }

    fn is_mining(&self) -> Result<bool> {
        Ok(false)
    }
}