use yew::prelude::*;
use yew_router::prelude::*;

use crate::format::vnd;
use crate::route::Route;
use crate::state::cart::{use_cart, CartAction};

#[function_component(Cart)]
pub fn cart() -> Html {
    let cart = use_cart();

    if cart.lines.is_empty() {
        return html! {
            <div class="gi-page gi-text-center">
                <div class="gi-empty">
                    <p>{ "Giỏ hàng của bạn đang trống." }</p>
                    <Link<Route> to={Route::Products} classes="gi-btn gi-btn--primary">{ "Tiếp tục mua sắm" }</Link<Route>>
                </div>
            </div>
        };
    }

    html! {
        <div class="gi-page">
            <h1>{ "Giỏ hàng" }</h1>
            <div class="gi-panel">
                { for cart.lines.iter().map(|line| {
                    let product_id = line.product_id.clone();
                    let cart_dec = cart.clone();
                    let pid_dec = product_id.clone();
                    let qty = line.qty;
                    let dec = Callback::from(move |_| cart_dec.dispatch(CartAction::SetQty { product_id: pid_dec.clone(), qty: qty - 1 }));

                    let cart_inc = cart.clone();
                    let pid_inc = product_id.clone();
                    let stock = line.stock;
                    let inc = Callback::from(move |_| cart_inc.dispatch(CartAction::SetQty { product_id: pid_inc.clone(), qty: (qty + 1).min(stock.max(1)) }));

                    let cart_rm = cart.clone();
                    let pid_rm = product_id.clone();
                    let remove = Callback::from(move |_| cart_rm.dispatch(CartAction::Remove(pid_rm.clone())));

                    html! {
                        <div class="gi-cart-row">
                            <div class="gi-cart-row__img"><img src={line.image_url.clone()} alt={line.name.clone()} /></div>
                            <div>
                                <div class="gi-cart-row__name">{ &line.name }</div>
                                <div class="gi-cart-row__price">{ vnd(line.price) }</div>
                            </div>
                            <div class="gi-qty-control">
                                <button onclick={dec} type="button">{ "-" }</button>
                                <span>{ line.qty }</span>
                                <button onclick={inc} type="button">{ "+" }</button>
                            </div>
                            <button class="gi-btn gi-btn--ghost gi-btn--sm" onclick={remove} type="button">{ "Xóa" }</button>
                        </div>
                    }
                }) }
            </div>
            <div class="gi-cart-summary gi-panel">
                <span>{ "Tổng cộng" }</span>
                <span class="gi-cart-summary__total">{ vnd(cart.total()) }</span>
            </div>
            <div class="gi-mt-1 gi-text-center">
                <Link<Route> to={Route::Checkout} classes="gi-btn gi-btn--primary">{ "Tiến hành đặt hàng" }</Link<Route>>
            </div>
        </div>
    }
}
