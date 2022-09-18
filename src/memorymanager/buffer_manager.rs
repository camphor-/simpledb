use anyhow::{anyhow, Result};
use std::thread;
use std::time::{Duration, SystemTime};
use std::{cell::RefCell, rc::Rc};

use super::super::filemanager::block_id::BlockId;
use super::super::filemanager::file_mgr::FileMgr;
use super::buffer::Buffer;
use super::log_manager::LogMgr;

const MAX_TIME: u128 = 10000;

fn waiting_too_long(starttime: SystemTime) -> bool {
    SystemTime::now()
        .duration_since(starttime)
        .unwrap()
        .as_millis()
        > MAX_TIME
}

pub struct BufferMgr {
    bufferpool: Vec<Rc<RefCell<Buffer>>>,
    num_available: usize,
}

impl BufferMgr {
    pub fn new(fm: Rc<RefCell<FileMgr>>, lm: Rc<LogMgr>, numbuffs: usize) -> Self {
        let mut bufferpool = Vec::with_capacity(numbuffs);

        for _ in 0..numbuffs {
            bufferpool.push(Rc::new(RefCell::new(Buffer::new(fm.clone(), lm.clone()))));
        }

        Self {
            bufferpool,
            num_available: numbuffs,
        }
    }

    pub fn pin(&mut self, blk: &BlockId) -> Result<Rc<RefCell<Buffer>>> {
        let timestamp = SystemTime::now();
        let mut buff = self.try_to_pin(blk);
        while buff.is_none() && !waiting_too_long(timestamp) {
            thread::sleep(Duration::from_millis(MAX_TIME as u64));
            buff = self.try_to_pin(blk);
        }

        if let Some(b) = buff {
            Ok(b)
        } else {
            return Err(anyhow!("pin"));
        }
    }

    pub fn unpin(&mut self, buff: &mut Buffer) -> Result<()> {
        buff.unpin();
        if buff.is_pinned() {
            self.num_available += 1;
        }
        Ok(())
    }

    pub fn available(&self) -> usize {
        self.num_available
    }

    pub fn flush_all(&self, txnum: i32) -> Result<()> {
        for buff in self.bufferpool.clone() {
            if buff.borrow().modifying_tx() == txnum {
                buff.borrow_mut().flush();
            }
        }
        Err(anyhow!("flush_all"))
    }

    fn try_to_pin(&mut self, blk: &BlockId) -> Option<Rc<RefCell<Buffer>>> {
        let buff_opt = self.find_existing_buffer(blk);
        let buff;

        if let Some(b) = buff_opt {
            buff = b;
        } else {
            let buff_opt = self.choose_unpinned_buffer();
            if let Some(b) = buff_opt {
                buff = b;
            } else {
                return None;
            }

            buff.borrow_mut().assign_to_block(blk);
        }

        if !buff.borrow().is_pinned() {
            self.num_available -= 1;
        }

        buff.borrow_mut().pin();
        Some(buff)
    }

    fn find_existing_buffer(&mut self, blk: &BlockId) -> Option<Rc<RefCell<Buffer>>> {
        for buff in self.bufferpool.clone() {
            if let Some(b) = buff.borrow().block() {
                if b == blk {
                    return Some(buff.clone());
                }
            }
        }
        None
    }

    fn choose_unpinned_buffer(&mut self) -> Option<Rc<RefCell<Buffer>>> {
        for buff in self.bufferpool.clone() {
            if !buff.borrow().is_pinned() {
                return Some(buff);
            }
        }
        None
    }
}
