#![allow(dead_code)]

use rusqlite::params;

use super::super::misc::{Error, Result, Uuid};
use super::Library;
use crate::get_db_or_false;

impl Library {
    pub(crate) fn is_tag_existed(&self, tag_uuid: &Uuid) -> bool {
        let db = get_db_or_false!(self.db);
        match db.query_row(
            "SELECT EXISTS(SELECT 1 FROM tag WHERE uuid = ?);",
            params![tag_uuid],
            |row| Ok(row.get(0)?),
        ) {
            Ok(v) => v,
            Err(_) => false,
        }
    }

    pub(crate) fn tag_exist_guard(&self, tag_uuid: &Uuid) -> Result<()> {
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
        let db = get_db_or_false!(self.db);
        match db.query_row(
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

    pub(crate) fn is_thumbnailed(&self, id: u64) -> bool {
        let hash = match self.get_media_hash(id) {
            Some(hash) => hash,
            None => return false,
        };
        let thumbnail_db = get_db_or_false!(self.thumbnail_db);
        match thumbnail_db.query_row(
            "SELECT EXISTS(SELECT 1 FROM thumbnail WHERE hash = ?);",
            params![hash],
            |row| Ok(row.get(0)?),
        ) {
            Ok(v) => v,
            Err(_) => false,
        }
    }
}
