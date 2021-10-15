use super::*;
use super::{PluginManager, PluginTrait};

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn load<T>(&mut self, lib: &mut Library, plugin: T) -> Result<()>
    where
        T: PluginTrait + 'static,
    {
        plugin.on_load(lib);
        self.plugins.push(Box::new(plugin));
        Ok(())
    }

    pub fn unload<T>(&mut self, lib: &mut Library, plugin: &T) -> Result<()>
    where
        T: PluginTrait,
    {
        let founded = self
            .plugins
            .binary_search_by_key(&plugin.name(), |p| &p.name());
        if let Ok(founded) = founded {
            plugin.on_unload(lib);
            self.plugins.remove(founded);
        }
        Ok(())
    }

    pub fn trigger(
        &self,
        action: TriggerType,
        lib: &mut Library,
        media: &mut Media,
    ) -> Result<Vec<(&'static str, u32)>> {
        let result = self
            .plugins
            .iter()
            .filter(|c| c.trigger().contains(&action))
            .map(|e| {
                let result = e.on_trigger(lib, media, action);
                (e.name(), result)
            })
            .collect();
        Ok(result)
    }
}

impl Library {
    pub fn plugin_load<T>(&mut self, plugin: T)
    where
        T: PluginTrait + 'static,
    {
        self.plugin_manager.load(&mut self, plugin);
    }
}
