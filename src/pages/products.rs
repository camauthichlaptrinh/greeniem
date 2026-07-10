use yew::prelude::*;

use crate::api;
use crate::components::captcha_gate::CaptchaGate;
use crate::components::product_card::ProductCard;
use crate::types::{Category, Product};

#[derive(PartialEq, Clone)]
enum LoadState {
    Loading,
    Loaded(Vec<Product>),
    NeedsCaptcha,
    Error(String),
}

#[function_component(Products)]
pub fn products() -> Html {
    let state = use_state(|| LoadState::Loading);
    let selected = use_state(|| None::<Category>);

    let fetch = {
        let state = state.clone();
        let selected = selected.clone();
        Callback::from(move |_: ()| {
            let state = state.clone();
            let path = match &*selected {
                Some(c) => format!("/products?category={}", c.as_query()),
                None => "/products".to_string(),
            };
            state.set(LoadState::Loading);
            wasm_bindgen_futures::spawn_local(async move {
                match api::get::<Vec<Product>>(&path).await {
                    Ok(products) => state.set(LoadState::Loaded(products)),
                    Err(api::ApiError::CaptchaRequired) => state.set(LoadState::NeedsCaptcha),
                    Err(e) => state.set(LoadState::Error(e.display())),
                }
            });
        })
    };

    {
        let fetch = fetch.clone();
        let selected = (*selected).clone();
        use_effect_with(selected, move |_| {
            fetch.emit(());
            || ()
        });
    }

    let on_select = {
        let selected = selected.clone();
        Callback::from(move |c: Option<Category>| selected.set(c))
    };

    html! {
        <div class="gi-section gi-container">
            <div class="gi-section__head">
                <h2>{ "Tất cả sản phẩm" }</h2>
            </div>
            <div class="gi-filter-bar">
                <button
                    class={classes!("gi-filter-chip", selected.is_none().then_some("gi-filter-chip--active"))}
                    onclick={{
                        let on_select = on_select.clone();
                        Callback::from(move |_| on_select.emit(None))
                    }}
                >
                    { "Tất cả" }
                </button>
                { for Category::ALL.iter().map(|c| {
                    let is_active = *selected == Some(c.clone());
                    let c2 = c.clone();
                    let on_select = on_select.clone();
                    html! {
                        <button
                            class={classes!("gi-filter-chip", is_active.then_some("gi-filter-chip--active"))}
                            onclick={Callback::from(move |_| on_select.emit(Some(c2.clone())))}
                        >
                            { c.label() }
                        </button>
                    }
                }) }
            </div>
            {
                match &*state {
                    LoadState::Loading => html! { <div class="gi-loading"><span class="gi-spinner"></span></div> },
                    LoadState::NeedsCaptcha => html! { <CaptchaGate on_verified={fetch.reform(|_| ())} /> },
                    LoadState::Error(msg) => html! { <div class="gi-empty">{ msg }</div> },
                    LoadState::Loaded(products) if products.is_empty() => html! {
                        <div class="gi-empty">{ "Không có sản phẩm nào trong danh mục này." }</div>
                    },
                    LoadState::Loaded(products) => html! {
                        <div class="gi-product-grid">
                            { for products.iter().map(|p| html! { <ProductCard product={p.clone()} /> }) }
                        </div>
                    },
                }
            }
        </div>
    }
}
