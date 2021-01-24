use super::library::*;

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
