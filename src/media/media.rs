use std::collections::HashMap;

use image::ImageFormat;
use image::io::Reader as ImageReader;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use serde::__private::TryFrom;

use super::{*};
use super::super::misc::{Error, Result};

impl Media {
    pub fn detailed(self, other: HashMap<String, String>) -> Media {
        let detail = match &self.kind {
            MediaType::Image => ImageDetail::get_detail(&self.filepath),
            MediaType::Text => TextDetail::get_detail(&self.filepath),
            MediaType::Audio => AudioDetail::get_detail(&self.filepath),
            MediaType::Video => VideoDetail::get_detail(&self.filepath),
            MediaType::Other => return {
                Media {
                    detail: Some(MediaDetail {
                        detail: TypesDetail::Other,
                        other,
                    }),
                    ..self
                }
            },
            MediaType::None => return self
        };
        match &detail {
            Ok(_) => {}
            Err(e) => { println!("{}", e); }
        }
        if detail.is_err() {
            return self;
        }
        let detail = detail.unwrap();
        Media {
            detail: Some(
                MediaDetail {
                    detail,
                    other,
                }
            ),
            ..self
        }
    }
}

impl Into<u64> for Media {
    fn into(self) -> u64 {
        self.id
    }
}

impl MediaType {
    pub fn get_typeid(&self) -> u32 {
        match self {
            MediaType::Image => 1,
            MediaType::Text => 2,
            MediaType::Audio => 3,
            MediaType::Video => 4,
            MediaType::Other => 10,
            MediaType::None => 99999,
        }
    }
}

impl From<MediaType> for u32 {
    fn from(v: MediaType) -> Self {
        v.get_typeid()
    }
}


impl TryFrom<u32> for MediaType {
    type Error = Error;
    fn try_from(v: u32) -> Result<Self> {
        Ok(
            match v {
                1 => MediaType::Image,
                2 => MediaType::Text,
                3 => MediaType::Audio,
                4 => MediaType::Video,
                10 => MediaType::Other,
                99999 => MediaType::None,
                _ => return Err(Error::TypeMismatch { val: v.to_string(), expect: "valid type id".to_string(), found: "invalid type id".to_string() })
            })
    }
}

impl ToSql for MediaType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let v: u32 = self.get_typeid();
        Ok(ToSqlOutput::from(v))
    }
}

impl FromSql for MediaType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value.as_i64().and_then(
            |v| {
                match MediaType::try_from(v as u32) {
                    Ok(v) => Ok(v),
                    Err(err) => Err(FromSqlError::Other(Box::new(err)))
                }
            }
        )
    }
}

impl MediaType {
    pub fn is_some(&self) -> bool {
        match self {
            MediaType::None => false,
            _ => true
        }
    }

    pub fn is_none(&self) -> bool {
        !self.is_some()
    }
}

trait GetDetail {
    fn get_detail(media_path: &str) -> Result<TypesDetail>;
}

impl GetDetail for ImageDetail {
    fn get_detail(media_path: &str) -> Result<TypesDetail> {
        let img = ImageReader::open(media_path)?;
        let format = img.format();
        let (width, height) = img.into_dimensions()?;
        let format = match format {
            Some(v) => match v {
                ImageFormat::Png => "PNG",
                ImageFormat::Jpeg => "JPG",
                ImageFormat::Gif => "GIF",
                ImageFormat::WebP => "WEBP",
                ImageFormat::Tiff => "TIFF",
                ImageFormat::Bmp => "BMP",
                ImageFormat::Ico => "ICO",
                _ => "OTHER"
            },
            None => return Err(Error::MediaDecode("Unknown Image format.".to_string()))
        }.to_string();
        Ok(TypesDetail::Image(ImageDetail {
            width,
            height,
            format,
        }))
    }
}

impl GetDetail for TextDetail {
    fn get_detail(media_path: &str) -> Result<TypesDetail> {
        let _ = media_path;
        unimplemented!()
    }
}

impl GetDetail for AudioDetail {
    fn get_detail(media_path: &str) -> Result<TypesDetail> {
        let _ = media_path;
        unimplemented!()
    }
}

impl GetDetail for VideoDetail {
    fn get_detail(media_path: &str) -> Result<TypesDetail> {
        let _ = media_path;
        unimplemented!()
    }
}

impl std::str::FromStr for MediaType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s.to_lowercase().as_str() {
            "image" => Self::Image,
            "text" => Self::Text,
            "audio" => Self::Audio,
            "video" => Self::Video,
            "other" => Self::Other,
            "none" => Self::None,
            _ => Self::Other
        })
    }
}