use chrono::{DateTime, FixedOffset, Local, Utc};
use num::FromPrimitive;
use rusqlite::params;
use serde::__private::TryFrom;

use super::{Media, MediaDetail, MediaType};
use super::super::library::Library;
use super::super::misc::{Error, Result, Uuid};

impl Media {
    pub fn detailed(self) -> Media {
        self
    }
}

impl Into<u64> for Media {
    fn into(self) -> u64 {
        self.id
    }
}
