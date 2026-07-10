use yew::prelude::*;
use yew_router::prelude::*;

use crate::api;
use crate::components::captcha_gate::CaptchaGate;
use crate::components::product_card::ProductCard;
use crate::route::Route;
use crate::types::{Category, Product};

#[derive(PartialEq, Clone)]
enum LoadState {
    Loading,
    Loaded(Vec<Product>),
    NeedsCaptcha,
    Error(String),
}

#[function_component(Home)]
pub fn home() -> Html {
    let state = use_state(|| LoadState::Loading);

    let fetch = {
        let state = state.clone();
        Callback::from(move |_: ()| {
            let state = state.clone();
            state.set(LoadState::Loading);
            wasm_bindgen_futures::spawn_local(async move {
                match api::get::<Vec<Product>>("/products").await {
                    Ok(products) => state.set(LoadState::Loaded(products)),
                    Err(api::ApiError::CaptchaRequired) => state.set(LoadState::NeedsCaptcha),
                    Err(e) => state.set(LoadState::Error(e.display())),
                }
            });
        })
    };

    {
        let fetch = fetch.clone();
        use_effect_with((), move |_| {
            fetch.emit(());
            || ()
        });
    }

    let featured: Vec<Product> = match &*state {
        LoadState::Loaded(products) => products.iter().take(8).cloned().collect(),
        _ => vec![],
    };

    html! {
        <>
            <section class="gi-hero">
                <span class="gi-hero__eyebrow">{ "Âm thanh tinh tế · Chế tác kim loại" }</span>
                <h1 class="gi-hero__title">
                    { "Trải nghiệm âm thanh " }<em>{ "đẳng cấp" }</em>{ " cùng GreenIEM" }
                </h1>
                <p class="gi-hero__subtitle">
                    { "IEM, dongle DAC/AMP, amplifier, loa bookshelf và phụ kiện âm thanh cao cấp — tuyển chọn cho người sành nghe." }
                </p>
                <div class="gi-hero__ctas">
                    <Link<Route> to={Route::Products} classes="gi-btn gi-btn--primary">{ "Khám phá sản phẩm" }</Link<Route>>
                </div>
            </section>

            <section class="gi-section">
                <div class="gi-section__head gi-container">
                    <h2>{ "Danh mục nổi bật" }</h2>
                </div>
                <div class="gi-cat-grid">
                    { for Category::ALL.iter().map(|c| html! {
                        <Link<Route> to={Route::Products} classes="gi-cat-card gi-panel">
                            <div class="gi-cat-card__icon">{ category_icon(c) }</div>
                            <div class="gi-cat-card__label">{ c.label() }</div>
                        </Link<Route>>
                    }) }
                </div>
            </section>

            <section class="gi-section">
                <div class="gi-section__head gi-container">
                    <h2>{ "Sản phẩm mới" }</h2>
                    <Link<Route> to={Route::Products} classes="gi-nav__link">{ "Xem tất cả →" }</Link<Route>>
                </div>
                {
                    match &*state {
                        LoadState::Loading => html! { <div class="gi-loading"><span class="gi-spinner"></span></div> },
                        LoadState::NeedsCaptcha => html! {
                            <CaptchaGate on_verified={fetch.reform(|_| ())} />
                        },
                        LoadState::Error(msg) => html! { <div class="gi-empty">{ msg }</div> },
                        LoadState::Loaded(_) if featured.is_empty() => html! {
                            <div class="gi-empty">{ "Chưa có sản phẩm nào. Hãy vào trang Admin để thêm sản phẩm đầu tiên." }</div>
                        },
                        LoadState::Loaded(_) => html! {
                            <div class="gi-product-grid">
                                { for featured.iter().map(|p| html! { <ProductCard product={p.clone()} /> }) }
                            </div>
                        },
                    }
                }
            </section>
        </>
    }
}

fn category_icon(c: &Category) -> &'static str {
    match c {
        Category::Iem => "🎧",
        Category::Dongle => "🔌",
        Category::Amplifier => "🔊",
        Category::Bookshelf => "📻",
        Category::Accessory => "🎛️",
    }
}
