use yew::prelude::*;

use crate::api::client::InstanceSystem;

/// Simple display config for system selector (UI only)
#[derive(Clone, PartialEq)]
pub struct SystemDisplay {
    pub name: String,
    pub display_name: String,
    pub k_notation: String,
}

#[derive(Properties, PartialEq)]
pub struct SystemSelectorProps {
    pub systems: Vec<SystemDisplay>,
    pub selected: String,
    pub on_select: Callback<String>,
    /// Non-canonical instance systems, offered by the Load control at the right
    /// of the menu bar (alongside the 12 canonical categories).
    #[prop_or_default]
    pub instance_systems: Vec<InstanceSystem>,
    /// Load an instance system (by id) into the canvas.
    #[prop_or_default]
    pub on_load: Option<Callback<String>>,
}

#[function_component(SystemSelector)]
pub fn system_selector(props: &SystemSelectorProps) -> Html {
    let load_open = use_state(|| false);

    html! {
        <nav class="top-nav">
            <div class="nav-items">
                {
                    props.systems.iter().map(|system| {
                        let is_selected = system.name == props.selected;
                        let system_name = system.name.clone();
                        let onclick = {
                            let on_select = props.on_select.clone();
                            Callback::from(move |_| {
                                on_select.emit(system_name.clone());
                            })
                        };

                        html! {
                            <button
                                class={ if is_selected { "nav-button selected" } else { "nav-button" } }
                                onclick={ onclick }
                                title={ system.k_notation.clone() }
                            >
                                { &system.display_name }
                            </button>
                        }
                    }).collect::<Html>()
                }
            </div>

            if let Some(on_load) = props.on_load.clone() {
                <div class="nav-load">
                    <button
                        class="load-button"
                        onclick={ { let o = load_open.clone(); Callback::from(move |_| o.set(!*o)) } }
                    >{ if *load_open { "Load ▴" } else { "Load ▾" } }</button>
                    if *load_open {
                        <div class="load-menu">
                            { for props.instance_systems.iter().map(|inst| {
                                let id = inst.id.clone();
                                let on_load = on_load.clone();
                                let o = load_open.clone();
                                let onclick = Callback::from(move |_| {
                                    on_load.emit(id.clone());
                                    o.set(false);
                                });
                                html! {
                                    <button class="load-item" onclick={ onclick }>
                                        { format!("{} · {}", inst.order, inst.name) }
                                    </button>
                                }
                            }) }
                        </div>
                    }
                </div>
            }
        </nav>
    }
}
