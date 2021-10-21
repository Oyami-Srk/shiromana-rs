use std::{
    io::{Read, Write},
    str,
};

use rusqlite::{params, DatabaseName};

use super::super::misc::{config, Error, Result};
use super::Library;
use std::sync::mpsc::{channel, Receiver};

macro_rules! unwrap_or_send_err {
    ( $result:expr, $tx:expr, $rx:expr) => {
        match $result {
            Ok(v) => v,
            Err(e) => {
                $tx.send(Err(e)).unwrap();
                return $rx;
            }
        }
    };
}

impl Library {
    fn make_thumbnail_no_check(&mut self, hash: &str) -> Receiver<Result<Vec<u8>>> {
        let (tx, rx) = channel();

        let media = self.get_media(match self.get_media_id(hash) {
            Some(id) => id,
            None => {
                tx.send(Err(Error::NotExists(format!("Media with hash {}", hash))))
                    .unwrap();
                return rx;
            }
        });
        let media = unwrap_or_send_err!(media, tx, rx);
        let thumbnail_db = self.thumbnail_db.clone();

        self.thread_pool.execute(move || {
            let result = (|| {
                let mut buffer: Vec<u8> = Vec::new();
                media.get_thumbnail(
                    &mut buffer,
                    config::THUMBNAIL_SIZE.0,
                    config::THUMBNAIL_SIZE.1,
                )?;
                let thumb_size = buffer.len();
                if thumb_size <= 0 {
                    return Err(Error::NoThumbnail);
                }
                println!(
                    "Generated thumbnail for {} with size {} bytes.",
                    media.hash, thumb_size
                );
                let thumbnail_db = thumbnail_db.get()?;
                thumbnail_db.execute(
                    "INSERT INTO thumbnail (hash, image, size) VALUES (?, ZEROBLOB(?), ?);",
                    params![media.hash, thumb_size, thumb_size],
                )?;
                let row_id = thumbnail_db.last_insert_rowid();
                println!("Thumbnail size: {} bytes.", thumb_size);
                let mut blob = thumbnail_db.blob_open(
                    DatabaseName::Main,
                    "thumbnail",
                    "image",
                    row_id,
                    false,
                )?;
                let wrote_size = blob.write(&buffer)?;
                assert_eq!(thumb_size, wrote_size); //  hope not panic
                Ok(buffer)
            })();
            let _ = tx.send(result); // drop result
        });
        rx
    }

    fn get_thumbnail_no_check(&self, hash: &str) -> Receiver<Result<Vec<u8>>> {
        let (tx, rx) = channel();
        let thumbnail_db = self.thumbnail_db.clone();
        let hash = hash.to_string();
        self.thread_pool.execute(move || {
            let result = (|| -> Result<Vec<u8>> {
                let thumbnail_db = thumbnail_db.get()?;
                let id = thumbnail_db.query_row(
                    "SELECT id FROM thumbnail WHERE hash = ?;",
                    params![hash],
                    |row| Ok(row.get(0)?),
                )?;
                let mut buffer: Vec<u8> = Vec::new();
                let mut blob =
                    thumbnail_db.blob_open(DatabaseName::Main, "thumbnail", "image", id, true)?;
                let read_size = blob.read_to_end(&mut buffer)?;
                let thumb_size: usize = thumbnail_db.query_row(
                    "SELECT size FROM thumbnail WHERE hash = ?;",
                    params![hash],
                    |row| Ok(row.get(0)?),
                )?;
                assert_eq!(read_size, thumb_size);
                Ok(buffer)
            })();
            let _ = tx.send(result); // drop result
        });
        rx
    }

    pub fn get_thumbnail(&mut self, id: u64) -> Receiver<Result<Vec<u8>>> {
        let is_thumbnailed = self.is_thumbnailed(id);
        let hash = &match self.get_media_hash(id) {
            Some(hash) => hash,
            None => panic!("Get media hash failed."),
        };
        if is_thumbnailed {
            self.get_thumbnail_no_check(hash)
        } else {
            self.make_thumbnail_no_check(hash)
        }
    }

    pub fn make_thumbnail(&mut self, id: u64) -> Receiver<Result<Vec<u8>>> {
        let is_thumbnailed = self.is_thumbnailed(id);
        let hash = &match self.get_media_hash(id) {
            Some(hash) => hash,
            None => panic!("Get media hash failed."),
        };
        println!(
            "Make thumbnail for id {}, with is_thumbnailed = {}",
            id, is_thumbnailed
        );
        if !is_thumbnailed {
            self.make_thumbnail_no_check(hash)
        } else {
            self.get_thumbnail_no_check(hash)
        }
    }

    pub fn wait_thumbnail(rx: Receiver<Result<Vec<u8>>>) -> Result<Vec<u8>> {
        rx.recv()?
    }
}
