#![allow(dead_code, unused)]

use uuid::Uuid;

mod misc;
mod library;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Read;

    use sha1::{Digest, Sha1};
    use uuid::Uuid;

    use crate::library::{Library, LibrarySummary};
    use crate::misc::*;

    #[test]
    fn it_works() {
        fs::remove_dir_all("test.mlib");
        /*
        let lib = match Library::create(".".to_string(), "test".to_string(), None) {
            Ok(v) => v,
            Err(e) => {
                println!("{}", e);
                panic!("Error");
            }
        };*/
        println!("SHA1:{}\nMD5:{}\nSHA256:{}",
                 HashAlgo::SHA1.do_hash("test/1.jpg".to_string()).unwrap(),
                 HashAlgo::MD5.do_hash("test/1.jpg".to_string()).unwrap(),
                 HashAlgo::SHA256.do_hash("test/1.jpg".to_string()).unwrap()
        );
    }
}

