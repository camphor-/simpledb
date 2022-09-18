extern crate simpledb;

use simpledb::filemanager::block_id::BlockId;
use simpledb::filemanager::file_mgr::FileMgr;
use simpledb::filemanager::page::{New, Page};
use simpledb::SimpleDB;

#[test]
fn filenamager_test() {
    // TODO: SimpleDB を使う

    let mut file_mgr = FileMgr::new("testdata", 1024).unwrap();

    let filename = "filemanager_test".to_string();

    let blk = BlockId::new(filename, 2);
    let mut p1 = Page::new(file_mgr.block_size());
    let pos1 = 88;
    p1.set_string(pos1, "abcdefghijklm".to_string()).unwrap();
    let size = Page::max_length("abcdefghijklm".len());
    let pos2 = pos1 + size;
    p1.set_i32(pos2, 345).unwrap();
    file_mgr.write(&blk, &mut p1).unwrap();

    let mut p2 = Page::new(file_mgr.block_size());
    file_mgr.read(&blk, &mut p2).unwrap();

    assert_eq!(p2.get_string(pos1).unwrap(), "abcdefghijklm".to_string());
    assert_eq!(p2.get_i32(pos2).unwrap(), 345);

    fs::remove_dir_all("testdata").unwrap();
}
