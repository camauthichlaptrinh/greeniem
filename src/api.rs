use gloo_net::http::{Request, RequestBuilder};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use wasm_bindgen::JsValue;

use crate::state::auth::AuthStore;
use crate::state::captcha::CaptchaStore;

#[derive(Debug, Clone)]
pub enum ApiError {
    CaptchaRequired,
    LockedOut(u64),
    Unauthorized(String),
    Message(String),
}

impl ApiError {
    pub fn display(&self) -> String {
        match self {
            ApiError::CaptchaRequired => "Cần xác minh captcha".to_string(),
            ApiError::LockedOut(secs) => {
                format!("Quá nhiều lần thử. Vui lòng thử lại sau {secs} giây.")
            }
            ApiError::Unauthorized(m) => m.clone(),
            ApiError::Message(m) => m.clone(),
        }
    }
}

fn base_url() -> String {
    let window = web_sys::window().expect("no window");
    if let Ok(base) = js_sys::Reflect::get(&window, &JsValue::from_str("__API_BASE__")) {
        if let Some(s) = base.as_string() {
            if !s.is_empty() {
                return s.trim_end_matches('/').to_string();
            }
        }
    }
    "http://localhost:8080".to_string()
}

fn url(path: &str) -> String {
    format!("{}/api{}", base_url(), path)
}

async fn handle_response<T: DeserializeOwned>(resp: gloo_net::http::Response) -> Result<T, ApiError> {
    let status = resp.status();
    if status == 204 {
        return serde_json::from_value(Value::Null)
            .map_err(|e| ApiError::Message(format!("Lỗi phân tích dữ liệu: {e}")));
    }

    let body: Value = resp
        .json()
        .await
        .unwrap_or_else(|_| Value::Object(Default::default()));

    if status >= 200 && status < 300 {
        serde_json::from_value(body).map_err(|e| ApiError::Message(format!("Lỗi dữ liệu: {e}")))
    } else {
        let code = body.get("error").and_then(|v| v.as_str()).unwrap_or("");
        if code == "captcha_required" {
            return Err(ApiError::CaptchaRequired);
        }
        if code == "locked_out" {
            let secs = body
                .get("retry_after_secs")
                .and_then(|v| v.as_u64())
                .unwrap_or(60);
            return Err(ApiError::LockedOut(secs));
        }
        let message = body
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("Đã xảy ra lỗi, vui lòng thử lại")
            .to_string();
        if status == 401 {
            return Err(ApiError::Unauthorized(message));
        }
        Err(ApiError::Message(message))
    }
}

fn with_auth_headers(mut req: RequestBuilder) -> RequestBuilder {
    if let Some(token) = AuthStore::token() {
        req = req.header("Authorization", &format!("Bearer {token}"));
    }
    if let Some(token) = CaptchaStore::token() {
        req = req.header("X-Captcha-Token", &token);
    }
    req
}

pub async fn get<T: DeserializeOwned>(path: &str) -> Result<T, ApiError> {
    let req = with_auth_headers(Request::get(&url(path)));
    let resp = req
        .send()
        .await
        .map_err(|e| ApiError::Message(format!("Không kết nối được máy chủ: {e}")))?;
    handle_response(resp).await
}

pub async fn post<B: Serialize, T: DeserializeOwned>(path: &str, body: &B) -> Result<T, ApiError> {
    let req = with_auth_headers(Request::post(&url(path)));
    let resp = req
        .json(body)
        .map_err(|e| ApiError::Message(format!("Lỗi dữ liệu gửi đi: {e}")))?
        .send()
        .await
        .map_err(|e| ApiError::Message(format!("Không kết nối được máy chủ: {e}")))?;
    handle_response(resp).await
}

pub async fn put<B: Serialize, T: DeserializeOwned>(path: &str, body: &B) -> Result<T, ApiError> {
    let req = with_auth_headers(Request::put(&url(path)));
    let resp = req
        .json(body)
        .map_err(|e| ApiError::Message(format!("Lỗi dữ liệu gửi đi: {e}")))?
        .send()
        .await
        .map_err(|e| ApiError::Message(format!("Không kết nối được máy chủ: {e}")))?;
    handle_response(resp).await
}

pub async fn patch<B: Serialize, T: DeserializeOwned>(path: &str, body: &B) -> Result<T, ApiError> {
    let req = with_auth_headers(Request::patch(&url(path)));
    let resp = req
        .json(body)
        .map_err(|e| ApiError::Message(format!("Lỗi dữ liệu gửi đi: {e}")))?
        .send()
        .await
        .map_err(|e| ApiError::Message(format!("Không kết nối được máy chủ: {e}")))?;
    handle_response(resp).await
}

pub async fn delete<T: DeserializeOwned>(path: &str) -> Result<T, ApiError> {
    let req = with_auth_headers(Request::delete(&url(path)));
    let resp = req
        .send()
        .await
        .map_err(|e| ApiError::Message(format!("Không kết nối được máy chủ: {e}")))?;
    handle_response(resp).await
}
