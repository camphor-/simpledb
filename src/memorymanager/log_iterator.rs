use super::super::filemanager::block_id::BlockId;
use super::super::filemanager::file_mgr::FileMgr;
use super::super::filemanager::page::{New, Page};
use anyhow::Result;

use std::cell::RefCell;
use std::rc::Rc;

pub struct LogIterator {
    fm: Rc<RefCell<FileMgr>>,
    blk: Rc<RefCell<BlockId>>,
    p: Page,
    currentpos: usize,
    boundary: usize,
}

impl LogIterator {
    pub fn new(fm: Rc<RefCell<FileMgr>>, blk: Rc<RefCell<BlockId>>) -> Result<Self> {
        let p = Page::new(fm.borrow().block_size());
        let mut log_iter = LogIterator {
            fm: fm,
            blk: blk,
            p,
            currentpos: 0,
            boundary: 0,
        };
        log_iter.move_to_block()?;
        Ok(log_iter)
    }

    pub fn has_next(&self) -> bool {
        self.currentpos < self.fm.borrow_mut().block_size() || self.blk.borrow().number() > 0
    }

    fn move_to_block(&mut self) -> Result<()> {
        self.fm.borrow_mut().read(&self.blk.borrow(), &mut self.p)?;
        self.boundary = self.p.get_i32(0)? as usize;
        self.currentpos = self.boundary;
        Ok(())
    }
}
