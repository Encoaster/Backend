//! Module describing the core Server

use super::{loader::PluginLoader, database::Database};
use crate::result::{Result, Error, ErrorKind};

use lazy_static::lazy_static;
use std::sync::{Mutex, Arc};
use std::cell::RefCell;
use std::path::Path;

lazy_static!{
    /// Global instance of Server
    pub static ref SERVER: Arc<Mutex<RefCell<Option<Server>>>> = Arc::new(Mutex::new(RefCell::new(None)));
}

/// The Server is at the top of the plugin chain
pub struct Server {
    /// The plugin loader
    plugin_loader:  PluginLoader,
    /// All registered Database backends
    databases:      Vec<Box<dyn Database>>
}

impl Server {

    /// Initialize the Server
    ///
    /// # Errors
    /// - If [`crate::plugin::loader::PluginLoader::load_plugins`] fails
    /// - if locking SERVER fails
    pub(crate) fn init(plugin_dir: &Path) -> Result<()> {
        let plugin_loader = PluginLoader::load_plugins(plugin_dir)?;

        let this = Self {
            plugin_loader,
            databases: Vec::default()
        };

        let lock = match SERVER.lock() {
            Ok(l) => l,
            Err(e) => return Err(Error::new(ErrorKind::LockError, format!("Failed to lock SERVER: {:?}", e)))
        };

        lock.replace(Some(this));

        Ok(())
    }

    /// Register a database backend
    pub(crate) fn regiser_database_backend(&mut self, database: Box<dyn Database>) {
        self.databases.push(database)
    }
}