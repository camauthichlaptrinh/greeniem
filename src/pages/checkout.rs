use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::api;
use crate::components::alert::Alert;
use crate::components::captcha_gate::CaptchaGate;
use crate::format::vnd;
use crate::route::Route;
use crate::state::cart::{use_cart, CartAction};
use crate::types::{CreateOrderItemRequest, CreateOrderRequest, CustomerInfo, Order};

#[derive(PartialEq, Clone)]
enum SubmitState {
    Idle,
    Submitting,
    NeedsCaptcha,
    Error(String),
}

#[function_component(Checkout)]
pub fn checkout() -> Html {
    let cart = use_cart();
    let navigator = use_navigator().unwrap();
    let customer = use_state(CustomerInfo::default);
    let submit_state = use_state(|| SubmitState::Idle);

    if cart.lines.is_empty() {
        return html! {
            <div class="gi-page gi-text-center">
                <div class="gi-empty">
                    <p>{ "Giỏ hàng trống, không thể thanh toán." }</p>
                    <Link<Route> to={Route::Products} classes="gi-btn gi-btn--primary">{ "Quay lại mua sắm" }</Link<Route>>
                </div>
            </div>
        };
    }

    let do_submit = {
        let submit_state = submit_state.clone();
        let cart = cart.clone();
        let navigator = navigator.clone();
        let customer = customer.clone();
        Callback::from(move |_: ()| {
            let submit_state = submit_state.clone();
            let cart = cart.clone();
            let navigator = navigator.clone();
            let req = CreateOrderRequest {
                items: cart
                    .lines
                    .iter()
                    .map(|l| CreateOrderItemRequest {
                        product_id: l.product_id.clone(),
                        qty: l.qty,
                    })
                    .collect(),
                customer: (*customer).clone(),
            };
            submit_state.set(SubmitState::Submitting);
            wasm_bindgen_futures::spawn_local(async move {
                match api::post::<_, Order>("/orders", &req).await {
                    Ok(order) => {
                        cart.dispatch(CartAction::Clear);
                        navigator.push(&Route::OrderSuccess { id: order.id });
                    }
                    Err(api::ApiError::CaptchaRequired) => {
                        submit_state.set(SubmitState::NeedsCaptcha)
                    }
                    Err(e) => submit_state.set(SubmitState::Error(e.display())),
                }
            });
        })
    };

    let onsubmit = {
        let do_submit = do_submit.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            do_submit.emit(());
        })
    };

    let field_input = |setter: Callback<String>| {
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            setter.emit(input.value());
        })
    };

    let on_name = {
        let customer = customer.clone();
        field_input(Callback::from(move |v| {
            let mut c = (*customer).clone();
            c.name = v;
            customer.set(c);
        }))
    };
    let on_phone = {
        let customer = customer.clone();
        field_input(Callback::from(move |v| {
            let mut c = (*customer).clone();
            c.phone = v;
            customer.set(c);
        }))
    };
    let on_address = {
        let customer = customer.clone();
        field_input(Callback::from(move |v| {
            let mut c = (*customer).clone();
            c.address = v;
            customer.set(c);
        }))
    };
    let on_note = {
        let customer = customer.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            let mut c = (*customer).clone();
            c.note = input.value();
            customer.set(c);
        })
    };

    let busy = matches!(*submit_state, SubmitState::Submitting);

    html! {
        <div class="gi-page gi-page--narrow">
            <h1>{ "Thanh toán" }</h1>
            <div class="gi-order-summary gi-panel">
                { for cart.lines.iter().map(|l| html! {
                    <div class="gi-order-summary__row">
                        <span>{ format!("{} x{}", l.name, l.qty) }</span>
                        <span>{ vnd(l.price * l.qty as i64) }</span>
                    </div>
                }) }
                <div class="gi-order-summary__total">
                    <span>{ "Tổng cộng" }</span>
                    <span>{ vnd(cart.total()) }</span>
                </div>
            </div>

            {
                match &*submit_state {
                    SubmitState::NeedsCaptcha => html! {
                        <CaptchaGate on_verified={do_submit.reform(|_| ())} />
                    },
                    _ => html! {
                        <form class="gi-form gi-panel" style="padding: 1.4rem;" onsubmit={onsubmit}>
                            if let SubmitState::Error(msg) = &*submit_state {
                                <Alert message={msg.clone()} />
                            }
                            <div class="gi-field">
                                <label>{ "Họ và tên" }</label>
                                <input class="gi-input" required=true value={customer.name.clone()} oninput={on_name} />
                            </div>
                            <div class="gi-field">
                                <label>{ "Số điện thoại" }</label>
                                <input class="gi-input" required=true value={customer.phone.clone()} oninput={on_phone} />
                            </div>
                            <div class="gi-field">
                                <label>{ "Địa chỉ giao hàng" }</label>
                                <input class="gi-input" required=true value={customer.address.clone()} oninput={on_address} />
                            </div>
                            <div class="gi-field">
                                <label>{ "Ghi chú (tùy chọn)" }</label>
                                <textarea class="gi-textarea" value={customer.note.clone()} oninput={on_note} />
                            </div>
                            <button class="gi-btn gi-btn--primary gi-btn--block" type="submit" disabled={busy}>
                                { if busy { "Đang đặt hàng..." } else { "Xác nhận đặt hàng" } }
                            </button>
                        </form>
                    },
                }
            }
        </div>
    }
}
