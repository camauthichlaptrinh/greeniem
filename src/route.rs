use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/san-pham")]
    Products,
    #[at("/san-pham/:slug")]
    ProductDetail { slug: String },
    #[at("/gio-hang")]
    Cart,
    #[at("/thanh-toan")]
    Checkout,
    #[at("/dat-hang-thanh-cong/:id")]
    OrderSuccess { id: String },
    #[at("/admin")]
    AdminLogin,
    #[at("/admin/san-pham")]
    AdminProducts,
    #[at("/admin/don-hang")]
    AdminOrders,
    #[at("/admin/cai-dat")]
    AdminSettings,
    #[not_found]
    #[at("/404")]
    NotFound,
}
