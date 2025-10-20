use std::time::Duration;

use crate::controller::Controller;
use leptos::task::spawn_local;
use leptos::{leptos_dom::logging::console_log, prelude::*};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct SerialArgs {
    port_name: String,
    baudrate: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

use serde_wasm_bindgen::from_value;
use leptos::*;


#[component]
pub fn App() -> impl IntoView {
    #[derive(Clone, PartialEq)]
    enum ConnectionStatus {
        Connected,
        NotConnected,
        Failed,
    }

    let theme = RwSignal::new(Theme::Light);

    let (rotation, setRotation) = signal(0);
    let (rightBicepContraction, setRightBicepContraction) = signal(25);
    let (rightFrontArmContraction, setRightFrontArmContraction) = signal(25);
    let (leftBicepContraction, setLeftBicepContraction) = signal(25);
    let (leftFrontArmContraction, setLeftFrontArmContraction) = signal(25);
    let (connectionStatus, setConnectionStatus) = signal(ConnectionStatus::NotConnected);
    let hovered = RwSignal::new(0);
    let selected = RwSignal::new(0);

    let is_animating = RwSignal::new(false);

    #[derive(Debug, Clone, Deserialize)]
    struct SerialPortInfo {
        port_name: String,
        port_type: String,
    }

    let (ports, set_ports) = signal(Vec::<SerialPortInfo>::new());
    let (menu_open, set_menu_open) = signal(false);
    let (chosen_port, set_chosen_port) = signal("".to_string());

    fn load_serial_ports(set_ports: WriteSignal<Vec<SerialPortInfo>>) {
        spawn_local(async move {
            let res = invoke_without_args("fetch_serial_ports").await;
            match from_value::<Vec<SerialPortInfo>>(res) {
                Ok(list) => {
                    leptos::logging::log!("Ports reçus : {:?}", list);
                    set_ports.set(list);
                }
                Err(e) => {
                    leptos::logging::log!("Erreur de parsing : {:?}", e);
                }
            }
        });
    }

    let changeThemeClick = move |_| {
        is_animating.set(true);

        // Après 300ms : changer le thème
        set_timeout(
            {
                let theme = theme.clone(); // clone si nécessaire
                move || {
                    theme.update(|t| {
                        *t = match t {
                            Theme::Light => Theme::Dark,
                            Theme::Dark => Theme::Light,
                        };
                    });
                }
            },
            Duration::from_millis(300),
        );

        // Après 600ms : arrêter l'animation
        set_timeout(
            move || {
                is_animating.set(false);
            },
            Duration::from_millis(1000),
        );
    };

    let listSerialPortsClick = move |_| {
        load_serial_ports(set_ports);
        set_menu_open.update(|v| *v = !*v);
    };
    



    fn calculateBicepCurve(wanted_contraction: f32) -> f32 {
        let a = 1.0 / 750.0;
        let b = -0.433333;
        let c = 10.0;
        a * wanted_contraction * wanted_contraction + b * wanted_contraction + c
    }

    fn calculateBicepIn(wanted_contraction: f32) -> f32 {
        let a = -0.0032;
        let b = 0.56;
        let c = 60.0;
        a * wanted_contraction * wanted_contraction + b * wanted_contraction + c
    }

    fn calculateFrontArmCurve(wanted_contraction: f32) -> f32 {
        let a = 0.002;
        let b = -0.49;
        let c = -1.0;
        a * wanted_contraction * wanted_contraction + b * wanted_contraction + c
    }

    fn calculateFrontArmIn(wanted_contraction: f32) -> f32 {
        let a = -0.0001333;
        let b = 0.20333;
        let c = 80.0;
        a * wanted_contraction * wanted_contraction + b * wanted_contraction + c
    }

    fn generate_bicep_curve_path(contraction: f32) -> String {
        let y = calculateBicepCurve(contraction) * 0.8;
        console_log(&y.to_string());
        format!(
            "
                M 15 10
                c {} 40 {} 60 -3.9 100.1 
                c 0.3 0.5 2.8 -1.4 5.5 -4.1 
                c 2.8 -2.8 5.5 -4.8 6 -4.6 
                c 0.5 0.1 2 1.8 3.3 3.7 
                l 0 3.5 l -5 5.1 
                l -8 4
                l -9.5 0
                C {} 80 {} 40 6 15
                l 0 -5
                l 9.5 0
                z",
            y, y, y, y
        )
    }

    fn generate_bicep_in_path(contraction: f32) -> String {
        let y = calculateBicepIn(contraction);
        console_log(&y.to_string());
        format!(
            "
                M 61.1 108.6 
                c -1.2 -1.8 -2.1 -3.8 -2.1 -4.3 
                c 0 -0.5 2.7 -2.5 6.1 -4.3 
                l 6.1 -3.5 
                C {} 60 {} 30 70 15
                l 0 -15 
                l 9 0 
                l 0 15
                C {} 30 {} 50 81 90
                l 0 13
                l -8 5
                c -4.4 2.6 -8.3 4.7 -8.7 4.7 
                c -0.4 0 -1.7 -1.5 -2.8 -3.4 z
        ",
            y,
            y,
            y + 8.0,
            y + 8.0
        )
    }

    fn generate_front_arm_curve_path(contraction: f32) -> String {
        let y = calculateFrontArmCurve(contraction);
        console_log(&y.to_string());
        format!(
            "
            M 3.1 7.2 
            C {} 45 {} 80 23.1 195 
            l 2.2 5 l 5 0 l 5 0 l -0.6 -3.7 
            C {} 130 {} 45 13.4 7.3 
            l 1.3 -6.3 l -5.3 0 l -5.2 0 l -1.1 6.2 
            z",
            y,
            y / 3.0,
            y / 3.0 + 25.0,
            y * 0.7
        )
    }

    fn generate_front_arm_in_path(contraction: f32) -> String {
        let y = calculateFrontArmIn(contraction);
        console_log(&y.to_string());
        format!(
            "
            M 71 0 
            l 1 5 
            C {} 30 {} 115.2 62 180 
            l -1 5.4 l 8.9 2.5 l 1.5 -5 
            C {} 110 {} 40 83 5 
            l -1 -5 l -11 0 
            z",
            y,
            y,
            y + 1.0,
            y * 1.2
        )
    }

    let path_right_bicep_curve =
        move || generate_bicep_curve_path(rightBicepContraction.get() as f32);
    let path_right_bicep_in = move || generate_bicep_in_path(rightBicepContraction.get() as f32);

    let path_left_bicep_curve =
        move || generate_bicep_curve_path(leftBicepContraction.get() as f32);
    let path_left_bicep_in = move || generate_bicep_in_path(leftBicepContraction.get() as f32);

    let path_right_front_arm_curve =
        move || generate_front_arm_curve_path(rightFrontArmContraction.get() as f32);
    let path_right_front_arm_in =
        move || generate_front_arm_in_path(rightFrontArmContraction.get() as f32);

    let path_left_front_arm_curve =
        move || generate_front_arm_curve_path(leftFrontArmContraction.get() as f32);
    let path_left_front_arm_in =
        move || generate_front_arm_in_path(leftFrontArmContraction.get() as f32);

    let view_for_selection = move || {
        match selected.get() {
        1 => view! {
            <Show when=move || {selected.get() == 1}>
                <Controller part_index=1 theme=theme min=0 max=100 step=5 set_value=setRightBicepContraction value=rightBicepContraction title="Right Bicep Controllers".to_string()/>
            </Show>
        }.into_any(),
        2 => view! {
            <Show when=move || {selected.get() == 2}>
                <Controller part_index=2 theme=theme min=0 max=100 step=5 set_value=setRightFrontArmContraction value=rightFrontArmContraction title="Right Front Arm Controllers".to_string()/>
            </Show>
        }.into_any(),
        3 => view! {
            <Show when=move || {selected.get() == 3}>
                <Controller part_index=3 theme=theme min=0 max=100 step=5 set_value=setLeftBicepContraction value=leftBicepContraction title="Left Bicep Controllers".to_string()/>
            </Show>
        }.into_any(),
        4 => view! {
            <Show when=move || {selected.get() == 4}>
                <Controller part_index=4 theme=theme min=0 max=100 step=5 set_value=setLeftFrontArmContraction value=leftFrontArmContraction title="Left Front Arm Controllers".to_string()/>
            </Show>
        }.into_any(),
        _ => view! {
            <p>""</p>
        }.into_any(),
    }
    };

    view! {
        <div style=move || format!("display: flex; width: 100vw; height: 100vh; background-color: {}; display: flex; flex-direction: row; justify-content: space-around; align-items: center;", if theme.get() == Theme::Dark {"rgb(30, 30, 30)"} else {"rgb(233, 233, 233)"})>
                <div style="position:absolute; top:0; right:0; width: 25vw; height:10vh; display: flex; flex-direction: row-reverse; justify-content: space-between;">
                    <div style="width: 5vw;height: 10vh; display: flex; justify-content: center; align-items: center" on:click=changeThemeClick>
                        <img
                            style=move || format!("position: absolute;animation: {}; width: 40%; height: 40%; opacity: {}", if is_animating.get() == true {"rotate 1s linear infinite"} else {""}, if theme.get() == Theme::Dark { 1.0 } else { 0.0 })
                            src="/assets/sun.svg"
                        />
                        <img
                            style=move || format!("position: absolute;animation: {}; width: 40%; height: 40%; opacity: {}", if is_animating.get() == true {"rotate 1s linear infinite"} else {""}, if theme.get() == Theme::Dark { 0.0 } else { 1.0 })
                            src="/assets/moon.svg"
                        />
                    </div>
                    <Show when=move || {menu_open.get() == true}>
                            <ul style="background: white; border: 1px solid gray; list-style: none; padding: 0; margin: 0; z-index: 1000; width: 200px">
                                <For  each=move || ports.get() key=|p| p.port_name.clone() let:p>
                                    {
                                        let port_name = p.port_name.clone();

                                        view! {
                                            <li
                                                style=move || format!(
                                                    "background-color: {}; padding: 0.5rem; cursor: pointer; border-bottom: 1px solid #ddd;",
                                                    if chosen_port.get() == p.port_name {
                                                        "#6acbfc"
                                                    } else {
                                                        "white"
                                                    }
                                                )
                                                on:click=move |_| {
                                                    set_chosen_port.set(port_name.clone());
                                                }
                                            >
                                                {port_name.clone()}
                                            </li>
                                        }
                                    }
                                </For>
                            </ul>
                    </Show>
                    <div style="z-index: 3; width: 5vw;height: 10vh; display: flex; justify-content: center; align-items: center">
                        <img
                            on:click=listSerialPortsClick  
                            on:mouseenter=move |_| {
                                hovered.update(|v| *v=9);
                            } 
                            on:mouseleave=move |_| {hovered.update(|v| *v = 0);}
                            style="position: absolute;width: 35%; height: 35%;" 
                            src=move || format!("/assets/{}.svg", if hovered.get()==9 {"plan-grey"} else {if theme.get() == Theme::Dark {"plan-white"} else {"plan"}})
                        />
                    </div>
                    
                </div>
                <div style=move || format!("width: 55%; aspect-ratio: 1/1; position: relative;transform-style: preserve-3d; transform: rotateY({}deg);", rotation.get())>
                    <img src=move || format!("/assets/{}", if theme.get()==Theme::Dark {"upper-body-white.svg"} else {"upper-body.svg"} ) style="width: 100%; aspect-ratio: 1/1;object-fit: contain; position: absolute; left: -1.05%;" />

                    <div
                        class="right_bicep_hover"
                        on:click=move |_| {
                            selected.update(|v| if *v == 1 {*v=0} else {*v=1});
                        }
                        on:mouseenter=move |_| {
                            hovered.update(|v| *v = 1);
                        }
                        on:mouseleave=move |_| {
                            hovered.update(|v| *v = 0);
                        }
                    >
                        <svg viewBox="-8 0 100 120" xmlns="http://www.w3.org/2000/svg" style={"width: 100%; height: 100%;"}>
                            <g transform="scale(1.1, -1.1) translate(2.3, -116)" fill=move || if selected.get()==1 || hovered.get()==1 {"#397bed"} else {if theme.get() == Theme::Dark {"white"} else {"black"}}>
                                <path
                                    id="right-bicep-curve"
                                    d={path_right_bicep_curve}
                                    stroke="none"
                                    transform="translate(3, -2)"
                                />
                                <path
                                    id="right-bicep-in"
                                    d={path_right_bicep_in}
                                    stroke="none"
                                    transform="translate(-4, -6)"
                                />
                            </g>
                        </svg>
                    </div>

                    <div
                        class="right_front_arm_hover"
                        on:click=move |_| {
                            selected.update(|v| if *v == 2 {*v=0} else {*v=2});
                        }
                        on:mouseenter=move |_| {
                            hovered.update(|v| *v = 2);
                        }
                        on:mouseleave=move |_| {
                            hovered.update(|v| *v = 0);
                        }
                    >
                        <svg viewBox="-8 0 87 199" xmlns="http://www.w3.org/2000/svg" style={"width: 100%; height: 100%;"}>
                            <g transform="scale(1, 1) translate(2, 0)" fill=move || if selected.get()==2 || hovered.get()==2 {"#397bed"} else {if theme.get() == Theme::Dark {"white"} else {"black"}}>
                                <path
                                    id="right-front-arm-curve"
                                    d={path_right_front_arm_curve}
                                    stroke="none"
                                    transform="translate(-8, -2)"
                                />

                                <path
                                    id="right-front-arm-in"
                                    d={path_right_front_arm_in}
                                    stroke="none"
                                    transform="translate(-8.5, 0)"
                                />
                            </g>
                        </svg>
                    </div>

                    <div
                        class="left_bicep_hover"
                        on:click=move |_| {
                            selected.update(|v| if *v == 3 {*v=0} else {*v=3});
                        }
                        on:mouseenter=move |_| {
                            hovered.update(|v| *v = 3);
                        }
                        on:mouseleave=move |_| {
                            hovered.update(|v| *v = 0);
                        }
                    >
                        <svg viewBox="-8 0 100 120" xmlns="http://www.w3.org/2000/svg" style={"width: 100%; height: 100%;"}>
                            <g transform="scale(-1.1, -1.1) translate(-75.5, -114)" fill=move || if selected.get()==3 || hovered.get()==3 {"#397bed"} else {if theme.get() == Theme::Dark {"white"} else {"black"}}>
                                <path
                                    id="right-bicep-curve"
                                    d={path_left_bicep_curve}
                                    stroke="none"
                                    transform="translate(3, -2)"
                                />
                                <path
                                    id="right-bicep-in"
                                    d={path_left_bicep_in}
                                    stroke="none"
                                    transform="translate(-4, -4)"
                                />
                            </g>
                        </svg>
                    </div>

                    <div
                        class="left_front_arm_hover"
                        on:click=move |_| {
                            selected.update(|v| if *v == 4 {*v=0} else {*v=4});
                        }
                        on:mouseenter=move |_| {
                            hovered.update(|v| *v = 4);
                        }
                        on:mouseleave=move |_| {
                            hovered.update(|v| *v = 0);
                        }
                    >
                        <svg viewBox="-8 0 87 199" xmlns="http://www.w3.org/2000/svg" style={"width: 100%; height: 100%;"}>
                            <g transform="scale(-1, 1) translate(-71, 0) " fill=move || if selected.get()==4 || hovered.get()==4 {"#397bed"} else {if theme.get() == Theme::Dark {"white"} else {"black"}}>
                                <path
                                    id="left-front-arm-curve"
                                    d={path_left_front_arm_curve}
                                    stroke="none"
                                    transform="translate(-8, -2)"
                                />

                                <path
                                    id="left-front-arm-in"
                                    d={path_left_front_arm_in}
                                    stroke="none"
                                    transform="translate(-8.5, 1)"
                                />
                            </g>
                        </svg>
                    </div>

                </div>
            <div class="controlCard" style=move || format!("background-color: {};", if theme.get() == Theme::Dark {"rgb(23, 23, 23)"} else {"white"})>
                <Show when=move || {selected.get() != 0}>
                        {view_for_selection}
                </Show>
                <div class="rotation_div" style="display: flex; width:50%; flex-direction: column; justify-content: center; margin-top: 50px;">
                    <p style=move || format!("font-size: 1.5rem; font-weight: 700; color: {};", if theme.get() == Theme::Dark {"white"} else {"rgb(23, 23, 23)"})>"Rotation : " {rotation} </p>
                    <input style="width:100%; background-color: white;" type="range" min=-180 max=180 step=5 value=rotation on:input=move |ev| {
                        let val = event_target_value(&ev)
                           .parse::<i16>()
                           .unwrap_or(0);
                           setRotation.set(val);
                        }
                    />

                </div>

                <Show when=move || {connectionStatus.get() == ConnectionStatus::NotConnected} >
                    <button on:click=move |_| {
                        spawn_local(async move {
                            let args = to_value(&SerialArgs {port_name: chosen_port.get(), baudrate: 9600}).unwrap();
                            let connection_res = invoke("init_serial", args).await.as_bool().unwrap();
                            match connection_res {
                                true => setConnectionStatus.set(ConnectionStatus::Connected),
                                false => setConnectionStatus.set(ConnectionStatus::Failed)
                            }
                        });
                    } class="button-9" style="background-color: #00aeff" role="button">"CONNECT"</button>
                </Show>
                <Show when=move || {connectionStatus.get() == ConnectionStatus::Connected} >
                    <button on:click=move |_| {} class="button-9" style="background-color: #4dc35d" role="button">"CONNECTED"</button>
                </Show>
                <Show when=move || {connectionStatus.get() == ConnectionStatus::Failed} >
                    <button on:click=move |_| {
                        spawn_local(async move {
                            let args = to_value(&SerialArgs {port_name: chosen_port.get(), baudrate: 9600}).unwrap();
                            let connection_res = invoke("init_serial", args).await.as_bool().unwrap();
                            match connection_res {
                                true => setConnectionStatus.set(ConnectionStatus::Connected),
                                false => setConnectionStatus.set(ConnectionStatus::Failed)
                            }
                        });
                    } class="button-9" style="background-color:rgb(221, 27, 27)" role="button">"CONNECTION FAILED"</button>
                </Show>
            </div>
        </div>
    }
}
