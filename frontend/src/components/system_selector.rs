use yew::prelude::*;

use crate::app::ViewMode;

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
    /// Which view of the Data is active (Graph or Table).
    pub mode: ViewMode,
    /// Switch the Data's view (Data · Graph · Table triad).
    pub on_set_mode: Callback<ViewMode>,
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

    let set_graph = {
        let on_set_mode = props.on_set_mode.clone();
        Callback::from(move |_| on_set_mode.emit(ViewMode::Graph))
    };
    let set_table = {
        let on_set_mode = props.on_set_mode.clone();
        Callback::from(move |_| on_set_mode.emit(ViewMode::Table))
    };
    let graph_class = if props.mode == ViewMode::Graph { "data-toggle active" } else { "data-toggle" };
    let table_class = if props.mode == ViewMode::Table { "data-toggle active" } else { "data-toggle" };

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

            // Data · Graph · Table — right of the menu. Switches how the scoped
            // Data is viewed (a graph, or a table).
            <div class="data-switch" title="Data — view as Graph or Table">
                <button class={ graph_class } onclick={ set_graph }>{ "Graph" }</button>
                <button class={ table_class } onclick={ set_table }>{ "Table" }</button>
            </div>
        </nav>
    }
}
