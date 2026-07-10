use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AlertProps {
    pub message: String,
    #[prop_or(false)]
    pub success: bool,
}

#[function_component(Alert)]
pub fn alert(props: &AlertProps) -> Html {
    let class = if props.success {
        "gi-alert gi-alert--success"
    } else {
        "gi-alert gi-alert--error"
    };
    html! { <div class={class}>{ &props.message }</div> }
}
