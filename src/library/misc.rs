use super::super::misc::{Error, Result};
use super::{Library, LibraryFeature, LibraryFeatures};
use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

impl FromStr for LibraryFeature {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "none" => Self::None,
            "generate_thumbnail_at_adding" => Self::GenerateThumbnailAtAdding,
            _ => Self::None,
        })
    }
}

impl std::fmt::Display for LibraryFeature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "None",
                Self::GenerateThumbnailAtAdding => "generate_thumbnail_at_adding",
            }
        )
    }
}

impl std::str::FromStr for LibraryFeatures {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let features = s
            .split(',')
            .map(|s| {
                LibraryFeature::from_str(s).expect(format!("Unsupported features: {}", s).as_str())
            })
            .collect();
        Ok(Self { features })
    }
}

impl std::fmt::Display for LibraryFeatures {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.features
                .iter()
                .map(|v| v.to_string())
                .filter(|x| x != "")
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl LibraryFeatures {
    pub fn new() -> Self {
        Self {
            features: HashSet::new(),
        }
    }

    pub fn add(&mut self, feature: LibraryFeature) {
        self.features.insert(feature);
    }

    pub fn remove(&mut self, feature: LibraryFeature) {
        self.features.remove(&feature);
    }

    pub fn contains(&self, feature: LibraryFeature) -> bool {
        self.features.contains(&feature)
    }

    pub fn with(self, feature: LibraryFeature) -> Self {
        let mut p = Self { ..self };
        p.add(feature);
        p
    }
}
