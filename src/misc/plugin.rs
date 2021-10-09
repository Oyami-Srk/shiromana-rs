use super::super::library::Library;
use super::Result;

pub trait PluginTrait {
    fn name(&self) -> &'static str;
    fn trigger(&self) -> &'static str;
}

pub struct Plugin {

}

impl PluginTrait for Plugin {
    fn name(&self) -> &'static str {
        todo!()
    }

    fn trigger(&self) -> &'static str {
        todo!()
    }
}

impl Library {
    fn attach_plugin(plugin: Plugin) -> Result<String> {
        let name = plugin.name();
        let trigger_cond = plugin.trigger();
        println!("Trying to attach a plugin with name \"{}\", triggered on \"{}\"", name, trigger_cond);
        Ok(name.to_string())
    }

    fn detach_plugin(name: String) -> Result<()> {
        Ok(())
    }
}