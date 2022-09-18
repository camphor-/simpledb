use anyhow::{anyhow, Error, Result};

use crate::filemanager::block_id::BlockId;

use super::{super::filemanager::file_mgr::FileMgr, log_manager::LogMgr};
use super::super::filemanager::page::{Page, New};

pub struct Buffer{}

impl Buffer {
    pub fn new(fm: &FileMgr, lm: &LogMgr) -> Buffer {
        Buffer {  }
    }

    // TODO: &mut Page を返した方が良さそう
    pub fn contents(&self) -> Page {
        Page::new(3)
    }

    // TODO: &BlockId を返した方が良さそう
    pub fn block(&self) -> BlockId {
        BlockId::new("hoge".to_string(), 0)
    }

    pub fn is_pinned() -> bool {
        true
    }

    pub fn setModified(&self, txnum: u32, lsn: u32) -> Result<()>  {
        Ok(())
    }

    pub fn modifyingTx(&self) -> u32 {
        0
    }
}
