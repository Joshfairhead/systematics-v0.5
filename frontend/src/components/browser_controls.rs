//! BrowserControls — the search / sort / filter control bar as a **decoupled view
//! module**. It knows nothing about the SPO model or the table's `ColKey`: it renders
//! from plain display data (chips + predicate tabs) and emits **keyed events**. The
//! controller (reference_browser) computes the data via `spo::` and handles the events.
//! So the view stays decoupled from model/controller (MVC), and the underlying query
//! mechanism can be swapped without touching this module.

use web_sys::HtmlInputElement;
use yew::prelude::*;

/// A toggleable chip — a column (Sort) or an object value (Filter). `key` is emitted
/// on click; `label` is shown; `on` is the selected state.
#[derive(Clone, PartialEq)]
pub struct ChipItem {
    pub key: String,
    pub label: String,
    pub on: bool,
}

/// A predicate tab in the Filter menu. `dotted` marks a predicate with active values.
#[derive(Clone, PartialEq)]
pub struct PredItem {
    pub key: String,
    pub label: String,
    pub active: bool,
    pub dotted: bool,
}

#[derive(Properties, PartialEq)]
pub struct BrowserControlsProps {
    /// Pre-rendered operation buttons (Extract·Load·Transform) and New — pass-through.
    pub elt_btns: Html,
    pub new_btn: Html,
    pub search: String,
    pub on_search: Callback<String>,
    // Sort (=) — which columns show.
    pub sort_open: bool,
    pub on_toggle_sort: Callback<()>,
    pub columns: Vec<ChipItem>,
    pub on_toggle_column: Callback<String>,
    // Filter (−) — the SPO predicate→object query (as plain data).
    pub filter_open: bool,
    pub filter_active: bool,
    pub on_toggle_filter: Callback<()>,
    pub predicates: Vec<PredItem>,
    pub on_select_pred: Callback<String>,
    pub active_pred_label: String,
    pub objects: Vec<ChipItem>,
    pub on_toggle_value: Callback<String>,
    // Status + selection.
    pub shown: usize,
    pub sel_count: usize,
    pub on_delete_selected: Callback<()>,
}

#[function_component(BrowserControls)]
pub fn browser_controls(props: &BrowserControlsProps) -> Html {
    let visible_cols = props.columns.iter().filter(|c| c.on).count();

    let on_search = {
        let cb = props.on_search.clone();
        Callback::from(move |e: InputEvent| {
            cb.emit(e.target_unchecked_into::<HtmlInputElement>().value())
        })
    };
    let toggle_sort = emit_unit(&props.on_toggle_sort);
    let toggle_filter = emit_unit(&props.on_toggle_filter);
    let delete_selected = emit_unit(&props.on_delete_selected);

    html! {
        <>
            <div class="ref-controlbar">
                { props.elt_btns.clone() }
                <input
                    class="ref-search"
                    type="text"
                    placeholder="Search…"
                    value={ props.search.clone() }
                    oninput={ on_search }
                />
                { props.new_btn.clone() }

                <div class="control-pop">
                    <button
                        class={ classes!("control-btn", props.sort_open.then_some("active")) }
                        onclick={ toggle_sort }
                        title="Sort — which columns show"
                    >{ format!("Sort ({visible_cols}) ▾") }</button>
                    if props.sort_open {
                        <div class="control-menu">
                            <span class="facet-label">{ "columns" }</span>
                            <div class="col-chips">
                                { for props.columns.iter().map(|c| chip(c, &props.on_toggle_column)) }
                            </div>
                        </div>
                    }
                </div>

                <div class="control-pop">
                    <button
                        class={ classes!("control-btn", (props.filter_active || props.filter_open).then_some("active")) }
                        onclick={ toggle_filter }
                        title="Filter — an SPO query: pick a predicate, then its objects"
                    >{ if props.filter_active { "Filter ● ▾" } else { "Filter ▾" } }</button>
                    if props.filter_open {
                        <div class="control-menu">
                            <span class="facet-label">{ "predicate (key), stackable" }</span>
                            <div class="facet-tabs">
                                { for props.predicates.iter().map(|p| pred_tab(p, &props.on_select_pred)) }
                            </div>
                            <span class="facet-label">
                                { format!("{} — objects (values)", props.active_pred_label) }
                            </span>
                            <div class="col-chips">
                                if props.objects.is_empty() {
                                    <span class="facet-hint">{ "no values in the current scope" }</span>
                                } else {
                                    { for props.objects.iter().map(|o| chip(o, &props.on_toggle_value)) }
                                }
                            </div>
                        </div>
                    }
                </div>

                <span class="ref-count">{ format!("{} shown", props.shown) }</span>
                if props.sel_count > 0 {
                    <button class="row-delete-btn" onclick={ delete_selected }
                        title="Delete the selected systems / monads / references">
                        { format!("🗑 Delete {} selected", props.sel_count) }
                    </button>
                }
            </div>
        </>
    }
}

/// A generic toggle chip that emits its `key`.
fn chip(item: &ChipItem, on_toggle: &Callback<String>) -> Html {
    let on_toggle = on_toggle.clone();
    let key = item.key.clone();
    let onclick = Callback::from(move |_: MouseEvent| on_toggle.emit(key.clone()));
    html! {
        <button class={ classes!("facet-chip", item.on.then_some("active")) } onclick={ onclick }>
            { &item.label }
        </button>
    }
}

/// A predicate tab that emits its `key` (a ● suffix marks active values).
fn pred_tab(item: &PredItem, on_select: &Callback<String>) -> Html {
    let on_select = on_select.clone();
    let key = item.key.clone();
    let onclick = Callback::from(move |_: MouseEvent| on_select.emit(key.clone()));
    html! {
        <button class={ classes!("facet-tab", item.active.then_some("active")) } onclick={ onclick }>
            { &item.label }{ if item.dotted { " ●" } else { "" } }
        </button>
    }
}

/// Adapt a `Callback<()>` to a `MouseEvent` click handler.
fn emit_unit(cb: &Callback<()>) -> Callback<MouseEvent> {
    let cb = cb.clone();
    Callback::from(move |_: MouseEvent| cb.emit(()))
}
