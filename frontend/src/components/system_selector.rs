use yew::prelude::*;

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
}

#[function_component(SystemSelector)]
pub fn system_selector(props: &SystemSelectorProps) -> Html {
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
        </nav>
    }
}
