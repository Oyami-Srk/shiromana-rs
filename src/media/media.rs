use chrono::{DateTime, FixedOffset, Local, Utc};
use num::FromPrimitive;
use rusqlite::params;
use serde::__private::TryFrom;

use super::{Media, MediaDetail, MediaType};
use super::super::library::Library;
use super::super::misc::{Error, Result, Uuid};

impl Media<'_> {
    pub fn from_id(lib: &Library, id: u64) -> Result<Media> {
        // TODO: figure out the time spend on selecting one column and more.
        let
            (hash, filename, filesize, caption, time_add, kind, sub_kind, kind_addition, series_uuid, series_no, comment):
            (String, String, usize, Option<String>, DateTime<Local>, u64, Option<String>, Option<String>, Option<String>, Option<u64>, Option<String>) =
            lib.db.query_row(
                "SELECT
                        hash, filename, filesize, caption, time_add,
                        type, sub_type, type_addition, series_uuid,
                        series_no, comment
                      FROM media WHERE id = ?;",
                params![id],
                |row| Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    row.get(6)?,
                    row.get(7)?,
                    row.get(8)?,
                    row.get(9)?,
                    row.get(10)?,
                )),
            )?;

        let kind: MediaType = match FromPrimitive::from_u64(kind) {
            Some(v) => v,
            None => return Err(Error::NotExists(format!("MediaType ID {}", kind)))
        };

        let series_uuid = match series_uuid {
            Some(v) => Some(Uuid::from_str(v.as_str())?),
            None => None
        };


        Ok(Media {
            id,
            library: lib,
            hash,
            filename,
            filesize,
            caption,
            time_add,
            kind,
            sub_kind,
            kind_addition,
            series_uuid,
            series_no,
            comment,
            detail: None,
        })
    }
}

impl Into<u64> for Media<'_> {
    fn into(self) -> u64 {
        self.id
    }
}
