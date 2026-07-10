use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::footer::Footer;
use crate::components::navbar::Navbar;
use crate::pages::admin::login::AdminLogin;
use crate::pages::admin::orders::AdminOrders;
use crate::pages::admin::products::AdminProducts;
use crate::pages::admin::settings::AdminSettings;
use crate::pages::cart::Cart;
use crate::pages::checkout::Checkout;
use crate::pages::home::Home;
use crate::pages::order_success::OrderSuccess;
use crate::pages::product_detail::ProductDetail;
use crate::pages::products::Products;
use crate::route::Route;
use crate::state::cart::CartProvider;

fn switch(route: Route) -> Html {
    match route {
        Route::AdminLogin => html! { <AdminLogin /> },
        Route::AdminProducts => html! { <AdminProducts /> },
        Route::AdminOrders => html! { <AdminOrders /> },
        Route::AdminSettings => html! { <AdminSettings /> },
        other => html! {
            <>
                <Navbar />
                <main class="gi-main">
                    { match other {
                        Route::Home => html! { <Home /> },
                        Route::Products => html! { <Products /> },
                        Route::ProductDetail { slug } => html! { <ProductDetail slug={slug} /> },
                        Route::Cart => html! { <Cart /> },
                        Route::Checkout => html! { <Checkout /> },
                        Route::OrderSuccess { id } => html! { <OrderSuccess id={id} /> },
                        Route::NotFound => html! {
                            <div class="gi-page gi-text-center">
                                <h1>{ "404" }</h1>
                                <p class="gi-muted">{ "Không tìm thấy trang." }</p>
                            </div>
                        },
                        _ => unreachable!(),
                    } }
                </main>
                <Footer />
            </>
        },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <CartProvider>
            <BrowserRouter>
                <div class="gi-app">
                    <Switch<Route> render={switch} />
                </div>
            </BrowserRouter>
        </CartProvider>
    }
}
