use std::rc::Rc;

use anyhow::{anyhow, Error, Result};

use crate::filemanager::block_id::BlockId;

use super::super::filemanager::page::{New, Page};
use super::{super::filemanager::file_mgr::FileMgr, log_manager::LogMgr};

pub struct Buffer {
    is_pinned: bool,
    page: Page,
    txnum: i32,
    lsn: i32,
    block_id: Option<BlockId>,
    fm: Rc<FileMgr>,
    lm: Rc<LogMgr>,
}

impl Buffer {
    pub fn new(fm: Rc<FileMgr>, lm: Rc<LogMgr>) -> Buffer {
        Buffer {
            is_pinned: false,
            page: Page::new(fm.block_size()),
            txnum: -1,
            lsn: -1,
            block_id: None,
            fm: fm,
            lm: lm,
        }
    }

    // TODO: &mut Page を返した方が良さそう
    pub fn contents(&self) -> Page {
        self.page
    }

    // TODO: &BlockId を返した方が良さそう
    pub fn block(&self) -> Option<BlockId> {
        self.block_id
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }

    pub fn set_modified(&self, txnum: u32, lsn: i32) {
        self.txnum = txnum as i32;
        if lsn > 0 {
            self.lsn = lsn;
        }
    }

    pub fn modifying_tx(&self) -> i32 {
        self.txnum
    }

    pub fn assign_to_block(&self, b: BlockId) {
        self.block_id = Some(b);
    }
    pub fn flush(&self) {
        if let Some(v) = self.block_id {
            if self.txnum > 0 {
                self.lm.flush(self.lsn);
                self.fm.write(&v, &mut self.page);
            }
        }
    }

    pub fn pin(&self) {
        self.is_pinned = true;
    }

    pub fn unpin(&self) {
        self.is_pinned = false;
    }
}
