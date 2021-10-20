#![allow(dead_code)]

use rusqlite::params;

use super::super::misc::{Error, Result, Uuid};
use super::Library;

impl Library {
    pub(crate) fn is_tag_existed(&self, tag_uuid: Uuid) -> bool {
        match self.db.query_row(
            "SELECT EXISTS(SELECT 1 FROM tag WHERE uuid = ?);",
            params![tag_uuid],
            |row| Ok(row.get(0)?),
        ) {
            Ok(v) => v,
            Err(_) => false,
        }
    }

    pub(crate) fn tag_exist_guard(&self, tag_uuid: Uuid) -> Result<()> {
        if !self.is_tag_existed(tag_uuid) {
            Err(Error::NotExists(format!(
                "Tag with uuid {} not exists.",
                tag_uuid
            )))
        } else {
            Ok(())
        }
    }

    pub(crate) fn is_media_existed(&self, id: u64) -> bool {
        match self.db.query_row(
            "SELECT EXISTS(SELECT 1 FROM media WHERE id = ?);",
            params![id],
            |row| Ok(row.get(0)?),
        ) {
            Ok(v) => v,
            Err(_) => false,
        }
    }

    pub(crate) fn media_exist_guard(&self, id: u64) -> Result<()> {
        if !self.is_media_existed(id) {
            Err(Error::NotExists(format!(
                "Media with id {} not exists.",
                id
            )))
        } else {
            Ok(())
        }
    }
}
