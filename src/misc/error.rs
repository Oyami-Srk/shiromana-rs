use std::fmt::Formatter;

use super::*;

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use self::Error::*;
        match &self {
            NotExists(s) => write!(f, "{} is not exists", s),
            AlreadyExists(s) => write!(f, "{} is already exists.", s),
            NotMatch(s) => write!(f, "Resource \"{}\" is not match.", s),
            Occupied(s) => write!(f, "Resource \"{}\" is occupied.", s),
            NotIn { a, b } => write!(f, "{} is not in {}.", a, b),
            IO(e) => write!(f, "IO Error. ({})", e),
            DB(e) => write!(f, "Database Error. ({})", e),
            DBPool(e) => write!(f, "Database Connection Pool Error. ({})", e),
            Other(s) => write!(f, "Other Error: {}", s),
            TypeMismatch { val, expect, found } => write!(
                f,
                "Type Mismatch for {}. Expect {}, found {}.",
                val, expect, found
            ),
            Locked(s) => write!(f, "Resource \"{}\" is locked.", s),
            LockError(s) => write!(f, "{}", s),
            JsonError(e) => write!(f, "Error when processing json. {}", e),
            NoneError => write!(f, "Some values goes none."), // TODO: indicated error msg
            MediaDecode(s) => write!(f, "Media decode error: {}", s),
            NoThumbnail => write!(f, "Media no Thumbnail"),
        }
    }
}

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Self {
        Error::DB(err)
    }
}

impl From<r2d2::Error> for Error {
    fn from(err: r2d2::Error) -> Self {
        Error::DBPool(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JsonError(err)
    }
}

impl From<::uuid::Error> for Error {
    fn from(err: ::uuid::Error) -> Self {
        Error::Other(format!("UUID error: {}", err))
    }
}

impl From<image::ImageError> for Error {
    fn from(err: image::ImageError) -> Self {
        Error::MediaDecode(format!("{}", err))
    }
}
