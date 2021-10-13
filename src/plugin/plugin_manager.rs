use super::*;
use super::{PluginManager, PluginTrait};

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn load<T>(&mut self, plugin: T) -> Result<()>
    where
        T: PluginTrait + 'static,
    {
        self.plugins.push(Box::new(plugin));
        Ok(())
    }

    pub fn unload<T>(&mut self, plugin: &T) -> Result<()>
    where
        T: PluginTrait,
    {
        let founded = self
            .plugins
            .binary_search_by_key(&plugin.name(), |p| &p.name());
        if let Ok(founded) = founded {
            self.plugins.remove(founded);
        }
        Ok(())
    }

    pub fn trigger(&self, action: TriggerType, lib: &mut Library, media: &mut Media) -> Result<()> {
        self.plugins
            .iter()
            .filter(|c| c.trigger().iter().any(|s| {
                match s {
                    action => true,
                    _ => false
                }
            }))
            .for_each(|e| {
                let result = e.on_trigger(lib, media, action);
            });
        Ok(())
    }
}
