use leptos::{IntoView, component, prelude::*, task::spawn_local, view};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

use crate::app::Theme;



#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct VibrationArgs {
    data: u8,
}
#[derive(Serialize, Deserialize)]
struct ContractionArgs {
    contraction: u8,
    part_index: u8
}

#[component]
pub fn Controller(
    min: u8,
    #[prop(default = 100)] max: u8,
    step:u8,
    set_value: WriteSignal<u8>,
    value: ReadSignal<u8>,
    title: String,
    part_index: u8,
    theme: RwSignal<Theme>
) -> impl IntoView {
    view! {
        <p style=move || format!("font-size: 2.5rem; font-weight: 900; margin-bottom: 10vh; text-align: center; color: {};", if theme.get() == Theme::Dark {"white"} else {"#333"})>
                {title}
        </p>
        <div style="display: flex; width: 50%; flex-direction: column; justify-content: center;">
            <p style=move || format!("font-size: 1.5rem; font-weight: 700; color: {};", if theme.get() == Theme::Dark {"white"} else {"#333"})>"Contraction : " {value}</p>
            <input 
                style="width: 100%;" 
                type="range" 
                value=value 
                min=min 
                max=max 
                step=step 
                on:input=move |ev| {
                    let val = event_target_value(&ev).parse::<u8>().unwrap_or(0);
                    set_value.set(val);
                    spawn_local(async move {
                        let args = to_value(&ContractionArgs {contraction: val, part_index: part_index}).unwrap();
                        let _ = invoke("send_contraction", args).await;
                    });
                }
            />
        </div>

        <button on:click=move |_|
            {
                spawn_local(async move {
                    let data = part_index;
                    let args = to_value(&VibrationArgs {data}).unwrap();
                    let _ = invoke("send_vibration", args).await;
                });
            }
            class="button-9" role="button">
            "VIBRATE"
        </button>
    }
}
