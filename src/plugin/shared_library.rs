use super::*;

impl PluginTrait for SharedLibrary {
    fn name(&self) -> &'static str {
        todo!()
    }

    fn trigger(&self) -> &'static str {
        todo!()
    }
}

impl Plugin {
    pub fn from_library(path: PathBuf) -> Result<SharedLibrary> {
        let library = unsafe { libloading::Library::new(path).map_err(|e| PluginError::Load(e))? };
         move || -> std::result::Result<SharedLibrary, LibError> {
            unsafe {
                let name: Symbol<FuncName> = library.get(b"name")?;
                let name= name.into_raw();
                Ok(SharedLibrary {
                    library,
                    name: name,
                })
            }
        }()
        .map_err(|e| PluginError::Load(e).into())
    }
}

