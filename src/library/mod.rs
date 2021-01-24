use std::{env, fs, io, path};
use std::any::Any;
use std::collections::HashSet;
use std::fmt;
use std::fs::ReadDir;
use std::ops::Add;
use std::str::FromStr;

use rusqlite;
use rusqlite::{Connection, params, params_from_iter};
use serde::{Deserialize, Serialize};

use crate::misc::{*};

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Debug)]
pub struct Library {
    db: rusqlite::Connection,
    shared_db: rusqlite::Connection,
    path: String,
    uuid: Uuid,
    library_name: String,
    master_name: Option<String>,
    schema: String,
    summary: LibrarySummary,
    hash_algo: HashAlgo,
    lock: Lock,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct LibraryMetadata {
    UUID: String,
    library_name: String,
    master_name: Option<String>,
    schema: String,
    hash_algo: String,
    summary: LibrarySummary,
}

pub enum MediaUpdateKey {
    Filename,
    Caption,
    SubType,
    TypeAddition,
    Comment,
}

impl MediaUpdateKey {
    fn to_key(&self) -> String {
        match self {
            MediaUpdateKey::Filename => "filename".to_string(),
            MediaUpdateKey::Caption => "caption".to_string(),
            MediaUpdateKey::SubType => "sub_type".to_string(),
            MediaUpdateKey::TypeAddition => "type_addition".to_string(),
            MediaUpdateKey::Comment => "comment".to_string()
        }
    }
}

impl Library {
    pub fn open(path: String) -> Result<Library> {
        let library_path = path::Path::new(&path);
        if !library_path.exists() {
            return Err(Error::NotExists(path));
        }
        if library_path.is_file() {
            return Err(Error::NotMatch("Library is not a folder.".to_string()));
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
                if !to_check.iter().all(|item| files_list.contains(&item.to_string())) {
                    return Err(Error::NotMatch("Library structure not match.".to_string()));
                }
            }
            Err(_) => { panic!("Cannot read dir") }
        }
        let lock: Lock = Lock::acquire(LockType::FolderLock, path.as_str())?;

        let current_workdir = std::env::current_dir()?;
        std::env::set_current_dir(&path)?;
        let metadata: LibraryMetadata = serde_json::from_str(
            fs::read_to_string(config::METADATA_FN)?.as_str()
        )?;

        let library_uuid = fs::read_to_string(config::FINGERPRINT_FN)?;
        if library_uuid != metadata.UUID {
            return Err(Error::NotMatch("Library UUID".to_string()));
        }
        let db = Connection::open(config::DATABASE_FN)?;
        let shared_db = Connection::open(config::SHARED_DATABASE_FN)?;
        let path = std::env::current_dir()?.to_str().unwrap().to_string();
        std::env::set_current_dir(current_workdir);

        Ok(Library {
            db,
            shared_db,
            path,
            uuid: Uuid::from_str(library_uuid.as_str()).unwrap(),
            library_name: metadata.library_name,
            master_name: metadata.master_name,
            schema: metadata.schema,
            summary: metadata.summary,
            hash_algo: HashAlgo::from_string(metadata.hash_algo)?,
            lock,
        })
    }

    pub fn create(path: String, library_name: String, master_name: Option<String>) -> Result<Library> {
        let library_path = path::PathBuf::from(path);
        let library_path = if library_path.is_absolute() {
            library_path
        } else {
            env::current_dir()?.join(library_path.as_path())
        }.join(library_name.clone().add(config::LIBRARY_EXT));
        if library_path.exists() {
            return Err(Error::AlreadyExists(library_path.to_str().unwrap().to_string()));
        }
        fs::create_dir(&library_path)?;
        let library_uuid = Uuid::new_v4();
        let metadata = LibraryMetadata {
            UUID: library_uuid.to_string(),
            library_name: library_name.clone(),
            master_name: master_name.clone(),
            schema: "Default".to_string(),
            summary: LibrarySummary {
                media_count: 0,
                series_count: 0,
                media_size: 0,
            },
            hash_algo: config::DEFAULT_HASH_ALGO.to_string(),
        };
        let current_dir = env::current_dir()?;
        let lock = Lock::acquire(LockType::FolderLock, library_path.to_str().unwrap())?;
        env::set_current_dir(&library_path)?;
        fs::write(config::FINGERPRINT_FN, &library_uuid.to_string()[..36])?;
        fs::write(config::METADATA_FN, serde_json::to_string(&metadata)?)?;
        let db = Connection::open(config::DATABASE_FN)?;
        db.execute_batch(
            "CREATE TABLE media(
                    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
                    hash CHAR(32) NOT NULL UNIQUE,
                    filename TEXT NOT NULL,
                    filesize INTEGER NOT NULL, /* Store in Bytes */
                    caption TEXT,
                    time_add TIMESTAMP NOT NULL DEFAULT(STRFTIME('%Y-%m-%d %H:%M:%f+00:00', 'NOW')),
                    type INTEGER NOT NULL,
                    sub_type CHAR(32),
                    type_addition TEXT,
                    series_uuid CHAR(36),
                    series_no INTEGER,
                    comment TEXT,
                    FOREIGN KEY(series_uuid) REFERENCES series(uuid)
                );

                CREATE TABLE media_detail(
                    id INTEGER PRIMARY KEY NOT NULL UNIQUE,
                    height INTEGER NOT NULL,
                    width INTEGER NOT NULL,
                    dpi TEXT NOT NULL,
                    format TEXT NOT NULL,
                    tags TEXT, /* Split by ',' */
                    FOREIGN KEY(id) REFERENCES media(id)
                );

                CREATE TABLE media_tags_ref(
                    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
                    media_id INTEGER NOT NULL,
                    tags_uuid CHAR(36) NOT NULL,
                    FOREIGN KEY(media_id) REFERENCES media(id)
                );

                CREATE TABLE series(
                   uuid CHAR(36) PRIMARY KEY NOT NULL UNIQUE,
                   caption TEXT,
                   media_count INTEGER,
                   comment TEXT
                );

                CREATE TABLE library(
                    uuid CHAR(36) PRIMARY KEY NOT NULL UNIQUE,
                    path TEXT NOT NULL,
                    comment TEXT
                );")?;
        db.execute(
            "INSERT INTO library (uuid, path) VALUES
                    (?, ?);",
            params![&library_uuid, env::current_dir()?.to_str()],
        )?;
        let shared_db = Connection::open(config::SHARED_DATABASE_FN)?;
        fs::create_dir(config::MEDIAS_FOLDER);
        env::set_current_dir(current_dir)?;
        let library_path = library_path.canonicalize()?;

        Ok(Library {
            db,
            shared_db,
            path: library_path.to_str().unwrap().to_string(),
            uuid: library_uuid,
            library_name,
            master_name,
            schema: "Default".to_string(),
            summary: LibrarySummary {
                media_count: 0,
                series_count: 0,
                media_size: 0,
            },
            hash_algo: HashAlgo::from_string(config::DEFAULT_HASH_ALGO.to_string())?,
            lock,
        })
    }

    pub fn add_media(&mut self, path: String, kind: MediaType, sub_kind: Option<String>,
                     kind_addition: Option<String>, caption: Option<String>,
                     comment: Option<String>) -> Result<u64> {
        let media_path = path::PathBuf::from(path);
        if !media_path.is_file() {
            return Err(err_type_mismatch_expect_dir_found_file!(media_path.to_str().unwrap().to_string()));
        }
        let file_hash = self.hash_algo.do_hash(media_path.to_str().unwrap().to_string())?;
        let file_name = media_path.file_name().unwrap().to_str().unwrap();
        let file_ext = media_path.extension().unwrap().to_str().unwrap();
        let new_path = path::PathBuf::new()
            .join(self.path.as_str())
            .join(config::MEDIAS_FOLDER)
            .join(&file_hash[..2])
            .join(format!("{}.{}", &file_hash[2..], file_ext));
        if new_path.exists() {
            return Err(Error::AlreadyExists(new_path.to_str().unwrap().to_string()));
        }
        fs::create_dir_all(new_path.parent().unwrap())?;
        fs::copy(&media_path, &new_path)?;
        let file_size = new_path.metadata()?.len();
        self.db.execute(
            "INSERT INTO media (hash, filename, filesize, caption, type, sub_type, type_addition, comment)
            VALUES (?,?,?,?,?,?,?,?);",
            params![file_hash, file_name, &file_size, caption, kind as u8, sub_kind, kind_addition, comment],
        )?;
        let id = self.db.last_insert_rowid() as u64;
        // TODO: return a media type when impl media mod
        self.summary.media_count += 1;
        self.summary.media_size += file_size as usize;
        Ok(id)
    }

    pub fn remove_media(&mut self, id: u64) -> Result<()> {
        let (file_hash, file_name, file_size): (String, String, usize) = self.db.query_row(
            "SELECT hash, filename, filesize FROM media WHERE id = ?;",
            params![&id],
            |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?))
            },
        )?;
        let ext: Vec<&str> = file_name.split('.').collect();
        let ext = ext[ext.len() - 1];
        let media_file = path::PathBuf::new()
            .join(self.path.as_str())
            .join(config::MEDIAS_FOLDER)
            .join(&file_hash[..2])
            .join(format!("{}.{}", &file_hash[2..], ext));
        println!("{}", media_file.to_str().unwrap().to_string());
        if !media_file.is_file() {
            panic!("Media file is not exists or not a regular file.");
        }
        self.db.execute(
            "DELETE FROM media WHERE id = ?;",
            params![id],
        );
        fs::remove_file(media_file);
        self.summary.media_size -= file_size;
        self.summary.media_count -= 1;
        Ok(())
    }

    pub fn update_media(&mut self, id: u64, key: MediaUpdateKey, value: String) -> Result<()> {
        self.db.execute(
            format!("UPDATE media SET {} = ? WHERE id = ?;", key.to_key()).as_str(),
            params![value, id],
        )?;
        Ok(())
    }

    pub fn create_series(&mut self, caption: Option<String>, comment: Option<String>) -> Result<Uuid> {
        let uuid = Uuid::new_v4();
        self.db.execute(
            "INSERT INTO series (uuid, caption, comment, media_count) VALUES (?, ?, ?, 0);",
            params![uuid, caption, comment],
        )?;
        self.summary.series_count += 1;
        Ok(uuid)
    }

    pub fn delete_series(&mut self, uuid: &Uuid) -> Result<()> {
        self.db.execute(
            "DELETE FROM series WHERE uuid = ?;",
            params![uuid],
        )?;
        self.db.execute(
            "UPDATE media SET series_uuid = NULL, series_no = NULL WHERE series_uuid = ?;",
            params![uuid],
        )?;
        self.summary.series_count -= 1;
        Ok(())
    }

    pub fn add_to_series(&mut self, id: u64, uuid: &Uuid, no: u64) -> Result<()> {
        let mut stmt = self.db.prepare(
            "SELECT series_no FROM media WHERE series_uuid = ?1 AND id != ?2;"
        )?;
        let iter = stmt.query_map(
            params![uuid, id],
            |row| {
                row.get(0)
            })?;
        let to_check: Vec<u64> = iter.map(|x| x.unwrap()).collect();
        if to_check.iter().any(|i| { *i == no }) {
            return Err(Error::Occupied(format!("occupied when add media(id {}) to series {} with no {}", id, uuid, no)));
        }
        println!("{:?}", to_check);
        self.db.execute(
            "UPDATE media SET series_uuid = ?, series_no = ? WHERE id = ?;",
            params![uuid, no, id],
        )?;
        self.db.execute(
            "UPDATE series SET media_count = media_count + 1 WHERE uuid = ?;",
            params![uuid],
        )?;
        Ok(())
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        let metadata = LibraryMetadata {
            UUID: self.uuid.to_string(),
            library_name: self.library_name.clone(),
            master_name: self.master_name.clone(),
            schema: self.schema.clone(),
            summary: self.summary.clone(),
            hash_algo: self.hash_algo.to_string(),
        };
        fs::write(path::PathBuf::new().
            join(&self.path[..]).
            join(config::METADATA_FN),
                  serde_json::to_string(&metadata).expect("Cannot serialize metadata.")).expect("Cannot write to metadata.");
    }
}
