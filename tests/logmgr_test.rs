use simpledb::filemanager::page::{New, Page};
use simpledb::{filemanager::file_mgr::FileMgr, memorymanager::log_manager::LogMgr};

use std::cell::RefCell;
use std::rc::Rc;

extern crate simpledb;

#[test]
pub fn log_mananger_test() {
    let fm = FileMgr::new("testdata", 400).unwrap();
    let mut lm = LogMgr::new(Rc::new(RefCell::new(fm)), "log_manager_test").unwrap();

    append_records(&mut lm, 1, 70);
    lm.flush(70).unwrap();
}

fn append_records(lm: &mut LogMgr, start: i32, end: i32) {
    for i in start..=end {
        let rec = create_record(format!("record{}", i), i);
        lm.append(rec).unwrap();
    }
}

fn create_record(s: String, n: i32) -> Vec<u8> {
    let npos = Page::max_length(s.len());
    let b = Vec::with_capacity(npos);
    let mut p = Page::new(b);
    p.set_string(0, s).unwrap();
    p.set_i32(npos, n).unwrap();
    return p.contents().clone();
}
