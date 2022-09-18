use super::super::filemanager::block_id::BlockId;
use super::super::filemanager::file_mgr::FileMgr;
use super::super::filemanager::page::{New, Page};
use anyhow::{anyhow, Error, Result};

use std::cell::RefCell;
use std::rc::Rc;

pub struct LogIterator {
    fm: RefCell<FileMgr>,
    blk: RefCell<BlockId>,
    p: Page,
    currentpos: usize,
    boundary: usize,
}

impl LogIterator {
    pub fn new(fm: RefCell<FileMgr>, blk: RefCell<BlockId>) -> Result<Self> {
        let p = Page::new(fm.borrow().block_size());
        let mut logIter = LogIterator {
            fm: fm,
            blk: blk,
            p: p,
            currentpos: 0,
            boundary: 0,
        };
        logIter.moveToBlock();
        Ok(logIter)
    }

    pub fn hasNext(&self) -> bool {
        self.currentpos < self.fm.borrow_mut().block_size() || self.blk.borrow().number() > 0
    }

    fn moveToBlock(&mut self) -> Result<()> {
        self.fm.borrow_mut().read(&self.blk.borrow(), &mut self.p)?;
        self.boundary = self.p.get_i32(0)? as usize;
        self.currentpos = self.boundary;
        Ok(())
    }
}
