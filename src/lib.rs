//! A library with common configuration items

/// Defines global state for this library
mod data;
pub use data::{
    get_app_id,
    set_app_id,
    get_proxy_password_var,
    get_filename
};

/// Defines common network configurations
mod net;
pub use net::{Network, Proxy};

mod util;
pub use util::{
    CFGResolver,
    read_yaml_from_file as read_config_from_yaml,
    get_config_file_name,
};