/// This file is a modified copy of the file from https://github.com/tonlabs/ton-labs-node
///
/// Changes:
/// - replaced old `failure` crate with `anyhow`
/// - moved all flags to meta
///
use std::sync::{Arc, Weak};

use anyhow::Result;
use tiny_adnl::utils::*;
use tokio::sync::RwLock;

use super::block_meta::BlockMeta;

pub struct BlockHandle {
    id: ton_block::BlockIdExt,
    meta: BlockMeta,
    block_data_lock: RwLock<()>,
    proof_data_block: RwLock<()>,
    cache: Arc<FxDashMap<ton_block::BlockIdExt, Weak<BlockHandle>>>,
}

impl BlockHandle {
    pub fn with_values(
        id: ton_block::BlockIdExt,
        meta: BlockMeta,
        cache: Arc<FxDashMap<ton_block::BlockIdExt, Weak<BlockHandle>>>,
    ) -> Self {
        Self {
            id,
            meta,
            block_data_lock: Default::default(),
            proof_data_block: Default::default(),
            cache,
        }
    }

    #[inline]
    pub fn id(&self) -> &ton_block::BlockIdExt {
        &self.id
    }

    #[inline]
    pub fn meta(&self) -> &BlockMeta {
        &self.meta
    }

    #[inline]
    pub fn block_data_lock(&self) -> &RwLock<()> {
        &self.block_data_lock
    }

    #[inline]
    pub fn proof_data_lock(&self) -> &RwLock<()> {
        &self.proof_data_block
    }

    pub fn has_proof_or_link(&self, is_link: &mut bool) -> bool {
        *is_link = !self.id.shard().is_masterchain();
        if *is_link {
            self.meta.has_proof_link()
        } else {
            self.meta.has_proof()
        }
    }

    pub fn masterchain_ref_seqno(&self) -> u32 {
        if self.id.shard().is_masterchain() {
            self.id.seq_no()
        } else {
            self.meta.masterchain_ref_seqno()
        }
    }

    pub fn set_masterchain_ref_seqno(&self, masterchain_ref_seqno: u32) -> Result<bool> {
        match self.meta.set_masterchain_ref_seqno(masterchain_ref_seqno) {
            0 => Ok(true),
            prev_seqno if prev_seqno == masterchain_ref_seqno => Ok(false),
            _ => Err(BlockHandleError::RefSeqnoAlreadySet.into()),
        }
    }
}

impl Drop for BlockHandle {
    fn drop(&mut self) {
        self.cache
            .remove_if(&self.id, |_, weak| weak.strong_count() == 0);
    }
}

#[derive(thiserror::Error, Debug)]
enum BlockHandleError {
    #[error("Different masterchain ref seqno has already been set")]
    RefSeqnoAlreadySet,
}
