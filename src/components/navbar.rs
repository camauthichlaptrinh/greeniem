use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;
use crate::state::cart::use_cart;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let cart = use_cart();
    let count = cart.item_count();
    let menu_open = use_state(|| false);

    let toggle_menu = {
        let menu_open = menu_open.clone();
        Callback::from(move |_| menu_open.set(!*menu_open))
    };

    // Clicking any link inside closes the menu (click bubbles up from the <a>).
    let close_menu = {
        let menu_open = menu_open.clone();
        Callback::from(move |_| menu_open.set(false))
    };

    let links_class = classes!("gi-nav__links", menu_open.then_some("gi-nav__links--open"));

    html! {
        <header class="gi-nav">
            <div class="gi-nav__inner">
                <Link<Route> to={Route::Home} classes="gi-logo">
                    { "Green" }<span class="gi-logo__mark">{ "IEM" }</span>
                </Link<Route>>
                <nav class={links_class} onclick={close_menu}>
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
                    <button class="gi-nav__toggle" onclick={toggle_menu} aria-label="Mở menu">
                        <span></span>
                        <span></span>
                        <span></span>
                    </button>
                </div>
            </div>
        </header>
    }
}
