//! Reference Browser — a comparative view over every citation.
//!
//! Two tabs over the enriched `allReferences` data:
//!  * **Table**: all references, sortable by column and filterable by
//!    perspective / source / order, with a free-text search — the provenance
//!    audit view.
//!  * **Compare**: a matrix of order × perspective, each cell showing how that
//!    perspective characterizes that order's system (its coherence value) with
//!    the citation as a tooltip — the comparative lens (keep every perspective,
//!    compare their references).
//!
//! The grouping key is the target system's *order* (perspectives cite their own
//! system ids, not a shared canonical address), resolved server-side into
//! `ReferenceView.target_system`.

use std::collections::{BTreeMap, BTreeSet};

use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

use crate::api::client::ReferenceView;

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
    let f_order = use_state(|| Option::<i32>::None);
    let search = use_state(String::new);

    let refs = &props.references;

    // Distinct dropdown values.
    let perspectives: BTreeSet<String> =
        refs.iter().map(persp).filter(|s| !s.is_empty()).collect();
    let sources: BTreeSet<String> = refs.iter().map(src).filter(|s| !s.is_empty()).collect();
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
                    Tab::Table => table_view(
                        refs, &sort_col, &sort_asc, &f_perspective, &f_source, &f_order,
                        &search, &perspectives, &sources, &orders,
                    ),
                    Tab::Compare => compare_view(refs),
                }
            }
        </div>
    }
}

#[allow(clippy::too_many_arguments)]
fn table_view(
    refs: &[ReferenceView],
    sort_col: &UseStateHandle<SortCol>,
    sort_asc: &UseStateHandle<bool>,
    f_perspective: &UseStateHandle<Option<String>>,
    f_source: &UseStateHandle<Option<String>>,
    f_order: &UseStateHandle<Option<i32>>,
    search: &UseStateHandle<String>,
    perspectives: &BTreeSet<String>,
    sources: &BTreeSet<String>,
    orders: &BTreeSet<i32>,
) -> Html {
    // Filter.
    let needle = search.to_lowercase();
    let mut rows: Vec<&ReferenceView> = refs
        .iter()
        .filter(|r| f_perspective.as_ref().is_none_or(|p| &persp(r) == p))
        .filter(|r| f_source.as_ref().is_none_or(|s| &src(r) == s))
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

    let header = |label: &str, col: SortCol| -> Html {
        let sort_col = sort_col.clone();
        let sort_asc = sort_asc.clone();
        let active = *sort_col == col;
        let arrow = if active {
            if *sort_asc { " ▲" } else { " ▼" }
        } else {
            ""
        };
        let onclick = Callback::from(move |_: MouseEvent| {
            if *sort_col == col {
                sort_asc.set(!*sort_asc);
            } else {
                sort_col.set(col);
                sort_asc.set(true);
            }
        });
        html! { <th class="ref-th" onclick={onclick}>{ label }{ arrow }</th> }
    };

    // Filter dropdown change handlers.
    let on_persp = {
        let f = f_perspective.clone();
        Callback::from(move |e: Event| {
            let v = e.target_unchecked_into::<HtmlSelectElement>().value();
            f.set(if v.is_empty() { None } else { Some(v) });
        })
    };
    let on_source = {
        let f = f_source.clone();
        Callback::from(move |e: Event| {
            let v = e.target_unchecked_into::<HtmlSelectElement>().value();
            f.set(if v.is_empty() { None } else { Some(v) });
        })
    };
    let on_order = {
        let f = f_order.clone();
        Callback::from(move |e: Event| {
            let v = e.target_unchecked_into::<HtmlSelectElement>().value();
            f.set(v.parse::<i32>().ok());
        })
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
            <div class="ref-filter-bar">
                <select class="ref-filter" onchange={on_persp}>
                    <option value="" selected={f_perspective.is_none()}>{ "All perspectives" }</option>
                    { for perspectives.iter().map(|p| html!{
                        <option value={p.clone()} selected={f_perspective.as_deref() == Some(p)}>{ p }</option>
                    }) }
                </select>
                <select class="ref-filter" onchange={on_source}>
                    <option value="" selected={f_source.is_none()}>{ "All sources" }</option>
                    { for sources.iter().map(|s| html!{
                        <option value={s.clone()} selected={f_source.as_deref() == Some(s)}>{ s }</option>
                    }) }
                </select>
                <select class="ref-filter" onchange={on_order}>
                    <option value="" selected={f_order.is_none()}>{ "All orders" }</option>
                    { for orders.iter().map(|o| html!{
                        <option value={o.to_string()} selected={**f_order == Some(*o)}>
                            { format!("{} — {}", o, order_name(*o)) }
                        </option>
                    }) }
                </select>
                <input
                    class="ref-search"
                    type="text"
                    placeholder="Search…"
                    value={ (**search).clone() }
                    oninput={on_search}
                />
                <span class="ref-count">{ format!("{} shown", rows.len()) }</span>
            </div>
            <div class="ref-table-wrap">
                <table class="ref-table">
                    <thead>
                        <tr>
                            { header("Perspective", SortCol::Perspective) }
                            { header("Source", SortCol::Source) }
                            { header("Artefact", SortCol::Artefact) }
                            <th class="ref-th">{ "Locator" }</th>
                            { header("Order", SortCol::Order) }
                            { header("Cites", SortCol::Fragment) }
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
                                    <td>{ src(r) }</td>
                                    <td title={ r.artefact.as_ref().and_then(|a| a.url.clone()).unwrap_or_default() }>{ art(r) }</td>
                                    <td>{ loc(r) }</td>
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
