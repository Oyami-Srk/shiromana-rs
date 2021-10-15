use super::*;
use super::{PluginTrait};
use super::super::library::Library;

impl Library {
    pub fn load_plugin<T>(&mut self, plugin: T) -> Result<()>
    where
        T: PluginTrait + 'static,
    {
        plugin.on_load(self);
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
            plugin.on_unload(self);
            self.plugins.remove(founded);
        }
        Ok(())
    }

    fn trigger(
        &self,
        action: TriggerType,
        lib: &mut Library,
        media: Option<&mut Media>,
    ) -> Result<Vec<(&'static str, u32)>> {
        let result = self
            .plugins
            .iter()
            .filter(|c| c.trigger().contains(&action))
            .map(|e| {
                let result = e.on_trigger(lib, &media, action);
                (e.name(), result)
            })
            .collect();
        Ok(result)
    }
}
