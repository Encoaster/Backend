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
    pub(crate) plugin_loader:  PluginLoader,
    /// All registered Database backends
    pub(crate) databases:      Vec<Box<dyn Database>>
}

impl Server {

    /// Initialize the Server
    ///
    /// # Errors
    /// - if locking SERVER fails
    pub(crate) fn init() -> Result<()> {
        let plugin_loader = PluginLoader::default();
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

    /// Load all plugins in `plugin_dir`
    ///
    /// # Errors
    /// - If [`crate::plugin::loader::PluginLoader::load_plugins`] fails
    pub(crate) fn load_plugins(&mut self, plugin_dir: &Path) -> Result<()> {
        let loader = &mut self.plugin_loader;
        loader.load_plugins(plugin_dir)
    }

    /// Register a database backend
    pub(crate) fn regiser_database_backend(&mut self, database: Box<dyn Database>) {
        self.databases.push(database)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn server_init() {
        Server::init().unwrap();
        let lock = SERVER.lock().unwrap();
        let server = lock.borrow();

        assert!(server.is_some());
    }

    #[test]
    fn load_plugins() {
        Server::init().unwrap();
        let lock = SERVER.lock().unwrap();
        let mut server_ref = lock.borrow_mut();
        let server = server_ref.as_mut().unwrap();

        let tmpdir = tempdir().unwrap();
        server.load_plugins(tmpdir.path()).unwrap();
        assert!(server.plugin_loader.plugins.is_empty());
    }
}