use std::fmt;

use super::LibrarySummary;

impl fmt::Display for LibrarySummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Media count: {}\nSeries count: {}\nMedia Size: {} KB\n",
            self.media_count,
            self.series_count,
            self.media_size / 1024
        )
    }
}
