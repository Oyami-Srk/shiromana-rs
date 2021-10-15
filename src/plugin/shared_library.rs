use super::*;
use std::str::FromStr;

impl PluginTrait for SharedLibrary {
    fn name(&self) -> &'static str {
        let p_str = (self.sym_name)();
        let cstr = unsafe { std::ffi::CStr::from_ptr(p_str) };
        cstr.to_str().unwrap()
    }

    fn trigger(&self) -> &Vec<TriggerType> {
        &self.trigger_vec
    }

    fn on_trigger(&self, lib: &mut Library, media: &mut Media, trigger_type: TriggerType) -> u32 {
        (self.sym_on_trigger)(lib, media, trigger_type.to_string().as_ptr() as *const i8)
    }

    fn on_load(&self, lib: &mut Library) -> u32 {
        (self.sym_on_load)(lib)
    }

    fn on_unload(&self, lib: &mut Library) -> u32 {
        (self.sym_on_unload)(lib)
    }
}

impl SharedLibrary {
    pub fn new(path: PathBuf) -> Result<SharedLibrary> {
        let library = unsafe { libloading::Library::new(path).map_err(|e| PluginError::Load(e))? };
        move || -> std::result::Result<SharedLibrary, LibError> {
            unsafe {
                let sym_name: Symbol<FuncName> = library.get(b"name")?;
                let sym_name = sym_name.into_raw();
                let sym_on_load: Symbol<FuncOnLoad> = library.get(b"on_load")?;
                let sym_on_load = sym_on_load.into_raw();
                let sym_on_unload: Symbol<FuncOnUnload> = library.get(b"on_unload")?;
                let sym_on_unload = sym_on_unload.into_raw();
                let sym_on_trigger: Symbol<FuncOnTrigger> = library.get(b"on_trigger")?;
                let sym_on_trigger = sym_on_trigger.into_raw();

                let trigger = library.get::<Symbol<FuncTriggerType>>(b"trigger")?;
                let trigger = trigger.into_raw();
                let p_trigger_str = (trigger)();
                let s = std::ffi::CStr::from_ptr(p_trigger_str);
                let trigger = s.to_str().map_err(|e| PluginError::Codec(e)).unwrap();
                let trigger_vec = trigger
                    .split(',')
                    .map(|s| TriggerType::from_str(s).unwrap())
                    .collect();
                println!("{:?}", trigger_vec);
                Ok(SharedLibrary {
                    library,
                    sym_name,
                    sym_on_load,
                    sym_on_unload,
                    sym_on_trigger,
                    trigger_vec,
                })
            }
        }()
        .map_err(|e| PluginError::Load(e).into())
    }
}
