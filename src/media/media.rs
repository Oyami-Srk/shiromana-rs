use std::borrow::Borrow;
use std::collections::HashMap;
use std::io::BufRead;

use image::io::Reader as ImageReader;
use image::ImageFormat;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use serde::__private::TryFrom;

use super::super::misc::{Error, Result};
use super::*;

impl Media {
    pub fn detailize(self, other: Option<HashMap<String, String>>) -> Media {
        let other = other.unwrap_or(HashMap::new());
        let detail = match &self.kind {
            MediaType::Image => ImageDetail::get_detail(&self.filepath),
            MediaType::Text => TextDetail::get_detail(&self.filepath),
            MediaType::Audio => AudioDetail::get_detail(&self.filepath),
            MediaType::Video => VideoDetail::get_detail(&self.filepath),
            MediaType::URL => URLDetail::get_detail(&self.filepath),
            MediaType::Other => {
                return {
                    Media {
                        detail: Some(MediaDetail {
                            detail: TypesDetail::Other,
                            other,
                        }),
                        ..self
                    }
                }
            }
            MediaType::None => return self,
        };
        match &detail {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e);
            }
        }
        if detail.is_err() {
            return self;
        }
        let detail = detail.unwrap();
        Media {
            detail: Some(MediaDetail { detail, other }),
            ..self
        }
    }

    pub fn get_thumbnail<W>(&self, image: &mut W, width: u32, height: u32) -> Result<()>
    where
        W: std::io::Write,
    {
        match self.kind {
            MediaType::Image => ImageDetail::get_thumbnail(&self.filepath, image, width, height),
            _ => Err(Error::NoThumbnail),
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
            MediaType::URL => 5,
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
        Ok(match v {
            1 => MediaType::Image,
            2 => MediaType::Text,
            3 => MediaType::Audio,
            4 => MediaType::Video,
            5 => MediaType::URL,
            10 => MediaType::Other,
            99999 => MediaType::None,
            _ => {
                return Err(Error::TypeMismatch {
                    val: v.to_string(),
                    expect: "valid type id".to_string(),
                    found: "invalid type id".to_string(),
                })
            }
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
        value
            .as_i64()
            .and_then(|v| match MediaType::try_from(v as u32) {
                Ok(v) => Ok(v),
                Err(err) => Err(FromSqlError::Other(Box::new(err))),
            })
    }
}

impl MediaType {
    pub fn is_some(&self) -> bool {
        match self {
            MediaType::None => false,
            _ => true,
        }
    }

    pub fn is_none(&self) -> bool {
        !self.is_some()
    }
}

trait Detailize {
    fn get_detail(media_path: &str) -> Result<TypesDetail>;
    fn get_thumbnail<W>(media_path: &str, image: &mut W, width: u32, height: u32) -> Result<()>
    where
        W: std::io::Write;
}

impl Detailize for ImageDetail {
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
                _ => "OTHER",
            },
            None => return Err(Error::MediaDecode("Unknown Image format.".to_string())),
        }
        .to_string();
        Ok(TypesDetail::Image(ImageDetail {
            width,
            height,
            format,
        }))
    }

    fn get_thumbnail<W>(media_path: &str, image: &mut W, width: u32, height: u32) -> Result<()>
    where
        W: std::io::Write,
    {
        let f = std::fs::File::open(media_path)?;
        let mut buffer_reader = std::io::BufReader::with_capacity(16, f);
        buffer_reader.fill_buf()?;
        let img_format = image::guess_format(buffer_reader.buffer())?;
        let mut img = ImageReader::new(buffer_reader);
        img.set_format(img_format);
        let img = img.decode()?;
        // TODO: Speed up thumbnailization
        let thumb = img.thumbnail(width, height);
        // thumb.write_to(image, ImageFormat::Png)?;
        thumb.write_to(image, ImageFormat::Jpeg)?;
        Ok(())
    }
}

impl Detailize for TextDetail {
    fn get_detail(media_path: &str) -> Result<TypesDetail> {
        let _ = media_path;
        unimplemented!()
    }

    fn get_thumbnail<W>(media_path: &str, image: &mut W, width: u32, height: u32) -> Result<()>
    where
        W: std::io::Write,
    {
        let _ = media_path;
        let _ = image;
        let _ = width;
        let _ = height;
        unimplemented!();
    }
}

impl Detailize for AudioDetail {
    fn get_detail(media_path: &str) -> Result<TypesDetail> {
        let _ = media_path;
        unimplemented!()
    }

    fn get_thumbnail<W>(media_path: &str, image: &mut W, width: u32, height: u32) -> Result<()>
    where
        W: std::io::Write,
    {
        let _ = media_path;
        let _ = image;
        let _ = width;
        let _ = height;
        unimplemented!();
    }
}

impl Detailize for VideoDetail {
    fn get_detail(media_path: &str) -> Result<TypesDetail> {
        let _ = media_path;
        unimplemented!()
    }

    fn get_thumbnail<W>(media_path: &str, image: &mut W, width: u32, height: u32) -> Result<()>
    where
        W: std::io::Write,
    {
        let _ = media_path;
        let _ = image;
        let _ = width;
        let _ = height;
        unimplemented!();
    }
}

impl Detailize for URLDetail {
    fn get_detail(media_path: &str) -> Result<TypesDetail> {
        let _ = media_path;
        unimplemented!()
    }

    fn get_thumbnail<W>(media_path: &str, image: &mut W, width: u32, height: u32) -> Result<()>
    where
        W: std::io::Write,
    {
        let _ = media_path;
        let _ = image;
        let _ = width;
        let _ = height;
        unimplemented!();
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
            "url" => Self::URL,
            "other" => Self::Other,
            "none" => Self::None,
            _ => Self::Other,
        })
    }
}
