#![macro_use]

pub mod config {
    pub const LIBRARY_EXT: &str = "mlib";
    pub const METADATA_FN: &str = "metadata.json";
    pub const DATABASE_FN: &str = "shiromana.db";
    pub const SHARED_DATABASE_FN: &str = "shared.db";
    pub const THUMBNAIL_DATABASE_FN: &str = "thumbnail.db";
    pub const FINGERPRINT_FN: &str = ".shiromana";
    pub const DEFAULT_MEDIAS_FOLDER: &str = "medias";
    pub const MEDIAS_HASH_LEVEL: u32 = 1;
    // max files is only for warning
    pub const MEDIAS_FOLDER_MAX_FILES: u32 = 10000;
    // pub const DEFAULT_HASH_ALGO: &str = "MD5";
    pub const DEFAULT_HASH_ALGO: &str = "BLAKE3";
    pub const LOCKFILE: &str = ".LOCK";
}

#[derive(Debug)]
pub enum Error {
    NotExists(String),
    // file or dir
    AlreadyExists(String),
    // file or dir
    Locked(String),
    NotMatch(String),
    TypeMismatch {
        val: String,
        expect: String,
        found: String,
    },
    Occupied(String),
    NotIn {
        a: String,
        b: String,
    },
    IO(std::io::Error),
    DB(rusqlite::Error),
    LockError(LockError),
    JsonError(serde_json::Error),
    Other(String),
    MediaDecode(String),
    Plugin(super::plugin::PluginError),
    NoneError,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum LockType {
    FileLock,
    FolderLock,
}

#[derive(Debug)]
pub enum LockStatus {
    Locked,
    Unlocked,
}

#[derive(Debug)]
pub enum LockError {
    LockFailed(String),
    UnlockFailed(String),
}

#[derive(Debug)]
pub struct Lock {
    kind: LockType,
    status: LockStatus,
    lock_file: std::path::PathBuf,
}

#[macro_export]
macro_rules! err_type_mismatch_expect_dir_found_file {
    ( $v: expr ) => {
        Error::TypeMismatch {
            val: $v,
            expect: "Dir".to_string(),
            found: "File".to_string(),
        }
    };
}

pub enum HashAlgo {
    MD5,
    SHA1,
    SHA256,
    BLAKE3,
}

#[derive(
    Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd, Copy, serde::Serialize, serde::Deserialize,
)]
pub struct Uuid(::uuid::Uuid);

mod error;
mod hash;
mod lock;
pub mod tools;
mod uuid;
