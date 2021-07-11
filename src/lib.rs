//! TODO: Crate documentation

#![deny(deprecated)]
#![deny(clippy::panic)]
#![deny(rust_2018_idioms)]
#![deny(clippy::decimal_literal_representation)]
#![deny(clippy::if_not_else)]
#![deny(clippy::large_digit_groups)]
#![deny(clippy::needless_continue)]

#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::missing_errors_doc)]

//TODO this is allowed temporary as the project is in early stages of development
#![allow(unused)]

mod result;
mod plugin;

pub use plugin::*;
pub use result::*;