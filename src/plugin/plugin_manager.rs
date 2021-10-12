use super::{PluginManager, PluginTrait};
use super::*;

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new()
        }
    }

    pub fn load<T>(&mut self, plugin: T) -> Result<()>
    where T: PluginTrait + 'static {
        self.plugins.push(Box::new(plugin));
        Ok(())
    }
}