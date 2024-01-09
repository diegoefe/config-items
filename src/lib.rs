//! A library with common configuration items and utility functions

/// Defines global state for this library
mod data;
use data::{
    get_app_id,
    get_proxy_password_var,
    get_yaml_filename
};
pub use data::set_app_id;

/// Defines common network configurations
mod net;
pub use net::{Network, Proxy};

/// Defines a logging configuration
mod lg;
pub use lg::{
    Logging,
    create_log_config,
};

/// Has utilities to load and setup configuration files
mod util;
pub use util::{
    CFGResolver, DefaultResolver,
    read_yaml_from_file as read_config_from_yaml,
    get_config_file_name,
};