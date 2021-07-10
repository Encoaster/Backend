//! Module for handling plugin loading

use crate::result::{Result, Error, ErrorKind};
use super::{PluginInstance, Plugin};

use libloading::{Library, Symbol};
use std::path::Path;
use std::fs;

/// Struct describing a loaded plugin
#[derive(Debug)]
pub struct LoadedPlugin {
    /// Instance of the Plugin
    plugin: Box<dyn Plugin>,
    /// The library the Plugin was loaded from
    lib:    Library
}

/// The PluginLoader itself
#[derive(Default, Debug)]
pub struct PluginLoader {
    /// The plugins that were loaded by this PluginLoader
    pub(crate) plugins: Vec<LoadedPlugin>
}

impl PluginLoader {

    /// Load all Plugins at the provided path
    /// Only files ending in `.so`, `.dll` or `.dylib` are considered for loading
    ///
    /// # Errors
    /// - If the path does not exist
    /// - If the path is not a folder
    /// - If reading the directory contents failed
    /// - If iterating over the directory contents failed
    /// - If loading an individual Plugin failed
    pub(crate) fn load_plugins(&mut self, path: &Path) -> Result<()> {
        if !path.exists() {
            return Err(Error::new(ErrorKind::PluginError, format!("Provided plugin path {:?} does not exist.", &path)));
        }

        if !path.is_dir() {
            return Err(Error::new(ErrorKind::PluginError, format!("Provided plugins path {:?} is not a directory.", &path)));
        }

        let rd = match fs::read_dir(&path) {
            Ok(rd) => rd,
            Err(e) => return Err(Error::new(ErrorKind::IoError(e), format!("Failed to read directory at path {:?}", &path)))
        };

        for dir_entry in rd {
            let de = match dir_entry {
                Ok(de) => de,
                Err(e) => return Err(Error::new(ErrorKind::IoError(e), "Failed to iterate over ReadDir".to_string()))
            };

            let path = de.path();
            if path.ends_with(".so") || path.ends_with(".dll") || path.ends_with(".dylib") {
                let plugin = self.load_plugin(&path)?;
                self.plugins.push(plugin);
            }
        }

        Ok(())
    }

    /// Load a Plugin at the provided Path
    /// The plugin must expose the `_create_plugin` symbol
    ///
    /// # Errors
    /// - When loading the library failed
    /// - When the library does not expose the `_create_plugin` symbol
    fn load_plugin(&self, path: &Path) -> Result<LoadedPlugin> {
        // SAFETY
        // This is unsafe because we're dealing with FFI, the 'other side' could be written in e.g C
        // The libraries are responsible for their own memory safety, all code on this side is safe.
        let plugin = unsafe {
            let lib = match Library::new(path) {
                Ok(l) => l,
                Err(e) => return Err(Error::new(ErrorKind::PluginError, format!("Failed to create library for plugin at {:?}: {:?}", &path, e)))
            };

            // The type returned by _create_plugin is supposed to impl Plugin
            // So we can safely cast from *mut T to *mut dyn Plugin
            // A clear example showing that this works: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=633ee58a4c267f03080e31f752d8c134
            let constructor: Symbol<'_, unsafe fn() -> *mut dyn Plugin> = match lib.get(b"_create_plugin") {
                Ok(s) => s,
                Err(_) => return Err(Error::new(ErrorKind::PluginError, format!("Plugin {:?} does not export the symbol '_create_plugin'", &path)))
            };

            let plugin_raw: *mut dyn Plugin = constructor();
            let plugin = Box::from_raw(plugin_raw);

            LoadedPlugin { plugin, lib }
        };

        let plugin_instance = PluginInstance { loader: self };
        plugin.plugin.on_load(plugin_instance);
        Ok(plugin)
    }
}

// We manually implement drop because we want to call on_unload() before dropping the Plugin
impl Drop for PluginLoader {
    fn drop(&mut self) {
        for loaded_plugin in self.plugins.drain(..) {
            loaded_plugin.plugin.on_unload();
        }
    }
}