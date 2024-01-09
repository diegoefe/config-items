use serde::Deserialize;
use std::error::Error;
use std::path::Path;
use yaml_merge_keys::*;
use std::fs::{self};
use log::*;

use crate::{get_app_id, get_yaml_filename};

/// Deserialize YAML object from String
pub fn read_yaml_from_string<T:for<'de> Deserialize<'de> >(str: &str) -> Result<T, Box<dyn Error>> {
    use yaml_merge_keys::serde_yaml::Value;
    let sy:Value = serde_yaml::from_str(str)?;
    let v: Result<Value, serde_yaml::Error> = serde_yaml::to_value(sy);
    
    let fix: Result<Value, MergeKeyError> = merge_keys_serde(v?);
    Ok(serde_yaml::from_value(fix?)?)
}

/// Read file into String and deserialize YAML object
pub fn read_yaml_from_file<T:for<'de> Deserialize<'de>, P: AsRef<Path>>(path: P) -> Result<T, Box<dyn Error>> {
    let sfile:String = fs::read_to_string(path)?.parse()?;
    read_yaml_from_string(&sfile)
}

pub trait CFGResolver {
    fn get_from_argument(&self) -> Option<&str>;
}

pub fn get_config_file_name(resolver:&impl CFGResolver) -> String {
    let config = match resolver.get_from_argument() {
        Some(cfg)=>cfg.to_string(),
        None=>{
            use std::env::*;
            let app_id = get_app_id().expect("To get app id!");
            match var(format!("{}_CONF_PATH", app_id).as_str()) {
                Ok(cfg)=>cfg,
                Err(_)=>{
                    let dir = match var(format!("{}_CONF_DIR", app_id).as_str()) {
                        Ok(d) => d,
                        Err(_)=>".".to_string()
                    };
                    let file = match var(format!("{}_CONF_FILE", app_id).as_str()) {
                        Ok(f)=>f,
                        Err(_)=>get_yaml_filename()
                    };
                    format!("{}/{}", dir, file)
                }
            }
        }
    };
    debug!("Using config file '{config}'");
    config
}