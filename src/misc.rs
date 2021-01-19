#![macro_use]

use std::convert::TryInto;
use std::fmt::Formatter;
use std::fs;
use std::hash::Hash;
use std::io::Read;
use std::path::{Path, PathBuf};

use md5::Md5;
use serde_json;
use sha1::{Digest, Sha1};
use sha2::Sha256;

use crate::misc::HashAlgo::SHA256;

pub enum MediaType {
    Image = 1,
    Text = 2,
    Audio = 3,
    Video = 4,
    Other = 10,
}

pub mod config {
    pub const LIBRARY_EXT: &str = ".mlib";
    pub const METADATA_FN: &str = "metadata.json";
    pub const DATABASE_FN: &str = "shiromana.db";
    pub const SHARED_DATABASE_FN: &str = "shared.db";
    pub const FINGERPRINT_FN: &str = ".shiromana";
    pub const MEDIAS_FOLDER: &str = "medias";
    pub const MEDIAS_HASH_LEVEL: u32 = 1;
    // max files is only for warning
    pub const MEDIAS_FOLDER_MAX_FILES: u32 = 10000;
    pub const DEFAULT_HASH_ALGO: &str = "MD5";
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
    TypeMismatch { val: String, expect: String, found: String },
    Occupied(String),
    NotIn { a: String, b: String },
    IO(std::io::Error),
    DB(rusqlite::Error),
    LockError(LockError),
    JsonError(serde_json::Error),
    Other(String),
    NoneError,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use self::Error::*;
        match &self {
            NotExists(s) =>
                write!(f, "{} is not exists.", s),
            AlreadyExists(s) =>
                write!(f, "{} is already exists.", s),
            NotMatch(s) =>
                write!(f, "Resource \"{}\" is not match.", s),
            Occupied(s) =>
                write!(f, "Resource \"{}\" is occupied.", s),
            NotIn { a, b } => write!(f, "{} is not in {}.", a, b),
            IO(e) => write!(f, "IO Error. ({})", e),
            DB(e) => write!(f, "Database Error. ({})", e),
            Other(s) => write!(f, "Other Error: {}", s),
            TypeMismatch { val, expect, found } =>
                write!(f, "Type Mismatch for {}. Expect {}, found {}.", val, expect, found),
            Locked(s) => write!(f, "Resource \"{}\" is locked.", s),
            LockError(s) => write!(f, "{}", s),
            JsonError(e) => write!(f, "Error when processing json. {}", e),
            NoneError => write!(f, "Some values goes none.") // TODO: indicated error msg
        }
    }
}

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Self {
        Error::DB(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JsonError(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub enum LockType {
    FileLock,
    FolderLock,
}

pub enum LockStatus {
    Locked,
    Unlocked,
}

#[derive(Debug)]
pub enum LockError {
    LockFailed(String),
    UnlockFailed(String),
}

impl std::error::Error for LockError {}

impl std::fmt::Display for LockError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LockError::LockFailed(s) =>
                write!(f, "Lock for \"{}\" is failed.", s),
            LockError::UnlockFailed(s) =>
                write!(f, "Unlock for \"{}\" is failed.", s),
        }
    }
}

pub struct Lock {
    kind: LockType,
    status: LockStatus,
    lock_file: PathBuf,
}

#[macro_export]
macro_rules! err_type_mismatch_expect_dir_found_file {
    ( $v: expr)  => {
        Error::TypeMismatch{
            val: $v,
            expect: "Dir".to_string(),
            found: "File".to_string()
        }
    };
}

impl Lock {
    // FileLock is not supported yet
    pub fn acquire(kind: LockType, location: &str) -> Result<Lock> {
        let lockfile_path: PathBuf = match &kind {
            LockType::FileLock => { unimplemented!() }
            LockType::FolderLock => {
                let p = Path::new(location);
                if !p.exists() {
                    return Err(Error::NotExists(location.to_string()));
                }
                if !p.is_dir() {
                    return Err(err_type_mismatch_expect_dir_found_file!(location.to_string()));
                }
                let p = p.join(Path::new(".LOCK"));
                if p.exists() {
                    return Err(Error::Locked(location.to_string()));
                }
                p
            }
        };
        fs::write(lockfile_path.as_path(), "")?;
        Ok(Lock {
            kind,
            status: LockStatus::Locked,
            lock_file: if lockfile_path.is_relative() {
                std::env::current_dir()?.join(lockfile_path)
            } else {
                lockfile_path
            },
        })
    }

    fn release(&mut self) -> Result<()> {
        if !self.lock_file.exists() {
            return Err(Error::LockError(LockError::UnlockFailed(self.lock_file.parent().
                unwrap().to_str().unwrap().to_string())));
        }
        match self.kind {
            LockType::FileLock => unimplemented!(),
            LockType::FolderLock => {
                fs::remove_file(self.lock_file.as_path())?
            }
        }
        self.status = LockStatus::Unlocked;
        Ok(())
    }
}

impl Drop for Lock {
    fn drop(&mut self) {
        self.release().expect("Cannot release lock.");
    }
}

pub enum HashAlgo {
    MD5,
    SHA1,
    SHA256,
}

impl std::fmt::Debug for HashAlgo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HashAlgo::MD5 => write!(f, "MD5"),
            HashAlgo::SHA1 => write!(f, "SHA1"),
            HashAlgo::SHA256 => write!(f, "SHA256")
        }
    }
}

impl HashAlgo {
    pub fn to_string(&self) -> String {
        match self {
            HashAlgo::MD5 => "MD5".to_string(),
            HashAlgo::SHA1 => "SHA1".to_string(),
            HashAlgo::SHA256 => "SHA256".to_string()
        }
    }

    pub fn from_string(s: String) -> Result<HashAlgo> {
        match s.as_str() {
            "MD5" => Ok(HashAlgo::MD5),
            "SHA1" => Ok(HashAlgo::SHA1),
            "SHA256" => Ok(HashAlgo::SHA256),
            _ => return Err(Error::NotExists(format!("Hash algo not exists: {}", s)))
        }
    }


    pub fn do_hash(&self, file_path: String) -> Result<String> {
        let path = std::path::PathBuf::from(file_path);
        if !path.exists() {
            return Err(Error::NotExists(path.to_str().unwrap().to_string()));
        }
        if !path.is_file() {
            return Err(err_type_mismatch_expect_dir_found_file!(path.to_str().unwrap().to_string()));
        }
        let mut file = fs::File::open(path)?;
        Ok(match self {
            HashAlgo::MD5 => {
                let mut hasher = Md5::new();
                std::io::copy(&mut file, &mut hasher);
                format!("{:X}", hasher.finalize())
            }
            HashAlgo::SHA1 => {
                let mut hasher = Sha1::new();
                std::io::copy(&mut file, &mut hasher);
                format!("{:X}", hasher.finalize())
            }
            HashAlgo::SHA256 => {
                let mut hasher = Sha256::new();
                std::io::copy(&mut file, &mut hasher);
                format!("{:X}", hasher.finalize())
            }
        })
    }
}