use rusqlite::params;

use super::super::misc::{Error, Result, Uuid};
use super::{Library, MediaSetType};

impl Library {
    pub fn create_set(
        &mut self,
        kind: MediaSetType,
        caption: String,
        comment: Option<String>,
    ) -> Result<Uuid> {
        let uuid = Uuid::new_v4();
        self.db.execute(
            &format!(
                "INSERT INTO {} (uuid, caption, comment, media_count) VALUES (?, ?, ?, 0);",
                match kind {
                    MediaSetType::Series => "series",
                    MediaSetType::Tag => "tags",
                }
            ),
            params![uuid, caption, comment],
        )?;
        match kind {
            MediaSetType::Series => self.summary.series_count += 1,
            MediaSetType::Tag => self.summary.tags_count += 1,
        }
        Ok(uuid)
    }

    pub fn delete_set(&mut self, kind: MediaSetType, uuid: &Uuid) -> Result<()> {
        self.db.execute(
            "DELETE FROM {} WHERE uuid = ?;",
            params![
                match kind {
                    MediaSetType::Series => "series",
                    MediaSetType::Tag => "tags",
                },
                uuid
            ],
        )?;
        self.db.execute(
            match kind {
                MediaSetType::Series => "DELETE FROM media_series_ref WHERE series_uuid = ?;",
                MediaSetType::Tag => "DELETE FROM media_tags_ref WHERE tags_uuid = ?;",
            },
            params![uuid],
        )?;
        self.db.execute(
            match kind {
                MediaSetType::Series => "DELETE FROM series WHERE uuid = ?;",
                MediaSetType::Tag => "DELETE FROM tags WHERE uuid = ?;",
            },
            params![uuid],
        )?;
        match kind {
            MediaSetType::Series => self.summary.series_count -= 1,
            MediaSetType::Tag => self.summary.tags_count -= 1,
        }
        Ok(())
    }

    // Return (Series_UUID, Tag_UUID)
    pub fn get_set_by_name(&self, caption: String) -> Result<(Option<Uuid>, Option<Uuid>)> {
        let series_uuid: Option<Uuid> = self
            .db
            .query_row(
                "SELECT uuid FROM series WHERE caption = ?;",
                params![caption],
                |row| Ok(row.get(0)?),
            )
            .unwrap_or(None);
        let tag_uuid: Option<Uuid> = self
            .db
            .query_row(
                "SELECT uuid FROM tags WHERE caption = ?;",
                params![caption],
                |row| Ok(row.get(0)?),
            )
            .unwrap_or(None);
        Ok((series_uuid, tag_uuid))
    }

    pub fn add_to_set(
        &mut self,
        kind: MediaSetType,
        id: u64,
        uuid: &Uuid,
        no: Option<u64>,
        unsorted: bool,
    ) -> Result<()> {
        match kind {
            MediaSetType::Series => {
                let mut stmt = self.db.prepare(
                    "SELECT series_no FROM media_series_ref WHERE series_uuid = ?1 AND media_id != ?2;"
                )?;
                let iter = stmt.query_map(params![uuid, id], |row| row.get(0))?;
                let to_check: Vec<u64> = iter.map(|x| x.unwrap()).collect();
                let no = if let Some(no) = no {
                    // if the no is specified.
                    if to_check.iter().any(|i| *i == no) {
                        return Err(Error::Occupied(format!(
                            "occupied when add media(id {}) to series {} with no {}",
                            id, uuid, no
                        )));
                    }
                    Some(no)
                } else {
                    // or this is a unsorted media
                    if unsorted {
                        None
                    } else {
                        // or not, we use the biggest no in the to_check list +1 to be the no
                        let biggest = to_check.iter().max();
                        Some(match biggest {
                            Some(m) => m + 1,
                            None => 1,
                        })
                    }
                };
                self.db.execute(
                    "INSERT INTO media_series_ref (media_id, series_uuid, series_no) VALUES (?, ?, ?)",
                    params![id, uuid, no],
                )?;
                self.db.execute(
                    "UPDATE series SET media_count = media_count + 1 WHERE uuid = ?;",
                    params![uuid],
                )?;
            }
            MediaSetType::Tag => {
                self.db.execute(
                    "INSERT INTO media_tags_ref (media_id, tag_uuid, series_no) VALUES (?, ?, ?)",
                    params![id, uuid, no],
                )?;
                self.db.execute(
                    "UPDATE tags SET media_count = media_count + 1 WHERE uuid = ?;",
                    params![uuid],
                )?;
            }
        }
        Ok(())
    }

    pub fn remove_from_set(&mut self, kind: MediaSetType, id: u64, uuid: &Uuid) -> Result<()> {
        self.db.execute(
            &format!(
                "DELETE FROM media_{}_ref WHERE media_id = ? AND series_uuid = ?;",
                match kind {
                    MediaSetType::Series => "series",
                    MediaSetType::Tag => "tags",
                }
            ),
            params![id, uuid],
        )?;
        self.db.execute(
            &format!(
                "UPDATE {} SET media_count = media_count - 1 WHERE uuid = ?;",
                match kind {
                    MediaSetType::Series => "series",
                    MediaSetType::Tag => "tags",
                }
            ),
            params![uuid],
        )?;
        Ok(())
    }

    pub fn update_series_no(
        &mut self,
        id: u64,
        series_uuid: &Uuid,
        no: u64,
        insert: bool,
    ) -> Result<()> {
        let mut stmt = self.db.prepare(
            "SELECT series_no FROM media_series_ref WHERE series_uuid = ?1 AND media_id != ?2;",
        )?;
        let iter = stmt.query_map(params![series_uuid, id], |row| row.get(0))?;
        let to_check: Vec<u64> = iter.map(|x| x.unwrap()).collect();
        if to_check.iter().any(|i| *i == no) {
            // insert or error
            if !insert {
                return Err(Error::Occupied(format!(
                    "occupied when add media(id {}) to series {} with no {}",
                    id, series_uuid, no
                )));
            }
            self.db.execute(
                "UPDATE media_series_ref SET series_no = series_no + 1 WHERE series_uuid = ? AND series_no >= ?;",
                params![series_uuid, no],
            )?;
        }
        self.db.execute(
            "UPDATE media_series_ref SET series_no = ? WHERE media_id = ?;",
            params![no, id],
        )?;
        Ok(())
    }

    pub fn trim_series_no(&mut self, uuid: &Uuid) -> Result<()> {
        // I sincerely recommend you not to use this function as much as possible
        let mut ids: Vec<(u64, u64)> = self
            .db
            .prepare("SELECT media_id, series_no FROM media_series_ref WHERE series_uuid = ?;")?
            .query_map(params![uuid], |row| Ok((row.get(0)?, row.get(1)?)))?
            .map(|x| x.unwrap())
            .collect();
        ids.sort_by(|a, b| a.1.cmp(&b.1));
        ids[0].1 = 1;
        for i in 1..ids.len() {
            ids[i].1 = ids[i - 1].1 + 1;
        }
        for (id, no) in ids {
            self.db.execute(
                "UPDATE media_series_ref SET series_no = ? WHERE media_id = ?;",
                params![no, id],
            )?;
        }
        Ok(())
    }

    pub fn query_series(&self, sql_stmt: &str) -> Result<Vec<Uuid>> {
        let _ = sql_stmt;
        unimplemented!()
    }
}
