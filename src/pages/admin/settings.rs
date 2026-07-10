use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::api;
use crate::components::alert::Alert;
use crate::pages::admin::layout::{AdminShell, AdminTab};
use crate::state::auth::AuthStore;
use crate::types::ChangePasswordRequest;

#[function_component(AdminSettings)]
pub fn admin_settings() -> Html {
    let current = use_state(String::new);
    let new_pw = use_state(String::new);
    let confirm = use_state(String::new);
    let error = use_state(|| None::<String>);
    let success = use_state(|| false);
    let busy = use_state(|| false);

    let onsubmit = {
        let current = current.clone();
        let new_pw = new_pw.clone();
        let confirm = confirm.clone();
        let error = error.clone();
        let success = success.clone();
        let busy = busy.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            error.set(None);
            success.set(false);

            if *new_pw != *confirm {
                error.set(Some("Mật khẩu mới không khớp".into()));
                return;
            }
            if new_pw.len() < 8 {
                error.set(Some("Mật khẩu mới tối thiểu 8 ký tự".into()));
                return;
            }

            let current = current.clone();
            let new_pw = new_pw.clone();
            let confirm = confirm.clone();
            let error = error.clone();
            let success = success.clone();
            let busy = busy.clone();
            busy.set(true);
            let body = ChangePasswordRequest {
                current_password: (*current).clone(),
                new_password: (*new_pw).clone(),
            };
            wasm_bindgen_futures::spawn_local(async move {
                let result = api::patch::<_, serde_json::Value>("/auth/password", &body).await;
                busy.set(false);
                match result {
                    Ok(_) => {
                        success.set(true);
                        current.set(String::new());
                        new_pw.set(String::new());
                        confirm.set(String::new());
                    }
                    Err(e) => error.set(Some(e.display())),
                }
            });
        })
    };

    let bind = |state: UseStateHandle<String>| {
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            state.set(input.value());
        })
    };

    let username = AuthStore::username().unwrap_or_default();

    html! {
        <AdminShell active={AdminTab::Settings}>
            <div class="gi-admin-topbar">
                <h1>{ "Bảo mật tài khoản" }</h1>
            </div>
            <div class="gi-panel" style="padding: 1.6rem; max-width: 420px;">
                <p class="gi-muted">{ format!("Đăng nhập với tài khoản: {username}") }</p>
                <form class="gi-form" onsubmit={onsubmit}>
                    if let Some(msg) = &*error {
                        <Alert message={msg.clone()} />
                    }
                    if *success {
                        <Alert message={"Đổi mật khẩu thành công.".to_string()} success={true} />
                    }
                    <div class="gi-field">
                        <label>{ "Mật khẩu hiện tại" }</label>
                        <input class="gi-input" type="password" required=true value={(*current).clone()} oninput={bind(current.clone())} />
                    </div>
                    <div class="gi-field">
                        <label>{ "Mật khẩu mới" }</label>
                        <input class="gi-input" type="password" required=true value={(*new_pw).clone()} oninput={bind(new_pw.clone())} />
                    </div>
                    <div class="gi-field">
                        <label>{ "Xác nhận mật khẩu mới" }</label>
                        <input class="gi-input" type="password" required=true value={(*confirm).clone()} oninput={bind(confirm.clone())} />
                    </div>
                    <button class="gi-btn gi-btn--primary gi-btn--block" type="submit" disabled={*busy}>
                        { if *busy { "Đang lưu..." } else { "Đổi mật khẩu" } }
                    </button>
                </form>
            </div>
        </AdminShell>
    }
}
