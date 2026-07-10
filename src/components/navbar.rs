use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;
use crate::state::cart::use_cart;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let cart = use_cart();
    let count = cart.item_count();

    html! {
        <header class="gi-nav">
            <div class="gi-nav__inner">
                <Link<Route> to={Route::Home} classes="gi-logo">
                    { "Green" }<span class="gi-logo__mark">{ "IEM" }</span>
                </Link<Route>>
                <nav class="gi-nav__links">
                    <Link<Route> to={Route::Products} classes="gi-nav__link">{ "Sản phẩm" }</Link<Route>>
                    <Link<Route> to={Route::Products} classes="gi-nav__link">{ "IEM" }</Link<Route>>
                    <Link<Route> to={Route::Products} classes="gi-nav__link">{ "Dongle" }</Link<Route>>
                    <Link<Route> to={Route::Products} classes="gi-nav__link">{ "Amplifier" }</Link<Route>>
                    <Link<Route> to={Route::Products} classes="gi-nav__link">{ "Loa Bookshelf" }</Link<Route>>
                </nav>
                <div class="gi-nav__actions">
                    <Link<Route> to={Route::Cart} classes="gi-cart-btn">
                        { "Giỏ hàng" }
                        if count > 0 {
                            <span class="gi-cart-badge">{ count }</span>
                        }
                    </Link<Route>>
                    <Link<Route> to={Route::AdminLogin} classes="gi-nav__link">{ "Admin" }</Link<Route>>
                </div>
            </div>
        </header>
    }
}
