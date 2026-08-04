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

/// A selectable column — one **tag key**. The view is composed by choosing which
/// keys to show (the Tag reconciler's own by-key action). Everything is a tag, so
/// a column is just a tag key surfaced.
#[derive(Clone, Copy, PartialEq)]
enum ColKey {
    Order,
    Perspective,
    Citation,
    Cites,
    Note,
}

/// Canonical column order (independent of the order keys were toggled on).
const ALL_COLS: [ColKey; 5] = [
    ColKey::Order,
    ColKey::Perspective,
    ColKey::Citation,
    ColKey::Cites,
    ColKey::Note,
];

impl ColKey {
    fn label(self) -> &'static str {
        match self {
            ColKey::Order => "Order",
            ColKey::Perspective => "Perspective",
            ColKey::Citation => "Citation",
            ColKey::Cites => "Cites",
            ColKey::Note => "Note",
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
/// Whether a reference is in the current selection (header order + search).
/// Shared by the table (what to show) and Extract (what to materialize).
fn passes_filters(r: &ReferenceView, filter_order: Option<i32>, needle: &str) -> bool {
    filter_order.is_none_or(|o| order_of(r) == Some(o))
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
    // Sort is by a column (tag key), toggled by clicking its header.
    let sort_col = use_state(|| ColKey::Order);
    let sort_asc = use_state(|| true);
    let search = use_state(String::new);
    let load_open = use_state(|| false);
    // Column configurator popover (which tag keys are shown as columns).
    let cols_open = use_state(|| false);
    // Which tag keys are shown as columns — the view is composed by key. Minimal
    // by default (seeing every key at once is rarely useful).
    let visible_cols = use_state(|| vec![ColKey::Order, ColKey::Citation]);

    let refs = &props.references;
    // The order filter comes from the header (Nullad = None = all).
    let filter_order = props.filter_order;

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
        .filter(|r| passes_filters(r, filter_order, &needle))
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
                        search: &search,
                        cols_open: &cols_open,
                        visible_cols: &visible_cols,
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
    /// Sort column (tag key) + direction — set by clicking a column header.
    sort_col: &'a UseStateHandle<ColKey>,
    sort_asc: &'a UseStateHandle<bool>,
    search: &'a UseStateHandle<String>,
    /// The column configurator popover (which tag keys are columns = Filter).
    cols_open: &'a UseStateHandle<bool>,
    /// Which tag keys are shown as columns (the view, composed by key).
    visible_cols: &'a UseStateHandle<Vec<ColKey>>,
}

fn table_view(ctx: TableCtx) -> Html {
    let TableCtx {
        refs,
        filter_order,
        sort_col,
        sort_asc,
        search,
        cols_open,
        visible_cols,
    } = ctx;

    // Filter — same predicate Extract uses (header order + search).
    let needle = search.to_lowercase();
    let mut rows: Vec<&ReferenceView> =
        refs.iter().filter(|r| passes_filters(r, filter_order, &needle)).collect();

    // Sort by the active column (tag key).
    let col = **sort_col;
    rows.sort_by(|a, b| {
        let ord = match col {
            ColKey::Order => order_of(a).cmp(&order_of(b)),
            ColKey::Perspective => persp(a).cmp(&persp(b)),
            ColKey::Citation => src(a).cmp(&src(b)),
            ColKey::Cites => frag(a).cmp(&frag(b)),
            ColKey::Note => a.note.cmp(&b.note),
        };
        if **sort_asc { ord } else { ord.reverse() }
    });

    // Column toggle — configure which tag keys are columns (Filter = the
    // configuration of headers). Everything is a tag; a column is a tag key.
    let col_chip = |k: ColKey| -> Html {
        let visible_cols = visible_cols.clone();
        let on = visible_cols.contains(&k);
        let onclick = Callback::from(move |_: MouseEvent| {
            let mut next = (*visible_cols).clone();
            if let Some(i) = next.iter().position(|c| *c == k) {
                next.remove(i);
            } else {
                next.push(k);
            }
            visible_cols.set(next);
        });
        html! {
            <button class={ classes!("facet-chip", on.then_some("active")) } onclick={ onclick }>
                { k.label() }
            </button>
        }
    };
    // Visible columns in canonical order (independent of toggle order).
    let cols: Vec<ColKey> = ALL_COLS.into_iter().filter(|c| visible_cols.contains(c)).collect();

    let cell = |k: ColKey, r: &ReferenceView| -> Html {
        match k {
            ColKey::Order => html! { { order_of(r).map(|o| format!("{} {}", o, order_name(o))).unwrap_or_default() } },
            ColKey::Perspective => persp_tag(r),
            ColKey::Citation => citation_tags(r),
            ColKey::Cites => {
                let f = frag(r);
                let cites = if f.is_empty() { "whole system".to_string() } else { f };
                let target_label = r.target_system.as_ref().map(|s| s.name.clone()).unwrap_or_else(|| r.target.clone());
                html! { <span title={ target_label }>{ cites }</span> }
            }
            ColKey::Note => html! { { r.note.clone().unwrap_or_default() } },
        }
    };

    // A sortable column header — click to sort by that tag; click again to flip.
    let header = |k: ColKey| -> Html {
        let sort_col = sort_col.clone();
        let sort_asc = sort_asc.clone();
        let active = *sort_col == k;
        let arrow = if active {
            if *sort_asc { " ▲" } else { " ▼" }
        } else {
            ""
        };
        let onclick = Callback::from(move |_: MouseEvent| {
            if *sort_col == k {
                sort_asc.set(!*sort_asc);
            } else {
                sort_col.set(k);
                sort_asc.set(true);
            }
        });
        html! {
            <th class={ classes!("ref-th", active.then_some("sorted")) } onclick={ onclick }>
                { k.label() }{ arrow }
            </th>
        }
    };

    let on_search = {
        let search = search.clone();
        Callback::from(move |e: InputEvent| {
            let v = e.target_unchecked_into::<HtmlInputElement>().value();
            search.set(v);
        })
    };
    let toggle_cols = {
        let s = cols_open.clone();
        Callback::from(move |_: MouseEvent| s.set(!*s))
    };

    html! {
        <>
            // Control bar: column configuration (Filter = which tag keys are the
            // headers) + search. Sort is by clicking a column header.
            <div class="ref-controlbar">
                <div class="control-pop">
                    <button
                        class={ classes!("control-btn", (**cols_open).then_some("active")) }
                        onclick={ toggle_cols }
                        title="Columns — choose which tag keys are shown (Filter = configuring the headers)"
                    >{ format!("Columns ({}) ▾", cols.len()) }</button>
                    if **cols_open {
                        <div class="control-menu">
                            <span class="facet-label">{ "columns · tag keys" }</span>
                            <div class="col-chips">
                                { for ALL_COLS.into_iter().map(col_chip) }
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
                if cols.is_empty() {
                    <p class="ref-empty">{ "No columns — pick tag keys under Columns ▾." }</p>
                } else {
                    <table class="ref-table">
                        <thead>
                            <tr>
                                { for cols.iter().map(|k| header(*k)) }
                            </tr>
                        </thead>
                        <tbody>
                            { for rows.iter().map(|r| html!{
                                <tr>
                                    { for cols.iter().map(|k| html!{ <td>{ cell(*k, r) }</td> }) }
                                </tr>
                            }) }
                        </tbody>
                    </table>
                }
            </div>
        </>
    }
}

/// The **Perspective** tag for a reference — its own column key (display only).
fn persp_tag(r: &ReferenceView) -> Html {
    let perspective = persp(r);
    if perspective.is_empty() {
        return html! {};
    }
    html! { <span class="tag tag-perspective">{ perspective }</span> }
}

/// Render a reference's **citation triad** in triad order — **Source · Artefact ·
/// Lookup** (display only; the Citation column key surfaces this).
fn citation_tags(r: &ReferenceView) -> Html {
    let source = src(r);
    let artefact = art(r);
    let locator = loc(r);

    let source_tag = (!source.is_empty())
        .then(|| html! { <span class="tag tag-source" title="Source">{ source }</span> });
    let artefact_url = r.artefact.as_ref().and_then(|a| a.url.clone()).unwrap_or_default();
    let artefact_tag = (!artefact.is_empty())
        .then(|| html! { <span class="tag tag-artefact" title={ artefact_url }>{ artefact }</span> });
    let locator_tag = (!locator.is_empty())
        .then(|| html! { <span class="tag tag-locator" title="Lookup">{ locator }</span> });

    html! {
        <span class="tags">
            { source_tag.unwrap_or_default() }
            { artefact_tag.unwrap_or_default() }
            { locator_tag.unwrap_or_default() }
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
