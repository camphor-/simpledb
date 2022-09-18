use simpledb::{memorymanager::log_manager::LogMgr, filemanager::file_mgr::FileMgr};
use simpledb::{filemanager::page::{Page, New}};

extern crate simpledb;

#[test]
pub fn log_mananger_test() {
    let fm = FileMgr::new("testdata", 30).unwrap();
    let lm = LogMgr::new(fm, "log_manager_test");

    lm.
}

fn append_records(lm: &LogMgr, start: i32, end: i32) {
    for i in start..=end {
        let rec = create_record(format!("record{}", i), i);
        let lsn = lm.append(rec);
    }
}

fn create_record(s: String, n: i32) -> Vec<u8> {
    let npos = Page::max_length(s.len());
    let b = Vec::with_capacity(npos);
    let p = Page::new(b);
    p.set_string(0, s).unwrap();
    p.set_i32(npos, n).unwrap();
    return b;
}
