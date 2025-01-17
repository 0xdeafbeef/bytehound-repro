/// This file is a modified copy of the file from https://github.com/tonlabs/ton-labs-node
///
/// Changes:
/// - replaced old `failure` crate with `anyhow`
/// - moved all flags here from block handle
/// - removed temporary unused flags
///
use std::io::{Read, Write};
use std::sync::atomic::{AtomicU64, Ordering};

use anyhow::Result;
use ton_types::ByteOrderRead;

use super::StoredValue;

#[derive(Debug, Default)]
pub struct BlockMeta {
    flags: AtomicU64,
    gen_utime: u32,
}

impl BlockMeta {
    pub fn from_block(block: &ton_block::Block) -> Result<Self> {
        let info = block.read_info()?;
        let flags = if info.key_block() {
            BLOCK_META_FLAG_IS_KEY_BLOCK
        } else {
            0
        };
        Ok(Self::with_data(flags, info.gen_utime().0, 0))
    }

    pub fn with_data(flags: u64, gen_utime: u32, masterchain_ref_seqno: u32) -> Self {
        Self {
            flags: AtomicU64::new(flags | masterchain_ref_seqno as u64),
            gen_utime,
        }
    }

    pub fn brief(&self) -> BriefBlockMeta {
        BriefBlockMeta {
            flags: self.flags.load(Ordering::Acquire),
            gen_utime: self.gen_utime,
        }
    }

    pub fn masterchain_ref_seqno(&self) -> u32 {
        self.flags.load(Ordering::Acquire) as u32
    }

    pub fn set_masterchain_ref_seqno(&self, seqno: u32) -> u32 {
        self.flags.fetch_or(seqno as u64, Ordering::Release) as u32
    }

    #[inline]
    pub fn gen_utime(&self) -> u32 {
        self.gen_utime
    }

    pub fn clear_data_and_proof(&self) {
        self.flags.fetch_and(CLEAR_DATA_MASK, Ordering::Release);
    }

    pub fn set_has_data(&self) -> bool {
        self.set_flag(BLOCK_META_FLAG_HAS_DATA)
    }

    pub fn has_data(&self) -> bool {
        self.test_flag(BLOCK_META_FLAG_HAS_DATA)
    }

    pub fn set_has_proof(&self) -> bool {
        self.set_flag(BLOCK_META_FLAG_HAS_PROOF)
    }

    pub fn has_proof(&self) -> bool {
        self.test_flag(BLOCK_META_FLAG_HAS_PROOF)
    }

    pub fn set_has_proof_link(&self) -> bool {
        self.set_flag(BLOCK_META_FLAG_HAS_PROOF_LINK)
    }

    pub fn has_proof_link(&self) -> bool {
        self.test_flag(BLOCK_META_FLAG_HAS_PROOF_LINK)
    }

    pub fn set_has_state(&self) -> bool {
        self.set_flag(BLOCK_META_FLAG_HAS_STATE)
    }

    pub fn has_state(&self) -> bool {
        self.test_flag(BLOCK_META_FLAG_HAS_STATE)
    }

    #[allow(unused)]
    pub fn set_has_persistent_state(&self) -> bool {
        self.set_flag(BLOCK_META_FLAG_HAS_PERSISTENT_STATE)
    }

    #[allow(unused)]
    pub fn has_persistent_state(&self) -> bool {
        self.test_flag(BLOCK_META_FLAG_HAS_PERSISTENT_STATE)
    }

    pub fn set_has_next1(&self) -> bool {
        self.set_flag(BLOCK_META_FLAG_HAS_NEXT_1)
    }

    pub fn has_next1(&self) -> bool {
        self.test_flag(BLOCK_META_FLAG_HAS_NEXT_1)
    }

    pub fn set_has_next2(&self) -> bool {
        self.set_flag(BLOCK_META_FLAG_HAS_NEXT_2)
    }

    pub fn has_next2(&self) -> bool {
        self.test_flag(BLOCK_META_FLAG_HAS_NEXT_2)
    }

    pub fn set_has_prev1(&self) -> bool {
        self.set_flag(BLOCK_META_FLAG_HAS_PREV_1)
    }

    pub fn has_prev1(&self) -> bool {
        self.test_flag(BLOCK_META_FLAG_HAS_PREV_1)
    }

    pub fn set_has_prev2(&self) -> bool {
        self.set_flag(BLOCK_META_FLAG_HAS_PREV_2)
    }

    pub fn has_prev2(&self) -> bool {
        self.test_flag(BLOCK_META_FLAG_HAS_PREV_2)
    }

    pub fn set_is_applied(&self) -> bool {
        self.set_flag(BLOCK_META_FLAG_IS_APPLIED)
    }

    pub fn is_applied(&self) -> bool {
        self.test_flag(BLOCK_META_FLAG_IS_APPLIED)
    }

    pub fn is_key_block(&self) -> bool {
        self.test_flag(BLOCK_META_FLAG_IS_KEY_BLOCK)
    }

    pub fn set_is_moving_to_archive(&self) -> bool {
        self.set_flag(BLOCK_META_FLAG_MOVING_TO_ARCHIVE)
    }

    pub fn set_is_archived(&self) -> bool {
        self.set_flag(BLOCK_META_FLAG_MOVED_TO_ARCHIVE)
    }

    pub fn is_archived(&self) -> bool {
        self.test_flag(BLOCK_META_FLAG_MOVED_TO_ARCHIVE)
    }

    fn test_flag(&self, flag: u64) -> bool {
        self.flags.load(Ordering::Acquire) & flag == flag
    }

    fn set_flag(&self, flag: u64) -> bool {
        self.flags.fetch_or(flag, Ordering::Release) & flag != flag
    }
}

impl StoredValue for BlockMeta {
    /// 8 bytes flags
    /// 4 bytes gen_utime
    const SIZE_HINT: usize = 8 + 4;

    type OnStackSlice = [u8; Self::SIZE_HINT];

    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        const FLAGS_MASK: u64 = 0x0000_ffff_ffff_ffff;
        let flags = self.flags.load(Ordering::Acquire) & FLAGS_MASK;

        writer.write_all(&flags.to_le_bytes())?;
        writer.write_all(&self.gen_utime.to_le_bytes())?;

        Ok(())
    }

    fn deserialize<R: Read>(reader: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        let flags = reader.read_le_u64()?;
        let gen_utime = reader.read_le_u32()?;

        Ok(Self {
            flags: AtomicU64::new(flags),
            gen_utime,
        })
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct BriefBlockMeta {
    flags: u64,
    gen_utime: u32,
}

impl BriefBlockMeta {
    #[inline]
    pub fn gen_utime(&self) -> u32 {
        self.gen_utime
    }

    #[inline]
    pub fn masterchain_ref_seqno(&self) -> u32 {
        self.flags as u32
    }

    #[inline]
    pub fn is_key_block(&self) -> bool {
        self.test_flag(BLOCK_META_FLAG_IS_KEY_BLOCK)
    }

    #[inline]
    fn test_flag(&self, flag: u64) -> bool {
        self.flags & flag == flag
    }
}

const BLOCK_META_FLAG_HAS_DATA: u64 = 1 << 32;
const BLOCK_META_FLAG_HAS_PROOF: u64 = 1 << (32 + 1);
const BLOCK_META_FLAG_HAS_PROOF_LINK: u64 = 1 << (32 + 2);
// skip flag 3 (processed by external listener)
const BLOCK_META_FLAG_HAS_STATE: u64 = 1 << (32 + 4);
const BLOCK_META_FLAG_HAS_PERSISTENT_STATE: u64 = 1 << (32 + 5);
const BLOCK_META_FLAG_HAS_NEXT_1: u64 = 1 << (32 + 6);
const BLOCK_META_FLAG_HAS_NEXT_2: u64 = 1 << (32 + 7);
const BLOCK_META_FLAG_HAS_PREV_1: u64 = 1 << (32 + 8);
const BLOCK_META_FLAG_HAS_PREV_2: u64 = 1 << (32 + 9);
const BLOCK_META_FLAG_IS_APPLIED: u64 = 1 << (32 + 10);
const BLOCK_META_FLAG_IS_KEY_BLOCK: u64 = 1 << (32 + 11);

const BLOCK_META_FLAG_MOVING_TO_ARCHIVE: u64 = 1 << (32 + 12);
const BLOCK_META_FLAG_MOVED_TO_ARCHIVE: u64 = 1 << (32 + 13);

const CLEAR_DATA_MASK: u64 =
    !(BLOCK_META_FLAG_HAS_DATA | BLOCK_META_FLAG_HAS_PROOF | BLOCK_META_FLAG_HAS_PROOF_LINK);
