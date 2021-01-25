mod summary;
mod library;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct LibrarySummary {
    pub media_count: usize,
    pub series_count: usize,
    pub media_size: usize,
}


#[derive(Debug)]
pub struct Library {
    pub(crate) db: rusqlite::Connection,
    shared_db: rusqlite::Connection,
    path: String,
    pub(crate) uuid: super::misc::Uuid,
    library_name: String,
    master_name: Option<String>,
    schema: String,
    summary: LibrarySummary,
    hash_algo: super::misc::HashAlgo,
    lock: super::misc::Lock,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[allow(non_snake_case)]
struct LibraryMetadata {
    UUID: String,
    library_name: String,
    master_name: Option<String>,
    schema: String,
    hash_algo: String,
    summary: LibrarySummary,
}

