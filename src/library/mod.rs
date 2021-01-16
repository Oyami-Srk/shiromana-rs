use std::{fs, io, path};
use std::collections::HashSet;
use std::fmt;
use std::fs::ReadDir;

use rusqlite;
use rusqlite::Connection;
use uuid;

use crate::misc::{config, Error, MediaType, Result};

pub struct LibrarySummary {
    pub media_count: usize,
    pub series_count: usize,
    pub media_size: usize,
}

impl fmt::Display for LibrarySummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Library Summary:\nMedia count: {}\nSeries count: {}\nMedia Size: {} KB\n",
               self.media_count, self.series_count, self.media_size)
    }
}

pub struct Library {
    db: rusqlite::Connection,
    shared_db: rusqlite::Connection,
    path: String,
    uuid: uuid::Uuid,
    library_name: String,
    schema: String,
}

impl Library {
    pub fn open(path: String) -> Result<'static, Library> {
        let library_path = path::Path::new(&path);
        if !library_path.exists() {
            return Err(Error::NotExists(Some(path)));
        }
        if library_path.is_file() {
            return Err(Error::NotMatch(Some("Library is not a folder.".to_string())));
        }
        match fs::read_dir(library_path) {
            Ok(files) => {
                let to_check: Vec<&str> = vec![config::METADATA_FN,
                                               config::FINGERPRINT_FN,
                                               config::DATABASE_FN,
                                               config::SHARED_DATABASE_FN,
                                               config::MEDIAS_FOLDER].
                    iter().copied().collect();
                let files_list = files.map(|entry| {
                    let entry = entry.unwrap();
                    let filename = entry.file_name().to_str().unwrap().to_string();
                    filename
                }).collect::<Vec<String>>();
                if to_check.iter().all(|item| files_list.contains(&item.to_string())) {
                    return Err(Error::Other("hahaha"));
                    // return Err(Error::NotMatch(Some("Library structure not match.".to_string())));
                }
            }
            Err(_) => { panic!("Cannot read dir") }
        }

        Ok(Library
        {
            db: Connection::open_in_memory()?,
            shared_db: Connection::open_in_memory()?,
            path,
            uuid: Default::default(),
            library_name: "".to_string(),
            schema: "".to_string(),
        }
        )
    }
}