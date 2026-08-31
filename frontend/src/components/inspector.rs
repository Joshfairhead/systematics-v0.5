//! Inspector — a self-contained view module (a prototype micro-module) that shows a
//! pinned **subject**'s quads grouped by predicate → ordinality → object, each with its
//! citation. Reciprocal traversal: "everything at node 1 → its terms across
//! perspectives; scope by source → one."
//!
//! It takes only **plain, pre-grouped data** (no SPO/triple-store dependency) plus a
//! close callback — so it composes with the browser without a back-dependency. In the
//! architecture octad this is the *Organisational-Modes ↔ Necessary-Resourcing* link.

use yew::prelude::*;

/// One object value and the citation(s) that assert it (deduped).
#[derive(Clone, PartialEq)]
pub struct ObjCite {
    pub object: String,
    pub citations: Vec<String>,
}

/// The objects at one topological ordinality (node index or edge `base-target`).
#[derive(Clone, PartialEq)]
pub struct PosGroup {
    pub ordinality: String,
    pub objects: Vec<ObjCite>,
}

/// One predicate's objects, grouped by ordinality. `is_edge` picks the ordinality label
/// (`edge 1-2` for connectives vs `node 1` for terms); `label` is the display name.
#[derive(Clone, PartialEq)]
pub struct PredGroup {
    pub label: String,
    pub is_edge: bool,
    pub positions: Vec<PosGroup>,
}

#[derive(Properties, PartialEq)]
pub struct InspectorProps {
    pub subject_name: String,
    /// Predicate groups, already in systematics order_cardinality (name · order_cardinality · coherence ·
    /// term-designation · connective-designation · term · connective · source).
    pub groups: Vec<PredGroup>,
    pub on_close: Callback<()>,
}

#[function_component(Inspector)]
pub fn inspector(props: &InspectorProps) -> Html {
    // Citations are optional — hidden by default for a clean read (user preference).
    let show_cites = use_state(|| false);
    let toggle_cites = {
        let show_cites = show_cites.clone();
        Callback::from(move |_: MouseEvent| show_cites.set(!*show_cites))
    };
    let close = {
        let on_close = props.on_close.clone();
        Callback::from(move |_: MouseEvent| on_close.emit(()))
    };
    let cites_on = *show_cites;

    html! {
        <div class="inspect-panel">
            <div class="inspect-head">
                <span class="inspect-title">{ &props.subject_name }</span>
                <span class="inspect-actions">
                    <button class="inspect-toggle" onclick={ toggle_cites }>
                        { if cites_on { "Citations ▾" } else { "Citations ▸" } }
                    </button>
                    <button class="row-delete" onclick={ close } title="Close">{ "✕" }</button>
                </span>
            </div>
            { for props.groups.iter().map(|g| render_pred(g, cites_on)) }
        </div>
    }
}

/// One predicate row: its label, then each ordinality on its own line, each object
/// (with its citation when expanded) one per line.
fn render_pred(g: &PredGroup, cites_on: bool) -> Html {
    html! {
        <div class="inspect-row">
            <span class="inspect-pred">{ &g.label }</span>
            <div class="inspect-positions">
                { for g.positions.iter().map(|p| {
                    let pos_label = if p.ordinality.is_empty() {
                        String::new()
                    } else if g.is_edge {
                        format!("edge {}", p.ordinality)
                    } else {
                        format!("node {}", p.ordinality)
                    };
                    html! {
                        <div class="inspect-posrow">
                            { if pos_label.is_empty() { html!{} } else { html!{ <span class="inspect-pos">{ pos_label }</span> } } }
                            <div class="inspect-objs">
                                { for p.objects.iter().map(|o| {
                                    let cite = o.citations.join(" ; ");
                                    html! {
                                        <div class="inspect-obj">
                                            <span class="tag tag-coherence">{ &o.object }</span>
                                            { if cites_on && !cite.is_empty() {
                                                html!{ <span class="inspect-cite">{ cite }</span> }
                                            } else { html!{} } }
                                        </div>
                                    }
                                }) }
                            </div>
                        </div>
                    }
                }) }
            </div>
        </div>
    }
}
