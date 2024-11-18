use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;

lazy_static! {
    static ref ID: Mutex<Option<String>> = Mutex::new(None);
}

/// Get Application id
/// 
/// If no a previous call to set_app_id() was issued, an attempt to deduce the id from the current executable name is made
pub fn get_app_id() -> Option<String> {
    // ID.lock().unwrap().to_owned()
    let oid = ID.lock().unwrap().to_owned();
    if oid.is_some() {
        oid
    } else {
        let exe = std::env::current_exe()
            .ok()?
            .file_name()?
            .to_str()?
            .to_owned()
            .to_uppercase();
        let parts:Vec<&str> = exe.split(".").collect();
        Some(parts[0].to_string())
    }
}

pub fn get_proxy_password_var() -> String {
    format!("{}_PP", get_app_id().expect("To get app id!"))
}

pub fn get_yaml_filename() -> String {
    format!("{}.yaml", get_app_id().expect("To get app id!").to_lowercase())
}

pub fn get_log_filename() -> String {
    format!("{}.log", get_app_id().expect("To get app id!").to_lowercase())
}

/// Obtain defined "app" variables. Warning must be called AFTER set_app_id()!
/// 
/// Returns (id, yaml_filename, log_filename)
pub fn get_app_vars() -> (String, String, String) {
    (get_app_id().unwrap(), get_yaml_filename(), get_log_filename())
}

/// Obtain watched environment variables. Warning must be called AFTER set_app_id()!
/// 
/// Returns (proxy_password_var, conf_path_var, conf_dir_var, conf_file_var)
pub fn get_env_vars() -> (String, String, String, String) {
    let app_id = get_app_id().expect("To get app id!");
    (get_proxy_password_var(), format!("{}_CONF_PATH", app_id), format!("{}_CONF_DIR", app_id), format!("{}_CONF_FILE", app_id))
}

/// Set the id of using app
pub fn set_app_id(id:&str) {
    *ID.lock().unwrap() = Some(id.to_string().to_uppercase())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        // let id = get_app_id();
        // assert_eq!(id, None);
        let my_id = "MYAPP";
        set_app_id(my_id);
        assert_eq!(get_app_id(), Some(my_id.to_string()));
        let my_id2 = "mylowercaseapp";
        set_app_id(my_id2);
        assert_eq!(get_app_id(), Some(my_id2.to_string().to_uppercase()));
    }
}