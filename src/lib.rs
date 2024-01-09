//! A library with common configuration items

/// Defines global state for this library
mod data;
pub use data::{get_app_id, set_app_id};

/// Defines common network configurations
mod net;
pub use net::{Network, Proxy};