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
    /// The selected header key (`"nullad"`, `"monad"`, …).
    pub selected: String,
    pub on_select: Callback<String>,
    /// Whether the data (reference) view is active — highlights the Data toggle.
    pub data_mode: bool,
    /// Flip between the graph canvas and the data view.
    pub on_toggle_data: Callback<()>,
}

#[function_component(SystemSelector)]
pub fn system_selector(props: &SystemSelectorProps) -> Html {
    // Nullad (order 0) leads the sequence — the unbounded "all", before Monad.
    let nullad_onclick = {
        let on_select = props.on_select.clone();
        Callback::from(move |_| on_select.emit("nullad".to_string()))
    };
    let nullad_class = if props.selected == "nullad" {
        "nav-button nav-nullad selected"
    } else {
        "nav-button nav-nullad"
    };

    let toggle_data = {
        let on_toggle_data = props.on_toggle_data.clone();
        Callback::from(move |_| on_toggle_data.emit(()))
    };

    html! {
        <nav class="top-nav">
            <div class="nav-items">
                <button
                    class={ nullad_class }
                    onclick={ nullad_onclick }
                    title={ "Nullad (order 0) — all & everything" }
                >{ "Nullad" }</button>
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

            // Data toggle — right of the menu. Switches the viewer into the
            // reference (data) view; the system buttons then filter it.
            <button
                class={ if props.data_mode { "data-toggle active" } else { "data-toggle" } }
                onclick={ toggle_data }
                title={ "Toggle the data (reference) view" }
            >{ "Data" }</button>
        </nav>
    }
}
