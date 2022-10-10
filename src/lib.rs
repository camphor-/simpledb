use anyhow::Result;
use filemanager::file_mgr::FileMgr;
use memorymanager::{buffer_manager::BufferMgr, log_manager::LogMgr};
use std::cell::RefCell;
use std::rc::Rc;

pub mod filemanager;
pub mod memorymanager;

pub struct SimpleDB {
    fm: Rc<RefCell<FileMgr>>,
    lm: Rc<RefCell<LogMgr>>,
    bm: BufferMgr,
}

impl SimpleDB {
    pub fn new(dirname: &str, blocksize: usize, buffsize: usize) -> Result<SimpleDB> {
        let fm = Rc::new(RefCell::new(FileMgr::new(dirname, blocksize)?));
        let lm = Rc::new(RefCell::new(LogMgr::new(fm.clone(), "simpledb.log")?));
        let bm = BufferMgr::new(fm.clone(), lm.clone(), buffsize);
        Ok(SimpleDB { fm, lm, bm })
    }

    pub fn file_mgr(&self) -> &RefCell<FileMgr> {
        &self.fm
    }

    pub fn log_mgr(&mut self) -> &RefCell<LogMgr> {
        &self.lm
    }

    pub fn buffer_mgr(&mut self) -> &mut BufferMgr {
        &mut self.bm
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
