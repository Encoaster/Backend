//! Module describing the core of the Plugin system

use super::plugin::loader::PluginLoader;
use database::Database;

use std::fmt::Debug;

pub(crate) mod loader;
pub mod server;
pub mod database;

/// Trait describing a Plugin and it's required functions
pub trait Plugin: Debug + Sync + Send {
    /// Function called when the Plugin is loaded
    fn on_load(&self, instance: PluginInstance<'_>);

    /// Function called when the Plugin is unloaded
    fn on_unload(&self);
}

/// An instance of a Plugin
pub struct PluginInstance<'loader> {
    /// Reference to the PluginLoader that created this PluginInstance
    loader: &'loader PluginLoader,
}

impl PluginInstance<'_> {

    /// Register a Database backend with the Server
    pub fn register_database_backend(&self, database: Box<dyn Database>) {
        //TODO
    }
}