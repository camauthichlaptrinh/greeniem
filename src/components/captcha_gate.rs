use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::api;
use crate::components::alert::Alert;
use crate::state::captcha::CaptchaStore;
use crate::types::{CaptchaChallenge, CaptchaVerifyResponse, VerifyCaptchaRequest};

#[derive(Properties, PartialEq)]
pub struct CaptchaGateProps {
    pub on_verified: Callback<()>,
}

/// Shown whenever a public request comes back with `captcha_required` — the
/// backend only asks for this once an IP has tripped the burst threshold, so
/// normal visitors never see it.
#[function_component(CaptchaGate)]
pub fn captcha_gate(props: &CaptchaGateProps) -> Html {
    let challenge = use_state(|| None::<CaptchaChallenge>);
    let answer = use_state(String::new);
    let error = use_state(|| None::<String>);
    let busy = use_state(|| false);

    {
        let challenge = challenge.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(c) = api::get::<CaptchaChallenge>("/captcha").await {
                    challenge.set(Some(c));
                }
            });
            || ()
        });
    }

    let oninput = {
        let answer = answer.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            answer.set(input.value());
        })
    };

    let onsubmit = {
        let challenge = challenge.clone();
        let answer = answer.clone();
        let error = error.clone();
        let busy = busy.clone();
        let on_verified = props.on_verified.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let Some(c) = (*challenge).clone() else { return };
            let Ok(parsed) = answer.parse::<u32>() else {
                error.set(Some("Vui lòng nhập một số".into()));
                return;
            };
            let error = error.clone();
            let busy = busy.clone();
            let challenge_state = challenge.clone();
            let answer_state = answer.clone();
            let on_verified = on_verified.clone();
            busy.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let req = VerifyCaptchaRequest {
                    challenge_id: c.challenge_id.clone(),
                    answer: parsed,
                };
                match api::post::<_, CaptchaVerifyResponse>("/captcha/verify", &req).await {
                    Ok(resp) => {
                        CaptchaStore::set(&resp.captcha_token);
                        busy.set(false);
                        on_verified.emit(());
                    }
                    Err(err) => {
                        busy.set(false);
                        error.set(Some(err.display()));
                        answer_state.set(String::new());
                        // fetch a fresh question since the old one is now consumed
                        let challenge_state = challenge_state.clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            if let Ok(c) = api::get::<CaptchaChallenge>("/captcha").await {
                                challenge_state.set(Some(c));
                            }
                        });
                    }
                }
            });
        })
    };

    html! {
        <div class="gi-captcha-box gi-panel">
            <p>{ "Hệ thống phát hiện lượng truy cập bất thường. Vui lòng xác minh để tiếp tục." }</p>
            if let Some(c) = &*challenge {
                <div class="gi-captcha-box__question">{ &c.question }</div>
                <form class="gi-form" onsubmit={onsubmit}>
                    <input
                        class="gi-input"
                        type="text"
                        inputmode="numeric"
                        placeholder="Nhập kết quả"
                        value={(*answer).clone()}
                        oninput={oninput}
                    />
                    if let Some(e) = &*error {
                        <Alert message={e.clone()} />
                    }
                    <button class="gi-btn gi-btn--primary gi-btn--block" type="submit" disabled={*busy}>
                        { if *busy { "Đang xác minh..." } else { "Xác minh" } }
                    </button>
                </form>
            } else {
                <div class="gi-loading"><span class="gi-spinner"></span></div>
            }
        </div>
    }
}
