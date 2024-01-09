use serde::{Serialize, Deserialize};
use log::*;

use crate::get_app_id;

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
    fn get_proxy_user(pxy:&Proxy) -> Option<String> {
        if let Some(user) = &pxy.user {
            Some(user.to_owned())
        } else if let Some(euser) = Proxy::get_env_var(Proxy::get_user_var()) {
            Some(euser)
        } else {
            None
        }
    }
    fn get_proxy_password(pxy:&Proxy) -> Option<String> {
        let proxy_pass_var = format!("{}_PP", get_app_id().expect("To get app id!"));
        if let Some(pass) = &pxy.password {
            Some(pass.to_owned())
        } else if let Some(epass) = Proxy::get_env_var(&proxy_pass_var) {
            Some(epass)
        } else {
            None
        }
    }
    pub fn get_proxy_url(pxy:&Proxy) -> String {
        let mut url = pxy.url.to_owned();
        if let Some(user) = Proxy::get_proxy_user(pxy) {
            if let Some(pass) = Proxy::get_proxy_password(pxy) {
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
                                pxy.port);
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
