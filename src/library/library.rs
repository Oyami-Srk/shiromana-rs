use std::{env, fmt, fs, path, str};
use std::ffi::OsStr;
use std::path::Path;

use rusqlite::Connection;
use rusqlite::params;
use textwrap::indent;

use crate::library::MediaSetType;
use crate::media::*;

use super::{Library, LibraryMetadata, LibrarySummary};
use super::super::media::MediaType;
use super::super::misc::{config, Error, HashAlgo, Lock, LockType, Result, tools, Uuid};

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
                                               config::SHARED_DATABASE_FN]
                    .iter().copied().collect();
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
        std::env::set_current_dir(current_workdir)?;

        Ok(Library {
            db,
            shared_db,
            path,
            uuid: library_uuid.as_str().parse().unwrap(),
            library_name: metadata.library_name,
            master_name: metadata.master_name,
            media_folder: metadata.media_folder,
            schema: metadata.schema,
            summary: metadata.summary,
            hash_algo: HashAlgo::from_string(metadata.hash_algo)?,
            lock,
        })
    }

    pub fn create(path: String, library_name: String, master_name: Option<String>, media_folder: Option<String>) -> Result<Library> {
        let library_path = path::PathBuf::from(path);
        let library_path = if library_path.is_absolute() {
            library_path
        } else {
            env::current_dir()?.join(library_path.as_path())
        }.join(format!("{}.{}", library_name, config::LIBRARY_EXT));
        if library_path.exists() {
            return Err(Error::AlreadyExists(library_path.to_str().unwrap().to_string()));
        }
        fs::create_dir(&library_path)?;
        let library_uuid = Uuid::new_v4();
        let media_folder = match media_folder {
            Some(v) => {
                if tools::is_valid_filename(&v) {
                    v
                } else {
                    config::DEFAULT_MEDIAS_FOLDER.to_string()
                }
            }
            None => config::DEFAULT_MEDIAS_FOLDER.to_string()
        };
        let metadata = LibraryMetadata {
            UUID: library_uuid.to_string(),
            library_name: library_name.clone(),
            master_name: master_name.clone(),
            schema: "Default".to_string(),
            media_folder: media_folder.clone(),
            summary: LibrarySummary {
                media_count: 0,
                series_count: 0,
                tags_count: 0,
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
                    comment TEXT
                );

                CREATE TABLE media_detail(
                    id INTEGER PRIMARY KEY NOT NULL UNIQUE,
                    tags TEXT, /* Split by ',' */
                    details TEXT NOT NULl, /* json format */
                    FOREIGN KEY(id) REFERENCES media(id)
                );

                CREATE TABLE media_tags_ref(
                    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
                    media_id INTEGER NOT NULL,
                    tags_uuid CHAR(36) NOT NULL,
                    FOREIGN KEY(media_id) REFERENCES media(id),
                    FOREIGN KEY(tags_uuid) REFERENCES tags(uuid),
                    CONSTRAINT unique_uuid_id UNIQUE (media_id, tags_uuid)
                );

                CREATE TABLE series(
                   uuid CHAR(36) PRIMARY KEY NOT NULL UNIQUE,
                   caption TEXT UNIQUE NOT NULL,
                   media_count INTEGER,
                   comment TEXT
                );

                CREATE TABLE tags(
                   uuid CHAR(36) PRIMARY KEY NOT NULL UNIQUE,
                   caption TEXT UNIQUE NOT NULL,
                   media_count INTEGER,
                   comment TEXT
                );

                CREATE TABLE library(
                    uuid CHAR(36) PRIMARY KEY NOT NULL UNIQUE,
                    path TEXT NOT NULL,
                    comment TEXT
                );

                CREATE TABLE media_location_ref(
                    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
                    media_id INTEGER NOT NULL,
                    path TEXT NOT NULL,
                    filename TEXT NOT NULL,
                    FOREIGN KEY(media_id) REFERENCES media(id),
                    CONSTRAINT unique_uuid_id UNIQUE (media_id, path)
                );
                ")?;
        db.execute(
            &format!("CREATE TABLE media_series_ref(
                    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
                    media_id INTEGER NOT NULL,
                    series_uuid CHAR(36) NOT NULL,
                    series_no INTEGER,
                    FOREIGN KEY(media_id) REFERENCES media(id),
                    FOREIGN KEY(series_uuid) REFERENCES series(uuid)
                    {}
                );",
                     if cfg!(feature = "no-duplication-in-series") {
                         ",CONSTRAINT unique_uuid_id UNIQUE (series_uuid, media_id)"
                     } else {
                         ""
                     }),
            params![],
        )?;
        db.execute(
            "INSERT INTO library (uuid, path) VALUES
                    (?, ?);",
            params![&library_uuid, env::current_dir()?.to_str()],
        )?;
        let shared_db = Connection::open(config::SHARED_DATABASE_FN)?;
        fs::create_dir(&media_folder)?;
        env::set_current_dir(current_dir)?;
        let library_path = library_path.canonicalize()?;

        Ok(Library {
            db,
            shared_db,
            path: library_path.to_str().unwrap().to_string(),
            uuid: library_uuid,
            library_name,
            master_name,
            media_folder,
            schema: "Default".to_string(),
            summary: LibrarySummary {
                media_count: 0,
                series_count: 0,
                tags_count: 0,
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
        let file_ext = media_path.extension().unwrap_or(OsStr::new("")).to_str().unwrap();
        let new_path = self.get_media_path_by_hash(&file_hash, file_ext);
        if new_path.exists() {
            // We believe that no collision on images
            return Err(Error::AlreadyExists(file_hash));
        }
        fs::create_dir_all(new_path.parent().unwrap())?;
        fs::copy(&media_path, &new_path)?;
        let file_size = new_path.metadata()?.len();
        self.db.execute(
            "INSERT INTO media (hash, filename, filesize, caption, type, sub_type, type_addition, comment)
            VALUES (?,?,?,?,?,?,?,?);",
            params![file_hash, file_name, &file_size, caption, kind, sub_kind, kind_addition, comment],
        )?;
        let id = self.db.last_insert_rowid() as u64;
        self.summary.media_count += 1;
        self.summary.media_size += file_size as usize;
        // insert into location ref
        let _ = self.db.execute(
            "INSERT INTO media_location_ref (media_id, path, filename) VALUES (?,?,?);",
            params![id, media_path.canonicalize()?.to_str(), media_path.file_stem().unwrap().to_str()],
        ); // ignore fails
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
        let media_file = self.get_media_path_by_hash(&file_hash, ext);
        println!("{}", media_file.to_str().unwrap().to_string());
        if !media_file.is_file() {
            panic!("Media file is not exists or not a regular file.");
        }
        self.db.execute(
            "DELETE FROM media WHERE id = ?;",
            params![id],
        )?;
        fs::remove_file(media_file)?;
        self.summary.media_size -= file_size;
        self.summary.media_count -= 1;
        // TODO: REMOVE series and tag notation
        Ok(())
    }

    pub fn update_media(&mut self, media: &mut Media) -> Result<()> {
        let is_hash_changed: bool = self.db.query_row(
            "SELECT hash != ? FROM media WHERE id = ?;",
            params![&media.hash, media.id],
            |row| Ok(row.get(0)?),
        )?;
        if is_hash_changed {
            // changing hash means to merge two media. if there is no media targeting changed hash
            // we fall. Btw, we tend to keep original media infos instead new one but set filename
            // to the new.
            let is_that_media_exists: bool = self.db.query_row(
                "SELECT EXISTS(SELECT 1 FROM media WHERE hash = ?);",
                params![&media.hash],
                |row| Ok(row.get(0)?),
            )?;
            if !is_that_media_exists {
                return Err(Error::NotExists(format!("Media with Hash {} do not exists.", media.hash)));
            }
            fs::remove_file(&media.filepath)?;
            self.summary.media_size -= media.filesize;
            self.summary.media_count -= 1;
            let new_id: u64 = self.db.query_row(
                "SELECT id FROM media WHERE hash = ?;",
                params![&media.hash],
                |row| Ok(row.get(0)?),
            )?;
            let new_media = self.get_media(new_id)?;
            media.filesize = new_media.filesize;
            media.filename = new_media.filename;
            media.filepath = new_media.filepath;
            self.db.execute(
                "DELETE FROM media WHERE id = ?;",
                params![new_media.id],
            )?; // drop new media from database
        }
        if let Some(detail) = &media.detail {
            let is_detail_exists: bool = self.db.query_row(
                "SELECT EXISTS(SELECT 1 FROM media_detail WHERE id = ?);",
                params![media.id],
                |row| Ok(row.get(0)?),
            )?;
            if is_detail_exists {
                self.db.execute(
                    "UPDATE media_detail SET details = ? WHERE id = ?;",
                    params![serde_json::to_string(detail)?, media.id],
                )?;
            } else {
                self.db.execute(
                    "INSERT INTO media_detail (id, details) VALUES (?, ?);",
                    params![media.id, serde_json::to_string(detail)?],
                )?;
            }
        }
        self.db.execute(
            "UPDATE media
                  SET hash = ?, filename = ?, filesize = ?, caption = ?, type = ?, sub_type = ?, type_addition = ?, comment = ?
                  WHERE id = ?;",
            params![media.hash, media.filename, media.filesize,
                            media.caption, media.kind, media.sub_kind,
                            media.kind_addition, media.comment, media.id],
        )?;
        Ok(())
    }

    pub fn create_set(&mut self, kind: MediaSetType, caption: String, comment: Option<String>) -> Result<Uuid> {
        let uuid = Uuid::new_v4();
        self.db.execute(
            &format!("INSERT INTO {} (uuid, caption, comment, media_count) VALUES (?, ?, ?, 0);",
                     match kind {
                         MediaSetType::Series => "series",
                         MediaSetType::Tag => "tags"
                     }),
            params![
                uuid, caption, comment],
        )?;
        match kind {
            MediaSetType::Series => self.summary.series_count += 1,
            MediaSetType::Tag => self.summary.tags_count += 1,
        }
        Ok(uuid)
    }

    pub fn delete_set(&mut self, kind: MediaSetType, uuid: &Uuid) -> Result<()> {
        self.db.execute(
            "DELETE FROM {} WHERE uuid = ?;",
            params![
        match kind {
        MediaSetType::Series => "series",
        MediaSetType::Tag => "tags"
        }, uuid],
        )?;
        self.db.execute(
            match kind {
                MediaSetType::Series => "DELETE FROM media_series_ref WHERE series_uuid = ?;",
                MediaSetType::Tag => "DELETE FROM media_tags_ref WHERE tags_uuid = ?;",
            },
            params![uuid],
        )?;
        self.db.execute(
            match kind {
                MediaSetType::Series => "DELETE FROM series WHERE uuid = ?;",
                MediaSetType::Tag => "DELETE FROM tags WHERE uuid = ?;",
            },
            params![uuid],
        )?;
        match kind {
            MediaSetType::Series => self.summary.series_count -= 1,
            MediaSetType::Tag => self.summary.tags_count -= 1,
        }
        Ok(())
    }

    // Return (Series_UUID, Tag_UUID)
    pub fn get_set_by_name(&self, caption: String) -> Result<(Option<Uuid>, Option<Uuid>)> {
        let series_uuid: Option<Uuid> = self.db.query_row(
            "SELECT uuid FROM series WHERE caption = ?;",
            params![caption],
            |row| Ok(row.get(0)?),
        ).unwrap_or(None);
        let tag_uuid: Option<Uuid> = self.db.query_row(
            "SELECT uuid FROM tags WHERE caption = ?;",
            params![caption],
            |row| Ok(row.get(0)?),
        ).unwrap_or(None);
        Ok((series_uuid, tag_uuid))
    }

    pub fn add_to_set(&mut self, kind: MediaSetType, id: u64, uuid: &Uuid, no: Option<u64>, unsorted: bool) -> Result<()> {
        match kind {
            MediaSetType::Series => {
                let mut stmt = self.db.prepare(
                    "SELECT series_no FROM media_series_ref WHERE series_uuid = ?1 AND media_id != ?2;"
                )?;
                let iter = stmt.query_map(
                    params![uuid, id],
                    |row| {
                        row.get(0)
                    })?;
                let to_check: Vec<u64> = iter.map(|x| x.unwrap()).collect();
                let no = if let Some(no) = no {
                    // if the no is specified.
                    if to_check.iter().any(|i| { *i == no }) {
                        return Err(Error::Occupied(format!("occupied when add media(id {}) to series {} with no {}", id, uuid, no)));
                    }
                    Some(no)
                } else {
                    // or this is a unsorted media
                    if unsorted {
                        None
                    } else {
                        // or not, we use the biggest no in the to_check list +1 to be the no
                        let biggest = to_check.iter().max();
                        Some(match biggest {
                            Some(m) => m + 1,
                            None => 1
                        })
                    }
                };
                self.db.execute(
                    "INSERT INTO media_series_ref (media_id, series_uuid, series_no) VALUES (?, ?, ?)",
                    params![id, uuid, no],
                )?;
                self.db.execute(
                    "UPDATE series SET media_count = media_count + 1 WHERE uuid = ?;",
                    params![uuid],
                )?;
            }
            MediaSetType::Tag => {
                self.db.execute(
                    "INSERT INTO media_tags_ref (media_id, tag_uuid, series_no) VALUES (?, ?, ?)",
                    params![id, uuid, no],
                )?;
                self.db.execute(
                    "UPDATE tags SET media_count = media_count + 1 WHERE uuid = ?;",
                    params![uuid],
                )?;
            }
        }
        Ok(())
    }

    pub fn remove_from_set(&mut self, kind: MediaSetType, id: u64, uuid: &Uuid) -> Result<()> {
        self.db.execute(
            &format!("DELETE FROM media_{}_ref WHERE media_id = ? AND series_uuid = ?;",
                     match kind {
                         MediaSetType::Series => "series",
                         MediaSetType::Tag => "tags",
                     }),
            params![id, uuid],
        )?;
        self.db.execute(
            &format!("UPDATE {} SET media_count = media_count - 1 WHERE uuid = ?;",
                     match kind {
                         MediaSetType::Series => "series",
                         MediaSetType::Tag => "tags",
                     }),
            params![uuid],
        )?;
        Ok(())
    }

    pub fn update_series_no(&mut self, id: u64, series_uuid: &Uuid, no: u64, insert: bool) -> Result<()> {
        let mut stmt = self.db.prepare(
            "SELECT series_no FROM media_series_ref WHERE series_uuid = ?1 AND media_id != ?2;"
        )?;
        let iter = stmt.query_map(
            params![series_uuid, id],
            |row| {
                row.get(0)
            })?;
        let to_check: Vec<u64> = iter.map(|x| x.unwrap()).collect();
        if to_check.iter().any(|i| { *i == no }) {
            // insert or error
            if !insert {
                return Err(Error::Occupied(format!("occupied when add media(id {}) to series {} with no {}", id, series_uuid, no)));
            }
            self.db.execute(
                "UPDATE media_series_ref SET series_no = series_no + 1 WHERE series_uuid = ? AND series_no >= ?;",
                params![series_uuid, no],
            )?;
        }
        self.db.execute(
            "UPDATE media_series_ref SET series_no = ? WHERE media_id = ?;",
            params![no, id],
        )?;
        Ok(())
    }

    pub fn trim_series_no(&mut self, uuid: &Uuid) -> Result<()> {
        // I sincerely recommend you not to use this function as much as possible
        let mut ids: Vec<(u64, u64)> = self.db.prepare(
            "SELECT media_id, series_no FROM media_series_ref WHERE series_uuid = ?;")?.query_map(
            params![uuid],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?.map(|x| x.unwrap()).collect();
        ids.sort_by(|a, b| a.1.cmp(&b.1));
        ids[0].1 = 1;
        for i in 1..ids.len() {
            ids[i].1 = ids[i - 1].1 + 1;
        }
        for (id, no) in ids {
            self.db.execute(
                "UPDATE media_series_ref SET series_no = ? WHERE media_id = ?;",
                params![no, id],
            )?;
        }
        Ok(())
    }

    pub fn get_media(&self, id: u64) -> Result<Media> {
        // TODO: figure out the time spend on selecting one column and more.
        let mut media =
            self.db.query_row(
                "SELECT
                        hash, filename, filesize, caption, time_add,
                        type, sub_type, type_addition, comment
                      FROM media WHERE id = ?;",
                params![id],
                |row|
                    {
                        let hash: String = row.get(0)?;
                        let file_name = row.get(1)?;
                        let file_ext = path::PathBuf::from(&file_name);
                        let file_ext = file_ext.extension().unwrap_or(OsStr::new("")).to_str().unwrap();
                        let filepath = self.get_media_path_by_hash(&hash, file_ext);
                        let filepath = filepath.to_str().unwrap().to_string();
                        let series_uuids: Vec<Uuid> = self.db.prepare(
                            "SELECT series_uuid FROM media_series_ref WHERE media_id = ?;")?
                            .query_map(params![id],
                                       |row| Ok(row.get(0)?))?
                            .map(|x| x.unwrap())
                            .collect();
                        let tags_uuids: Vec<Uuid> = self.db.prepare(
                            "SELECT tags_uuid FROM media_tags_ref WHERE media_id = ?;")?
                            .query_map(params![id],
                                       |row| Ok(row.get(0)?))?
                            .map(|x| x.unwrap())
                            .collect();
                        Ok(Media {
                            id,
                            library_uuid: self.uuid.clone(),
                            hash,
                            filename: file_name,
                            filepath,
                            filesize: row.get(2)?,
                            caption: row.get(3)?,
                            time_add: row.get(4)?,
                            kind: row.get(5)?,
                            sub_kind: row.get(6)?,
                            kind_addition: row.get(7)?,
                            comment: row.get(8)?,
                            series: series_uuids,
                            tag: tags_uuids,
                            detail: None,
                        })
                    },
            )?;

        let is_detailed: bool = self.db.query_row(
            "SELECT EXISTS(SELECT 1 FROM media_detail WHERE id = ?);",
            params![id],
            |row| Ok(row.get(0)?),
        )?;
        if is_detailed & &media.kind.is_some() {
            let detail: String = self.db.query_row(
                "SELECT details FROM media_detail WHERE id = ?;",
                params![id],
                |row| Ok(row.get(0)?),
            )?;

            media.detail = Some(
                serde_json::from_str(&detail)?
            );
        }

        Ok(media)
    }

    pub fn get_media_by_filename(&self, filename: String) -> Result<Vec<u64>> {
        let filename_stem = Path::new(&filename).file_stem().unwrap().to_str().unwrap();

        let mut result: Vec<u64> = vec![];
        let id_fn: Vec<(u64, String)> = self.db.prepare(
            "SELECT id, filename FROM media_location_ref WHERE filename LIKE ? ESCAPE '\\';"
        )?.query_map(
            params![filename_stem.replace("%", "\\%") + "%"],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?.map(|x| x.unwrap()).collect();

        for v in id_fn.iter() {
            if v.1 == filename_stem || v.1 == filename {
                result.push(v.0);
            }
        };
        result.sort();
        result.dedup();
        Ok(result)
    }

    pub fn get_next_no_in_series(&self, uuid: &Uuid) -> Result<Option<u64>> {
        let max_no: Option<u64> =
            self.db.prepare(
                "SELECT MAX(series_no) as max_no FROM media_series_ref WHERE series_uuid = ?;"
            )?
                .query_row(params![uuid], |row| Ok(row.get(0)?))?;
        Ok(max_no.map(|v| v + 1))
    }

    pub fn query_media(&self, sql_stmt: &str) -> Result<Vec<u64>> {
        Ok(
            self.db.prepare(&format!("SELECT id FROM media WHERE {};", sql_stmt))?
                .query_map(params![],
                           |row| Ok(row.get(0)?))?
                .map(|x| x.unwrap()).collect())
    }

    pub fn query_series(&self, sql_stmt: &str) -> Result<Vec<Uuid>> {
        let _ = sql_stmt;
        unimplemented!()
    }

    pub fn get_library_name(&self) -> &String {
        &self.library_name
    }

    pub fn get_master_name(&self) -> &Option<String> {
        &self.master_name
    }

    pub fn get_uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn get_path(&self) -> &String {
        &self.path
    }

    pub fn get_schema(&self) -> &String {
        &self.schema
    }

    pub fn get_summary(&self) -> &LibrarySummary {
        &self.summary
    }

    pub fn get_hash_size(&self) -> usize {
        self.hash_algo.get_size()
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
            media_folder: self.media_folder.clone(),
        };
        fs::write(path::PathBuf::new().
            join(&self.path[..]).
            join(config::METADATA_FN),
                  serde_json::to_string(&metadata).expect("Cannot serialize metadata.")).expect("Cannot write to metadata.");
    }
}

impl fmt::Display for Library {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Library name: {}\nMaster name: {}\nUUID: {}\nPath: {}\nschema: {}\nLibrary Summary:\n{}",
               self.library_name,
               self.master_name.as_ref().unwrap_or(&"".to_string()),
               self.uuid,
               self.path,
               self.schema,
               indent(&format!("{}", self.summary), "    |-"))
    }
}

impl Library {
    // private method
    fn get_media_path_by_hash(&self, hash: &str, ext: &str) -> path::PathBuf {
        // this method do not promise the existence.
        path::PathBuf::new()
            .join(self.path.as_str())
            .join(&self.media_folder)
            .join(&hash[..2])
            .join(format!("{}{}",
                          &hash[2..],
                          if ext != "" {
                              format!(".{}", ext)
                          } else {
                              "".to_string()
                          }
            ))
    }
}