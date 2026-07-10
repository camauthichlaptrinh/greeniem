use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use yew::prelude::*;

const CART_KEY: &str = "greeniem_cart";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CartLine {
    pub product_id: String,
    pub name: String,
    pub price: i64,
    pub image_url: String,
    pub stock: i32,
    pub qty: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CartState {
    pub lines: Vec<CartLine>,
}

impl CartState {
    fn load() -> Self {
        LocalStorage::get(CART_KEY).unwrap_or_default()
    }

    fn persist(&self) {
        let _ = LocalStorage::set(CART_KEY, self);
    }

    pub fn total(&self) -> i64 {
        self.lines.iter().map(|l| l.price * l.qty as i64).sum()
    }

    pub fn item_count(&self) -> i32 {
        self.lines.iter().map(|l| l.qty).sum()
    }
}

pub enum CartAction {
    Add(CartLine),
    SetQty { product_id: String, qty: i32 },
    Remove(String),
    Clear,
}

impl Reducible for CartState {
    type Action = CartAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut lines = self.lines.clone();
        match action {
            CartAction::Add(line) => {
                if let Some(existing) = lines.iter_mut().find(|l| l.product_id == line.product_id)
                {
                    existing.qty = (existing.qty + line.qty).min(existing.stock.max(1));
                } else {
                    lines.push(line);
                }
            }
            CartAction::SetQty { product_id, qty } => {
                if qty <= 0 {
                    lines.retain(|l| l.product_id != product_id);
                } else if let Some(existing) =
                    lines.iter_mut().find(|l| l.product_id == product_id)
                {
                    existing.qty = qty.min(existing.stock.max(1));
                }
            }
            CartAction::Remove(product_id) => {
                lines.retain(|l| l.product_id != product_id);
            }
            CartAction::Clear => {
                lines.clear();
            }
        }
        let next = CartState { lines };
        next.persist();
        Rc::new(next)
    }
}

pub type CartContext = UseReducerHandle<CartState>;

#[derive(Properties, PartialEq)]
pub struct CartProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component(CartProvider)]
pub fn cart_provider(props: &CartProviderProps) -> Html {
    let cart = use_reducer(CartState::load);
    html! {
        <ContextProvider<CartContext> context={cart}>
            { props.children.clone() }
        </ContextProvider<CartContext>>
    }
}

#[hook]
pub fn use_cart() -> CartContext {
    use_context::<CartContext>().expect("CartContext not provided")
}
