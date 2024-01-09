use serde::{Serialize, Deserialize};
use log::*;

use crate::get_proxy_password_var;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Proxy {
    pub url: String,
    pub port: i32,
    pub user: Option<String>,
    pub password: Option<String>,
}

impl Proxy {
    fn get_env_var(name:&str) -> Option<String> {
        match std::env::var(name) {
            Ok(pp) => Some(pp),
            Err(_) => None
        }
    }
    fn get_user_var() -> &'static str {
        let os = std::env::consts::OS;
        if os == "windows" {
            "USERNAME"
        } else {
            "USER"
        }
    }
    pub fn get_user(&self) -> Option<String> {
        if let Some(user) = &self.user {
            Some(user.to_owned())
        } else if let Some(euser) = Proxy::get_env_var(Proxy::get_user_var()) {
            Some(euser)
        } else {
            None
        }
    }
    pub fn get_password(&self) -> Option<String> {
        if let Some(pass) = &self.password {
            Some(pass.to_owned())
        } else if let Some(epass) = Proxy::get_env_var(&get_proxy_password_var()) {
            Some(epass)
        } else {
            None
        }
    }
    pub fn get_url(&self) -> String {
        let mut url = self.url.to_owned();
        if let Some(user) = self.get_user() {
            if let Some(pass) = self.get_password() {
                let parts:Vec<&str> = url.split("://").collect();
                let def_proto = format!("http://");
                let mut proto = def_proto.as_str();
                let host = if parts.len() == 1 {
                    &url
                } else {
                    proto = parts[0];
                    parts[1]
                };
                url = format!("{}{}:{}@{}:{}",
                                proto,
                                user,
                                pass,
                                host,
                                self.port);
            }
            debug!("proxy url [{url}]");
        }
        url
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Network {
    pub proxy: Option<Proxy>,
    pub skip_ssl_validation: Option<bool>,
}

impl Network {
    pub fn skip_ssl_validation(&self) -> bool {
        if let Some(ssv) = self.skip_ssl_validation {
            ssv
        } else {
            false
        }
    }
}
