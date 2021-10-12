use std::os::raw::c_char;
use std::path::PathBuf;

use super::library::Library;
use super::misc::Result;

use libloading;
use libloading::{Error as LibError, Symbol};
#[cfg(unix)]
use libloading::os::unix::Symbol as RawSymbol;
#[cfg(windows)]
use libloading::os::windows::Symbol as RawSymbol;


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
    sym_name: RawSymbol<FuncName>,
}

#[derive(Debug)]
pub enum PluginError {
    Load(LibError),
}

#[derive(Debug)]
pub struct PluginManager {
    plugins: Vec<Box<dyn PluginTrait>>,
}

pub mod misc;
pub mod plugin_manager;
pub mod shared_library;