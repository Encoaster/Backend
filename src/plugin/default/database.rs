//! Default implementation of Database
//! This implementation is for SQLite

use crate::plugin::database::{Database, SqlValue, ValueType};
use crate::result::{Error, ErrorKind, Result};
use std::collections::HashMap;
use rusqlite::{Connection, ToSql};
use crate::plugin::server::Server;

impl rusqlite::ToSql for SqlValue {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        match self {
            Self::Text(str) => {
                str.to_sql()
            },
            Self::Long(num) => {
                num.to_sql()
            },
            Self::Boolean(bool) => {
                bool.to_sql()
            },
            Self::Float(float) => {
                float.to_sql()
            }
        }
    }
}

/// Default database, uses SQLite
#[derive(Debug)]
pub struct DefaultDatabase;

impl Database for DefaultDatabase {
    fn query(&self, table: &str, cols: &[(&str, ValueType)], conditions: &[(&str, bool, SqlValue)]) -> Result<Vec<HashMap<String, SqlValue>>> {
        let conn = Self::get_conn()?;

        // Build the select bit of the query
        let col_select: String = cols.iter()
            .map(|(col_name, _)| col_name.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let mut stmt = format!("SELECT {} FROM {}", col_select, table);

        // Build the WHERE clause of the query
        for i in 0..conditions.len() {
            let (col_name, and, _) = conditions.get(i).unwrap();
            if i == 0 {
                stmt.push_str(&format!("WHERE {} = ?", col_name));
                continue;
            }

            if *and {
                stmt.push_str(&format!("AND WHERE {} = ?", col_name));
            } else {
                stmt.push_str(&format!("OR WHERE {} = ?", col_name));
            }
        }

        // Create a prepared stmt from the built query
        let mut prepared_stmt = match conn.prepare(&stmt) {
            Ok(stmt) => stmt,
            Err(e) => return Err(Error::new(ErrorKind::Database, format!("Failed to prepare Statement '{}': {:?}", &stmt, e)))
        };

        // Query the database, if the caller provided conditions, pass those in too
        let mut returned_rows = if conditions.is_empty() {
            // Caller supplied to conditions, so we don't provide any params
            match prepared_stmt.query([]) {
                Ok(rows) => rows,
                Err(e) => return Err(Error::new(ErrorKind::Database, format!("Failed to query SQLite database: {:?}", e)))
            }
        } else {
            // Convert the conditions provided by the caller to a vec of &dyn ToSql
            let condition = conditions.iter()
                .map(|(_, _, col_value)| col_value)
                .map(|v| v as &dyn ToSql)
                .collect::<Vec<&dyn ToSql>>();

            // Query the database with the conditions as params
            match prepared_stmt.query(condition.as_slice()) {
                Ok(rows) => rows,
                Err(e) => return Err(Error::new(ErrorKind::Database, format!("Failed to query SQLite database: {:?}", e)))
            }
        };

        let mut result = Vec::new();

        // Iterate over the returned Rows to build a Vec<HashMap<col_name, col_value>>
        while let Ok(Some(row)) = returned_rows.next() {
            let mut row_map = HashMap::new();
            for (col_name, col_type) in cols.iter() {
                let col_value = match col_type {
                    ValueType::Text => {
                        match row.get::<&str, String>(col_name) {
                            Ok(r) => SqlValue::from(r),
                            Err(e) => return Err(Error::new(ErrorKind::Database, format!("Unable to select column '{}' from returned row", col_name)))
                        }
                    },
                    ValueType::Long => {
                        match row.get::<&str, i64>(col_name) {
                            Ok(r) => SqlValue::from(r),
                            Err(e) => return Err(Error::new(ErrorKind::Database, format!("Unable to select column '{}' from returned row", col_name)))
                        }
                    },
                    ValueType::Boolean => {
                        match row.get::<&str, bool>(col_name) {
                            Ok(r) => SqlValue::from(r),
                            Err(e) => return Err(Error::new(ErrorKind::Database, format!("Unable to select column '{}' from returned row", col_name)))
                        }
                    },
                    ValueType::Float => {
                        match row.get::<&str, f64>(col_name) {
                            Ok(r) => SqlValue::from(r),
                            Err(e) => return Err(Error::new(ErrorKind::Database, format!("Unable to select column '{}' from returned row", col_name)))
                        }
                    }
                };

                row_map.insert(col_name.to_string(), col_value);
            }

            result.push(row_map);
        }

        Ok(result)
    }

    fn insert(&self, table: &str, cols: &[(&str, SqlValue)]) -> Result<()> {
        todo!()
    }

    fn update(&self, table: &str, cols: &[(&str, ValueType)], conditions: &[(&str, bool, SqlValue)]) -> Result<()> {
        todo!()
    }
}

impl DefaultDatabase {
    /// Create a new DefaultDatabase
    pub fn new() -> Self {
        Self
    }

    /// Create a SQLite database connection
    ///
    /// # Errors
    /// - If creating the Encoaster data directory fails
    /// - If opening the SQLite connection fails
    fn get_conn() -> Result<Connection> {
        use std::fs;

        let mut database_path = Server::get_data_folder();
        if !database_path.exists() {
            match fs::create_dir_all(&database_path) {
                Ok(_) => {},
                Err(e) => return Err(Error::new(ErrorKind::IoError(e), "Failed to create Encoaster data folder".to_string()))
            }
        }

        database_path.push("data.db3");
        match Connection::open(&database_path) {
            Ok(c) => Ok(c),
            Err(e) => Err(Error::new(ErrorKind::Database, format!("Failed to create SQLite database connection: {:?}", e)))
        }
    }
}