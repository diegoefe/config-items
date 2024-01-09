use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;

lazy_static! {
    static ref ID: Mutex<Option<String>> = Mutex::new(None);
}

/// Get the id of using app
pub fn get_app_id() -> Option<String> {
    ID.lock().unwrap().to_owned()
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
        let id = get_app_id();
        assert_eq!(id, None);
        let my_id = "MYAPP";
        set_app_id(my_id);
        assert_eq!(get_app_id(), Some(my_id.to_string()));
        let my_id2 = "mylowercaseapp";
        set_app_id(my_id2);
        assert_eq!(get_app_id(), Some(my_id2.to_string().to_uppercase()));
    }
}