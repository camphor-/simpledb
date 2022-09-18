use super::super::filemanager::file_mgr::FileMgr;
use super::super::filemanager::block_id::BlockId;
use super::super::filemanager::page::Page;

pub struct LogIterator {
    fm: FileMgr,
    blk: BlockId,
    p: Page,
    currentpos: i32,
    boundary: i32,
}

impl LogIterator {
    pub fn new(fm: FileMgr, blk: BlockId) {
        LogIterator {
            fm: fm,
            blk: blk,
            p: Page::new(fm.block_size()),
            currentpos: 0,
            boundary: 0,
        }
    }
}
