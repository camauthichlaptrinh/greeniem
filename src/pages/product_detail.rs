use yew::prelude::*;
use yew_router::prelude::*;

use crate::api;
use crate::components::captcha_gate::CaptchaGate;
use crate::format::vnd;
use crate::route::Route;
use crate::state::cart::{use_cart, CartAction, CartLine};
use crate::types::Product;

#[derive(Properties, PartialEq)]
pub struct ProductDetailProps {
    pub slug: String,
}

#[derive(PartialEq, Clone)]
enum LoadState {
    Loading,
    Loaded(Product),
    NeedsCaptcha,
    Error(String),
}

#[function_component(ProductDetail)]
pub fn product_detail(props: &ProductDetailProps) -> Html {
    let state = use_state(|| LoadState::Loading);
    let active_image = use_state(|| 0usize);
    let qty = use_state(|| 1i32);
    let cart = use_cart();
    let navigator = use_navigator().unwrap();

    let fetch = {
        let state = state.clone();
        let slug = props.slug.clone();
        Callback::from(move |_: ()| {
            let state = state.clone();
            let path = format!("/products/{slug}");
            state.set(LoadState::Loading);
            wasm_bindgen_futures::spawn_local(async move {
                match api::get::<Product>(&path).await {
                    Ok(p) => state.set(LoadState::Loaded(p)),
                    Err(api::ApiError::CaptchaRequired) => state.set(LoadState::NeedsCaptcha),
                    Err(e) => state.set(LoadState::Error(e.display())),
                }
            });
        })
    };

    {
        let fetch = fetch.clone();
        let slug = props.slug.clone();
        use_effect_with(slug, move |_| {
            fetch.emit(());
            || ()
        });
    }

    match &*state {
        LoadState::Loading => html! { <div class="gi-loading"><span class="gi-spinner"></span></div> },
        LoadState::NeedsCaptcha => html! {
            <CaptchaGate on_verified={fetch.reform(|_| ())} />
        },
        LoadState::Error(msg) => html! { <div class="gi-empty">{ msg }</div> },
        LoadState::Loaded(p) => {
            let images = if p.image_urls.is_empty() {
                vec![p.image_url.clone()]
            } else {
                p.image_urls.clone()
            };
            let main_img = images.get(*active_image).cloned().unwrap_or_default();

            let dec_qty = {
                let qty = qty.clone();
                Callback::from(move |_| qty.set((*qty - 1).max(1)))
            };
            let inc_qty = {
                let qty = qty.clone();
                let stock = p.stock;
                Callback::from(move |_| qty.set((*qty + 1).min(stock.max(1))))
            };

            let add_to_cart = {
                let cart = cart.clone();
                let p = p.clone();
                let qty = qty.clone();
                let navigator = navigator.clone();
                Callback::from(move |_| {
                    cart.dispatch(CartAction::Add(CartLine {
                        product_id: p.id.clone(),
                        name: p.name.clone(),
                        price: p.price,
                        image_url: p.image_url.clone(),
                        stock: p.stock,
                        qty: *qty,
                    }));
                    navigator.push(&Route::Cart);
                })
            };

            html! {
                <div class="gi-detail">
                    <div>
                        <div class="gi-gallery__main">
                            <img src={main_img} alt={p.name.clone()} />
                        </div>
                        if images.len() > 1 {
                            <div class="gi-gallery__thumbs">
                                { for images.iter().enumerate().map(|(i, url)| {
                                    let active_image = active_image.clone();
                                    let is_active = i == *active_image;
                                    html! {
                                        <div
                                            class={classes!("gi-gallery__thumb", is_active.then_some("gi-gallery__thumb--active"))}
                                            onclick={Callback::from(move |_| active_image.set(i))}
                                        >
                                            <img src={url.clone()} alt="" />
                                        </div>
                                    }
                                }) }
                            </div>
                        }
                    </div>
                    <div>
                        <span class="gi-detail__cat">{ p.category.label() }</span>
                        <h1 class="gi-detail__title">{ &p.name }</h1>
                        <div class="gi-price-tag">{ vnd(p.price) }</div>
                        <p class="gi-detail__desc">{ &p.description }</p>
                        if p.stock <= 0 {
                            <span class="gi-badge gi-badge--out">{ "Hết hàng" }</span>
                        } else {
                            <div class="gi-detail__actions">
                                <div class="gi-qty-control">
                                    <button onclick={dec_qty} type="button">{ "-" }</button>
                                    <span>{ *qty }</span>
                                    <button onclick={inc_qty} type="button">{ "+" }</button>
                                </div>
                                <button class="gi-btn gi-btn--primary" onclick={add_to_cart}>
                                    { "Thêm vào giỏ hàng" }
                                </button>
                            </div>
                            <p class="gi-muted gi-mt-1">{ format!("Còn {} sản phẩm trong kho", p.stock) }</p>
                        }
                    </div>
                </div>
            }
        }
    }
}
