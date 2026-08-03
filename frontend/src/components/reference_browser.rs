//! Reference Browser — the **Nullad page**: the raw registry, a data view over
//! every citation in the tool.
//!
//! It also hosts the **ELT triad** (Extract · Load · Transform) — the operation
//! edge of the architecture. Load is wired (`loadPerspective` → `on_load`);
//! Extract and Transform are the other two edges of the triad, present but not
//! yet wired to backend operations (labelled honestly as such).
//!
//! Two tabs over the enriched `allReferences` data:
//!  * **Table**: all references, sortable by column and filterable by
//!    **button facets** (perspective / source / artefact / order) with a
//!    free-text search — the provenance audit view. Each row carries its
//!    **citation triad** (source · locator · artefact) as clickable tags.
//!  * **Compare**: a matrix of order × perspective, each cell showing how that
//!    perspective characterizes that order's system (its coherence value) with
//!    the citation as a tooltip — the comparative lens.
//!
//! The grouping key is the target system's *order* (perspectives cite their own
//! system ids, not a shared canonical address), resolved server-side into
//! `ReferenceView.target_system`.

use std::collections::{BTreeMap, BTreeSet};

use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::api::client::{InstanceSystem, ReferenceView};

/// Standard order labels (the invariant systematics names), used to label the
/// compare-matrix rows regardless of how each perspective names its systems.
const ORDER_NAMES: [&str; 12] = [
    "Monad", "Dyad", "Triad", "Tetrad", "Pentad", "Hexad", "Heptad", "Octad",
    "Ennead", "Decad", "Undecad", "Dodecad",
];

fn order_name(order: i32) -> String {
    ORDER_NAMES
        .get((order - 1) as usize)
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("order {order}"))
}

#[derive(Properties, PartialEq)]
pub struct ReferenceBrowserProps {
    pub references: Vec<ReferenceView>,
    /// Non-canonical instance systems, offered by the Load edge of the ELT triad.
    #[prop_or_default]
    pub instance_systems: Vec<InstanceSystem>,
    /// Load an instance system (by id) into the graph.
    pub on_load: Callback<String>,
}

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    Table,
    Compare,
}

#[derive(Clone, Copy, PartialEq)]
enum SortCol {
    Perspective,
    Source,
    Artefact,
    Order,
    Fragment,
}

impl SortCol {
    fn label(self) -> &'static str {
        match self {
            SortCol::Perspective => "Perspective",
            SortCol::Source => "Source",
            SortCol::Artefact => "Artefact",
            SortCol::Order => "Order",
            SortCol::Fragment => "Cites",
        }
    }
}

// -- small field accessors (references carry Option-wrapped nested data) --
fn persp(r: &ReferenceView) -> String {
    r.perspective_name.clone().unwrap_or_default()
}
fn src(r: &ReferenceView) -> String {
    r.source.as_ref().map(|s| s.name.clone()).unwrap_or_default()
}
fn art(r: &ReferenceView) -> String {
    r.artefact.as_ref().map(|a| a.title.clone()).unwrap_or_default()
}
fn loc(r: &ReferenceView) -> String {
    r.lookup.as_ref().map(|l| l.locator.clone()).unwrap_or_default()
}
fn order_of(r: &ReferenceView) -> Option<i32> {
    r.target_system.as_ref().map(|s| s.order)
}
fn frag(r: &ReferenceView) -> String {
    r.target_fragment.clone().unwrap_or_default()
}
fn provenance(r: &ReferenceView) -> String {
    let s = src(r);
    let l = loc(r);
    if l.is_empty() {
        s
    } else if s.is_empty() {
        l
    } else {
        format!("{s} · {l}")
    }
}

#[function_component(ReferenceBrowser)]
pub fn reference_browser(props: &ReferenceBrowserProps) -> Html {
    let tab = use_state(|| Tab::Table);
    let sort_col = use_state(|| SortCol::Order);
    let sort_asc = use_state(|| true);
    let f_perspective = use_state(|| Option::<String>::None);
    let f_source = use_state(|| Option::<String>::None);
    let f_artefact = use_state(|| Option::<String>::None);
    let f_order = use_state(|| Option::<i32>::None);
    let search = use_state(String::new);
    let load_open = use_state(|| false);

    let refs = &props.references;

    // Distinct facet values.
    let perspectives: BTreeSet<String> =
        refs.iter().map(persp).filter(|s| !s.is_empty()).collect();
    let sources: BTreeSet<String> = refs.iter().map(src).filter(|s| !s.is_empty()).collect();
    let artefacts: BTreeSet<String> = refs.iter().map(art).filter(|s| !s.is_empty()).collect();
    let orders: BTreeSet<i32> = refs.iter().filter_map(order_of).collect();

    let switch_tab = {
        let tab = tab.clone();
        move |t: Tab| {
            let tab = tab.clone();
            Callback::from(move |_: MouseEvent| tab.set(t))
        }
    };

    html! {
        <div class="reference-browser">
            // ELT triad — the operation edge. Extract · Load · Transform.
            { elt_triad(&props.instance_systems, &props.on_load, &load_open) }

            <div class="ref-tabs">
                <button
                    class={ if *tab == Tab::Table { "ref-tab active" } else { "ref-tab" } }
                    onclick={ switch_tab(Tab::Table) }
                >{ "Table" }</button>
                <button
                    class={ if *tab == Tab::Compare { "ref-tab active" } else { "ref-tab" } }
                    onclick={ switch_tab(Tab::Compare) }
                >{ "Compare by order" }</button>
                <span class="ref-count">{ format!("{} references", refs.len()) }</span>
            </div>
            {
                match *tab {
                    Tab::Table => table_view(TableCtx {
                        refs,
                        sort_col: &sort_col,
                        sort_asc: &sort_asc,
                        f_perspective: &f_perspective,
                        f_source: &f_source,
                        f_artefact: &f_artefact,
                        f_order: &f_order,
                        search: &search,
                        perspectives: &perspectives,
                        sources: &sources,
                        artefacts: &artefacts,
                        orders: &orders,
                    }),
                    Tab::Compare => compare_view(refs),
                }
            }
        </div>
    }
}

/// The ELT triad control: Extract · Load · Transform. Load opens a menu of
/// instance systems; Extract and Transform are the triad's other two edges,
/// present but not yet wired to backend operations.
fn elt_triad(
    instance_systems: &[InstanceSystem],
    on_load: &Callback<String>,
    load_open: &UseStateHandle<bool>,
) -> Html {
    let toggle = {
        let o = load_open.clone();
        Callback::from(move |_: MouseEvent| o.set(!*o))
    };

    html! {
        <div class="elt-triad" title="Operation edge (ELT): Extract · Load · Transform">
            <button
                class="elt-btn"
                disabled=true
                title="Extract — select systems from the Nullad to link into a Monad (an active filter). Not yet wired."
            >{ "Extract" }</button>

            <div class="elt-load">
                <button class="elt-btn" onclick={ toggle }>
                    { if **load_open { "Load ▴" } else { "Load ▾" } }
                </button>
                if **load_open {
                    <div class="load-menu">
                        if instance_systems.is_empty() {
                            <span class="load-empty">{ "no instance systems" }</span>
                        }
                        { for instance_systems.iter().map(|inst| {
                            let id = inst.id.clone();
                            let on_load = on_load.clone();
                            let o = load_open.clone();
                            let onclick = Callback::from(move |_: MouseEvent| {
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

            <button
                class="elt-btn"
                disabled=true
                title="Transform — apply a Functor to a loaded system. Not yet wired."
            >{ "Transform" }</button>
        </div>
    }
}

/// Grouped arguments for the table view (keeps the signature under one struct).
struct TableCtx<'a> {
    refs: &'a [ReferenceView],
    sort_col: &'a UseStateHandle<SortCol>,
    sort_asc: &'a UseStateHandle<bool>,
    f_perspective: &'a UseStateHandle<Option<String>>,
    f_source: &'a UseStateHandle<Option<String>>,
    f_artefact: &'a UseStateHandle<Option<String>>,
    f_order: &'a UseStateHandle<Option<i32>>,
    search: &'a UseStateHandle<String>,
    perspectives: &'a BTreeSet<String>,
    sources: &'a BTreeSet<String>,
    artefacts: &'a BTreeSet<String>,
    orders: &'a BTreeSet<i32>,
}

/// A toggle chip for a `String`-valued filter facet. Clicking a chip sets the
/// filter; clicking the active chip clears it. These are the filter *buttons*.
fn string_chip(state: &UseStateHandle<Option<String>>, value: &str) -> Html {
    let is_active = state.as_deref() == Some(value);
    let onclick = {
        let state = state.clone();
        let value = value.to_string();
        Callback::from(move |_: MouseEvent| {
            if state.as_deref() == Some(value.as_str()) {
                state.set(None);
            } else {
                state.set(Some(value.clone()));
            }
        })
    };
    html! {
        <button class={ classes!("facet-chip", is_active.then_some("active")) } onclick={ onclick }>
            { value }
        </button>
    }
}

/// A whole facet row: a label, an "All" chip (clears the filter), then one
/// toggle chip per distinct value.
fn string_facet(label: &str, values: &BTreeSet<String>, state: &UseStateHandle<Option<String>>) -> Html {
    let clear = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| state.set(None))
    };
    html! {
        <div class="facet">
            <span class="facet-label">{ label }</span>
            <button
                class={ classes!("facet-chip", state.is_none().then_some("active")) }
                onclick={ clear }
            >{ "All" }</button>
            { for values.iter().map(|v| string_chip(state, v)) }
        </div>
    }
}

fn table_view(ctx: TableCtx) -> Html {
    let TableCtx {
        refs,
        sort_col,
        sort_asc,
        f_perspective,
        f_source,
        f_artefact,
        f_order,
        search,
        perspectives,
        sources,
        artefacts,
        orders,
    } = ctx;

    // Filter.
    let needle = search.to_lowercase();
    let mut rows: Vec<&ReferenceView> = refs
        .iter()
        .filter(|r| f_perspective.as_ref().is_none_or(|p| &persp(r) == p))
        .filter(|r| f_source.as_ref().is_none_or(|s| &src(r) == s))
        .filter(|r| f_artefact.as_ref().is_none_or(|a| &art(r) == a))
        .filter(|r| f_order.as_ref().is_none_or(|o| order_of(r) == Some(*o)))
        .filter(|r| {
            if needle.is_empty() {
                return true;
            }
            let hay = format!(
                "{} {} {} {} {} {}",
                persp(r),
                src(r),
                art(r),
                loc(r),
                r.target,
                r.note.clone().unwrap_or_default()
            )
            .to_lowercase();
            hay.contains(&needle)
        })
        .collect();

    // Sort.
    let col = **sort_col;
    rows.sort_by(|a, b| {
        let ord = match col {
            SortCol::Perspective => persp(a).cmp(&persp(b)),
            SortCol::Source => src(a).cmp(&src(b)),
            SortCol::Artefact => art(a).cmp(&art(b)),
            SortCol::Order => order_of(a).cmp(&order_of(b)),
            SortCol::Fragment => frag(a).cmp(&frag(b)),
        };
        if **sort_asc { ord } else { ord.reverse() }
    });

    // Sort buttons (one per sortable column; clicking the active one flips
    // direction). Sorting is button-driven, not just header clicks.
    let sort_button = |c: SortCol| -> Html {
        let sort_col = sort_col.clone();
        let sort_asc = sort_asc.clone();
        let active = *sort_col == c;
        let arrow = if active {
            if *sort_asc { " ▲" } else { " ▼" }
        } else {
            ""
        };
        let onclick = Callback::from(move |_: MouseEvent| {
            if *sort_col == c {
                sort_asc.set(!*sort_asc);
            } else {
                sort_col.set(c);
                sort_asc.set(true);
            }
        });
        html! {
            <button class={ classes!("sort-chip", active.then_some("active")) } onclick={ onclick }>
                { c.label() }{ arrow }
            </button>
        }
    };

    let on_search = {
        let search = search.clone();
        Callback::from(move |e: InputEvent| {
            let v = e.target_unchecked_into::<HtmlInputElement>().value();
            search.set(v);
        })
    };

    html! {
        <>
            <div class="ref-controls">
                <div class="ref-search-row">
                    <input
                        class="ref-search"
                        type="text"
                        placeholder="Search…"
                        value={ (**search).clone() }
                        oninput={ on_search }
                    />
                    <span class="ref-count">{ format!("{} shown", rows.len()) }</span>
                </div>

                // Sort — buttons.
                <div class="facet">
                    <span class="facet-label">{ "Sort" }</span>
                    { sort_button(SortCol::Perspective) }
                    { sort_button(SortCol::Source) }
                    { sort_button(SortCol::Artefact) }
                    { sort_button(SortCol::Order) }
                    { sort_button(SortCol::Fragment) }
                </div>

                // Filter — buttons. Facets are the citation/reference data:
                // perspective, source, artefact, order.
                { string_facet("Perspective", perspectives, f_perspective) }
                { string_facet("Source", sources, f_source) }
                { string_facet("Artefact", artefacts, f_artefact) }
                <div class="facet">
                    <span class="facet-label">{ "Order" }</span>
                    <button
                        class={ classes!("facet-chip", f_order.is_none().then_some("active")) }
                        onclick={ { let f = f_order.clone(); Callback::from(move |_: MouseEvent| f.set(None)) } }
                    >{ "All" }</button>
                    { for orders.iter().map(|o| {
                        let is_active = **f_order == Some(*o);
                        let f = f_order.clone();
                        let val = *o;
                        let onclick = Callback::from(move |_: MouseEvent| {
                            if *f == Some(val) { f.set(None) } else { f.set(Some(val)) }
                        });
                        html! {
                            <button
                                class={ classes!("facet-chip", is_active.then_some("active")) }
                                onclick={ onclick }
                                title={ order_name(*o) }
                            >{ o.to_string() }</button>
                        }
                    }) }
                </div>
            </div>

            <div class="ref-table-wrap">
                <table class="ref-table">
                    <thead>
                        <tr>
                            <th class="ref-th">{ "Perspective" }</th>
                            <th class="ref-th">{ "Tags (source · locator · artefact)" }</th>
                            <th class="ref-th">{ "Order" }</th>
                            <th class="ref-th">{ "Cites" }</th>
                            <th class="ref-th">{ "Note" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for rows.iter().map(|r| {
                            let cites = {
                                let f = frag(r);
                                if f.is_empty() { "whole system".to_string() } else { f }
                            };
                            let target_label = r.target_system.as_ref()
                                .map(|s| s.name.clone())
                                .unwrap_or_else(|| r.target.clone());
                            html! {
                                <tr>
                                    <td>{ persp(r) }</td>
                                    <td>{ citation_tags(r, f_source, f_artefact) }</td>
                                    <td>{ order_of(r).map(|o| format!("{} {}", o, order_name(o))).unwrap_or_default() }</td>
                                    <td title={ target_label }>{ cites }</td>
                                    <td>{ r.note.clone().unwrap_or_default() }</td>
                                </tr>
                            }
                        }) }
                    </tbody>
                </table>
            </div>
        </>
    }
}

/// Render a reference's **citation triad** (source · locator · artefact) as
/// tags. Source and artefact tags are clickable — they set the matching filter,
/// so tagging and filtering are the same button surface. The citation format is
/// the fixed triad for now, but the row is extendable to other systematic
/// formats.
fn citation_tags(
    r: &ReferenceView,
    f_source: &UseStateHandle<Option<String>>,
    f_artefact: &UseStateHandle<Option<String>>,
) -> Html {
    let source = src(r);
    let locator = loc(r);
    let artefact = art(r);

    let source_tag = (!source.is_empty()).then(|| {
        let f = f_source.clone();
        let v = source.clone();
        let onclick = Callback::from(move |_: MouseEvent| f.set(Some(v.clone())));
        html! { <button class="tag tag-source" onclick={ onclick } title="Source (filter)">{ source.clone() }</button> }
    });
    let artefact_url = r.artefact.as_ref().and_then(|a| a.url.clone()).unwrap_or_default();
    let artefact_tag = (!artefact.is_empty()).then(|| {
        let f = f_artefact.clone();
        let v = artefact.clone();
        let onclick = Callback::from(move |_: MouseEvent| f.set(Some(v.clone())));
        html! { <button class="tag tag-artefact" onclick={ onclick } title={ artefact_url }>{ artefact.clone() }</button> }
    });
    let locator_tag = (!locator.is_empty())
        .then(|| html! { <span class="tag tag-locator">{ locator }</span> });

    html! {
        <span class="tags">
            { source_tag.unwrap_or_default() }
            { locator_tag.unwrap_or_default() }
            { artefact_tag.unwrap_or_default() }
        </span>
    }
}

fn compare_view(refs: &[ReferenceView]) -> Html {
    // (order, perspective) -> (coherence value, provenance). First wins (all
    // references from one perspective to one order-N system share its value).
    let mut cells: BTreeMap<(i32, String), (String, String)> = BTreeMap::new();
    let mut orders: BTreeSet<i32> = BTreeSet::new();
    let mut perspectives: BTreeSet<String> = BTreeSet::new();

    for r in refs {
        if let Some(sys) = &r.target_system {
            let p = persp(r);
            if p.is_empty() {
                continue;
            }
            orders.insert(sys.order);
            perspectives.insert(p.clone());
            cells
                .entry((sys.order, p))
                .or_insert_with(|| (sys.coherence.clone(), provenance(r)));
        }
    }

    let perspectives: Vec<String> = perspectives.into_iter().collect();

    html! {
        <div class="ref-table-wrap">
            <table class="ref-table compare-matrix">
                <thead>
                    <tr>
                        <th class="ref-th">{ "Order" }</th>
                        { for perspectives.iter().map(|p| html!{ <th class="ref-th">{ p }</th> }) }
                    </tr>
                </thead>
                <tbody>
                    { for orders.iter().map(|o| html!{
                        <tr>
                            <td class="compare-order">{ format!("{} {}", o, order_name(*o)) }</td>
                            { for perspectives.iter().map(|p| {
                                match cells.get(&(*o, p.clone())) {
                                    Some((coherence, prov)) => html!{
                                        <td class="compare-cell" title={ prov.clone() }>{ coherence }</td>
                                    },
                                    None => html!{ <td class="compare-cell empty">{ "—" }</td> },
                                }
                            }) }
                        </tr>
                    }) }
                </tbody>
            </table>
        </div>
    }
}
