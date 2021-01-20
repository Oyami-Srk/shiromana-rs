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
        let mut lib = match Library::create(".".to_string(), "test".to_string(), None) {
            Ok(mut v) => {
                let id = v.add_media("test/1.jpg".to_string(), MediaType::Image, None, None, None, None).expect("??");
                v.remove_media(id).expect("??");
            }
            Err(e) => {
                println!("{}", e);
                panic!("Error");
            }
        };
    }
}

