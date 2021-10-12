use std::ffi::CStr;

use super::*;

impl PluginTrait for SharedLibrary {
    fn name(&self) -> &'static str {
        let p_str = (self.sym_name)();
        let cstr = unsafe {
            CStr::from_ptr(p_str)
        };
        cstr.to_str().unwrap()
    }

    fn trigger(&self) -> &'static str {
        todo!()
    }
}

impl SharedLibrary {
    pub fn new(path: PathBuf) -> Result<SharedLibrary> {
        let library = unsafe { libloading::Library::new(path).map_err(|e| PluginError::Load(e))? };
         move || -> std::result::Result<SharedLibrary, LibError> {
            unsafe {
                let name: Symbol<FuncName> = library.get(b"name")?;
                let name= name.into_raw();
                Ok(SharedLibrary {
                    library,
                    sym_name: name,
                })
            }
        }()
        .map_err(|e| PluginError::Load(e).into())
    }
}

