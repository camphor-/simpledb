use simpledb::filemanager::block_id::BlockId;
use simpledb::SimpleDB;
use std::collections::HashMap;
use std::fs;

#[test]
fn buffermgrtest() {
    let mut db = SimpleDB::new("testdata", 400, 3).unwrap();
    let bm = db.buffer_mgr();

    let mut buff = Vec::with_capacity(6);
    buff.push(
        bm.pin(&BlockId::new("buffer_manager_test".to_string(), 0))
            .unwrap(),
    );
    println!("hoge");
    buff.push(
        bm.pin(&BlockId::new("buffer_manager_test".to_string(), 1))
            .unwrap(),
    );
    println!("fuga");
    buff.push(
        bm.pin(&BlockId::new("buffer_manager_test".to_string(), 2))
            .unwrap(),
    );
    bm.unpin(buff[1].borrow_mut()).unwrap();
    println!("piyo");
    buff.push(
        bm.pin(&BlockId::new("buffer_manager_test".to_string(), 0))
            .unwrap(),
    );
    println!("hogehoge");
    buff.push(
        bm.pin(&BlockId::new("buffer_manager_test".to_string(), 1))
            .unwrap(),
    );
    println!("yeah");
    assert_eq!(0, bm.available());

    assert!(bm
        .pin(&BlockId::new("buffer_manager_test".to_string(), 3))
        .is_err());

    bm.unpin(buff[2].borrow_mut()).unwrap();
    println!("fugafuga");
    buff.push(
        bm.pin(&BlockId::new("buffer_manager_test".to_string(), 3))
            .unwrap(),
    );
    println!("piyopiyo");

    let exp = HashMap::from([
        (0, BlockId::new("buffer_manager_test".to_string(), 0)),
        (3, BlockId::new("buffer_manager_test".to_string(), 0)),
        (4, BlockId::new("buffer_manager_test".to_string(), 1)),
        (5, BlockId::new("buffer_manager_test".to_string(), 3)),
    ]);
    for (i, b) in buff.iter().enumerate() {
        if i != 1 && i != 2 {
            assert_eq!(exp.get(&i).unwrap(), b.borrow_mut().block().unwrap());
        } else {
            assert!(i == 1 || i == 2);
        }
    }

    fs::remove_dir_all("testdata").unwrap();
}
