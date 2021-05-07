use std::fmt::Formatter;
use std::fs;

use blake3;
use md5::Md5;
use sha1::{Digest, Sha1};
use sha2::Sha256;

use super::*;

impl std::fmt::Debug for HashAlgo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HashAlgo::MD5 => write!(f, "MD5"),
            HashAlgo::SHA1 => write!(f, "SHA1"),
            HashAlgo::SHA256 => write!(f, "SHA256"),
            HashAlgo::BLAKE3 => write!(f, "BLAKE3"),
        }
    }
}

impl HashAlgo {
    pub fn to_string(&self) -> String {
        match self {
            HashAlgo::MD5 => "MD5".to_string(),
            HashAlgo::SHA1 => "SHA1".to_string(),
            HashAlgo::SHA256 => "SHA256".to_string(),
            HashAlgo::BLAKE3 => "BLAKE3".to_string(),
        }
    }

    pub fn from_string(s: String) -> Result<HashAlgo> {
        match s.as_str() {
            "MD5" => Ok(HashAlgo::MD5),
            "SHA1" => Ok(HashAlgo::SHA1),
            "SHA256" => Ok(HashAlgo::SHA256),
            "BLAKE3" => Ok(HashAlgo::BLAKE3),
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
                std::io::copy(&mut file, &mut hasher)?;
                format!("{:X}", hasher.finalize())
            }
            HashAlgo::SHA1 => {
                let mut hasher = Sha1::new();
                std::io::copy(&mut file, &mut hasher)?;
                format!("{:X}", hasher.finalize())
            }
            HashAlgo::SHA256 => {
                let mut hasher = Sha256::new();
                std::io::copy(&mut file, &mut hasher)?;
                format!("{:X}", hasher.finalize())
            }
            HashAlgo::BLAKE3 => {
                let mut hasher = blake3::Hasher::new();
                std::io::copy(&mut file, &mut hasher)?;
                format!("{:X}", hasher.finalize())
            }
        })
    }
}
