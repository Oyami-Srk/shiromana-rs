mod guards;
mod lib_ops;
mod media_ops;
mod misc;
mod series_ops;
mod summary;
mod tag_ops;
mod thumbnail;

type SQLite = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct LibrarySummary {
    pub media_count: usize,
    pub series_count: usize,
    pub tag_count: usize,
    pub media_size: usize,
}

#[derive(Debug)]
pub struct Library {
    pub version: semver::Version,

    pub(crate) db: SQLite,
    #[allow(dead_code)]
    pub(crate) shared_db: SQLite,
    pub(crate) thumbnail_db: SQLite,

    path: String,
    pub uuid: super::misc::Uuid,
    library_name: String,
    master_name: Option<String>,
    schema: String,
    media_folder: String,
    summary: LibrarySummary,
    hash_algo: super::misc::HashAlgo,
    #[allow(dead_code)]
    lock: super::misc::Lock,
    features: LibraryFeatures,
    thread_pool: threadpool::ThreadPool,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct LibraryMetadata {
    UUID: String,
    library_name: String,
    master_name: Option<String>,
    media_folder: String,
    schema: String,
    hash_algo: String,
    summary: LibrarySummary,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum LibraryFeature {
    None,
    GenerateThumbnailAtAdding,
}

#[derive(Debug)]
pub struct LibraryFeatures {
    features: std::collections::HashSet<LibraryFeature>,
}

#[macro_export]
macro_rules! get_db_or_err {
    ( $db:expr ) => {
        $db.get()?
    };
}

#[macro_export]
macro_rules! get_db_or_none {
    ( $db:expr ) => {
        match $db.get() {
            Ok(db) => db,
            Err(_) => return None,
        }
    };
}

#[macro_export]
macro_rules! get_db_or_false {
    ( $db:expr ) => {
        match $db.get() {
            Ok(db) => db,
            Err(_) => return false,
        }
    };
}
