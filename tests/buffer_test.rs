use simpledb::SimpleDB;
use simpledb::{memorymanager::log_manager::LogMgr, filemanager::file_mgr::FileMgr};
use simpledb::{filemanager::page::{Page, New}};
use simpledb::{memorymanager::buffer_manager::BufferMgr};

pub fn buffer_test() {
    let mut db = SimpleDB::new("testdata", 400, 3).unwrap();
    let bm = db.buffer_mgr();

    let idx1 = bm.pin(&BlockId::new("testfile", 1)).unwrap();
    let buff1 = bm.buffer(idx1);
    let p = buff1.contents();
    let n = p.get_int(80);
    p.set_int(80, n + 1);
    buff1.set_modified(1, 0);
    assert_eq!(1, n + 1);
    bm.unpin(idx1);

    let mut idx2 = bm.pin(&BlockId::new("testfile", 2)).unwrap();
    bm.pin(&BlockId::new("testfile", 3)).unwrap();
    bm.pin(&BlockId::new("testfile", 4)).unwrap();

    bm.unpin(idx2);
    idx2 = bm.pin(&BlockId::new("testfile", 1)).unwrap();
    let buff2 = bm.buffer(idx2);
    let p2 = buff2.contents();
    p2.set_int(80, 9999);
    buff2.set_modified(1, 0);
    bm.unpin(idx2);

    fs::remove_dir_all("testdata").unwrap();
}
