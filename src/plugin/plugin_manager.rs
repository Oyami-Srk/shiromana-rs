use super::PluginManager;

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new()
        }
    }
}