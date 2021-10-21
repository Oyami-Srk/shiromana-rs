use std::{fs, path, path::Path, str};

use rusqlite::params;

use super::super::media::{Media, MediaType};
use super::super::misc::{Error, Result, Uuid};
use super::{Library, LibraryFeature};
use crate::{err_type_mismatch_expect_dir_found_file, get_db_or_none};

impl Library {
    pub fn add_media(
        &mut self,
        path: String,
        kind: MediaType,
        sub_kind: Option<String>,
        kind_addition: Option<String>,
        caption: Option<String>,
        comment: Option<String>,
    ) -> Result<u64> {
        if let MediaType::URL = kind {
            return self.add_url(path, sub_kind, kind_addition, caption, comment);
        }
        let media_path = path::PathBuf::from(path);
        if !media_path.is_file() {
            return Err(err_type_mismatch_expect_dir_found_file!(media_path
                .to_str()
                .unwrap()
                .to_string()));
        }
        let file_hash = self
            .hash_algo
            .do_hash(media_path.to_str().unwrap().to_string())?;
        let file_name = media_path.file_name().unwrap().to_str().unwrap();
        let new_path = self.get_media_path_by_hash(&file_hash);
        if new_path.exists() {
            // We believe that no collision on images
            let id = self
                .query_media(&format!("hash = '{}'", file_hash))
                .map_err(|e| {
                    Error::AlreadyExists(file_hash.clone() + "," + e.to_string().as_str())
                })?
                .first()
                .map(|v| *v);
            return if let Some(id) = id {
                let _ = self.db.get()?.execute(
                    "INSERT INTO media_location_ref (media_id, path, filename) VALUES (?,?,?);",
                    params![
                        id,
                        media_path.canonicalize()?.to_str(),
                        media_path.file_stem().unwrap().to_str()
                    ],
                ); // ignore fails
                Err(Error::AlreadyExists(id.to_string()))
            } else {
                // shouldn't reach
                Err(Error::AlreadyExists(file_hash))
            };
        }
        fs::create_dir_all(new_path.parent().unwrap())?;
        fs::copy(&media_path, &new_path)?;
        let file_size = new_path.metadata()?.len();
        let db = self.db.get()?;
        db.execute(
            "INSERT INTO media (hash, filename, filesize, caption, type, sub_type, type_addition, comment)
            VALUES (?,?,?,?,?,?,?,?);",
            params![file_hash, file_name, &file_size, caption, kind, sub_kind, kind_addition, comment],
        )?;
        let id = db.last_insert_rowid() as u64;
        self.summary.media_count += 1;
        self.summary.media_size += file_size as usize;
        // insert into location ref
        let _ = db.execute(
            "INSERT INTO media_location_ref (media_id, path, filename) VALUES (?,?,?);",
            params![
                id,
                media_path.canonicalize()?.to_str(),
                media_path.file_stem().unwrap().to_str()
            ],
        ); // ignore fails
           // check features
        if self
            .features
            .contains(LibraryFeature::GenerateThumbnailAtAdding)
        {
            // generate thumbnail image at adding
            let _ = self.make_thumbnail(id);
        }
        Ok(id)
    }

    pub fn add_url(
        &mut self,
        url: String,
        sub_kind: Option<String>,
        kind_addition: Option<String>,
        caption: Option<String>,
        comment: Option<String>,
    ) -> Result<u64> {
        let hash = self.hash_algo.do_hash_str(&url)?;
        let db = self.db.get()?;
        db.execute(
            "INSERT INTO media (hash, filename, filesize, caption, type, sub_type, type_addition, comment)
            VALUES (?,?,?,?,?,?,?,?);",
            params![hash, url, 0, caption, MediaType::URL, sub_kind, kind_addition, comment],
        )?;
        let id = db.last_insert_rowid() as u64;
        self.summary.media_count += 1;
        Ok(id)
    }

    pub fn remove_media(&mut self, id: u64) -> Result<()> {
        let db = self.db.get()?;
        let (file_hash, file_size): (String, usize) = db.query_row(
            "SELECT hash, filesize FROM media WHERE id = ?;",
            params![&id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?;
        let media_file = self.get_media_path_by_hash(&file_hash);
        println!("{}", media_file.to_str().unwrap().to_string());
        if !media_file.is_file() {
            panic!("Media file is not exists or not a regular file.");
        }
        db.execute("DELETE FROM media WHERE id = ?;", params![id])?;
        fs::remove_file(&media_file)?;
        println!("Removed {:?}", media_file);
        self.summary.media_size -= file_size;
        self.summary.media_count -= 1;
        // TODO: REMOVE series and tag notation
        // TODO: REMOVE thumbnail if necessary
        // TODO: REMOVE details if necessary
        Ok(())
    }

    pub fn update_media(&mut self, media: &mut Media) -> Result<()> {
        let db = self.db.get()?;
        let is_hash_changed: bool = db.query_row(
            "SELECT hash != ? FROM media WHERE id = ?;",
            params![&media.hash, media.id],
            |row| Ok(row.get(0)?),
        )?;
        if is_hash_changed {
            // changing hash means to merge two media. if there is no media targeting changed hash
            // we fall. Btw, we tend to keep original media infos instead new one but set filename
            // to the new.
            let is_that_media_exists: bool = db.query_row(
                "SELECT EXISTS(SELECT 1 FROM media WHERE hash = ?);",
                params![&media.hash],
                |row| Ok(row.get(0)?),
            )?;
            if !is_that_media_exists {
                return Err(Error::NotExists(format!(
                    "Media with Hash {} do not exists.",
                    media.hash
                )));
            }
            fs::remove_file(&media.filepath)?;
            self.summary.media_size -= media.filesize;
            self.summary.media_count -= 1;
            let new_id: u64 = db.query_row(
                "SELECT id FROM media WHERE hash = ?;",
                params![&media.hash],
                |row| Ok(row.get(0)?),
            )?;
            let new_media = self.get_media(new_id)?;
            media.filesize = new_media.filesize;
            media.filename = new_media.filename;
            media.filepath = new_media.filepath;
            db.execute("DELETE FROM media WHERE id = ?;", params![new_media.id])?;
            // drop new media from database
        }
        if let Some(detail) = &media.detail {
            let db = self.db.get()?;
            let is_detail_exists: bool = db.query_row(
                "SELECT EXISTS(SELECT 1 FROM media_detail WHERE id = ?);",
                params![media.id],
                |row| Ok(row.get(0)?),
            )?;
            if is_detail_exists {
                db.execute(
                    "UPDATE media_detail SET details = ? WHERE id = ?;",
                    params![serde_json::to_string(detail)?, media.id],
                )?;
            } else {
                db.execute(
                    "INSERT INTO media_detail (id, details) VALUES (?, ?);",
                    params![media.id, serde_json::to_string(detail)?],
                )?;
            }
        }
        db.execute(
            "UPDATE media
                  SET hash = ?, filename = ?, filesize = ?, caption = ?, type = ?, sub_type = ?, type_addition = ?, comment = ?
                  WHERE id = ?;",
            params![media.hash, media.filename, media.filesize,
                            media.caption, media.kind, media.sub_kind,
                            media.kind_addition, media.comment, media.id],
        )?;
        Ok(())
    }

    pub fn get_media(&self, id: u64) -> Result<Media> {
        let db = self.db.get()?;
        // TODO: figure out the time spend on selecting one column and more.
        let mut media = db.query_row(
            "SELECT
                        hash, filename, filesize, caption, time_add,
                        type, sub_type, type_addition, comment
                      FROM media WHERE id = ?;",
            params![id],
            |row| {
                let hash: String = row.get(0)?;
                let file_name = row.get(1)?;
                let filepath = self.get_media_path_by_hash(&hash);
                let filepath = filepath.to_str().unwrap().to_string();
                let series_uuids: Vec<Uuid> = db
                    .prepare("SELECT series_uuid FROM media_series_ref WHERE media_id = ?;")?
                    .query_map(params![id], |row| Ok(row.get(0)?))?
                    .map(|x| x.unwrap())
                    .collect();
                let tags_uuids: Vec<Uuid> = db
                    .prepare("SELECT tag_uuid FROM media_tag_ref WHERE media_id = ?;")?
                    .query_map(params![id], |row| Ok(row.get(0)?))?
                    .map(|x| x.unwrap())
                    .collect();
                Ok(Media {
                    id,
                    library_uuid: self.uuid.clone(),
                    hash,
                    filename: file_name,
                    filepath,
                    filesize: row.get(2)?,
                    caption: row.get(3)?,
                    time_add: row.get(4)?,
                    kind: row.get(5)?,
                    sub_kind: row.get(6)?,
                    kind_addition: row.get(7)?,
                    comment: row.get(8)?,
                    series: series_uuids,
                    tag: tags_uuids,
                    detail: None,
                })
            },
        )?;

        let is_detailed: bool = db.query_row(
            "SELECT EXISTS(SELECT 1 FROM media_detail WHERE id = ?);",
            params![id],
            |row| Ok(row.get(0)?),
        )?;
        if is_detailed & &media.kind.is_some() {
            let detail: String = db.query_row(
                "SELECT details FROM media_detail WHERE id = ?;",
                params![id],
                |row| Ok(row.get(0)?),
            )?;

            media.detail = Some(serde_json::from_str(&detail)?);
        }

        Ok(media)
    }

    pub fn detailize(&self, id: u64) -> Result<()> {
        let media = self.get_media(id)?;
        media.detailize(None);
        Ok(())
    }

    pub fn get_media_by_filename(&self, filename: String) -> Result<Vec<u64>> {
        let db = self.db.get()?;
        let filename_stem = Path::new(&filename).file_stem().unwrap().to_str().unwrap();

        let mut result: Vec<u64> = vec![];
        let id_fn: Vec<(u64, String)> = db
            .prepare(
                "SELECT id, filename FROM media_location_ref WHERE filename LIKE ? ESCAPE '\\';",
            )?
            .query_map(params![filename_stem.replace("%", "\\%") + "%"], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })?
            .map(|x| x.unwrap())
            .collect();

        for v in id_fn.iter() {
            if v.1 == filename_stem || v.1 == filename {
                result.push(v.0);
            }
        }
        result.sort();
        result.dedup();
        Ok(result)
    }

    pub fn get_next_no_in_series(&self, uuid: &Uuid) -> Result<Option<u64>> {
        let db = self.db.get()?;
        let max_no: Option<u64> = db
            .prepare(
                "SELECT MAX(series_no) as max_no FROM media_series_ref WHERE series_uuid = ?;",
            )?
            .query_row(params![uuid], |row| Ok(row.get(0)?))?;
        Ok(max_no.map(|v| v + 1))
    }

    pub fn query_media(&self, sql_stmt: &str) -> Result<Vec<u64>> {
        Ok(self
            .db
            .get()?
            .prepare(&format!("SELECT id FROM media WHERE {};", sql_stmt))?
            .query_map(params![], |row| Ok(row.get(0)?))?
            .map(|x| x.unwrap())
            .collect())
    }

    pub fn get_media_hash(&self, id: u64) -> Option<String> {
        let db = get_db_or_none!(self.db);

        match db.query_row("SELECT hash FROM media WHERE id = ?;", params![id], |row| {
            Ok(row.get(0)?)
        }) {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }

    pub fn get_media_id(&self, hash: &str) -> Option<u64> {
        let db = get_db_or_none!(self.db);

        match db.query_row(
            "SELECT id FROM media WHERE hash = ?;",
            params![hash],
            |row| Ok(row.get(0)?),
        ) {
            Ok(id) => Some(id),
            Err(_) => None,
        }
    }
}

impl Library {
    // private method
    fn get_media_path_by_hash(&self, hash: &str) -> path::PathBuf {
        // this method do not promise the existence.
        path::PathBuf::new()
            .join(self.path.as_str())
            .join(&self.media_folder)
            .join(&hash[..2])
            .join(format!("{}", &hash[2..],))
    }
}
