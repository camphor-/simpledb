use simpledb::SimpleDB;
use simpledb::{memorymanager::log_manager::LogMgr, filemanager::file_mgr::FileMgr};
use simpledb::{filemanager::page::{Page, New}};
use simpledb::{memorymanager::buffer_manager::BufferMgr};

pub fn buffer_test() {
    let simpledb = SimpleDB::new("testdata", 400, 3);
    let fm 
    let fm = FileMgr::new("testdata", 400);
    let lm = LogMgr::new(fm, "buffer_test");
    let bm = BufferMgr::new(fm, lm, 3);
}
