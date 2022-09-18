use anyhow::Result;

use super::super::filemanager::page::{New, Page};
use super::log_iterator::LogIterator;
use crate::filemanager::block_id::BlockId;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

use super::super::filemanager::file_mgr::FileMgr;

pub struct LogMgr {
    fm: RefCell<FileMgr>,
    logfile: String,
    logpage: Page,
    currentblk: BlockId,
    latestLSN: i32,
    lastSavedLSN: i32,
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
            lastSavedLSN: 0,
        })
    }

    pub fn iterator(&self) -> Result<LogIterator> {
        self.private_flush();
        LogIterator::new(self.fm, RefCell::from(self.currentblk))
    }

    pub fn append(&self, logrec: Vec<u8>) -> Result<i32> {
        let mut boundary = self.logpage.get_i32(0)?;
        let recsize = logrec.len();
        let bytesneeded = recsize + 4;
        if boundary - (bytesneeded as i32) < 4 {
            self.private_flush();
            self.currentblk = self.appendNewBlock()?;
            boundary = self.logpage.get_i32(0)?;
        }
        let recpos = (boundary as usize) - bytesneeded;
        self.logpage.set_bytes(recpos, logrec);
        self.logpage.set_i32(0, recpos as i32);
        self.latestLSN += 1;
        Ok(self.latestLSN)
    }

    fn appendNewBlock(&self) -> Result<BlockId> {
        let blk = self.fm.borrow().append(&self.logfile)?;
        self.logpage
            .set_i32(0, self.fm.borrow().block_size() as i32);
        self.fm.borrow().write(&self.currentblk, &mut &self.logpage);
        Ok(blk)
    }

    pub fn flush(&self, lsn: i32) {
        if lsn >= self.lastSavedLSN {
            self.private_flush();
        }
    }

    fn private_flush(&self) {
        self.fm
            .borrow_mut()
            .write(&self.currentblk, &mut self.logpage);
        self.lastSavedLSN = self.latestLSN;
    }
}
