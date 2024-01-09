use config_items::*;
use log::*;
use std::error::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    name: String,
    network: Option<Network>,
    #[serde(default)] // use defaults if not present
    logging: Logging,
}

struct MyConfig {}
impl CFGResolver for MyConfig {
    fn get_from_argument(&self) -> Option<&str> {
        // None
        Some("./examples/myapp.yaml")
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    set_app_id("MYAPP");
    let cfg_file = get_config_file_name(&MyConfig{});
    println!("Using config file [{cfg_file}]");
    let cfg:Config = read_config_from_yaml(&cfg_file)?;
    // println!("Config is [{cfg:#?}]");
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

