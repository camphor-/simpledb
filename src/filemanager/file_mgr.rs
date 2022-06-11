use anyhow::Result;
use std::collections::HashMap;
use std::fs;

pub struct FileMgr {
    db_directory: File,
    blocksize: usize,
    is_new: bool,
    open_files: HashMap,
}

impl FileMgr {
    pub fn New(db_directory_path: Path, blocksize: usize) -> Result<Self> {
        let is_new = !db_directory_path.is_dir();
        if is_new {
            fs::create_dir_all(db_directory_path);
        }
        for entry in fs::read_dir(db_directory_path)? {
            let entry = entry.Ok()?;
            if entry.file_type().ok()?.is_file() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.starts_with("temp") {
                        fs::remove_dir_all(entry.path());
                    }
                }
            }
        }
        Ok(FileMgr {
            db_directory,
            blocksize,
            is_new,
            open_files: HashMap::new(),
        })
    }
}
