# config-items
  [![crate](https://img.shields.io/crates/v/config-items.svg)](https://crates.io/crates/config-items)
  ![crate](https://img.shields.io/crates/d/config-items)
  [![LICENSE](https://img.shields.io/crates/l/config-items.svg)](https://github.com/diegoefe/config-items/blob/main/LICENSE)
  [![docs](https://docs.rs/config-items/badge.svg)](https://docs.rs/config-items)

config-items is a library with common configuration items and utility functions.

See [examples](https://github.com/diegoefe/config-items/tree/main/examples) for usage.

## Install

```toml
# Cargo.toml
[dependencies]
config-items = "0.1"
```

## Usage
Define your own "Config" struct and add predefined items from this library.

Sample configuration for this example is [here](examples/myapp.yaml).

```rust
use config_items::*;
use log::*;
use std::error::Error;
use serde::Deserialize;
use clap::{Arg, Command, ArgMatches};

fn make_args<'a>() -> Command<'a> {
    Command::new("tupacrs")
        .author("My Self <myself@myserver.com>")
        .about("Abput my app")
        .arg(Arg::new("config")
            .short('c')
            .long("config-file")
            .takes_value(true)
            .help(
"Select config filename.
You can also define the pair of environment variables:
  MYAPP_CONF_DIR (defaults to '.')
  MYAPP_CONF_FILE (defaults to 'tupacrs.yaml')
or (all in one)
  MYAPP_CONF_PATH (full path to config file)"))
}

#[derive(Deserialize, Debug)]
struct Config {
    name: String,
    network: Option<Network>, // from config_items
    #[serde(default)] // use defaults if not present
    logging: Logging, // from config_items
}

struct MyArgResolver<'a> {
    matches:&'a ArgMatches
}
impl<'a> CFGResolver for MyArgResolver<'a> {
    fn get_from_argument(&self) -> Option<&str> {
        self.matches.value_of("config")
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // This is the recommended setup!!!
    set_app_id("MYAPP"); // Without this call an attempt to deduce the id from the current executable name will be made
    let (app_id, app_yaml, app_log) = get_app_vars();
    println!("app vars: id=[{app_id}], yaml=[{app_yaml}], log=[{app_log}]");
    let (v_pp, v_c_path, v_c_dir, v_c_file) = get_env_vars();
    println!("env vars: proxy_password=[{v_pp}], config: path=[{v_c_path}], path=[{v_c_dir}], path=[{v_c_file}]");
    let matches = make_args().get_matches();
    let cfg_file = get_config_file_name(&MyArgResolver{matches:&matches});
    let cfg:Config = read_config_from_yaml(&cfg_file)?;
    println!("Using config [{}]", cfg.name);
    if let Some(net) = cfg.network {
        println!(" Got network settings: validate ssl:{}", net.skip_ssl_validation());
        if let Some(proxy) = net.proxy {
            // println!("  Got proxy settings: {proxy:?}");
            if let Some(user) = proxy.get_user() {
                println!("   Proxy user will be [{user}]")
            }
            if let Some(pass) = proxy.get_password() {
                println!("   Proxy password will be [{pass}]")
            }
            println!("Final proxy url will be [{}]", proxy.get_url())
        }
    }
    println!("Logging with: {:?}", cfg.logging);
    cfg.logging.init()?;
    info!("This will be logged");
    Ok(())
}

```

## License

Licensed under MIT license ([LICENSE-MIT](LICENSE) or <http://opensource.org/licenses/MIT>)

