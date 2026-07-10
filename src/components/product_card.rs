use yew::prelude::*;
use yew_router::prelude::*;

use crate::format::vnd;
use crate::route::Route;
use crate::types::Product;

#[derive(Properties, PartialEq)]
pub struct ProductCardProps {
    pub product: Product,
}

#[function_component(ProductCard)]
pub fn product_card(props: &ProductCardProps) -> Html {
    let p = &props.product;
    html! {
        <Link<Route> to={Route::ProductDetail { slug: p.slug.clone() }} classes="gi-card gi-panel">
            <div class="gi-card__img-wrap">
                <img class="gi-card__img" src={p.image_url.clone()} alt={p.name.clone()} loading="lazy" />
            </div>
            <div class="gi-card__body">
                <span class="gi-card__cat">{ p.category.label() }</span>
                <span class="gi-card__title">{ &p.name }</span>
                <div class="gi-card__foot">
                    <span class="gi-card__price">{ vnd(p.price) }</span>
                    if p.stock <= 0 {
                        <span class="gi-badge gi-badge--out">{ "Hết hàng" }</span>
                    }
                </div>
            </div>
        </Link<Route>>
    }
}
