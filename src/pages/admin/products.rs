use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::api;
use crate::components::alert::Alert;
use crate::format::vnd;
use crate::pages::admin::layout::{AdminShell, AdminTab};
use crate::route::Route;
use crate::state::auth::AuthStore;
use crate::types::{Category, CreateProductRequest, Product, UpdateProductRequest};

#[derive(Clone, PartialEq, Default)]
struct ProductForm {
    id: Option<String>,
    name: String,
    category: Category,
    description: String,
    price: String,
    image_urls: String,
    stock: String,
    active: bool,
}

impl From<&Product> for ProductForm {
    fn from(p: &Product) -> Self {
        Self {
            id: Some(p.id.clone()),
            name: p.name.clone(),
            category: p.category.clone(),
            description: p.description.clone(),
            price: p.price.to_string(),
            image_urls: p.image_urls.join("\n"),
            stock: p.stock.to_string(),
            active: p.active,
        }
    }
}

#[function_component(AdminProducts)]
pub fn admin_products() -> Html {
    let navigator = use_navigator().unwrap();
    let products = use_state(Vec::<Product>::new);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let modal = use_state(|| None::<ProductForm>);
    let modal_error = use_state(|| None::<String>);
    let saving = use_state(|| false);

    let reload = {
        let products = products.clone();
        let loading = loading.clone();
        let error = error.clone();
        let navigator = navigator.clone();
        Callback::from(move |_: ()| {
            let products = products.clone();
            let loading = loading.clone();
            let error = error.clone();
            let navigator = navigator.clone();
            loading.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                match api::get::<Vec<Product>>("/admin/products").await {
                    Ok(list) => {
                        products.set(list);
                        loading.set(false);
                    }
                    Err(api::ApiError::Unauthorized(_)) => {
                        AuthStore::clear();
                        navigator.push(&Route::AdminLogin);
                    }
                    Err(e) => {
                        error.set(Some(e.display()));
                        loading.set(false);
                    }
                }
            });
        })
    };

    {
        let reload = reload.clone();
        use_effect_with((), move |_| {
            reload.emit(());
            || ()
        });
    }

    let open_create = {
        let modal = modal.clone();
        let modal_error = modal_error.clone();
        Callback::from(move |_| {
            modal_error.set(None);
            modal.set(Some(ProductForm {
                active: true,
                ..Default::default()
            }));
        })
    };

    let open_edit = {
        let modal = modal.clone();
        let modal_error = modal_error.clone();
        Callback::from(move |p: Product| {
            modal_error.set(None);
            modal.set(Some(ProductForm::from(&p)));
        })
    };

    let close_modal = {
        let modal = modal.clone();
        Callback::from(move |_| modal.set(None))
    };

    let delete_product = {
        let reload = reload.clone();
        let error = error.clone();
        Callback::from(move |id: String| {
            let reload = reload.clone();
            let error = error.clone();
            let confirmed = web_sys::window()
                .and_then(|w| {
                    w.confirm_with_message("Xóa sản phẩm này? Hành động không thể hoàn tác.")
                        .ok()
                })
                .unwrap_or(false);
            if !confirmed {
                return;
            }
            wasm_bindgen_futures::spawn_local(async move {
                let path = format!("/admin/products/{id}");
                match api::delete::<serde_json::Value>(&path).await {
                    Ok(_) => reload.emit(()),
                    Err(e) => error.set(Some(e.display())),
                }
            });
        })
    };

    let toggle_active = {
        let reload = reload.clone();
        let error = error.clone();
        Callback::from(move |p: Product| {
            let reload = reload.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let path = format!("/admin/products/{}", p.id);
                let body = UpdateProductRequest {
                    active: Some(!p.active),
                    ..Default::default()
                };
                match api::put::<_, Product>(&path, &body).await {
                    Ok(_) => reload.emit(()),
                    Err(e) => error.set(Some(e.display())),
                }
            });
        })
    };

    let submit_modal = {
        let modal = modal.clone();
        let modal_error = modal_error.clone();
        let saving = saving.clone();
        let reload = reload.clone();
        Callback::from(move |_: ()| {
            let Some(form) = (*modal).clone() else { return };
            let modal = modal.clone();
            let modal_error = modal_error.clone();
            let saving = saving.clone();
            let reload = reload.clone();

            let price: i64 = match form.price.trim().parse() {
                Ok(v) if v >= 0 => v,
                _ => {
                    modal_error.set(Some("Giá không hợp lệ".into()));
                    return;
                }
            };
            let stock: i32 = match form.stock.trim().parse() {
                Ok(v) if v >= 0 => v,
                _ => {
                    modal_error.set(Some("Số lượng kho không hợp lệ".into()));
                    return;
                }
            };
            if form.name.trim().is_empty() {
                modal_error.set(Some("Tên sản phẩm bắt buộc".into()));
                return;
            }
            let image_urls: Vec<String> = form
                .image_urls
                .lines()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty())
                .collect();
            if image_urls.is_empty() {
                modal_error.set(Some("Cần ít nhất 1 URL ảnh".into()));
                return;
            }

            saving.set(true);
            modal_error.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                let result = if let Some(id) = &form.id {
                    let body = UpdateProductRequest {
                        name: Some(form.name.clone()),
                        category: Some(form.category.clone()),
                        description: Some(form.description.clone()),
                        price: Some(price),
                        image_urls: Some(image_urls),
                        stock: Some(stock),
                        active: Some(form.active),
                    };
                    api::put::<_, Product>(&format!("/admin/products/{id}"), &body).await
                } else {
                    let body = CreateProductRequest {
                        name: form.name.clone(),
                        category: form.category.clone(),
                        description: form.description.clone(),
                        price,
                        image_urls,
                        stock,
                        active: form.active,
                    };
                    api::post::<_, Product>("/admin/products", &body).await
                };
                saving.set(false);
                match result {
                    Ok(_) => {
                        modal.set(None);
                        reload.emit(());
                    }
                    Err(e) => modal_error.set(Some(e.display())),
                }
            });
        })
    };

    let onsubmit = {
        let submit_modal = submit_modal.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            submit_modal.emit(());
        })
    };

    html! {
        <AdminShell active={AdminTab::Products}>
            <div class="gi-admin-topbar">
                <h1>{ "Sản phẩm" }</h1>
                <button class="gi-btn gi-btn--primary" onclick={open_create}>{ "+ Thêm sản phẩm" }</button>
            </div>

            if let Some(msg) = &*error {
                <Alert message={msg.clone()} />
            }

            {
                if *loading {
                    html! { <div class="gi-loading"><span class="gi-spinner"></span></div> }
                } else if products.is_empty() {
                    html! { <div class="gi-empty">{ "Chưa có sản phẩm nào." }</div> }
                } else {
                    html! {
                        <div class="gi-table-wrap gi-panel">
                            <table class="gi-table">
                                <thead>
                                    <tr>
                                        <th></th>
                                        <th>{ "Tên" }</th>
                                        <th>{ "Danh mục" }</th>
                                        <th>{ "Giá" }</th>
                                        <th>{ "Kho" }</th>
                                        <th>{ "Trạng thái" }</th>
                                        <th>{ "Hành động" }</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    { for products.iter().map(|p| {
                                        let p1 = p.clone();
                                        let p2 = p.clone();
                                        let pid = p.id.clone();
                                        html! {
                                            <tr>
                                                <td><div class="gi-thumb-sm"><img src={p.image_url.clone()} alt="" /></div></td>
                                                <td>{ &p.name }</td>
                                                <td>{ p.category.label() }</td>
                                                <td>{ vnd(p.price) }</td>
                                                <td>{ p.stock }</td>
                                                <td>
                                                    <span class={classes!("gi-status-pill", if p.active { "gi-status-pill--completed" } else { "gi-status-pill--cancelled" })}>
                                                        { if p.active { "Đang bán" } else { "Đã ẩn" } }
                                                    </span>
                                                </td>
                                                <td>
                                                    <div style="display:flex; gap:0.4rem; flex-wrap: wrap;">
                                                        <button class="gi-btn gi-btn--sm gi-btn--ghost" onclick={open_edit.reform(move |_| p1.clone())}>{ "Sửa" }</button>
                                                        <button class="gi-btn gi-btn--sm gi-btn--ghost" onclick={toggle_active.reform(move |_| p2.clone())}>
                                                            { if p.active { "Ẩn" } else { "Hiện" } }
                                                        </button>
                                                        <button class="gi-btn gi-btn--sm gi-btn--danger" onclick={delete_product.reform(move |_| pid.clone())}>{ "Xóa" }</button>
                                                    </div>
                                                </td>
                                            </tr>
                                        }
                                    }) }
                                </tbody>
                            </table>
                        </div>
                    }
                }
            }

            if let Some(form) = (*modal).clone() {
                <ProductModal
                    form={form}
                    saving={*saving}
                    error={(*modal_error).clone()}
                    onsubmit={onsubmit}
                    onclose={close_modal}
                    onchange={Callback::from({
                        let modal = modal.clone();
                        move |f: ProductForm| modal.set(Some(f))
                    })}
                />
            }
        </AdminShell>
    }
}

#[derive(Properties, PartialEq)]
struct ProductModalProps {
    form: ProductForm,
    saving: bool,
    error: Option<String>,
    onsubmit: Callback<SubmitEvent>,
    onclose: Callback<()>,
    onchange: Callback<ProductForm>,
}

#[function_component(ProductModal)]
fn product_modal(props: &ProductModalProps) -> Html {
    let form = props.form.clone();
    let is_edit = form.id.is_some();

    let update_input = |onchange: Callback<ProductForm>, f: fn(&mut ProductForm, String)| {
        let form = form.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut next = form.clone();
            f(&mut next, input.value());
            onchange.emit(next);
        })
    };

    let update_textarea = |onchange: Callback<ProductForm>, f: fn(&mut ProductForm, String)| {
        let form = form.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            let mut next = form.clone();
            f(&mut next, input.value());
            onchange.emit(next);
        })
    };

    let on_name = update_input(props.onchange.clone(), |f, v| f.name = v);
    let on_desc = update_textarea(props.onchange.clone(), |f, v| f.description = v);
    let on_price = update_input(props.onchange.clone(), |f, v| f.price = v);
    let on_stock = update_input(props.onchange.clone(), |f, v| f.stock = v);
    let on_images = update_textarea(props.onchange.clone(), |f, v| f.image_urls = v);

    let on_category = {
        let onchange = props.onchange.clone();
        let form = form.clone();
        Callback::from(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            let mut next = form.clone();
            if let Some(c) = Category::from_query(&select.value()) {
                next.category = c;
            }
            onchange.emit(next);
        })
    };

    let on_active = {
        let onchange = props.onchange.clone();
        let form = form.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut next = form.clone();
            next.active = input.checked();
            onchange.emit(next);
        })
    };

    let onclose = props.onclose.clone();
    let stop_prop = Callback::from(|e: MouseEvent| e.stop_propagation());

    html! {
        <div class="gi-modal-backdrop" onclick={Callback::from(move |_| onclose.emit(()))}>
            <div class="gi-modal gi-panel" onclick={stop_prop}>
                <div class="gi-modal__head">
                    <h3>{ if is_edit { "Sửa sản phẩm" } else { "Thêm sản phẩm" } }</h3>
                    <button class="gi-btn gi-btn--ghost gi-btn--sm" onclick={{
                        let onclose = props.onclose.clone();
                        Callback::from(move |_| onclose.emit(()))
                    }}>{ "✕" }</button>
                </div>
                <form class="gi-form" onsubmit={props.onsubmit.clone()}>
                    if let Some(err) = &props.error {
                        <Alert message={err.clone()} />
                    }
                    <div class="gi-field">
                        <label>{ "Tên sản phẩm" }</label>
                        <input class="gi-input" value={form.name.clone()} oninput={on_name} required=true />
                    </div>
                    <div class="gi-row">
                        <div class="gi-field">
                            <label>{ "Danh mục" }</label>
                            <select class="gi-select" onchange={on_category}>
                                { for Category::ALL.iter().map(|c| html! {
                                    <option value={c.as_query()} selected={*c == form.category}>{ c.label() }</option>
                                }) }
                            </select>
                        </div>
                        <div class="gi-field">
                            <label>{ "Giá (VNĐ)" }</label>
                            <input class="gi-input" value={form.price.clone()} oninput={on_price} inputmode="numeric" required=true />
                        </div>
                        <div class="gi-field">
                            <label>{ "Tồn kho" }</label>
                            <input class="gi-input" value={form.stock.clone()} oninput={on_stock} inputmode="numeric" required=true />
                        </div>
                    </div>
                    <div class="gi-field">
                        <label>{ "Mô tả" }</label>
                        <textarea class="gi-textarea" value={form.description.clone()} oninput={on_desc} />
                    </div>
                    <div class="gi-field">
                        <label>{ "URL ảnh (mỗi dòng 1 URL, khuyến nghị 3 tấm)" }</label>
                        <textarea class="gi-textarea" value={form.image_urls.clone()} oninput={on_images} placeholder="https://..." />
                    </div>
                    <div class="gi-field" style="flex-direction: row; align-items: center; gap: 0.6rem;">
                        <input type="checkbox" checked={form.active} onchange={on_active} id="active-checkbox" />
                        <label for="active-checkbox" style="margin: 0;">{ "Hiển thị công khai" }</label>
                    </div>
                    <button class="gi-btn gi-btn--primary gi-btn--block" type="submit" disabled={props.saving}>
                        { if props.saving { "Đang lưu..." } else { "Lưu sản phẩm" } }
                    </button>
                </form>
            </div>
        </div>
    }
}
