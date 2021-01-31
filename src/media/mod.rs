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
    Image = 1,
    Text = 2,
    Audio = 3,
    Video = 4,
    Other = 10,
    None = 99999,
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
    pub library_uuid: super::misc::Uuid,
    pub hash: String,
    pub filename: String,
    pub filepath: String,
    pub(crate) filesize: usize,
    pub caption: Option<String>,
    pub(crate) time_add: chrono::DateTime<chrono::Local>,
    pub kind: MediaType,
    pub sub_kind: Option<String>,
    pub kind_addition: Option<String>,
    pub(crate) series_uuid: Option<super::misc::Uuid>,
    pub(crate) series_no: Option<u64>,
    pub comment: Option<String>,
    pub detail: Option<MediaDetail>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct MediaDetail {
    detail: TypesDetail,
    other: std::collections::HashMap<String, String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum TypesDetail {
    Image(ImageDetail),
    Video(VideoDetail),
    Audio(AudioDetail),
    Text(TextDetail),
    Other,
}


#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ImageDetail {
    height: u32,
    width: u32,
    format: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct TextDetail {
    words: usize,
    language: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct VideoDetail {
    height: u64,
    width: u64,
    time_len: u64,
    // In second
    frame_rates: u64,
    codec: String,
    bit_rates: u64,
    // In bit per second
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
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
            series_title: None,
            series_no: None,
        }
    }
}
