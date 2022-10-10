use anyhow::Result;

use super::super::filemanager::page::{New, Page};
use super::log_iterator::LogIterator;
use crate::filemanager::block_id::BlockId;
use std::cell::RefCell;
use std::rc::Rc;

use super::super::filemanager::file_mgr::FileMgr;

pub struct LogMgr {
    fm: Rc<RefCell<FileMgr>>,
    logfile: String,
    logpage: Page,
    currentblk: Rc<RefCell<BlockId>>,
    latest_lsn: i32,
    last_saved_lsn: i32,
}

impl LogMgr {
    pub fn new(fm: Rc<RefCell<FileMgr>>, logfile: &str) -> Result<Self> {
        let currentblk;
        let logsize = fm.borrow().length(&logfile.to_string())?;
        let mut logpage = Page::new(fm.borrow().block_size());
        if logsize == 0 {
            currentblk = RefCell::from(fm.borrow_mut().append(&logfile.to_string())?);
            logpage.set_i32(0, fm.borrow().block_size() as i32)?;
            fm.borrow_mut().write(&currentblk.borrow(), &mut logpage)?;
        } else {
            currentblk = RefCell::from(BlockId::new(logfile.to_string(), logsize - 1));
            fm.borrow_mut().read(&currentblk.borrow(), &mut logpage)?;
        }
        Ok(LogMgr {
            fm: fm,
            logfile: logfile.to_string(),
            logpage,
            currentblk: Rc::new(currentblk),
            latest_lsn: 0,
            last_saved_lsn: 0,
        })
    }

    pub fn iterator(&mut self) -> Result<LogIterator> {
        self.private_flush()?;
        LogIterator::new(self.fm.clone(), self.currentblk.clone())
    }

    pub fn append(&mut self, logrec: Vec<u8>) -> Result<i32> {
        let mut boundary = self.logpage.get_i32(0)?;
        let recsize = logrec.len();
        let bytesneeded = recsize + 4;
        if boundary - (bytesneeded as i32) < 4 {
            self.private_flush()?;
            self.currentblk = Rc::new(self.append_new_block()?);
            boundary = self.logpage.get_i32(0)?;
        }
        let recpos = (boundary as usize) - bytesneeded;
        self.logpage.set_bytes(recpos, logrec)?;
        self.logpage.set_i32(0, recpos as i32)?;
        self.latest_lsn += 1;
        Ok(self.latest_lsn)
    }

    fn append_new_block(&mut self) -> Result<RefCell<BlockId>> {
        let blk = self.fm.borrow_mut().append(&self.logfile)?;
        self.logpage
            .set_i32(0, self.fm.borrow_mut().block_size() as i32)?;
        self.fm
            .borrow_mut()
            .write(&self.currentblk.borrow_mut(), &mut self.logpage)?;
        Ok(RefCell::from(blk))
    }

    pub fn flush(&mut self, lsn: i32) -> Result<()> {
        if lsn >= self.last_saved_lsn {
            self.private_flush()?;
        }
        Ok(())
    }

    fn private_flush(&mut self) -> Result<()> {
        self.fm
            .borrow_mut()
            .write(&self.currentblk.borrow_mut(), &mut self.logpage)?;
        self.last_saved_lsn = self.latest_lsn;
        Ok(())
    }
}
