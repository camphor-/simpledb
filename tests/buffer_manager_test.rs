use simpledb::filemanager::block_id::BlockId;
use simpledb::SimpleDB;
use std::collections::HashMap;
use std::fs;

fn buffermgrtest() {
    let mut db = SimpleDB::new("testdata", 400, 3).unwrap();
    let bm = db.buffer_mgr();

    let mut buff = Vec::with_capacity(6);
    buff.push(
        bm.pin(&BlockId::new("buffer_manager_test".to_string(), 0))
            .unwrap(),
    );
    buff.push(
        bm.pin(&BlockId::new("buffer_manager_test".to_string(), 1))
            .unwrap(),
    );
    buff.push(
        bm.pin(&BlockId::new("buffer_manager_test".to_string(), 2))
            .unwrap(),
    );
    bm.unpin(buff[1]);
    buff[1] = 10;
    buff.push(
        bm.pin(&BlockId::new("buffer_manager_test".to_string(), 0))
            .unwrap(),
    );
    buff.push(
        bm.pin(&BlockId::new("buffer_manager_test".to_string(), 1))
            .unwrap(),
    );
    assert_eq!(0, bm.available());

    assert!(bm
        .pin(&BlockId::new("buffer_manager_test".to_string(), 3))
        .is_err());

    bm.unpin(buff[2]);
    buff[2] = 10;
    buff.push(
        bm.pin(&BlockId::new("buffer_manager_test".to_string(), 3))
            .unwrap(),
    );

    let exp = HashMap::from([
        (0, BlockId::new("buffer_manager_test".to_string(), 0)),
        (3, BlockId::new("buffer_manager_test".to_string(), 0)),
        (4, BlockId::new("buffer_manager_test".to_string(), 1)),
        (5, BlockId::new("buffer_manager_test".to_string(), 3)),
    ]);
    for (i, idx) in buff.iter().enumerate() {
        if *idx != 10 {
            let b = bm.buffer(*idx);
            assert_eq!(exp.get(&i).unwrap(), b.block().as_ref().unwrap());
        } else {
            assert!(i == 1 || i == 2);
        }
    }

    fs::remove_dir_all("testdata").unwrap();
}
