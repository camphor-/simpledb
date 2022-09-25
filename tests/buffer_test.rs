use simpledb::filemanager::page::{New, Page};
use simpledb::memorymanager::buffer_manager::BufferMgr;
use simpledb::filemanager::block_id::BlockId;
use simpledb::SimpleDB;
use simpledb::{filemanager::file_mgr::FileMgr, memorymanager::log_manager::LogMgr};
use std::fs;

pub fn buffer_test() {
    let mut db = SimpleDB::new("testdata", 400, 3).unwrap();
    let bm = db.buffer_mgr();

    let buff1 = bm.pin(&BlockId::new("buffer_test".to_string(), 1)).unwrap();
    let mut buff1_ref = buff1.borrow_mut();
    let p = buff1_ref.contents();
    let n = p.get_i32(80).unwrap();
    p.set_i32(80, n + 1);
    buff1_ref.set_modified(1, 0);
    assert_eq!(1, n + 1);
    bm.unpin(buff1);

    let mut idx2 = bm.pin(&BlockId::new("buffer_test".to_string(), 2)).unwrap();
    bm.pin(&BlockId::new("buffer_test".to_string(), 3)).unwrap();
    bm.pin(&BlockId::new("buffer_test".to_string(), 4)).unwrap();

    bm.unpin(idx2);
    let buff2 = bm.pin(&BlockId::new("buffer_test".to_string(), 1)).unwrap();
    let p2 = buff2.borrow().contents();
    p2.set_i32(80, 9999);
    buff2.borrow_mut().set_modified(1, 0);
    bm.unpin(idx2);

    fs::remove_dir_all("testdata").unwrap();
}
