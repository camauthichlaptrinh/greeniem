use gloo_storage::{LocalStorage, Storage};

const CAPTCHA_TOKEN_KEY: &str = "greeniem_captcha_token";

pub struct CaptchaStore;

impl CaptchaStore {
    pub fn token() -> Option<String> {
        LocalStorage::get(CAPTCHA_TOKEN_KEY).ok()
    }

    pub fn set(token: &str) {
        let _ = LocalStorage::set(CAPTCHA_TOKEN_KEY, token);
    }
}
