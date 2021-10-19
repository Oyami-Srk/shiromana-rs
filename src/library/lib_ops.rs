use std::{env, fs, path, str, str::FromStr};

use rusqlite::params;
use rusqlite::Connection;

use super::super::misc::{config, tools, Error, HashAlgo, Lock, LockType, Result, Uuid};
use super::{Library, LibraryFeatures, LibraryMetadata, LibrarySummary};

use semver;

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
                let to_check: Vec<&str> = vec![
                    config::METADATA_FN,
                    config::FINGERPRINT_FN,
                    config::DATABASE_FN,
                    config::SHARED_DATABASE_FN,
                ]
                .iter()
                .copied()
                .collect();
                let files_list = files
                    .map(|entry| {
                        let entry = entry.unwrap();
                        let filename = entry.file_name().to_str().unwrap().to_string();
                        filename
                    })
                    .collect::<Vec<String>>();
                if !to_check
                    .iter()
                    .all(|item| files_list.contains(&item.to_string()))
                {
                    return Err(Error::NotMatch("Library structure not match.".to_string()));
                }
            }
            Err(_) => {
                panic!("Cannot read dir")
            }
        }
        let lock: Lock = Lock::acquire(LockType::FolderLock, path.as_str())?;

        let current_workdir = std::env::current_dir()?;
        std::env::set_current_dir(&path)?;
        let metadata: LibraryMetadata =
            serde_json::from_str(fs::read_to_string(config::METADATA_FN)?.as_str())?;

        let library_uuid = fs::read_to_string(config::FINGERPRINT_FN)?;
        if library_uuid != metadata.UUID {
            return Err(Error::NotMatch("Library UUID".to_string()));
        }
        let db = Connection::open(config::DATABASE_FN)?;
        let shared_db = Connection::open(config::SHARED_DATABASE_FN)?;
        let thumbnail_db = Connection::open(config::THUMBNAIL_DATABASE_FN)?;
        let path = std::env::current_dir()?.to_str().unwrap().to_string();
        std::env::set_current_dir(current_workdir)?;

        let version: String = db.query_row("SELECT version FROM metadata", params![], |row| {
            Ok(row.get(0)?)
        })?;
        let version = semver::Version::parse(version.as_str()).expect("Broken DB metadata");

        let features: String = db.query_row("SELECT features FROM metadata", params![], |row| {
            Ok(row.get(0)?)
        })?;
        let features = LibraryFeatures::from_str(features.as_str()).unwrap();

        Ok(Library {
            version,
            db,
            shared_db,
            thumbnail_db,
            path,
            uuid: library_uuid.as_str().parse().unwrap(),
            library_name: metadata.library_name,
            master_name: metadata.master_name,
            media_folder: metadata.media_folder,
            schema: metadata.schema,
            summary: metadata.summary,
            hash_algo: HashAlgo::from_string(metadata.hash_algo)?,
            lock,
            features,
        })
    }

    pub fn create(
        path: String,
        library_name: String,
        master_name: Option<String>,
        media_folder: Option<String>,
        features: LibraryFeatures,
    ) -> Result<Library> {
        let library_path = path::PathBuf::from(path);
        let library_path = if library_path.is_absolute() {
            library_path
        } else {
            env::current_dir()?.join(library_path.as_path())
        }
        .join(format!("{}.{}", library_name, config::LIBRARY_EXT));
        if library_path.exists() {
            return Err(Error::AlreadyExists(
                library_path.to_str().unwrap().to_string(),
            ));
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
            None => config::DEFAULT_MEDIAS_FOLDER.to_string(),
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

                CREATE TABLE metadata(
                    version TEXT NOT NULL NOT NULL UNIQUE,
                    features TEXT NOT NULL NOT NULL UNIQUE,
                );
                ",
        )?;
        db.execute(
            &format!(
                "CREATE TABLE media_series_ref(
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
                }
            ),
            params![],
        )?;
        db.execute(
            "INSERT INTO library (uuid, path) VALUES
                    (?, ?);",
            params![&library_uuid, env::current_dir()?.to_str()],
        )?;
        let shared_db = Connection::open(config::SHARED_DATABASE_FN)?;
        let thumbnail_db = Connection::open(config::THUMBNAIL_DATABASE_FN)?;
        thumbnail_db.execute_batch(
            "
                CREATE TABLE metadata(
                    library_uuid CHAR(36) PRIMARY KEY NOT NULL UNIQUE
                );
                CREATE TABLE thumbnail(
                    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
                    filename TEXT NOT NULL UNIQUE,
                    image BLOB,
                    size INTEGER NOT NULL,
                );
                ",
        )?;
        thumbnail_db.execute(
            "INSERT INTO metadata (library_uuid) VALUES (?);",
            params![&library_uuid],
        )?;
        fs::create_dir(&media_folder)?;
        env::set_current_dir(current_dir)?;
        let library_path = library_path.canonicalize()?;
        let version = semver::Version::new(
            env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
            env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
            env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
        );
        db.execute(
            "INSERT INTO metadata (version, features) VALUES (?, ?);",
            params![version.to_string(), features.to_string(),],
        )?;

        Ok(Library {
            version,
            db,
            shared_db,
            thumbnail_db,
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
            features,
        })
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
