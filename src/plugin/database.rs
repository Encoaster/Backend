//! Module describing database handling through Plugins
use crate::result::Result;

use std::collections::HashMap;
use std::fmt::Debug;

/// SQL Column value
#[derive(Debug, Clone)]
pub enum SqlValue {
    /// Text value
    Text(String),
    /// Long value
    Long(i64),
    /// Floating point value
    Float(f64),
    /// Boolean value
    Boolean(bool)
}

/// SQL Column type
#[derive(Debug, Clone)]
pub enum ValueType {
    /// Text value (`String`)
    Text,
    /// Long value (`i64`)
    Long,
    /// Floating point value (`f64`)
    Float,
    /// Boolean value (`bool`)
    Boolean
}

/// Trait describing a Database backend
pub trait Database: Debug + Send + Sync {
    /// Query a row from the database
    ///
    /// # Errors
    /// This is upto the implementor, usually Err is returned when:
    /// - Creating the database connection failed
    /// - Querying the database failed
    fn query(&self, table: &str, cols: &[(&str, ValueType)], conditions: &[(&str, SqlValue)]) -> Result<HashMap<String, SqlValue>>;

    /// Insert a row into the database
    ///
    /// # Errors
    /// This is upto the implementor, usually Err is returned when:
    /// - Creating the database connection failed
    /// - Querying the database failed
    fn insert(&self, table: &str, cols: &[(&str, SqlValue)]) -> Result<()>;

    /// Update a row in the database
    ///
    /// # Errors
    /// This is upto the implementor, usually Err is returned when:
    /// - Creating the database connection failed
    /// - Querying the database failed
    fn update(&self, table: &str, cols: &[(&str, ValueType)], conditions: &[(&str, SqlValue)]) -> Result<()>;
}