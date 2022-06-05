use std::vec;
use anyhow::Result;

trait New<T> {
    fn new(c: T) -> Page;
}

pub struct Page {
    bb: Vec<u8>
}

impl New<usize> for Page {
    fn new(blocksize: usize) -> Self {
        Page {
            bb: vec![0; blocksize]
        }
    }
}

impl New<Vec<u8>> for Page {
    fn new(b: Vec<u8>) -> Self {
        Page {
            bb: b
        }
    }
}

// [0x01, 0x02, 0x03, 0x04, 0x05, 0x06]

impl Page {
    fn get_i32(&self, offset: usize) -> Result<i32> {
        if offset + 4 - 1 < self.bb.len() {
            let bytes = &self.bb[offset..offset + 4];
            Ok(i32::from_be_bytes((*bytes).try_into()?))
        } else {
            Err("hoge")
        }
    }
}
