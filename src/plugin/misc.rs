use super::*;

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Load(err) => write!(f, "Loading Library Failed due to {}", err),
            Self::Codec(err) => write!(f, "Convert to UTF8 Error due to {}", err),
        }
    }
}

impl std::fmt::Debug for dyn PluginTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl std::cmp::PartialEq for dyn PluginTrait + 'static {
    fn eq(&self, other: &Self) -> bool {
        self.name().eq(other.name())
    }
}

impl std::cmp::PartialOrd for dyn PluginTrait + 'static {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name().partial_cmp(other.name())
    }
}

impl std::cmp::Ord for dyn PluginTrait + 'static {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name().cmp(other.name())
    }
}

impl std::cmp::Eq for dyn PluginTrait + 'static {}

impl std::str::FromStr for TriggerType {
    type Err = super::super::misc::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s.to_lowercase().as_str() {
            "media_add" => Self::MediaAdd,
            "media_remove" => Self::MediaRemove,
            "media_modify" => Self::MediaModify,
            "set_add" => Self::SetAdd,
            "set_remove" => Self::SetRemove,
            "media_add_to_set" => Self::MediaAddToSet,
            "media_remove_from_set" => Self::MediaRemoveFromSet,
            "get_media" => Self::GetMedia,
            "detailize" => Self::Detailize,
            "query_media" => Self::QueryMedia,
            "query_set" => Self::QuerySet,
            "none" => Self::None,
            _ => Self::None,
        })
    }
}

impl std::fmt::Display for TriggerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
