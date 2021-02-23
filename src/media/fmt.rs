use std::fmt::{Display, Formatter, Result};

use textwrap::indent;

use crate::media::{AudioDetail, ImageDetail, MediaDetail, TextDetail, TypesDetail, VideoDetail};

use super::Media;
use super::MediaType;

impl Display for MediaType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            MediaType::Image => write!(f, "Image"),
            MediaType::Text => write!(f, "Text"),
            MediaType::Audio => write!(f, "Audio"),
            MediaType::Video => write!(f, "Video"),
            MediaType::Other => write!(f, "Other"),
            MediaType::None => write!(f, "None")
        }
    }
}

impl Display for Media {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut s: String = String::new();

        s.push_str(&format!(
            "Media ID: {}\nLibrary UUID: {}\nHash: {}\nFile name: {}\nAdd Time: {}\n\
            Media Type: {}\nMedia Size: {:.2} KB\n",
            self.id, self.library_uuid, self.hash, self.filename, self.time_add,
            self.kind, self.filesize / 1024,
        ));

        if let Some(v) = &self.caption {
            s.push_str(&format!("Caption: {}\n", v));
        }
        if let Some(v) = &self.sub_kind {
            s.push_str(&format!("Sub Type: {}\n", v));
        }
        if let Some(v) = &self.kind_addition {
            s.push_str(&format!("Type Addition: {}\n", v));
        }
        if let Some(v) = &self.series_uuid {
            s.push_str(&format!("Series UUID: {}\n", v));
        }
        if let Some(v) = &self.series_no {
            s.push_str(&format!("Series No: #{}\n", v));
        }
        if let Some(v) = &self.comment {
            s.push_str(&format!("Comment: {}\n", v));
        }
        if let Some(v) = &self.detail {
            s.push_str(&format!("Details: \n{}", indent(&format!("{}", v), "    ")));
        }
        s.push_str(&format!("File path: {}", self.filepath));

        write!(f, "{}", s)
    }
}

impl Display for MediaDetail {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}\n", self.detail)?;
        write!(f, "Other details: {}", serde_json::to_string_pretty(&self.other).unwrap())
    }
}

impl Display for TypesDetail {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            TypesDetail::Image(v) => write!(f, "{}", v),
            TypesDetail::Video(v) => write!(f, "{}", v),
            TypesDetail::Audio(v) => write!(f, "{}", v),
            TypesDetail::Text(v) => write!(f, "{}", v),
            TypesDetail::Other => write!(f, "Other has no specified detail field.")
        }
    }
}

impl Display for ImageDetail {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Image Resolution: {} x {}\nImage Format: {}", self.width, self.height, self.format)
    }
}

impl Display for TextDetail {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let _ = f;
        unimplemented!()
    }
}

impl Display for AudioDetail {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let _ = f;
        unimplemented!()
    }
}

impl Display for VideoDetail {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let _ = f;
        unimplemented!()
    }
}