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
            _ => return Err(Error::NotExists(format!("Hash algo not exists: {}", s))),
        }
    }

    fn do_hash_for_reader<R: ?Sized>(algo: &HashAlgo, reader: &mut R) -> Result<String>
    where
        R: std::io::Read,
    {
        Ok(match algo {
            HashAlgo::MD5 => {
                let mut hasher = Md5::new();
                std::io::copy(reader, &mut hasher)?;
                format!("{:X}", hasher.finalize())
            }
            HashAlgo::SHA1 => {
                let mut hasher = Sha1::new();
                std::io::copy(reader, &mut hasher)?;
                format!("{:X}", hasher.finalize())
            }
            HashAlgo::SHA256 => {
                let mut hasher = Sha256::new();
                std::io::copy(reader, &mut hasher)?;
                format!("{:X}", hasher.finalize())
            }
            HashAlgo::BLAKE3 => {
                let mut hasher = blake3::Hasher::new();
                std::io::copy(reader, &mut hasher)?;
                format!("{}", hasher.finalize().to_hex().to_uppercase())
            }
        })
    }

    pub fn do_hash(&self, file_path: String) -> Result<String> {
        let path = std::path::PathBuf::from(file_path);
        if !path.exists() {
            return Err(Error::NotExists(path.to_str().unwrap().to_string()));
        }
        if !path.is_file() {
            return Err(err_type_mismatch_expect_dir_found_file!(path
                .to_str()
                .unwrap()
                .to_string()));
        }
        let mut file = fs::File::open(path)?;
        Self::do_hash_for_reader(self, &mut file)
    }

    pub fn do_hash_str(&self, s: &str) -> Result<String> {
        let mut s: &[u8] = s.as_bytes();
        Self::do_hash_for_reader(self, &mut s)
    }

    pub fn get_size(&self) -> usize {
        match self {
            HashAlgo::MD5 => Md5::output_size(),
            HashAlgo::SHA1 => Sha1::output_size(),
            HashAlgo::SHA256 => Sha256::output_size(),
            HashAlgo::BLAKE3 => blake3::OUT_LEN,
        }
    }
}
