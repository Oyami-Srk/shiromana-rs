use super::PluginManager;

/*
impl std::fmt::Debug for PluginManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PluginManager")
        .field("plugins", self.plugins.iter().map(
            |b| b.name())
        ).finish()
    }
}
*/

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new()
        }
    }
}