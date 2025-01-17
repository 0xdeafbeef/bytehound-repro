/// This file is a modified copy of the file from https://github.com/tonlabs/ton-labs-node
///
/// Changes:
/// - replaced old `failure` crate with `anyhow`
///
use anyhow::Result;
use parking_lot::Mutex;
use std::sync::Arc;
use ton_api::ton;

use crate::storage::NodeStateStorage;

macro_rules! define_node_state {
    ($ident:ident) => {
        pub struct $ident {
            cache: Mutex<Option<ton::ton_node::blockidext::BlockIdExt>>,
            storage: NodeStateStorage,
        }

        impl $ident {
            pub fn new(db: &Arc<rocksdb::DB>) -> Result<Self> {
                Ok(Self {
                    cache: Mutex::new(None),
                    storage: NodeStateStorage::with_db(db)?,
                })
            }

            const fn get_key() -> &'static str {
                stringify!($ident)
            }

            pub fn load_from_db(&self) -> Result<ton::ton_node::blockidext::BlockIdExt> {
                {
                    let lock = self.cache.lock();
                    if let Some(a) = &*lock {
                        return Ok(a.clone());
                    }
                }
                let value = self.storage.load(Self::get_key())?;
                let value = bincode::deserialize::<ton::ton_node::blockidext::BlockIdExt>(&value)?;
                *self.cache.lock() = Some(value.clone());
                Ok(value)
            }

            pub fn store_into_db(
                &self,
                value: ton::ton_node::blockidext::BlockIdExt,
            ) -> Result<()> {
                let bytes = bincode::serialize(&value)?;
                self.storage.store(Self::get_key(), bytes)?;
                *self.cache.lock() = Some(value);
                Ok(())
            }
        }
    };
}

define_node_state!(LastMcBlockId);
define_node_state!(InitMcBlockId);
define_node_state!(ShardsClientMcBlockId);
