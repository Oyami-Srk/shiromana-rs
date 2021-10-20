use rusqlite::params;

use super::super::misc::{Error, Result, Uuid};
use super::Library;

impl Library {
    pub fn create_series(&mut self, caption: String, comment: Option<String>) -> Result<Uuid> {
        let db = self.db.get()?;
        let uuid = Uuid::new_v4();
        db.execute(
            "INSERT INTO series (uuid, caption, comment, media_count) VALUES (?, ?, ?, 0);",
            params![uuid, caption, comment],
        )?;
        self.summary.series_count += 1;
        Ok(uuid)
    }

    pub fn delete_series(&mut self, uuid: &Uuid) -> Result<()> {
        let db = self.db.get()?;
        db.execute(
            "DELETE FROM media_series_ref WHERE series_uuid = ?;",
            params![uuid],
        )?;
        db.execute("DELETE FROM series WHERE uuid = ?;", params![uuid])?;
        self.summary.series_count -= 1;
        Ok(())
    }

    // Return (Series_UUID, Tag_UUID)
    pub fn get_series_by_name(&self, caption: String) -> Result<(Option<Uuid>, Option<Uuid>)> {
        let db = self.db.get()?;
        let series_uuid: Option<Uuid> = db
            .query_row(
                "SELECT uuid FROM series WHERE caption = ?;",
                params![caption],
                |row| Ok(row.get(0)?),
            )
            .unwrap_or(None);
        let tag_uuid: Option<Uuid> = db
            .query_row(
                "SELECT uuid FROM tags WHERE caption = ?;",
                params![caption],
                |row| Ok(row.get(0)?),
            )
            .unwrap_or(None);
        Ok((series_uuid, tag_uuid))
    }

    pub fn add_to_series(
        &mut self,
        id: u64,
        uuid: &Uuid,
        no: Option<u64>,
        unsorted: bool,
    ) -> Result<()> {
        let db = self.db.get()?;
        let mut stmt = db.prepare(
            "SELECT series_no FROM media_series_ref WHERE series_uuid = ?1 AND media_id != ?2;",
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
        db.execute(
            "INSERT INTO media_series_ref (media_id, series_uuid, series_no) VALUES (?, ?, ?)",
            params![id, uuid, no],
        )?;
        db.execute(
            "UPDATE series SET media_count = media_count + 1 WHERE uuid = ?;",
            params![uuid],
        )?;
        Ok(())
    }

    pub fn remove_from_series(&mut self, id: u64, uuid: &Uuid) -> Result<()> {
        let db = self.db.get()?;
        db.execute(
            "DELETE FROM media_series_ref WHERE media_id = ? AND series_uuid = ?;",
            params![id, uuid],
        )?;
        db.execute(
            "UPDATE series SET media_count = media_count - 1 WHERE uuid = ?;",
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
        let db = self.db.get()?;
        let mut stmt = db.prepare(
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
            db.execute(
                "UPDATE media_series_ref SET series_no = series_no + 1 WHERE series_uuid = ? AND series_no >= ?;",
                params![series_uuid, no],
            )?;
        }
        db.execute(
            "UPDATE media_series_ref SET series_no = ? WHERE media_id = ?;",
            params![no, id],
        )?;
        Ok(())
    }

    pub fn trim_series_no(&mut self, uuid: &Uuid) -> Result<()> {
        // I sincerely recommend you not to use this function as much as possible
        let db = self.db.get()?;
        let mut ids: Vec<(u64, u64)> = db
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
            db.execute(
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
