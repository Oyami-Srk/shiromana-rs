use crate::misc::Error;

use super::super::library::Library;
use super::PluginTrait;
use super::*;

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
        &mut self,
        action: TriggerType,
        media: Option<&mut Media>,
    ) -> Result<Vec<(&'static str, u32)>> {
        let media = if action.need_media() {
            if media.is_none() {
                return Err(Error::NoneError);
            }
            media.unwrap()
        } else {
            std::ptr::null_mut::<Media>()
        };
        let list: Vec<&Box<dyn PluginTrait>> = self
            .plugins
            .iter()
            .filter(|c| c.trigger().contains(&action))
            .collect();
        let mut result = vec![];
        for plugin in list {
            let ret = plugin.on_trigger(self, unsafe { media.as_mut() }, action);
            result.push((plugin.name(), ret));
        }
        /*
        .map(|e| {
            let result = if action.need_media() {
                e.on_trigger(self, Some(unsafe { media.as_mut() }.unwrap()), action)
            } else {
                e.on_trigger(self, None, action)
            };
            (e.name(), result)
        })
        .collect();*/

        Ok(result)
    }
}
