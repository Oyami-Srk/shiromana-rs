mod media;
mod fmt;

pub enum MediaUpdateKey {
    Filename,
    Caption,
    SubType,
    TypeAddition,
    Comment,
}

#[derive(num_derive::FromPrimitive, Copy, Clone)]
pub enum MediaType {
    Image = 1,
    Text = 2,
    Audio = 3,
    Video = 4,
    Other = 10,
}

impl MediaUpdateKey {
    pub fn to_key(&self) -> String {
        match self {
            MediaUpdateKey::Filename => "filename".to_string(),
            MediaUpdateKey::Caption => "caption".to_string(),
            MediaUpdateKey::SubType => "sub_type".to_string(),
            MediaUpdateKey::TypeAddition => "type_addition".to_string(),
            MediaUpdateKey::Comment => "comment".to_string()
        }
    }
}

pub struct Media<'l> {
    pub(crate) id: u64,
    pub(crate) library: &'l super::library::Library,
    pub(crate) hash: String,
    pub(crate) filename: String,
    pub(crate) filesize: usize,
    pub(crate) caption: Option<String>,
    pub(crate) time_add: chrono::DateTime<chrono::Local>,
    pub(crate) kind: MediaType,
    pub(crate) sub_kind: Option<String>,
    pub(crate) kind_addition: Option<String>,
    pub(crate) series_uuid: Option<super::misc::Uuid>,
    pub(crate) series_no: Option<u64>,
    pub(crate) comment: Option<String>,
    pub(crate) detail: Option<MediaDetail>,
}


pub struct MediaDetail {
    height: u64,
    width: u64,
    // dpi stores like x, y
    dpi: (u8, u8),
    format: String,
    tags: Vec<String>,
}
