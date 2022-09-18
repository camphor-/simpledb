use std::{rc::Rc, cell::RefCell};
use {filemanager::{file_mgr:: FileMgr}};
use memorymanager::{log_manager::LogMgr, buffer_manager::BufferMgr};
use anyhow::Result;

pub mod filemanager;
pub mod memorymanager;

pub struct SimpleDB {
    fm: Rc<FileMgr>,
    lm: Rc<RefCell<LogMgr>>,
    bm: BufferMgr,
}

impl SimpleDB {
    pub fn new(dirname: &str, blocksize: usize, buffsize: usize) -> Result<SimpleDB> {
        let fm = Rc::new(FileMgr::new(dirname, blocksize));
        let lm = Rc::new(RefCell::new(LogMgr::new(fm.clone(), "simpledb.log"))?);
        let bm = BufferMgr::new(fm.clone(), lm.clone(), buffsize as u32);
        Ok(SimpleDB { fm, lm, bm })
    }

    pub fn file_mgr(&self) -> &FileMgr {
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
