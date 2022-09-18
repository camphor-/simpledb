use anyhow::{anyhow, Result};
use std::vec;

pub trait New<T> {
    fn new(c: T) -> Page;
}

pub struct Page {
    bb: Vec<u8>,
}

impl New<usize> for Page {
    fn new(blocksize: usize) -> Self {
        Page {
            bb: vec![0; blocksize],
        }
    }
}

impl New<Vec<u8>> for Page {
    fn new(b: Vec<u8>) -> Self {
        Page { bb: b }
    }
}

impl Page {
    pub fn get_i32(&self, offset: usize) -> Result<i32> {
        if offset + 4 - 1 < self.bb.len() {
            let bytes = &self.bb[offset..offset + 4];
            Ok(i32::from_be_bytes((*bytes).try_into()?))
        } else {
            Err(anyhow!("out of bounds."))
        }
    }

    pub fn set_i32(&mut self, offset: usize, n: i32) -> Result<()> {
        if offset + 4 - 1 < self.bb.len() {
            let bytes = n.to_be_bytes();
            for i in 0..4 {
                self.bb[offset + i] = bytes[i];
            }
            Ok(())
        } else {
            Err(anyhow!("out of bounds."))
        }
    }

    pub fn get_bytes(&self, offset: usize) -> Result<Vec<u8>> {
        let length = self.get_i32(offset)? as usize;

        if offset + 4 + length - 1 < self.bb.len() {
            Ok(self.bb[offset + 4..offset + 4 + length].to_vec())
        } else {
            Err(anyhow!("out of bounds."))
        }
    }

    pub fn set_bytes(&mut self, offset: usize, bytes: Vec<u8>) -> Result<()> {
        let length: usize = bytes.len();

        if offset + 4 + length - 1 < self.bb.len() {
            self.set_i32(offset, length as i32)?;
            for i in 0..length {
                self.bb[offset + 4 + i] = bytes[i];
            }
            Ok(())
        } else {
            Err(anyhow!("out of bounds."))
        }
    }

    pub fn get_string(&self, offset: usize) -> Result<String> {
        let bytes = self.get_bytes(offset)?;
        Ok(String::from_utf8(bytes)?)
    }

    pub fn set_string(&mut self, offset: usize, s: String) -> Result<()> {
        let bytes = s.into_bytes();
        self.set_bytes(offset, bytes)
    }

    pub fn contents(&mut self) -> &mut Vec<u8> {
        &mut self.bb
    }

    pub fn max_length(strlen: usize) -> usize {
        4 + (strlen * 1) // bytes per char = 1
    }
}
