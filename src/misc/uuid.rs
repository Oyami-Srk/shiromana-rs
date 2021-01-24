use std::fmt::Formatter;

use ::uuid::Uuid as _Uuid;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};

use super::*;

impl Uuid {
    pub fn to_string(&self) -> String {
        self.0.to_hyphenated().to_string().to_uppercase()
    }

    pub fn new_v4() -> Uuid {
        Uuid(_Uuid::new_v4())
    }

    pub fn from_str(s: &str) -> Result<Uuid> {
        Ok(Uuid(_Uuid::parse_str(s)?))
    }
}

impl ToSql for Uuid {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.to_string()))
    }
}

impl FromSql for Uuid {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value.as_str()
            .and_then(|s| match Uuid::from_str(s) {
                Ok(dt) => Ok(dt),
                Err(err) => Err(FromSqlError::Other(Box::new(err)))
            })
    }
}

impl std::fmt::Display for Uuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
