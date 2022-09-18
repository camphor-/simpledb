use anyhow::{Result};

use super::super::filemanager::file_mgr::FileMgr;

pub struct LogMgr{}

impl LogMgr {
    pub fn new(fm: FileMgr, logfile: &str) -> LogMgr{
        LogMgr {  }
    }

    pub fn append(rec: Vec<u8>) -> Result<u32> {
        Ok(0)
    }

    pub fn flush(lsn: u32) -> Result<()> {
        Ok(())
    }

    // pub fn iterator
}
