pub mod library;
pub mod media;
pub mod misc;

#[cfg(test)]
#[allow(dead_code, unused)]
mod tests {
    use std::collections::HashMap;
    use std::fs;
    use std::io::Write;

    use sha1::Digest;
    use uuid::Uuid;

    use crate::library::*;
    use crate::media::*;
    use crate::misc::*;

    fn insert_test_images(lib: &mut Library) -> [u64; 6] {
        let id1 = lib
            .add_media(
                "test/1.jpg".to_string(),
                MediaType::Image,
                None,
                None,
                None,
                None,
            )
            .expect("??");
        let id2 = lib
            .add_media(
                "test/2.jpg".to_string(),
                MediaType::Image,
                None,
                None,
                None,
                None,
            )
            .expect("??");
        let id3 = lib
            .add_media(
                "test/3.jpg".to_string(),
                MediaType::Image,
                None,
                None,
                None,
                None,
            )
            .expect("??");
        lib.remove_media(id2).unwrap();
        let id2 = lib
            .add_media(
                "test/2.jpg".to_string(),
                MediaType::Image,
                None,
                None,
                None,
                None,
            )
            .expect("??");
        let id4 = lib
            .add_media(
                "test/4.jpg".to_string(),
                MediaType::Image,
                None,
                None,
                None,
                None,
            )
            .expect("??");
        let id5 = lib
            .add_media(
                "test/5.gif".to_string(),
                MediaType::Image,
                None,
                None,
                None,
                None,
            )
            .expect("??");
        let id6 = lib
            .add_media(
                "test/6.jpg".to_string(),
                MediaType::Image,
                None,
                None,
                None,
                None,
            )
            .expect("??");
        [id1, id2, id3, id4, id5, id6]
    }

    // #[test]
    fn it_works() {
        let mut lib = Library::open("test.mlib".to_string()).expect("?");
        println!("{}", lib);
        let t1 = lib.get_media(1).unwrap();
        println!("{}", t1);
        let mut t1 = t1.detailize(None);
        println!("{}", t1);
        println!("{}", serde_json::to_string_pretty(&t1).unwrap());
        lib.update_media(&mut t1);
    }

    // #[test]
    fn test_get_by_name() {
        let mut lib = Library::open("test.mlib".to_string()).expect("?");
        dbg!(lib.get_media_by_filename("1.jpg".to_string()));
    }

    #[test]
    fn __it_works() {
        println!("test!");
        println!("test2!");
        fs::remove_dir_all("test.mlib");
        let lib = match Library::create(
            ".".to_string(),
            "test".to_string(),
            None,
            Some("Mass".to_string()),
            LibraryFeatures::new().with(LibraryFeature::GenerateThumbnailAtAdding),
        ) {
            Ok(mut v) => {
                let [id1, id2, id3, id4, id5, id6] = insert_test_images(&mut v);
                let series_uuid = v
                    .create_set(
                        MediaSetType::Series,
                        "test".to_string(),
                        Some("for_test".to_string()),
                    )
                    .unwrap();
                println!("Create new series with uuid: {}", series_uuid);
                v.add_to_set(MediaSetType::Series, id1, &series_uuid, Some(9), false);
                v.add_to_set(MediaSetType::Series, id2, &series_uuid, Some(2), false);
                v.add_to_set(MediaSetType::Series, id3, &series_uuid, Some(4), false);
                v.add_to_set(MediaSetType::Series, id4, &series_uuid, Some(6), false);
                v.remove_from_set(MediaSetType::Series, id2, &series_uuid);
                v.trim_series_no(&series_uuid);
                assert_eq!(v.get_next_no_in_series(&series_uuid).unwrap(), Some(4));

                v.add_url(
                    "https://doc.rust-lang.org/stable/std/index.html".into(),
                    None,
                    None,
                    None,
                    None,
                );

                println!("{}", v);
                let media1 = v.get_media(id1).unwrap();
                let media2 = v.get_media(id2).unwrap();
                println!(
                    "Media Info ( ID {} ):\n{}",
                    id1,
                    textwrap::indent(&format!("{}", media1), "    ")
                );
                println!(
                    "Media Info ( ID {} ):\n{}",
                    id2,
                    textwrap::indent(&format!("{}", media2), "    ")
                );
                if true {
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
                println!("{}", v.to_string());
            }
            Err(e) => {
                println!("{}", e);
                panic!("Error");
            }
        };
    }

    #[test]
    fn test_thumbnail() -> std::result::Result<(), crate::misc::Error> {
        fs::remove_dir_all("test.mlib");
        let mut lib = match Library::create(
            ".".to_string(),
            "test".to_string(),
            None,
            Some("Mass".to_string()),
            LibraryFeatures::new(),
        ) {
            Ok(lib) => lib,
            Err(e) => return Err(e),
        };
        let [id1, id2, id3, id4, id5, id6] = insert_test_images(&mut lib);
        println!(
            "Inserted 5 test images: {}, {}, {}, {}, {}, {}",
            id1, id2, id3, id4, id5, id6
        );
        lib.make_thumbnail(id5)?;
        let buffer = lib.get_thumbnail(id5)?;
        if let Some(buffer) = buffer {
            let mut file = std::fs::File::create("test.png")?;
            file.write(&buffer);
        }
        Ok(())
    }
}
