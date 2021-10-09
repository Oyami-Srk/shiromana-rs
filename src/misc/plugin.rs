use std::path::PathBuf;

use super::super::library::Library;
use super::Result;
use libloading;

#[cfg(unix)]
use libloading::os::unix::Symbol as RawSymbol;
#[cfg(windows)]
use libloading::os::windows::Symbol as RawSymbol;

use libloading::{Error as LibError, Symbol};

use std::os::raw::c_char;

pub trait PluginTrait {
    fn name(&self) -> &'static str;
    fn trigger(&self) -> &'static str;
}

pub struct Plugin {}

type FuncName = extern "C" fn() -> *const c_char;
type FuncTriggerType = extern "C" fn() -> *const c_char;
type FuncOnLoad = extern "C" fn(*mut Library) -> u32;
type FuncOnUnload = extern "C" fn(*mut Library) -> u32;

pub struct SharedLibrary {
    library: libloading::Library,
    name: RawSymbol<FuncName>,
}

#[derive(Debug)]
pub enum PluginError {
    Load(LibError),
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Load(err) => write!(f, "Loading Library Failed due to {}", err),
        }
    }
}

impl PluginTrait for Plugin {
    fn name(&self) -> &'static str {
        todo!()
    }

    fn trigger(&self) -> &'static str {
        todo!()
    }
}

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

pub struct PluginManager {
    plugins: Vec<Box<dyn PluginTrait>>,
}

impl PluginManager {}
