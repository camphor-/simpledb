use anyhow::{Result};

use std::borrow::Borrow;
use std::rc::Rc;
use std::cell::RefCell;
use crate::filemanager::block_id::BlockId;
use super::super::filemanager::page::{New, Page};

use super::super::filemanager::file_mgr::FileMgr;

pub struct LogMgr{
    fm: RefCell<FileMgr>,
    logfile: String,
    logpage: Page,
    currentblk: BlockId,
    latestLSN: i32,
    lastSaveLSN: i32,
}

impl LogMgr {
    pub fn new(fm: RefCell<FileMgr>, logfile: String) -> Result<Self> {
        let mut currentblk;
        let logsize = fm.borrow().length(&logfile)?;
        let logpage = Page::new(fm.borrow().block_size());
        if logsize == 0 {
            currentblk = fm.borrow_mut().append(&logfile)?;
            logpage.set_i32(0, fm.borrow().block_size() as i32);
            fm.borrow().write(&currentblk, &mut logpage);
        } else {
            currentblk = BlockId::new(logfile, logsize - 1);
            fm.borrow().read(&currentblk, &mut logpage);
        }
        Ok(LogMgr {
            fm,
            logfile,
            logpage,
            currentblk,
            latestLSN: 0,
            lastSaveLSN: 0,
        })
    }

    pub fn append(&self, rec: Vec<u8>) -> Result<u32> {
        Ok(0)
    }

    pub fn flush(&self, lsn: u32) -> Result<()> {
        Ok(())
    }

    // pub fn iterator
}
