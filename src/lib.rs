#![allow(dead_code, unused)]

use uuid::Uuid;

mod misc;
mod library;
mod media;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Read;
    use std::thread;

    use sha1::{Digest, Sha1};
    use uuid::Uuid;

    use crate::library::*;
    use crate::media::*;
    use crate::misc::*;

    #[test]
    fn it_works() {
        fs::remove_dir_all("test.mlib");
        let mut lib = match Library::create(".".to_string(), "test".to_string(), None, Some("Mass".to_string())) {
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
                let media1 = v.get_media(id1).unwrap();
                let media2 = v.get_media(id2).unwrap();
                println!("Media Info ( ID {} ):\n{}", id1, textwrap::indent(&format!("{}", media1), "    "));
                println!("Media Info ( ID {} ):\n{}", id2, textwrap::indent(&format!("{}", media2), "    "));
                println!("Trying to adding huge amount of files.");
                let begin = chrono::Local::now();
                let files = fs::read_dir("test/Fatkun").unwrap();
                for f in files {
                    let adding = || {
                        let f = f.unwrap().path().to_str().unwrap().to_string();
                        print!("Adding {} ...", &f);
                        match v.add_media(f.clone(), MediaType::Image, None, None, None, None) {
                            Err(e) => println!("Error when adding {}: {}", f, e),
                            Ok(_) => println!("Done"),
                        }
                    };
                    adding();
                }
                let end = chrono::Local::now();
                println!("Time usage: {}", end - begin);
            }
            Err(e) => {
                println!("{}", e);
                panic!("Error");
            }
        };
    }
}

