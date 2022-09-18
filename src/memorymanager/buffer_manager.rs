use anyhow::{anyhow, Error, Result};

use super::super::filemanager::block_id::BlockId;
use super::super::filemanager::file_mgr::FileMgr;
use super::buffer::Buffer;
use super::log_manager::LogMgr;

pub struct BufferMgr {}

impl BufferMgr {
    pub fn new(fm: FileMgr, lm: LogMgr, numbuffs: u32) -> BufferMgr {
        BufferMgr {}
    }

    pub fn pin(&self, blk: &BlockId) -> Result<Buffer> {
        Ok(Buffer {})
    }

    pub fn unpin(&self, buff: &Buffer) -> Result<()> {
        Ok(())
    }

    pub fn available(&self) -> u32 {
        0
    }

    pub fn flushAll(&self, txnum: u32) -> Result<()> {
        Ok(())
    }
}
