use std::fmt::{Display, Formatter, Result};

use textwrap::indent;

use super::Media;
use super::MediaType;

impl Display for MediaType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            MediaType::Image(v) => write!(f, "Image{}", if v.is_some() { "(Detailed)" } else { "" }),
            MediaType::Text(v) => write!(f, "Text{}", if v.is_some() { "(Detailed)" } else { "" }),
            MediaType::Audio(v) => write!(f, "Audio{}", if v.is_some() { "(Detailed)" } else { "" }),
            MediaType::Video(v) => write!(f, "Video{}", if v.is_some() { "(Detailed)" } else { "" }),
            MediaType::Other(v) => write!(f, "Other{}", if v.is_some() { "(Detailed)" } else { "" }),
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
        s.push_str(&format!("File path: {}", self.filepath));

        write!(f, "{}", s)
    }
}

