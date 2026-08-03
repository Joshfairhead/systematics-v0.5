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
    /// The order selected in the header — filters the view to that system.
    /// `None` = Nullad = the whole registry (no order filter).
    #[prop_or_default]
    pub filter_order: Option<i32>,
    /// Extract (Nullad → Monad): materialize the current selection as a Monad.
    pub on_extract: Callback<ExtractRequest>,
    /// Feedback from the last Extract (e.g. "Extracted 'Monad …' (n members)").
    #[prop_or_default]
    pub extract_note: Option<String>,
}

/// A request to Extract the current selection into a Monad — a provisional name
/// plus the selected member addresses (`system:<id>`, …).
#[derive(Clone, PartialEq)]
pub struct ExtractRequest {
    pub name: String,
    pub members: Vec<String>,
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
/// Whether a reference is in the current selection (header order + facets +
/// search). Shared by the table (what to show) and Extract (what to materialize).
fn passes_filters(
    r: &ReferenceView,
    filter_order: Option<i32>,
    f_perspective: Option<&str>,
    f_source: Option<&str>,
    f_artefact: Option<&str>,
    needle: &str,
) -> bool {
    filter_order.is_none_or(|o| order_of(r) == Some(o))
        && f_perspective.is_none_or(|p| persp(r).as_str() == p)
        && f_source.is_none_or(|s| src(r).as_str() == s)
        && f_artefact.is_none_or(|a| art(r).as_str() == a)
        && (needle.is_empty() || {
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
            hay.contains(needle)
        })
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
    let search = use_state(String::new);
    let load_open = use_state(|| false);
    // The Tag reconciler popover. Tag (=) reconciles Sort (+) and Filter (−):
    // everything in the view is a tag, so both operations act on tags. Tags
    // themselves are always shown (the reconciler is the whole).
    let tags_open = use_state(|| false);

    let refs = &props.references;
    // The order filter comes from the header (Nullad = None = all).
    let filter_order = props.filter_order;

    // Distinct facet values (perspective/source/artefact refine within the
    // header's order selection).
    let perspectives: BTreeSet<String> =
        refs.iter().map(persp).filter(|s| !s.is_empty()).collect();
    let sources: BTreeSet<String> = refs.iter().map(src).filter(|s| !s.is_empty()).collect();
    let artefacts: BTreeSet<String> = refs.iter().map(art).filter(|s| !s.is_empty()).collect();

    // Caption for the active header scope.
    let scope_label = match filter_order {
        Some(o) => format!("{} {} only", o, order_name(o)),
        None => "Nullad — all orders".to_string(),
    };

    // The Extract selection: distinct target systems of the currently-filtered
    // references, as `system:<id>` member addresses. This is what Extract
    // materializes into a Monad.
    let needle = search.to_lowercase();
    let mut seen = BTreeSet::new();
    let extract_members: Vec<String> = refs
        .iter()
        .filter(|r| {
            passes_filters(
                r,
                filter_order,
                f_perspective.as_deref(),
                f_source.as_deref(),
                f_artefact.as_deref(),
                &needle,
            )
        })
        .filter_map(|r| r.target_system.as_ref().map(|s| format!("system:{}", s.id)))
        .filter(|m| seen.insert(m.clone()))
        .collect();
    // Provisional Monad name (the "integral" naming is a later refinement).
    let extract_name = match filter_order {
        Some(o) => format!("Monad — {} {}", o, order_name(o)),
        None => format!("Monad — Nullad selection ({})", extract_members.len()),
    };

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
            { elt_triad(EltCtx {
                instance_systems: &props.instance_systems,
                on_load: &props.on_load,
                load_open: &load_open,
                extract_members: &extract_members,
                extract_name: &extract_name,
                on_extract: &props.on_extract,
                extract_note: props.extract_note.as_deref(),
            }) }

            <div class="ref-tabs">
                <button
                    class={ if *tab == Tab::Table { "ref-tab active" } else { "ref-tab" } }
                    onclick={ switch_tab(Tab::Table) }
                >{ "Table" }</button>
                <button
                    class={ if *tab == Tab::Compare { "ref-tab active" } else { "ref-tab" } }
                    onclick={ switch_tab(Tab::Compare) }
                >{ "Compare by order" }</button>
                <span class="ref-scope">{ scope_label }</span>
                <span class="ref-count">{ format!("{} references", refs.len()) }</span>
            </div>
            {
                match *tab {
                    Tab::Table => table_view(TableCtx {
                        refs,
                        filter_order,
                        sort_col: &sort_col,
                        sort_asc: &sort_asc,
                        f_perspective: &f_perspective,
                        f_source: &f_source,
                        f_artefact: &f_artefact,
                        search: &search,
                        perspectives: &perspectives,
                        sources: &sources,
                        artefacts: &artefacts,
                        tags_open: &tags_open,
                    }),
                    Tab::Compare => compare_view(refs, filter_order),
                }
            }
        </div>
    }
}

/// Grouped arguments for the ELT triad control.
struct EltCtx<'a> {
    instance_systems: &'a [InstanceSystem],
    on_load: &'a Callback<String>,
    load_open: &'a UseStateHandle<bool>,
    /// The current selection to Extract, as `system:<id>` member addresses.
    extract_members: &'a [String],
    /// Provisional name for the Monad Extract would create.
    extract_name: &'a str,
    on_extract: &'a Callback<ExtractRequest>,
    /// Feedback from the last Extract, if any.
    extract_note: Option<&'a str>,
}

/// The ELT triad control: Extract · Load · Transform.
/// - **Extract** materializes the current selection into a Monad (Nullad → Monad).
/// - **Load** opens a menu of instance systems.
/// - **Transform** (apply a Functor) is the third edge, not yet wired.
fn elt_triad(ctx: EltCtx) -> Html {
    let EltCtx {
        instance_systems,
        on_load,
        load_open,
        extract_members,
        extract_name,
        on_extract,
        extract_note,
    } = ctx;

    let toggle = {
        let o = load_open.clone();
        Callback::from(move |_: MouseEvent| o.set(!*o))
    };

    let can_extract = !extract_members.is_empty();
    let on_extract_click = {
        let on_extract = on_extract.clone();
        let name = extract_name.to_string();
        let members = extract_members.to_vec();
        Callback::from(move |_: MouseEvent| {
            on_extract.emit(ExtractRequest {
                name: name.clone(),
                members: members.clone(),
            })
        })
    };
    let extract_title = format!(
        "Extract — materialize this selection ({} systems) into a Monad (Nullad → Monad)",
        extract_members.len()
    );

    html! {
        <div class="elt-triad" title="Operation edge (ELT): Extract · Load · Transform">
            <button
                class="elt-btn"
                disabled={ !can_extract }
                onclick={ on_extract_click }
                title={ extract_title }
            >{ format!("Extract ({})", extract_members.len()) }</button>

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

            if let Some(note) = extract_note {
                <span class="elt-note">{ note }</span>
            }
        </div>
    }
}

/// Grouped arguments for the table view (keeps the signature under one struct).
struct TableCtx<'a> {
    refs: &'a [ReferenceView],
    /// Order filter from the header (`None` = Nullad = all).
    filter_order: Option<i32>,
    sort_col: &'a UseStateHandle<SortCol>,
    sort_asc: &'a UseStateHandle<bool>,
    f_perspective: &'a UseStateHandle<Option<String>>,
    f_source: &'a UseStateHandle<Option<String>>,
    f_artefact: &'a UseStateHandle<Option<String>>,
    search: &'a UseStateHandle<String>,
    perspectives: &'a BTreeSet<String>,
    sources: &'a BTreeSet<String>,
    artefacts: &'a BTreeSet<String>,
    /// The Tag reconciler popover (holds the Sort/Filter × key/value tree).
    tags_open: &'a UseStateHandle<bool>,
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
        filter_order,
        sort_col,
        sort_asc,
        f_perspective,
        f_source,
        f_artefact,
        search,
        perspectives,
        sources,
        artefacts,
        tags_open,
    } = ctx;

    // Filter — same predicate Extract uses (header order + facets + search).
    let needle = search.to_lowercase();
    let mut rows: Vec<&ReferenceView> = refs
        .iter()
        .filter(|r| {
            passes_filters(
                r,
                filter_order,
                f_perspective.as_deref(),
                f_source.as_deref(),
                f_artefact.as_deref(),
                &needle,
            )
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

    let toggle_tags = {
        let s = tags_open.clone();
        Callback::from(move |_: MouseEvent| s.set(!*s))
    };
    let filters_active = f_perspective.is_some() || f_source.is_some() || f_artefact.is_some();
    let sort_summary = format!("{} {}", (**sort_col).label(), if **sort_asc { "▲" } else { "▼" });

    html! {
        <>
            // Control bar: the Tag reconciler (holding the Sort/Filter tree) to the
            // left of the search. Tag (=) reconciles Sort (+) and Filter (−).
            <div class="ref-controlbar">
                <div class="control-pop">
                    <button
                        class={ classes!("control-btn", (filters_active || **tags_open).then_some("active")) }
                        onclick={ toggle_tags }
                        title="Tag (=) reconciles Sort (+) and Filter (−) — everything here is a tag"
                    >{ "Tags ▾" }</button>
                    if **tags_open {
                        <div class="control-menu tag-tree">
                            <div class="tag-root">{ "Tag (=)" }</div>
                            <div class="tag-branches">
                                // Sort (+): prioritise the list by a tag.
                                <div class="tag-branch">
                                    <div class="tag-branch-head">{ format!("Sort (+) · {sort_summary}") }</div>
                                    <div class="tag-leaf">
                                        <span class="tag-leaf-label">{ "by key" }</span>
                                        { sort_button(SortCol::Order) }
                                        { sort_button(SortCol::Perspective) }
                                        { sort_button(SortCol::Source) }
                                        { sort_button(SortCol::Artefact) }
                                        { sort_button(SortCol::Fragment) }
                                    </div>
                                    <div class="tag-leaf">
                                        <span class="tag-leaf-label">{ "by value" }</span>
                                        <span class="tag-leaf-todo">{ "forthcoming" }</span>
                                    </div>
                                </div>
                                // Filter (−): add/remove tags from the query.
                                <div class="tag-branch">
                                    <div class="tag-branch-head">{ "Filter (−)" }</div>
                                    <div class="tag-leaf">
                                        <span class="tag-leaf-label">{ "by key" }</span>
                                        <span class="tag-leaf-todo">{ "forthcoming" }</span>
                                    </div>
                                    <div class="tag-leaf tag-leaf-col">
                                        <span class="tag-leaf-label">{ "by value" }</span>
                                        { string_facet("Perspective", perspectives, f_perspective) }
                                        { string_facet("Source", sources, f_source) }
                                        { string_facet("Artefact", artefacts, f_artefact) }
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                </div>

                <input
                    class="ref-search"
                    type="text"
                    placeholder="Search…"
                    value={ (**search).clone() }
                    oninput={ on_search }
                />
                <span class="ref-count">{ format!("{} shown", rows.len()) }</span>
            </div>

            <div class="ref-table-wrap">
                <table class="ref-table">
                    <thead>
                        <tr>
                            <th class="ref-th">{ "Order" }</th>
                            <th class="ref-th">{ "Cites" }</th>
                            <th class="ref-th">{ "Tags" }</th>
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
                                    <td>{ order_of(r).map(|o| format!("{} {}", o, order_name(o))).unwrap_or_default() }</td>
                                    <td title={ target_label }>{ cites }</td>
                                    <td>{ citation_tags(r, f_perspective, f_source, f_artefact) }</td>
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

/// Render a reference's tags — its **perspective** plus the **citation triad**
/// (source · locator · artefact). Perspective / source / artefact are clickable
/// (they set the matching filter), so tagging and filtering share one surface.
/// Folded here (shown only when the Tags column is on) to declutter the table.
fn citation_tags(
    r: &ReferenceView,
    f_perspective: &UseStateHandle<Option<String>>,
    f_source: &UseStateHandle<Option<String>>,
    f_artefact: &UseStateHandle<Option<String>>,
) -> Html {
    let perspective = persp(r);
    let source = src(r);
    let locator = loc(r);
    let artefact = art(r);

    let perspective_tag = (!perspective.is_empty()).then(|| {
        let f = f_perspective.clone();
        let v = perspective.clone();
        let onclick = Callback::from(move |_: MouseEvent| f.set(Some(v.clone())));
        html! { <button class="tag tag-perspective" onclick={ onclick } title="Perspective (filter)">{ perspective.clone() }</button> }
    });
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
            { perspective_tag.unwrap_or_default() }
            { source_tag.unwrap_or_default() }
            { locator_tag.unwrap_or_default() }
            { artefact_tag.unwrap_or_default() }
        </span>
    }
}

fn compare_view(refs: &[ReferenceView], filter_order: Option<i32>) -> Html {
    // (order, perspective) -> (coherence value, provenance). First wins (all
    // references from one perspective to one order-N system share its value).
    // The header's order selection scopes which rows appear (Nullad = all).
    let mut cells: BTreeMap<(i32, String), (String, String)> = BTreeMap::new();
    let mut orders: BTreeSet<i32> = BTreeSet::new();
    let mut perspectives: BTreeSet<String> = BTreeSet::new();

    for r in refs {
        if let Some(sys) = &r.target_system {
            if filter_order.is_some_and(|o| o != sys.order) {
                continue;
            }
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
