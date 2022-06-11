use anyhow::{anyhow, Error, Result};
use std::collections::HashMap;
use std::fs::{self, File};
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

    fn get_file(&mut self, file_name: String) -> Result<File> {
        let f = self.open_files.get(&file_name);
        if let Some(f) = f {
            return Ok(f.try_clone()?);
        } else {
            let file_name = format!("{}/{}", self.db_directory, file_name);
            let path = Path::new(&file_name);
            let db_table = File::open(&path)?;
            if let Some(f) = self.open_files.insert(file_name, db_table) {
                return Ok(f.try_clone().unwrap());
            }
        }

        Err(anyhow!(""))
    }
}
