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
    /// Which order keys are *reachable*. `None` = canonical (all enabled). When a
    /// sequence/monad is loaded, only the orders it contains stay clickable; the
    /// rest are greyed out (impossible in that context). Nullad is always enabled.
    #[prop_or_default]
    pub enabled: Option<Vec<String>>,
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

    // Data · Graph · Table condensed to a single "View" toggle (swaps the view).
    let other_mode = if props.mode == ViewMode::Graph { ViewMode::Table } else { ViewMode::Graph };
    let current_label = if props.mode == ViewMode::Graph { "Graph" } else { "Table" };
    let toggle_view = {
        let on_set_mode = props.on_set_mode.clone();
        Callback::from(move |_| on_set_mode.emit(other_mode))
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
                        // In a sequence context, orders the sequence doesn't contain
                        // are unreachable → greyed out and unclickable.
                        let disabled = props
                            .enabled
                            .as_ref()
                            .is_some_and(|e| !e.contains(&system.name));
                        let system_name = system.name.clone();
                        let onclick = {
                            let on_select = props.on_select.clone();
                            Callback::from(move |_| {
                                on_select.emit(system_name.clone());
                            })
                        };

                        let class = if is_selected {
                            "nav-button selected"
                        } else if disabled {
                            "nav-button disabled"
                        } else {
                            "nav-button"
                        };
                        html! {
                            <button
                                class={ class }
                                disabled={ disabled }
                                onclick={ onclick }
                                title={ if disabled { "Not in this monad's sequence".to_string() } else { system.k_notation.clone() } }
                            >
                                { &system.display_name }
                            </button>
                        }
                    }).collect::<Html>()
                }
            </div>

            // Data · Graph · Table — a bare boolean toggle labelled only "View"
            // (off = Table, on = Graph).
            <button
                class="view-switch"
                onclick={ toggle_view }
                title={ format!("View: {current_label} — click to switch") }
            >
                <span class="view-switch-label">{ "View" }</span>
                <span class={ if props.mode == ViewMode::Graph { "view-switch-track on" } else { "view-switch-track" } }>
                    <span class="view-switch-thumb" />
                </span>
            </button>
        </nav>
    }
}
