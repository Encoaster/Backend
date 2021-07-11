//! Module describing the core Server

use super::{loader::PluginLoader, database::Database};
use crate::result::{Result, Error, ErrorKind};

use lazy_static::lazy_static;
use std::sync::{Mutex, Arc};
use std::cell::RefCell;
use std::path::{PathBuf, Path};

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

    /// Register the default backends. A backend will only be registered if _no_ backend has been registered yet. This function should be called _after_ all Plugins have been loaded.
    pub(crate) fn register_default_backends(&mut self) {
        if self.databases.is_empty() {
            let database = super::default::database::DefaultDatabase::new();
            self.regiser_database_backend(Box::new(database))
        }
    }

    /// Get the data folder in which Encoaster's data for the backend server is stored.
    /// The folder might not exist yet!
    pub fn get_data_folder() -> PathBuf {
        #[cfg(unix)] let path = "/var/encoaster/";
        #[cfg(windows)] let path = r#"C:\Program Files\Encoaster\"#;

        PathBuf::from(path)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile::tempdir;

    /// Create a default instance of Server
    fn local_default_server() -> Server {
        Server {
            plugin_loader: Default::default(),
            databases: Vec::default()
        }
    }

    #[test]
    fn server_init() {
        Server::init().unwrap();
        let lock = SERVER.lock().unwrap();
        let server = lock.borrow();

        assert!(server.is_some());
    }

    #[test]
    fn load_plugins() {
        let mut server = local_default_server();

        let tmpdir = tempdir().unwrap();
        server.load_plugins(tmpdir.path()).unwrap();
        assert!(server.plugin_loader.plugins.is_empty());
    }

    #[test]
    fn register_defaults_with_database() {
        let mut server = local_default_server();

        let default_databse = super::super::default::database::DefaultDatabase::new();
        server.regiser_database_backend(Box::new(default_databse));

        server.register_default_backends();

        assert_eq!(server.databases.len(), 1);
    }

    #[test]
    fn register_defaults_without_database() {
        let mut server = local_default_server();

        server.register_default_backends();
        assert_eq!(server.databases.len(), 1);
    }
}