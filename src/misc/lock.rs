#![macro_use]

use std::fmt::Formatter;
use std::fs;
use std::path::{Path, PathBuf};

use super::*;

impl std::error::Error for LockError {}

impl std::fmt::Display for LockError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LockError::LockFailed(s) => write!(f, "Lock for \"{}\" is failed.", s),
            LockError::UnlockFailed(s) => write!(f, "Unlock for \"{}\" is failed.", s),
        }
    }
}

impl Lock {
    // FileLock is not supported yet
    pub fn acquire(kind: LockType, location: &str) -> Result<Lock> {
        let lockfile_path: PathBuf = match &kind {
            LockType::FileLock => {
                unimplemented!()
            }
            LockType::FolderLock => {
                let p = Path::new(location);
                if !p.exists() {
                    return Err(Error::NotExists(location.to_string()));
                }
                if !p.is_dir() {
                    return Err(err_type_mismatch_expect_dir_found_file!(
                        location.to_string()
                    ));
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
            return Err(Error::LockError(LockError::UnlockFailed(
                self.lock_file
                    .parent()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            )));
        }
        match self.kind {
            LockType::FileLock => unimplemented!(),
            LockType::FolderLock => fs::remove_file(self.lock_file.as_path())?,
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
