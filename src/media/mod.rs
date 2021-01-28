mod media;
mod fmt;

pub enum MediaUpdateKey {
    Filename,
    Caption,
    SubType,
    TypeAddition,
    Comment,
}

pub enum MediaType {
    Image(Option<ImageDetail>),
    // = 1,
    Text(Option<TextDetail>),
    // = 2,
    Audio(Option<AudioDetail>),
    // = 3,
    Video(Option<VideoDetail>),
    // = 4,
    Other(Option<String>),
    // = 10,
    None,
    // = 99999, for type is not sure or not decided yet.
}

pub mod types {
    use super::MediaType;

    pub const IMAGE: MediaType = MediaType::Image(None);
    pub const TEXT: MediaType = MediaType::Text(None);
    pub const AUDIO: MediaType = MediaType::Audio(None);
    pub const VIDEO: MediaType = MediaType::Video(None);
    pub const OTHER: MediaType = MediaType::Other(None);
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

pub struct Media {
    pub(crate) id: u64,
    pub(crate) library_uuid: super::misc::Uuid,
    pub(crate) hash: String,
    pub(crate) filename: String,
    pub(crate) filepath: String,
    pub(crate) filesize: usize,
    pub(crate) caption: Option<String>,
    pub(crate) time_add: chrono::DateTime<chrono::Local>,
    pub(crate) kind: MediaType,
    pub(crate) sub_kind: Option<String>,
    pub(crate) kind_addition: Option<String>,
    pub(crate) series_uuid: Option<super::misc::Uuid>,
    pub(crate) series_no: Option<u64>,
    pub(crate) comment: Option<String>,
}


pub struct ImageDetail {
    height: u64,
    width: u64,
    // dpi stores like x, y
    dpi: (u8, u8),
    author: Option<String>,
    address: Option<String>,
}

pub struct TextDetail {
    words: usize,
    language: String,
    author: Option<String>,
    address: Option<String>,
}

pub struct VideoDetail {
    height: u64,
    width: u64,
    time_len: u64,
    // In second
    frame_rates: u64,
    codec: String,
    bit_rates: u64, // In bit per second
}

pub struct AudioDetail {
    time_len: u64,
    codec: String,
    bit_rates: u64,
}

pub struct AddingMediaParam {
    caption: Option<String>,
    kind: MediaType,
    sub_kind: Option<String>,
    kind_addition: Option<String>,
    comment: Option<String>,
    is_series: bool,
    series_title: Option<String>,
    series_no: Option<u64>,
}

impl Default for AddingMediaParam {
    fn default() -> Self {
        AddingMediaParam {
            caption: None,
            kind: MediaType::None,
            sub_kind: None,
            kind_addition: None,
            comment: None,
            is_series: false,
            series_title: None,
            series_no: None,
        }
    }
}
