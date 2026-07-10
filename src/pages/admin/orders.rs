use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::api;
use crate::components::alert::Alert;
use crate::format::vnd;
use crate::pages::admin::layout::{AdminShell, AdminTab};
use crate::route::Route;
use crate::state::auth::AuthStore;
use crate::types::{Order, OrderStatus, UpdateOrderStatusRequest};

fn status_class(status: &OrderStatus) -> &'static str {
    match status {
        OrderStatus::Pending => "gi-status-pill--pending",
        OrderStatus::Confirmed => "gi-status-pill--confirmed",
        OrderStatus::Shipping => "gi-status-pill--shipping",
        OrderStatus::Completed => "gi-status-pill--completed",
        OrderStatus::Cancelled => "gi-status-pill--cancelled",
    }
}

fn status_query(s: &str) -> Option<OrderStatus> {
    match s {
        "pending" => Some(OrderStatus::Pending),
        "confirmed" => Some(OrderStatus::Confirmed),
        "shipping" => Some(OrderStatus::Shipping),
        "completed" => Some(OrderStatus::Completed),
        "cancelled" => Some(OrderStatus::Cancelled),
        _ => None,
    }
}

fn status_key(s: &OrderStatus) -> &'static str {
    match s {
        OrderStatus::Pending => "pending",
        OrderStatus::Confirmed => "confirmed",
        OrderStatus::Shipping => "shipping",
        OrderStatus::Completed => "completed",
        OrderStatus::Cancelled => "cancelled",
    }
}

#[function_component(AdminOrders)]
pub fn admin_orders() -> Html {
    let navigator = use_navigator().unwrap();
    let orders = use_state(Vec::<Order>::new);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    let reload = {
        let orders = orders.clone();
        let loading = loading.clone();
        let error = error.clone();
        let navigator = navigator.clone();
        Callback::from(move |_: ()| {
            let orders = orders.clone();
            let loading = loading.clone();
            let error = error.clone();
            let navigator = navigator.clone();
            loading.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                match api::get::<Vec<Order>>("/admin/orders").await {
                    Ok(list) => {
                        orders.set(list);
                        loading.set(false);
                    }
                    Err(api::ApiError::Unauthorized(_)) => {
                        AuthStore::clear();
                        navigator.push(&Route::AdminLogin);
                    }
                    Err(e) => {
                        error.set(Some(e.display()));
                        loading.set(false);
                    }
                }
            });
        })
    };

    {
        let reload = reload.clone();
        use_effect_with((), move |_| {
            reload.emit(());
            || ()
        });
    }

    let update_status = {
        let reload = reload.clone();
        let error = error.clone();
        Callback::from(move |(id, status): (String, OrderStatus)| {
            let reload = reload.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let path = format!("/admin/orders/{id}");
                let body = UpdateOrderStatusRequest { status };
                match api::patch::<_, Order>(&path, &body).await {
                    Ok(_) => reload.emit(()),
                    Err(e) => error.set(Some(e.display())),
                }
            });
        })
    };

    html! {
        <AdminShell active={AdminTab::Orders}>
            <div class="gi-admin-topbar">
                <h1>{ "Đơn hàng" }</h1>
            </div>

            if let Some(msg) = &*error {
                <Alert message={msg.clone()} />
            }

            {
                if *loading {
                    html! { <div class="gi-loading"><span class="gi-spinner"></span></div> }
                } else if orders.is_empty() {
                    html! { <div class="gi-empty">{ "Chưa có đơn hàng nào." }</div> }
                } else {
                    html! {
                        <div class="gi-table-wrap gi-panel">
                            <table class="gi-table">
                                <thead>
                                    <tr>
                                        <th>{ "Mã đơn" }</th>
                                        <th>{ "Khách hàng" }</th>
                                        <th>{ "Sản phẩm" }</th>
                                        <th>{ "Tổng tiền" }</th>
                                        <th>{ "Trạng thái" }</th>
                                        <th>{ "Cập nhật" }</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    { for orders.iter().map(|o| {
                                        let id = o.id.clone();
                                        let update_status = update_status.clone();
                                        let onchange = Callback::from(move |e: Event| {
                                            let select: HtmlSelectElement = e.target_unchecked_into();
                                            if let Some(s) = status_query(&select.value()) {
                                                update_status.emit((id.clone(), s));
                                            }
                                        });
                                        html! {
                                            <tr>
                                                <td>{ format!("#{}", &o.id[..o.id.len().min(8)]) }</td>
                                                <td>
                                                    <div>{ &o.customer.name }</div>
                                                    <div class="gi-muted">{ &o.customer.phone }</div>
                                                    <div class="gi-muted">{ &o.customer.address }</div>
                                                </td>
                                                <td>
                                                    { for o.items.iter().map(|i| html! {
                                                        <div>{ format!("{} x{}", i.name, i.qty) }</div>
                                                    }) }
                                                </td>
                                                <td>{ vnd(o.total) }</td>
                                                <td>
                                                    <span class={classes!("gi-status-pill", status_class(&o.status))}>
                                                        { o.status.label() }
                                                    </span>
                                                </td>
                                                <td>
                                                    <select class="gi-select" onchange={onchange}>
                                                        { for OrderStatus::all().iter().map(|s| html! {
                                                            <option value={status_key(s)} selected={*s == o.status}>{ s.label() }</option>
                                                        }) }
                                                    </select>
                                                </td>
                                            </tr>
                                        }
                                    }) }
                                </tbody>
                            </table>
                        </div>
                    }
                }
            }
        </AdminShell>
    }
}
