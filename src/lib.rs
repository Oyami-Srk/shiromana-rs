#![allow(dead_code, unused)]

use uuid::Uuid;

mod misc;
mod library;
mod media;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Read;

    use sha1::{Digest, Sha1};
    use uuid::Uuid;

    use crate::library::*;
    use crate::media::*;
    use crate::misc::*;

    #[test]
    fn it_works() {
        fs::remove_dir_all("test.mlib");
        let mut lib = match Library::create(".".to_string(), "test".to_string(), None) {
            Ok(mut v) => {
                let id1 = v.add_media("test/1.jpg".to_string(), MediaType::Image, None, None, None, None).expect("??");
                let id2 = v.add_media("test/2.jpg".to_string(), MediaType::Image, None, None, None, None).expect("??");
                let id3 = v.add_media("test/3.jpg".to_string(), MediaType::Image, None, None, None, None).expect("??");
                v.remove_media(id2);
                let id2 = v.add_media("test/2.jpg".to_string(), MediaType::Image, None, None, None, None).expect("??");
                let id4 = v.add_media("test/4.jpg".to_string(), MediaType::Image, None, None, None, None).expect("??");
                let id5 = v.add_media("test/5.jpg".to_string(), MediaType::Image, None, None, None, None).expect("??");
                let series_uuid = v.create_series(Some("test".to_string()), Some("for_test".to_string())).unwrap();
                println!("Create new series with uuid: {}", series_uuid);
                v.add_to_series(id1, &series_uuid, 9);
                v.add_to_series(id2, &series_uuid, 2);
                v.add_to_series(id3, &series_uuid, 4);
                v.add_to_series(id4, &series_uuid, 6);
                v.update_media(id1, MediaUpdateKey::Comment, "Test".to_string()).expect("??");
                v.remove_from_series(id2);
                v.trim_series_no(&series_uuid);

                println!("{}", v);
            }
            Err(e) => {
                println!("{}", e);
                panic!("Error");
            }
        };
    }
}

