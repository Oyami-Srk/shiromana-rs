use rusqlite::params;

use super::super::misc::{Error, Result, Uuid};
use super::Library;

impl Library {
    pub fn add_tag(&mut self, id: u64, tag_uuid: &Uuid) -> Result<()> {
        let db = self.db.get()?;
        self.tag_exist_guard(tag_uuid)?;
        self.media_exist_guard(id)?;

        db.execute(
            "INSERT INTO media_tag_ref (media_id, tag_uuid) VALUES (?, ?);",
            params![id, tag_uuid],
        )?;
        db.execute(
            "UPDATE tag SET media_count = media_count + 1 WHERE uuid = ?;",
            params![tag_uuid],
        )?;
        Ok(())
    }

    pub fn remove_tag(&mut self, id: u64, tag_uuid: &Uuid) -> Result<()> {
        let db = self.db.get()?;
        self.tag_exist_guard(tag_uuid)?;
        self.media_exist_guard(id)?;

        let is_media_has_tag: bool = db.query_row(
            "SELECT EXISTS(SELECT 1 FROM media_tag_ref WHERE media_id = ? AND tag_uuid = ?);",
            params![id, tag_uuid],
            |row| Ok(row.get(0)?),
        )?;
        if !is_media_has_tag {
            return Ok(());
        }

        db.execute(
            "DELECT FROM media_tag_ref WHERE media_id = ?, AND tag_uuid = ?;",
            params![id, tag_uuid],
        )?;
        db.execute(
            "UPDATE tag SET media_count = media_count - 1 WHERE uuid =?;",
            params![tag_uuid],
        )?;

        Ok(())
    }

    // return uuid if tag is already existed
    pub fn create_tag(&mut self, caption: String, comment: Option<String>) -> Result<Uuid> {
        match self.get_tag_by_caption(&caption) {
            Ok(uuid) => return Ok(uuid), // exists
            Err(Error::NotExists(_)) => (),
            Err(e) => return Err(e),
        };

        let db = self.db.get()?;
        let uuid = Uuid::new_v4();
        db.execute(
            "INSERT INTO tag (uuid, caption, media_count, comment) VALUES (?, ?, 0, ?);",
            params![&uuid, caption, comment],
        )?;
        self.summary.tag_count += 1;
        Ok(uuid)
    }

    pub fn delete_tag(&mut self, tag_uuid: Uuid) -> Result<()> {
        self.tag_exist_guard(&tag_uuid)?;

        let db = self.db.get()?;
        db.execute(
            "DELETE FROM media_tag_ref WHERE tag_uuid = ?;",
            params![tag_uuid],
        )?;
        db.execute("DELETE FROM tag WHERE uuid = ?;", params![tag_uuid])?;
        self.summary.tag_count -= 1;
        Ok(())
    }

    // TODO: impl search tags via caption using fuzzy matching
    pub fn get_tag_by_caption(&self, caption: &str) -> Result<Uuid> {
        let tag_uuid = self
            .db
            .get()?
            .query_row(
                "SELECT uuid FROM tag WHERE caption = ?;",
                params![caption],
                |row| Ok(row.get(0)?),
            )
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => {
                    Error::NotExists(format!("Tag with caption {} not exists.", caption))
                }
                _ => Error::DB(e),
            })?;
        Ok(tag_uuid)
    }
}
