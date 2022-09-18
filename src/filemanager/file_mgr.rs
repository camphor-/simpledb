use anyhow::{anyhow, Error, Result};
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::ops::Deref;
use std::os::unix::prelude::FileExt;
use std::path::Path;

use super::block_id::BlockId;
use super::page::Page;

pub struct FileMgr {
    db_directory: String,
    blocksize: usize,
    is_new: bool,
    open_files: HashMap<String, File>,
}

impl FileMgr {
    pub fn new(db_directory_path: &str, blocksize: usize) -> Result<Self> {
        let path = Path::new(db_directory_path);
        let is_new = !path.is_dir();
        if is_new {
            fs::create_dir_all(path)?;
        }
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.starts_with("temp") {
                        fs::remove_dir_all(entry.path())?;
                    }
                }
            }
        }
        Ok(FileMgr {
            db_directory: db_directory_path.to_string(),
            blocksize,
            is_new,
            open_files: HashMap::new(),
        })
    }

    pub fn read(&mut self, blk: &BlockId, p: &mut Page) -> Result<()> {
        let blocksize = self.block_size();
        let f = self.get_file(&blk.filename())?;
        f.read_at(p.contents().as_mut_slice(), blk.number() * blocksize as u64)?;
        Ok(())
    }

    // メモ: p.contents を mutable で持ちたくない
    pub fn write(&mut self, blk: &BlockId, p: &mut Page) -> Result<()> {
        let blocksize = self.block_size();
        let f = self.get_file(&blk.filename())?;
        f.write_at(
            p.contents().clone().as_slice(),
            blk.number() * blocksize as u64,
        )?;
        Ok(())
    }

    pub fn append(&mut self, filename: &String) -> Result<BlockId> {
        let newblknum = self.length(filename)?;
        let blk = BlockId::new(filename.clone(), newblknum);
        let blocksize = self.block_size();

        let v = vec![0; self.blocksize];
        let b = v.as_slice();
        let f = self.get_file(&blk.filename())?;
        f.write_at(b, newblknum * blocksize as u64)?;

        Ok(blk)
    }

    pub fn length(&self, filename: &String) -> Result<u64> {
        Ok(fs::metadata(filename)?.len() as u64)
    }

    pub fn is_new(&self) -> bool {
        self.is_new
    }

    pub fn block_size(&self) -> usize {
        self.blocksize
    }

    fn get_file(&mut self, file_name: &String) -> Result<File> {
        let f = self.open_files.get(file_name);
        if let Some(f) = f {
            return Ok(f.try_clone()?);
        } else {
            let file_name = format!("{}/{}", self.db_directory, file_name);
            let path = Path::new(&file_name);
            let f = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(&path)?;
            self.open_files.insert(file_name.clone(), f);
            if let Some(f) = self.open_files.get(&file_name) {
                return Ok(f.try_clone()?);
            }
        }

        Err(anyhow!(""))
    }
}
