use chrono::{DateTime, FixedOffset, Local, Utc};
use num::FromPrimitive;
use num_traits::{AsPrimitive, ToPrimitive};
use rusqlite::params;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use serde::__private::TryFrom;

use super::{*};
use super::super::library::Library;
use super::super::misc::{Error, Result, Uuid};

impl Media {
    pub fn detailed(self) -> Media {
        unimplemented!()
    }
}

impl Into<u64> for Media {
    fn into(self) -> u64 {
        self.id
    }
}

impl MediaType {
    pub fn get_typeid(&self) -> u32 {
        match self {
            MediaType::Image(_) => 1,
            MediaType::Text(_) => 2,
            MediaType::Audio(_) => 3,
            MediaType::Video(_) => 4,
            MediaType::Other(_) => 10,
            MediaType::None => 99999,
        }
    }
}


impl From<MediaType> for u32 {
    fn from(v: MediaType) -> Self {
        v.get_typeid()
    }
}


impl TryFrom<u32> for MediaType {
    type Error = Error;
    fn try_from(v: u32) -> Result<Self> {
        Ok(
            match v {
                1 => MediaType::Image(None),
                2 => MediaType::Image(None),
                3 => MediaType::Image(None),
                4 => MediaType::Image(None),
                10 => MediaType::Image(None),
                99999 => MediaType::Image(None),
                _ => return Err(Error::TypeMismatch { val: v.to_string(), expect: "valid type id".to_string(), found: "invalid type id".to_string() })
            })
    }
}

impl ToSql for MediaType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let v: u32 = self.get_typeid();
        Ok(ToSqlOutput::from(v))
    }
}

impl FromSql for MediaType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value.as_i64().and_then(
            |v| {
                match MediaType::try_from(v as u32) {
                    Ok(v) => Ok(v),
                    Err(err) => Err(FromSqlError::Other(Box::new(err)))
                }
            }
        )
    }
}

