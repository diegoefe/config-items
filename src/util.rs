use serde::Deserialize;
use std::path::Path;
use yaml_merge_keys::*;
use std::fs::{self};
use log::*;

use crate::{get_env_vars, get_yaml_filename, res::SRes};

/// Deserialize YAML object from String
pub fn read_yaml_from_string<T:for<'de> Deserialize<'de> >(str: &str) -> SRes<T> {
    use yaml_merge_keys::serde_yaml::Value;
    let sy:Value = serde_yaml::from_str(str)?;
    let v: Result<Value, serde_yaml::Error> = serde_yaml::to_value(sy);
    
    let fix: Result<Value, MergeKeyError> = merge_keys_serde(v?);
    Ok(serde_yaml::from_value(fix?)?)
}

/// Read file into String and deserialize YAML object
pub fn read_yaml_from_file<T:for<'de> Deserialize<'de>, P: AsRef<Path>>(path: P) -> SRes<T> {
    let sfile:String = fs::read_to_string(path)?.parse()?;
    read_yaml_from_string(&sfile)
}

/// trait that asks for config filename (takes precedence over other guessings)
pub trait CFGResolver {
    fn get_from_argument(&self) -> Option<&str>;
}

/// Default resolver that does not resolves to any alterantive/custom configuration filename
pub struct DefaultResolver {}
impl CFGResolver for DefaultResolver {
    fn get_from_argument(&self) -> Option<&str> { None }
}

/// Get name of config file resulting of various options
pub fn get_config_file_name(resolver:&impl CFGResolver) -> String {
    let config = match resolver.get_from_argument() {
        Some(cfg)=>cfg.to_string(),
        None=>{
            let (_, conf_path, conf_dir, conf_file)=get_env_vars();
            use std::env::*;
            match var(&conf_path) {
                Ok(cfg)=>cfg,
                Err(_)=>{
                    let dir = match var(&conf_dir) {
                        Ok(d) => d,
                        Err(_)=>".".to_string()
                    };
                    let file = match var(&conf_file) {
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
