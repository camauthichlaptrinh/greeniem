use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;

#[derive(Properties, PartialEq)]
pub struct OrderSuccessProps {
    pub id: String,
}

#[function_component(OrderSuccess)]
pub fn order_success(props: &OrderSuccessProps) -> Html {
    html! {
        <div class="gi-page gi-text-center">
            <div class="gi-panel" style="padding: 3rem 2rem;">
                <div style="font-size: 3rem;">{ "✅" }</div>
                <h1>{ "Đặt hàng thành công!" }</h1>
                <p class="gi-muted">
                    { format!("Mã đơn hàng của bạn: #{}", &props.id[..props.id.len().min(8)]) }
                </p>
                <p class="gi-muted">{ "GreenIEM sẽ liên hệ với bạn sớm nhất để xác nhận đơn hàng." }</p>
                <Link<Route> to={Route::Products} classes="gi-btn gi-btn--primary gi-mt-1">{ "Tiếp tục mua sắm" }</Link<Route>>
            </div>
        </div>
    }
}
