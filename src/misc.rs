use std::fmt::Formatter;

pub enum MediaType {
    Image = 1,
    Text = 2,
    Audio = 3,
    Video = 4,
    Other = 10,
}

pub mod config {
    pub const LIBRARY_EXT: &str = ".mlib";
    pub const METADATA_FN: &str = "metadata.json";
    pub const DATABASE_FN: &str = "shiromana.db";
    pub const SHARED_DATABASE_FN: &str = "shared.db";
    pub const FINGERPRINT_FN: &str = ".shiromana";
    pub const MEDIAS_FOLDER: &str = "medias";
    pub const MEDIAS_HASH_LEVEL: u32 = 1;
    // max files is only for warning
    pub const MEDIAS_FOLDER_MAX_FILES: u32 = 10000;
    pub const HASH_ALGO: &str = "MD5";
    pub const LOCKFILE: &str = ".LOCK";
}

#[derive(Debug)]
pub enum Error<'s> {
    NotExists(Option<String>),
    // file or dir
    AlreadyExists(Option<String>),
    // file or dir
    Locked(Option<String>),
    CannotAcquireLock(Option<String>),
    NotMatch(Option<String>),
    Occupied(Option<String>),
    NotIn(Option<(String, String)>),
    IO(std::io::Error),
    DB(rusqlite::Error),
    Other(&'s str),
}

impl std::error::Error for Error<'_> {}

impl std::fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use self::Error::*;
        match &self {
            NotExists(s) =>
                write!(f, "Not exists. ({})", s.as_ref().unwrap_or(&"Not given".to_string())),
            AlreadyExists(s) =>
                write!(f, "Already exists. ({})", s.as_ref().unwrap_or(&"Not given".to_string())),
            Locked(s) =>
                write!(f, "Resource Locked. ({})", s.as_ref().unwrap_or(&"Not given".to_string())),
            CannotAcquireLock(s) =>
                write!(f, "Cannot Acquire Lock. ({})", s.as_ref().unwrap_or(&"Not given".to_string())),
            NotMatch(s) =>
                write!(f, "Resource Not Match. ({})", s.as_ref().unwrap_or(&"Not given".to_string())),
            Occupied(s) =>
                write!(f, "Resource Occupied. ({})", s.as_ref().unwrap_or(&"Not given".to_string())),
            NotIn(t) =>
                match t {
                    Some(t) => write!(f, "{} is not in {}", t.0, t.1),
                    None => write!(f, "Resource is not In some condition for this.")
                },
            IO(e) => write!(f, "IO Error. ({})", e),
            DB(e) => write!(f, "Database Error. ({})", e),
            Other(s) => write!(f, "Other Error: {}", s)
        }
    }
}

impl From<rusqlite::Error> for Error<'_> {
    fn from(err: rusqlite::Error) -> Self {
        Error::DB(err)
    }
}

impl From<std::io::Error> for Error<'_> {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err)
    }
}

pub type Result<'a, T> = std::result::Result<T, Error<'a>>;
