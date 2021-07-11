//! Module describing the core of the Plugin system

use super::plugin::loader::PluginLoader;
use database::Database;

use std::fmt::Debug;

pub(crate) mod loader;
pub(crate) mod default;

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

/// This macro helps you declare your plugin.
/// What this macro will do is add a function `_create_plugin() -> *mut T` which is called when your plugin is loaded.
/// It is required for the type being passed in to impl [`Plugin`]
#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:path) => {
        #[no_mangle]
        fn _create_plugin() -> *mut $plugin_type {
            // make sure the constructor is the correct type.
            let constructor: fn() -> $plugin_type = $constructor;

            let object = constructor();
            let boxed: Box<$plugin_type> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, Default)]
    struct TestPlugin;

    impl Plugin for TestPlugin {
        fn on_load(&self, instance: PluginInstance<'_>) {}
        fn on_unload(&self) {}
    }

    declare_plugin!(TestPlugin, TestPlugin::default);

    #[test]
    fn declare_plugin() {
        let plugin = _create_plugin();

        let mut default_plugin = TestPlugin::default();
        let default_plugin_boxed = Box::new(default_plugin);
        assert_eq!(plugin, Box::into_raw(default_plugin_boxed));
    }
}