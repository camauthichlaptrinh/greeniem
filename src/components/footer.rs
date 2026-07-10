use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="gi-footer">
            <div class="gi-footer__inner">
                <div>
                    <div class="gi-footer__brand">{ "GreenIEM" }</div>
                    <div>{ "IEM · Dongle DAC/AMP · Amplifier · Loa Bookshelf · Phụ kiện âm thanh" }</div>
                </div>
                <div>
                    <div>{ "Hotline: 1900 0000" }</div>
                    <div>{ "support@greeniem.vn" }</div>
                </div>
            </div>
            <div class="gi-container gi-mt-1">
                { format!("© {} GreenIEM. All rights reserved.", 2026) }
            </div>
        </footer>
    }
}
