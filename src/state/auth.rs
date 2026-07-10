use gloo_storage::{LocalStorage, Storage};

const TOKEN_KEY: &str = "greeniem_admin_token";
const USERNAME_KEY: &str = "greeniem_admin_username";

pub struct AuthStore;

impl AuthStore {
    pub fn token() -> Option<String> {
        LocalStorage::get(TOKEN_KEY).ok()
    }

    pub fn username() -> Option<String> {
        LocalStorage::get(USERNAME_KEY).ok()
    }

    pub fn set(token: &str, username: &str) {
        let _ = LocalStorage::set(TOKEN_KEY, token);
        let _ = LocalStorage::set(USERNAME_KEY, username);
    }

    pub fn clear() {
        LocalStorage::delete(TOKEN_KEY);
        LocalStorage::delete(USERNAME_KEY);
    }

    pub fn is_logged_in() -> bool {
        Self::token().is_some()
    }
}
