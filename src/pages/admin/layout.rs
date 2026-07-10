use yew::prelude::*;
use yew_router::prelude::*;

use crate::api;
use crate::route::Route;
use crate::state::auth::AuthStore;
use crate::types::MeResponse;

#[derive(Clone, Copy, PartialEq)]
pub enum AdminTab {
    Products,
    Orders,
    Settings,
}

#[derive(Properties, PartialEq)]
pub struct AdminShellProps {
    pub active: AdminTab,
    pub children: Html,
}

#[derive(PartialEq, Clone)]
enum AuthCheck {
    Checking,
    Ok,
    Denied,
}

/// Wraps every /admin/* page: verifies the stored token against `/auth/me`
/// (so an expired/tampered token bounces back to login instead of showing a
/// broken dashboard) and renders the sidebar nav + logout action.
#[function_component(AdminShell)]
pub fn admin_shell(props: &AdminShellProps) -> Html {
    let navigator = use_navigator().unwrap();
    let check = use_state(|| AuthCheck::Checking);

    {
        let check = check.clone();
        let navigator = navigator.clone();
        use_effect_with((), move |_| {
            if !AuthStore::is_logged_in() {
                navigator.push(&Route::AdminLogin);
                check.set(AuthCheck::Denied);
                return;
            }
            wasm_bindgen_futures::spawn_local(async move {
                match api::get::<MeResponse>("/auth/me").await {
                    Ok(_) => check.set(AuthCheck::Ok),
                    Err(_) => {
                        AuthStore::clear();
                        navigator.push(&Route::AdminLogin);
                        check.set(AuthCheck::Denied);
                    }
                }
            });
        });
    }

    if *check != AuthCheck::Ok {
        return html! { <div class="gi-loading"><span class="gi-spinner"></span></div> };
    }

    let logout = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            AuthStore::clear();
            navigator.push(&Route::AdminLogin);
        })
    };

    let link_class = |tab: AdminTab| {
        classes!(
            "gi-admin-sidebar__link",
            (props.active == tab).then_some("gi-admin-sidebar__link--active")
        )
    };

    html! {
        <div class="gi-admin-shell gi-container">
            <aside class="gi-admin-sidebar">
                <div class="gi-admin-sidebar__title">{ "GreenIEM Admin" }</div>
                <Link<Route> to={Route::AdminProducts} classes={link_class(AdminTab::Products)}>{ "Sản phẩm" }</Link<Route>>
                <Link<Route> to={Route::AdminOrders} classes={link_class(AdminTab::Orders)}>{ "Đơn hàng" }</Link<Route>>
                <Link<Route> to={Route::AdminSettings} classes={link_class(AdminTab::Settings)}>{ "Bảo mật tài khoản" }</Link<Route>>
                <button class="gi-btn gi-btn--ghost gi-mt-1" onclick={logout}>{ "Đăng xuất" }</button>
            </aside>
            <div class="gi-admin-content">
                { props.children.clone() }
            </div>
        </div>
    }
}
