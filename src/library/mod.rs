mod library;
mod summary;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct LibrarySummary {
    pub media_count: usize,
    pub series_count: usize,
    pub tags_count: usize,
    pub media_size: usize,
}

#[derive(Debug)]
pub struct Library {
    pub version: semver::Version,
    pub(crate) db: rusqlite::Connection,
    pub(crate) shared_db: rusqlite::Connection,
    pub(crate) thumbnail_db: rusqlite::Connection,
    path: String,
    pub uuid: super::misc::Uuid,
    library_name: String,
    master_name: Option<String>,
    schema: String,
    media_folder: String,
    summary: LibrarySummary,
    hash_algo: super::misc::HashAlgo,
    lock: super::misc::Lock,
    features: LibraryFeatures,
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

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum MediaSetType {
    Tag,
    Series,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum LibraryFeature {
    None,
}

#[derive(Debug)]
pub struct LibraryFeatures {
    features: std::collections::HashSet<LibraryFeature>,
}
