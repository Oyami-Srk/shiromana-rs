use std::os::raw::c_char;
use std::path::PathBuf;

use super::library::Library;
use super::media::Media;
use super::misc::Result;

use libloading;
#[cfg(unix)]
use libloading::os::unix::Symbol as RawSymbol;
#[cfg(windows)]
use libloading::os::windows::Symbol as RawSymbol;
use libloading::{Error as LibError, Symbol};

pub trait PluginTrait {
    fn name(&self) -> &'static str;
    fn trigger(&self) -> &Vec<TriggerType>;
    fn on_trigger(&self, lib: &mut Library, media: &mut Media, trigger_type: TriggerType) -> u32;
    fn on_load(&self, lib: &mut Library) -> u32;
    fn on_unload(&self, lib: &mut Library) -> u32;
}

pub struct Plugin {}

type FuncName = extern "C" fn() -> *const c_char;
type FuncTriggerType = extern "C" fn() -> *const c_char;
type FuncOnLoad = extern "C" fn(*mut Library) -> u32;
type FuncOnUnload = extern "C" fn(*mut Library) -> u32;
type FuncOnTrigger = extern "C" fn(*mut Library, *mut Media, *const c_char) -> u32;

pub struct SharedLibrary {
    #[allow(unused)]
    library: libloading::Library,
    sym_name: RawSymbol<FuncName>,
    sym_on_trigger: RawSymbol<FuncOnTrigger>,
    sym_on_load: RawSymbol<FuncOnLoad>,
    sym_on_unload: RawSymbol<FuncOnUnload>,
    trigger_vec: Vec<TriggerType>,
}

#[derive(Debug)]
pub enum PluginError {
    Load(LibError),
    Codec(std::str::Utf8Error),
}

#[derive(Debug)]
pub struct PluginManager {
    plugins: Vec<Box<dyn PluginTrait>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TriggerType {
    None,
    MediaAdd,
    MediaRemove,
    MediaModify,
    SetAdd,
    SetRemove,
    MediaAddToSet,
    MediaRemoveFromSet,
    GetMedia,
    Detailize,
    QueryMedia,
    QuerySet,
}

pub mod misc;
pub mod plugin_manager;
pub mod shared_library;
