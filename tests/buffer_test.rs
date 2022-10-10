use simpledb::filemanager::block_id::BlockId;
use simpledb::SimpleDB;
use std::fs;

#[test]
pub fn buffer_test() {
    let mut db = SimpleDB::new("testdata", 400, 3).unwrap();
    let bm = db.buffer_mgr();

    let buff1 = bm.pin(&BlockId::new("buffer_test".to_string(), 1)).unwrap();
    let mut buff1_ref = buff1.borrow_mut();
    let p = buff1_ref.contents();
    let n = p.get_i32(80).unwrap();
    p.set_i32(80, n + 1).unwrap();
    buff1_ref.set_modified(1, 0);
    assert_eq!(1, n + 1);
    bm.unpin(buff1_ref).unwrap();

    let buff2 = bm.pin(&BlockId::new("buffer_test".to_string(), 2)).unwrap();
    let buff2_ref = buff2.borrow_mut();
    bm.pin(&BlockId::new("buffer_test".to_string(), 3)).unwrap();
    bm.pin(&BlockId::new("buffer_test".to_string(), 4)).unwrap();

    bm.unpin(buff2_ref).unwrap();

    let buff3 = bm.pin(&BlockId::new("buffer_test".to_string(), 1)).unwrap();
    let mut buff3_ref = buff3.borrow_mut();
    let p2 = buff3_ref.contents();
    p2.set_i32(80, 9999).unwrap();
    buff3_ref.set_modified(1, 0);
    bm.unpin(buff3_ref).unwrap();

    fs::remove_dir_all("testdata").unwrap();
}
