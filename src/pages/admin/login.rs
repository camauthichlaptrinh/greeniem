use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::api;
use crate::components::alert::Alert;
use crate::route::Route;
use crate::state::auth::AuthStore;
use crate::types::{AuthResponse, BootstrapRequest, BootstrapStatus, LoginRequest};

#[derive(PartialEq, Clone)]
enum Mode {
    Loading,
    Bootstrap,
    Login,
}

#[function_component(AdminLogin)]
pub fn admin_login() -> Html {
    let mode = use_state(|| Mode::Loading);
    let username = use_state(String::new);
    let password = use_state(String::new);
    let confirm = use_state(String::new);
    let error = use_state(|| None::<String>);
    let busy = use_state(|| false);
    let navigator = use_navigator().unwrap();

    {
        let navigator = navigator.clone();
        use_effect_with((), move |_| {
            if AuthStore::is_logged_in() {
                navigator.push(&Route::AdminProducts);
            }
            || ()
        });
    }

    {
        let mode = mode.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match api::get::<BootstrapStatus>("/auth/bootstrap-status").await {
                    Ok(s) if s.needs_setup => mode.set(Mode::Bootstrap),
                    _ => mode.set(Mode::Login),
                }
            });
            || ()
        });
    }

    let onsubmit = {
        let mode = mode.clone();
        let username = username.clone();
        let password = password.clone();
        let confirm = confirm.clone();
        let error = error.clone();
        let busy = busy.clone();
        let navigator = navigator.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let mode = mode.clone();
            let username = username.clone();
            let password = password.clone();
            let confirm = confirm.clone();
            let error = error.clone();
            let busy = busy.clone();
            let navigator = navigator.clone();

            if *mode == Mode::Bootstrap && *password != *confirm {
                error.set(Some("Mật khẩu xác nhận không khớp".into()));
                return;
            }

            busy.set(true);
            error.set(None);
            let is_bootstrap = *mode == Mode::Bootstrap;
            let u = (*username).clone();
            let p = (*password).clone();

            wasm_bindgen_futures::spawn_local(async move {
                let result = if is_bootstrap {
                    api::post::<_, AuthResponse>(
                        "/auth/bootstrap",
                        &BootstrapRequest {
                            username: u,
                            password: p,
                        },
                    )
                    .await
                } else {
                    api::post::<_, AuthResponse>(
                        "/auth/login",
                        &LoginRequest {
                            username: u,
                            password: p,
                        },
                    )
                    .await
                };
                busy.set(false);
                match result {
                    Ok(resp) => {
                        AuthStore::set(&resp.token, &resp.username);
                        navigator.push(&Route::AdminProducts);
                    }
                    Err(e) => error.set(Some(e.display())),
                }
            });
        })
    };

    let on_username = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };
    let on_password = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };
    let on_confirm = {
        let confirm = confirm.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            confirm.set(input.value());
        })
    };

    if *mode == Mode::Loading {
        return html! { <div class="gi-loading"><span class="gi-spinner"></span></div> };
    }

    let is_bootstrap = *mode == Mode::Bootstrap;

    html! {
        <div class="gi-auth-shell">
            <div class="gi-auth-card gi-panel">
                <h2>{ if is_bootstrap { "Khởi tạo tài khoản Admin" } else { "Đăng nhập Admin" } }</h2>
                <p class="gi-auth-card__subtitle">
                    { if is_bootstrap {
                        "Chưa có tài khoản quản trị nào. Tạo tài khoản admin đầu tiên để bắt đầu."
                    } else {
                        "Đăng nhập để quản lý sản phẩm và đơn hàng."
                    } }
                </p>
                <form class="gi-form" onsubmit={onsubmit}>
                    if let Some(msg) = &*error {
                        <Alert message={msg.clone()} />
                    }
                    <div class="gi-field">
                        <label>{ "Tên đăng nhập" }</label>
                        <input class="gi-input" required=true value={(*username).clone()} oninput={on_username} />
                    </div>
                    <div class="gi-field">
                        <label>{ "Mật khẩu" }</label>
                        <input class="gi-input" type="password" required=true value={(*password).clone()} oninput={on_password} />
                    </div>
                    if is_bootstrap {
                        <div class="gi-field">
                            <label>{ "Xác nhận mật khẩu" }</label>
                            <input class="gi-input" type="password" required=true value={(*confirm).clone()} oninput={on_confirm} />
                        </div>
                    }
                    <button class="gi-btn gi-btn--primary gi-btn--block" type="submit" disabled={*busy}>
                        { if *busy { "Đang xử lý..." } else if is_bootstrap { "Tạo tài khoản admin" } else { "Đăng nhập" } }
                    </button>
                </form>
            </div>
        </div>
    }
}
