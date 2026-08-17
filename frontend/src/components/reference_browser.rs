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

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::api::client::{InstanceSystem, ReferenceView, SequenceView};
use crate::components::inspector::{Inspector, ObjCite, PosGroup, PredGroup};
// The SPO triple store + query API live in their own module (swappable spike).
use crate::components::spo::{
    all_predicates, art, build_triples, frag, loc, order_name, order_of, persp, pred_label,
    pred_objects_scoped, src, Triple,
};

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
    /// Feedback from the last Extract / author (shown in the Editor).
    #[prop_or_default]
    pub extract_note: Option<String>,
    /// Author a new System from custom term/connective values (the editor).
    pub on_author: Callback<AuthorRequest>,
    /// Enter a sequence/monad (its member addresses) — header buttons then
    /// navigate within it. `on_load` is the per-system fallback.
    #[prop_or_default]
    pub on_view_sequence: Callback<Vec<String>>,
    /// Delete a sequence/monad by id (the ✕ on a monad row).
    #[prop_or_default]
    pub on_delete_sequence: Callback<String>,
    /// Delete selected rows by address (`system:` / `sequence:` / `reference:`).
    #[prop_or_default]
    pub on_delete_rows: Callback<Vec<String>>,
    /// When a **bucket** monad is entered, scope the table to just these member
    /// addresses (`system:<id>`). `None` = no scope (the whole registry).
    #[prop_or_default]
    pub scope_ids: Option<Vec<String>>,
    /// Canonical term/connective values per order — the editor prefills from these
    /// ("open the canonical system, then customise").
    #[prop_or_default]
    pub templates: Vec<SystemTemplate>,
    /// The focused system's raw nodes (terms) + edges (connectives), shown as rows
    /// when the Term/Connective filter is on (off by default).
    #[prop_or_default]
    pub raw_elements: Vec<RawElement>,
    /// Every Sequence / Monad in the graph — shown as rows (Cites = its members),
    /// so monads (e.g. the Architecture Monad) and their members are visible.
    #[prop_or_default]
    pub sequences: Vec<SequenceView>,
}

/// A raw node (term) or edge (connective) of the focused system — a data row.
#[derive(Clone, PartialEq)]
pub struct RawElement {
    pub name: String,
    pub order: i32,
    /// false = node (term), true = edge (connective).
    pub is_edge: bool,
}

/// The canonical term/connective values for one order — an editor prefill source.
#[derive(Clone, PartialEq)]
pub struct SystemTemplate {
    pub order: i32,
    pub terms: Vec<String>,
    pub connectives: Vec<String>,
}

/// A request to author (create) a System from custom values — the editor path.
#[derive(Clone, PartialEq)]
pub struct AuthorRequest {
    pub name: String,
    pub order: i32,
    pub terms: Vec<String>,
    pub connectives: Vec<String>,
}

/// A request to Extract the current selection into a Monad — a provisional name
/// plus the selected member addresses (`system:<id>`, …).
#[derive(Clone, PartialEq)]
pub struct ExtractRequest {
    pub name: String,
    pub members: Vec<String>,
}


/// A selectable column — one **tag key**. The view is composed by choosing which
/// keys to show (the Tag reconciler's own by-key action). Everything is a tag, so
/// a column is just a tag key surfaced.
#[derive(Clone, Copy, PartialEq)]
enum ColKey {
    Order,
    Name,
    Perspective,
    Citation,
    Cites,
    Note,
}

/// Canonical column order (independent of the order keys were toggled on).
const ALL_COLS: [ColKey; 6] = [
    ColKey::Order,
    ColKey::Name,
    ColKey::Perspective,
    ColKey::Citation,
    ColKey::Cites,
    ColKey::Note,
];

impl ColKey {
    fn label(self) -> &'static str {
        match self {
            ColKey::Order => "Order",
            ColKey::Name => "Name",
            ColKey::Perspective => "Perspective",
            ColKey::Citation => "Citation",
            ColKey::Cites => "Cites",
            ColKey::Note => "Note",
        }
    }
}

/// A Nullad row is an **element**: either a System (a fragment/system in the
/// graph) or a Reference (a citation). Everything is data; both are rows.
#[derive(Clone, Copy)]
enum Row<'a> {
    Sys(&'a InstanceSystem),
    Ref(&'a ReferenceView),
    Raw(&'a RawElement),
    Seq(&'a SequenceView),
}

impl Row<'_> {
    fn order(&self) -> Option<i32> {
        match self {
            Row::Sys(s) => Some(s.order),
            Row::Ref(r) => order_of(r),
            Row::Raw(e) => Some(e.order),
            // A monad isn't tied to a K_n order; None sorts it to the top so
            // monads (Architecture Monad, Data, …) are easy to find.
            Row::Seq(_) => None,
        }
    }
    /// Lower-cased haystack for free-text search.
    fn hay(&self) -> String {
        match self {
            Row::Sys(s) => s.name.to_lowercase(),
            Row::Raw(e) => e.name.to_lowercase(),
            Row::Seq(s) => format!("{} {}", s.name, s.members.join(" ")).to_lowercase(),
            Row::Ref(r) => format!(
                "{} {} {} {} {} {}",
                persp(r), src(r), art(r), loc(r), r.target,
                r.note.clone().unwrap_or_default()
            )
            .to_lowercase(),
        }
    }
    /// `system:<id>` address for Extract (a Reference contributes its target).
    fn system_addr(&self) -> Option<String> {
        match self {
            Row::Sys(s) => Some(format!("system:{}", s.id)),
            Row::Ref(r) => r.target_system.as_ref().map(|s| format!("system:{}", s.id)),
            Row::Raw(_) | Row::Seq(_) => None,
        }
    }
}

/// Whether a **row** passes the header **order** scope and free-text **search**.
/// (The Filter proper is the SPO constraints — see `passes_constraints`.)
fn passes_row(row: Row, filter_order: Option<i32>, needle: &str) -> bool {
    filter_order.is_none_or(|o| row.order() == Some(o))
        && (needle.is_empty() || row.hay().contains(needle))
}
/// Whether a row falls inside a **bucket scope** (a monad's member addresses).
/// `None` = no scope (whole registry). A scoped view lists only the members
/// (systems addressed as `system:<id>`, plus references onto them).
fn in_scope(row: &Row, scope: Option<&[String]>) -> bool {
    match scope {
        None => true,
        Some(members) => row.system_addr().is_some_and(|a| members.iter().any(|m| *m == a)),
    }
}
/// A row's **deletable address** (`system:` / `sequence:` / `reference:`), used by
/// row-select CRUD. Raw nodes/edges are ephemeral (derived) — not deletable.
fn row_addr(row: &Row) -> Option<String> {
    match row {
        Row::Sys(s) => Some(format!("system:{}", s.id)),
        Row::Seq(s) => Some(format!("sequence:{}", s.id)),
        Row::Ref(r) => Some(format!("reference:{}", r.id)),
        Row::Raw(_) => None,
    }
}

/// A row's **subject id** -- what triples are keyed by. References assert about their
/// target system; raw nodes/edges are contextual (no independent subject).
fn row_subject(row: &Row) -> Option<String> {
    match row {
        Row::Sys(s) => Some(s.id.clone()),
        Row::Seq(s) => Some(s.id.clone()),
        Row::Ref(r) => r.target_system.as_ref().map(|s| s.id.clone()),
        Row::Raw(_) => None,
    }
}

/// A row passes the Filter when its subject satisfies **every** stacked constraint
/// (predicate -> selected objects): AND across predicates, OR within a predicate.
fn passes_constraints(
    row: &Row,
    constraints: &HashMap<String, HashSet<String>>,
    triples: &[Triple],
) -> bool {
    if constraints.is_empty() {
        return true;
    }
    let Some(subject) = row_subject(row) else {
        return true; // raw rows are contextual, not independent subjects
    };
    constraints.iter().all(|(pred, vals)| {
        vals.is_empty()
            || triples
                .iter()
                .any(|t| t.subject == subject && &t.predicate == pred && vals.contains(&t.object))
    })
}

/// All Nullad rows (every System + Sequence/Monad + Reference + focused raw
/// node/edge), unfiltered.
fn all_rows<'a>(
    systems: &'a [InstanceSystem],
    seqs: &'a [SequenceView],
    refs: &'a [ReferenceView],
    raw: &'a [RawElement],
) -> Vec<Row<'a>> {
    systems
        .iter()
        .map(Row::Sys)
        .chain(seqs.iter().map(Row::Seq))
        .chain(refs.iter().map(Row::Ref))
        .chain(raw.iter().map(Row::Raw))
        .collect()
}

#[function_component(ReferenceBrowser)]
pub fn reference_browser(props: &ReferenceBrowserProps) -> Html {
    let search = use_state(String::new);
    // Sort (=) selects the header tags (which tag keys are columns).
    let sort_open = use_state(|| false);
    let visible_cols = use_state(|| vec![ColKey::Order, ColKey::Name, ColKey::Citation]);
    // Filter (−) scopes the data returned, by cite-degree. Default: Systems only —
    // coherence/designations/terms/connectives are opt-in.
    let filter_open = use_state(|| false);
    // SPO filter over a flat triple store: which predicate (key) is being *viewed*
    // in the menu, and the **stacked** constraints (predicate → selected objects),
    // which AND together. Everything is a predicate now — `type` is not special.
    let active_pred = use_state(|| "order".to_string());
    let spo_constraints = use_state(HashMap::<String, HashSet<String>>::new);
    // Reciprocal traversal: a **pinned subject** whose quads (predicate · object ·
    // source) are shown — the S→(P,O,source) read, "a location advertising its
    // values across sections".
    let inspect = use_state(|| Option::<String>::None);
    // Row-select CRUD: the set of selected row addresses (system:/sequence:/reference:).
    let selected = use_state(HashSet::<String>::new);
    // Editor: author a new System from custom values (the app-authored path).
    let editor_open = use_state(|| false);
    let ed_name = use_state(String::new);
    let ed_order = use_state(|| 3i32);
    let ed_terms = use_state(|| vec![String::new(); 3]);
    let ed_conns = use_state(|| vec![String::new(); 3]);

    let refs = &props.references;
    let systems = &props.instance_systems;
    let raw = &props.raw_elements;
    let seqs = &props.sequences;
    // Flatten everything into SPO triples — the uniform substrate the Filter queries.
    let triples = build_triples(systems, refs, seqs);
    // The order filter comes from the header (Nullad = None = all).
    let filter_order = props.filter_order;
    // A bucket monad scopes the view to its members (a group for sorting).
    let scope = props.scope_ids.as_deref();

    // The Extract selection: distinct systems among the currently-filtered rows
    // (Systems directly; References via their target), as `system:<id>` members.
    let needle = search.to_lowercase();
    let mut seen = BTreeSet::new();
    let extract_members: Vec<String> = all_rows(systems, seqs, refs, raw)
        .into_iter()
        .filter(|row| passes_row(*row, filter_order, &needle))
        .filter(|row| in_scope(row, scope))
        .filter(|row| passes_constraints(row, &spo_constraints, &triples))
        .filter_map(|row| row.system_addr())
        .filter(|m| seen.insert(m.clone()))
        .collect();
    // Provisional Monad name (the "integral" naming is a later refinement).
    let extract_name = match filter_order {
        Some(o) => format!("Monad — {} {}", o, order_name(o)),
        None => format!("Monad — Nullad selection ({})", extract_members.len()),
    };

    // ---- Editor form: author a System from custom values ----
    let toggle_editor = { let o = editor_open.clone(); Callback::from(move |_: MouseEvent| o.set(!*o)) };
    let on_ed_name = { let s = ed_name.clone(); Callback::from(move |e: InputEvent| s.set(e.target_unchecked_into::<HtmlInputElement>().value())) };
    let on_ed_order = {
        let (ed_order, ed_terms, ed_conns) = (ed_order.clone(), ed_terms.clone(), ed_conns.clone());
        Callback::from(move |e: InputEvent| {
            let n = e.target_unchecked_into::<HtmlInputElement>().value().parse::<i32>().unwrap_or(3).clamp(1, 12);
            ed_order.set(n);
            ed_terms.set(vec![String::new(); n as usize]);
            ed_conns.set(vec![String::new(); (n * (n - 1) / 2) as usize]);
        })
    };
    // An input bound to index `i` of a Vec<String> state.
    let vec_input = |state: &UseStateHandle<Vec<String>>, i: usize, ph: String| -> Html {
        let val = state.get(i).cloned().unwrap_or_default();
        let state = state.clone();
        let oninput = Callback::from(move |e: InputEvent| {
            let v = e.target_unchecked_into::<HtmlInputElement>().value();
            let mut next = (*state).clone();
            if i < next.len() { next[i] = v; }
            state.set(next);
        });
        html! { <input class="ed-input" placeholder={ph} value={ val } oninput={ oninput } /> }
    };
    let ed_order_val = *ed_order;
    let on_create = {
        let (on_author, ed_name, ed_terms, ed_conns) = (props.on_author.clone(), ed_name.clone(), ed_terms.clone(), ed_conns.clone());
        Callback::from(move |_: MouseEvent| {
            on_author.emit(AuthorRequest {
                name: (*ed_name).clone(),
                order: ed_order_val,
                terms: (*ed_terms).clone(),
                connectives: (*ed_conns).clone(),
            });
        })
    };
    // Prefill from the canonical system of the current order ("open the canonical").
    let on_prefill = {
        let (templates, ed_order, ed_terms, ed_conns) =
            (props.templates.clone(), ed_order.clone(), ed_terms.clone(), ed_conns.clone());
        Callback::from(move |_: MouseEvent| {
            if let Some(t) = templates.iter().find(|t| t.order == *ed_order) {
                ed_terms.set(t.terms.clone());
                ed_conns.set(t.connectives.clone());
            }
        })
    };
    let can_create = !ed_name.trim().is_empty()
        && ed_terms.iter().all(|t| !t.trim().is_empty())
        && ed_conns.iter().all(|c| !c.trim().is_empty());

    // Load opens the OS file browser to pick a JSON system file (import format TBD).
    let on_file = Callback::from(move |e: Event| {
        let input = e.target_unchecked_into::<HtmlInputElement>();
        if let Some(f) = input.files().and_then(|fs| fs.get(0)) {
            web_sys::console::log_1(
                &format!("Load: {} ({} bytes) — JSON import TBD", f.name(), f.size()).into(),
            );
        }
    });
    let can_extract = !extract_members.is_empty();
    let on_extract_click = {
        let on_extract = props.on_extract.clone();
        let name = extract_name.clone();
        let members = extract_members.clone();
        Callback::from(move |_: MouseEvent| {
            on_extract.emit(ExtractRequest { name: name.clone(), members: members.clone() })
        })
    };
    let extract_title = format!(
        "Extract — materialize this selection ({} systems) into a Monad (Nullad → Monad)",
        extract_members.len()
    );

    // New (left of Sort) folds the data-entry plane down under the search bar.
    let new_btn = html! {
        <button
            class={ classes!("elt-btn", (*editor_open).then_some("active")) }
            onclick={ toggle_editor.clone() }
            title="New — author a system from custom terms/connectives"
        >{ if *editor_open { "New ▴" } else { "New ▾" } }</button>
    };
    // Extract · Load · Transform (right of the search bar) — the operation edge.
    let elt_btns = html! {
        <>
            <button class="elt-btn" disabled={ !can_extract } onclick={ on_extract_click } title={ extract_title }>
                { format!("Extract ({})", extract_members.len()) }
            </button>
            <label class="elt-btn" title="Load — open a JSON system file (import format TBD)">
                { "Load ↥" }
                <input type="file" accept="application/json,.json" style="display:none;" onchange={ on_file } />
            </label>
            <button class="elt-btn" disabled=true title="Transform — apply a Functor to a loaded system. Not yet wired.">
                { "Transform" }
            </button>
            if let Some(note) = props.extract_note.as_deref() {
                <span class="elt-note">{ note }</span>
            }
        </>
    };
    // The data-entry plane — folds under the control bar when New is open. Temporary
    // scaffolding until on-graph label editing lands.
    let editor_form = if *editor_open {
        html! {
            <div class="editor-form">
                <div class="editor-row">
                    <input class="ed-input ed-name" placeholder="New system name" value={ (*ed_name).clone() } oninput={ on_ed_name } />
                    <label class="ed-label">{ "Order" }
                        <input class="ed-input ed-order" type="number" min="1" max="12" value={ ed_order_val.to_string() } oninput={ on_ed_order } />
                    </label>
                    <button class="elt-btn" onclick={ on_prefill } title="Prefill terms/connectives from the canonical system of this order">{ "↺ Canonical" }</button>
                    <button class="elt-btn" disabled={ !can_create } onclick={ on_create }>{ "Create system" }</button>
                </div>
                <div class="editor-fields">
                    <span class="facet-label">{ format!("Terms ({})", ed_order_val) }</span>
                    { for (0..ed_order_val as usize).map(|i| vec_input(&ed_terms, i, format!("term {}", i + 1))) }
                </div>
                <div class="editor-fields">
                    <span class="facet-label">{ format!("Connectives ({})", ed_order_val * (ed_order_val - 1) / 2) }</span>
                    { for (0..(ed_order_val * (ed_order_val - 1) / 2) as usize).map(|j| vec_input(&ed_conns, j, format!("edge {}", j + 1))) }
                </div>
            </div>
        }
    } else {
        html! {}
    };

    html! {
        <div class="reference-browser">
            { table_view(TableCtx {
                refs,
                systems,
                seqs,
                raw,
                on_load: &props.on_load,
                on_view_sequence: &props.on_view_sequence,
                on_delete_sequence: &props.on_delete_sequence,
                on_delete_rows: &props.on_delete_rows,
                filter_order,
                scope,
                search: &search,
                sort_open: &sort_open,
                visible_cols: &visible_cols,
                filter_open: &filter_open,
                active_pred: &active_pred,
                spo_constraints: &spo_constraints,
                triples: &triples,
                inspect: &inspect,
                selected: &selected,
                new_btn,
                elt_btns,
                editor_form,
            }) }
        </div>
    }
}

/// Grouped arguments for the table view (keeps the signature under one struct).
struct TableCtx<'a> {
    refs: &'a [ReferenceView],
    /// Every system in the graph — shown as rows alongside references.
    systems: &'a [InstanceSystem],
    /// Every Sequence / Monad — shown as rows (Cites = members).
    seqs: &'a [SequenceView],
    /// The focused system's raw nodes/edges (shown when Term/Connective on).
    raw: &'a [RawElement],
    /// Click a system row to view it (loads into the graph).
    on_load: &'a Callback<String>,
    /// Click a monad row to enter it (navigate its members via the header).
    on_view_sequence: &'a Callback<Vec<String>>,
    /// Delete a monad row (the ✕).
    on_delete_sequence: &'a Callback<String>,
    /// Delete selected rows by address (row-select CRUD).
    on_delete_rows: &'a Callback<Vec<String>>,
    /// Order filter from the header (`None` = Nullad = all).
    filter_order: Option<i32>,
    /// Bucket scope — when a bucket monad is entered, show only its members.
    scope: Option<&'a [String]>,
    search: &'a UseStateHandle<String>,
    /// Sort (=) popover — selecting the header tags (which keys are columns).
    sort_open: &'a UseStateHandle<bool>,
    visible_cols: &'a UseStateHandle<Vec<ColKey>>,
    /// Filter (−) popover — the SPO predicate→object query.
    filter_open: &'a UseStateHandle<bool>,
    /// The viewed predicate (menu), the stacked constraints, and the triple store.
    active_pred: &'a UseStateHandle<String>,
    spo_constraints: &'a UseStateHandle<HashMap<String, HashSet<String>>>,
    triples: &'a [Triple],
    /// Reciprocal traversal: the pinned subject whose quads are shown.
    inspect: &'a UseStateHandle<Option<String>>,
    /// Selected row addresses (row-select CRUD).
    selected: &'a UseStateHandle<HashSet<String>>,
    /// New toggle (placed left of Sort); ELT buttons (right of search); and the
    /// editor plane that folds under the control bar. Pre-rendered in the body.
    new_btn: Html,
    elt_btns: Html,
    editor_form: Html,
}

fn table_view(ctx: TableCtx) -> Html {
    let TableCtx {
        refs,
        systems,
        seqs,
        raw,
        on_load,
        on_view_sequence,
        on_delete_sequence,
        on_delete_rows,
        filter_order,
        scope,
        search,
        sort_open,
        visible_cols,
        filter_open,
        active_pred,
        spo_constraints,
        triples,
        inspect,
        selected,
        new_btn,
        elt_btns,
        editor_form,
    } = ctx;

    // Filter — same predicate Extract uses (header order + cite-degree + search).
    // Rows: every System + Monad + Reference + the focused system's raw nodes/edges.
    let needle = search.to_lowercase();
    let mut rows: Vec<Row> = all_rows(systems, seqs, refs, raw)
        .into_iter()
        .filter(|row| passes_row(*row, filter_order, &needle))
        .filter(|row| in_scope(row, scope))
        .filter(|row| passes_constraints(row, spo_constraints, triples))
        .collect();
    // A reference is **metadata on its subject system**, not a peer row. If the
    // system it cites is already shown, fold the reference away (this is what made
    // a whole-system citation appear as a second, duplicate "system"). References
    // whose subject isn't shown still surface (they're the only representation).
    let shown_sys: HashSet<&str> = rows
        .iter()
        .filter_map(|r| match r {
            Row::Sys(s) => Some(s.id.as_str()),
            _ => None,
        })
        .collect();
    rows.retain(|r| match r {
        Row::Ref(rf) => rf
            .target_system
            .as_ref()
            .is_none_or(|ts| !shown_sys.contains(ts.id.as_str())),
        _ => true,
    });
    // Default row order: by systematic order (the header axis).
    rows.sort_by_key(|row| row.order());

    // Sort (=) — select which tag keys are the columns (the header tags).
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
    // Filter (−), SPO query — pick the **predicate** (key). Switching predicate keeps
    // the other constraints (they stack). A dot marks a predicate with active values.
    let pred_tab = |p: String| -> Html {
        let on = *(*active_pred) == p;
        let active = spo_constraints.get(&p).is_some_and(|v| !v.is_empty());
        let ap = active_pred.clone();
        let pc = p.clone();
        let onclick = Callback::from(move |_: MouseEvent| ap.set(pc.clone()));
        html! {
            <button class={ classes!("facet-tab", on.then_some("active")) } onclick={ onclick }>
                { pred_label(&p) }{ if active { " ●" } else { "" } }
            </button>
        }
    };
    // …then pick its **objects** (values) — the SPO drill-down, scoped to the current
    // constraints. Chips show the bare object (provenance lives in the ⌕ inspector,
    // not cluttering every chip). Toggling updates this predicate's constraint slot.
    let val_chip = |p: String, v: String| -> Html {
        let on = spo_constraints.get(&p).is_some_and(|s| s.contains(&v));
        let sc = spo_constraints.clone();
        let pc = p.clone();
        let vc = v.clone();
        let onclick = Callback::from(move |_: MouseEvent| {
            let mut next = (*sc).clone();
            let slot = next.entry(pc.clone()).or_default();
            if !slot.remove(&vc) {
                slot.insert(vc.clone());
            }
            next.retain(|_, s| !s.is_empty()); // drop empty slots (no constraint)
            sc.set(next);
        });
        html! {
            <button class={ classes!("facet-chip", on.then_some("active")) } onclick={ onclick }>
                { v }
            </button>
        }
    };
    // Visible columns in canonical order (independent of toggle order).
    let cols: Vec<ColKey> = ALL_COLS.into_iter().filter(|c| visible_cols.contains(c)).collect();

    let cell = |k: ColKey, row: &Row| -> Html {
        let order_cell = |o: Option<i32>| html! { { o.map(|o| format!("{} {}", o, order_name(o))).unwrap_or_default() } };
        match (k, row) {
            (ColKey::Order, _) => order_cell(row.order()),
            (ColKey::Name, Row::Sys(s)) => {
                let load = {
                    let on_load = on_load.clone();
                    let id = s.id.clone();
                    Callback::from(move |_: MouseEvent| on_load.emit(id.clone()))
                };
                let insp = {
                    let inspect = inspect.clone();
                    let id = s.id.clone();
                    Callback::from(move |e: MouseEvent| {
                        e.stop_propagation();
                        inspect.set(Some(id.clone()));
                    })
                };
                html! {
                    <span class="sys-cell">
                        <button class="tag tag-system row-open" onclick={ load } title="View this system">{ &s.name }</button>
                        <button class="row-inspect" onclick={ insp } title="Inspect its triples — predicate · object · source">{ "⌕" }</button>
                    </span>
                }
            }
            (ColKey::Name, Row::Ref(r)) => {
                // A reference's cited system is clickable → load it into the graph
                // (so DU citations like "Dramatic Universe I Heptad" are viewable).
                match r.target_system.as_ref() {
                    Some(s) => {
                        let on_load = on_load.clone();
                        let id = s.id.clone();
                        let onclick = Callback::from(move |_: MouseEvent| on_load.emit(id.clone()));
                        html! { <button class="tag tag-system row-open" onclick={ onclick } title="View the cited system in the graph">{ &s.name }</button> }
                    }
                    None => html! { { r.target.clone() } },
                }
            }
            (ColKey::Name, Row::Raw(e)) => {
                let cls = if e.is_edge { "tag tag-locator" } else { "tag tag-perspective" };
                html! { <span class={ cls }>{ &e.name }</span> }
            }
            (ColKey::Cites, Row::Raw(e)) => html! { { if e.is_edge { "edge" } else { "node" } } },
            (_, Row::Raw(_)) => html! {},
            // A Monad / Sequence row: name as a tag (member count), clickable to
            // view its first system member in the graph.
            (ColKey::Name, Row::Seq(s)) => {
                let label = format!("⬡ {} ({})", s.name, s.members.len());
                let del = {
                    let on_delete_sequence = on_delete_sequence.clone();
                    let id = s.id.clone();
                    let onclick = Callback::from(move |e: MouseEvent| {
                        e.stop_propagation();
                        on_delete_sequence.emit(id.clone());
                    });
                    html! { <button class="row-delete" onclick={ onclick } title="Delete this monad">{ "✕" }</button> }
                };
                let name_tag = if s.members.iter().any(|m| m.starts_with("system:")) {
                    let on_view_sequence = on_view_sequence.clone();
                    let members = s.members.clone();
                    let onclick = Callback::from(move |_: MouseEvent| on_view_sequence.emit(members.clone()));
                    html! { <button class="tag tag-monad row-open" onclick={ onclick } title="Enter this monad — header buttons navigate its members">{ label }</button> }
                } else {
                    html! { <span class="tag tag-monad">{ label }</span> }
                };
                html! { <span class="monad-cell">{ name_tag }{ del }</span> }
            }
            (ColKey::Cites, Row::Seq(s)) => html! {
                <span class="tags">
                    { for s.members.iter().map(|m| html!{ <span class="tag tag-member">{ m }</span> }) }
                </span>
            },
            (_, Row::Seq(_)) => html! {},
            (ColKey::Perspective, Row::Ref(r)) => persp_tag(r),
            (ColKey::Citation, Row::Ref(r)) => citation_tags(r),
            (ColKey::Cites, Row::Ref(r)) => {
                let f = frag(r);
                let cites = if f.is_empty() { "whole system".to_string() } else { f };
                let target_label = r.target_system.as_ref().map(|s| s.name.clone()).unwrap_or_else(|| r.target.clone());
                html! { <span title={ target_label }>{ cites }</span> }
            }
            (ColKey::Note, Row::Ref(r)) => html! { { r.note.clone().unwrap_or_default() } },
            // System rows carry no perspective/citation/cites/note — a whole system.
            (ColKey::Cites, Row::Sys(_)) => html! { { "whole system" } },
            (_, Row::Sys(_)) => html! {},
        }
    };

    let on_search = {
        let search = search.clone();
        Callback::from(move |e: InputEvent| {
            let v = e.target_unchecked_into::<HtmlInputElement>().value();
            search.set(v);
        })
    };
    let toggle_sort = { let s = sort_open.clone(); Callback::from(move |_: MouseEvent| s.set(!*s)) };
    let toggle_filter = { let s = filter_open.clone(); Callback::from(move |_: MouseEvent| s.set(!*s)) };
    let scoped = !spo_constraints.is_empty();

    // Reciprocal traversal — a pinned SUBJECT advertising its values (S → P·O·source).
    let inspect_panel = if let Some(sid) = (**inspect).clone() {
        let name = systems
            .iter()
            .find(|s| s.id == sid)
            .map(|s| s.name.clone())
            .unwrap_or_else(|| sid.clone());
        // Group the subject's triples: predicate → position → object → citation(s).
        // Grouping by position is the bijective mapping — every perspective's value at
        // the SAME node lands together (node 1: Will · Function[DU1] · Affirmation[DU2]).
        let mut by_pred: BTreeMap<String, BTreeMap<String, BTreeMap<String, BTreeSet<String>>>> =
            BTreeMap::new();
        for t in triples.iter().filter(|t| t.subject == sid) {
            let cites = by_pred
                .entry(t.predicate.clone())
                .or_default()
                .entry(t.position.clone())
                .or_default()
                .entry(t.object.clone())
                .or_default();
            if !t.citation.is_empty() {
                cites.insert(t.citation.clone());
            }
        }
        // Systematics predicate order; any others appended after.
        const ORDER: [&str; 8] = [
            "name", "order", "coherence", "term-designation", "connective-designation",
            "term", "connective", "source",
        ];
        let mut preds: Vec<String> = ORDER
            .iter()
            .map(|s| s.to_string())
            .filter(|p| by_pred.contains_key(p))
            .collect();
        for p in by_pred.keys() {
            if !preds.contains(p) {
                preds.push(p.clone());
            }
        }
        let groups: Vec<PredGroup> = preds
            .into_iter()
            .map(|p| {
                let positions = by_pred.remove(&p).unwrap_or_default();
                PredGroup {
                    label: pred_label(&p),
                    is_edge: p == "connective",
                    positions: positions
                        .into_iter()
                        .map(|(position, objs)| PosGroup {
                            position,
                            objects: objs
                                .into_iter()
                                .map(|(object, cites)| ObjCite {
                                    object,
                                    citations: cites.into_iter().collect(),
                                })
                                .collect(),
                        })
                        .collect(),
                }
            })
            .collect();
        let on_close = {
            let inspect = inspect.clone();
            Callback::from(move |_: ()| inspect.set(None))
        };
        html! { <Inspector subject_name={ name } groups={ groups } on_close={ on_close } /> }
    } else {
        html! {}
    };

    // Row-select CRUD: delete the selected addresses, then clear the selection.
    let on_delete_selected = {
        let selected = selected.clone();
        let on_delete_rows = on_delete_rows.clone();
        Callback::from(move |_: MouseEvent| {
            let addrs: Vec<String> = selected.iter().cloned().collect();
            if !addrs.is_empty() {
                on_delete_rows.emit(addrs);
            }
            selected.set(HashSet::new());
        })
    };
    let sel_count = selected.len();

    html! {
        <>
            // Control bar (single line): Extract·Load·Transform (left) · search ·
            // New · Sort (=) · Filter (−) (right). ELT is the operation edge; Sort
            // selects header tags (columns); Filter scopes the data (by cite-degree).
            <div class="ref-controlbar">
                { elt_btns }
                <input
                    class="ref-search"
                    type="text"
                    placeholder="Search…"
                    value={ (**search).clone() }
                    oninput={ on_search }
                />
                { new_btn }
                <div class="control-pop">
                    <button
                        class={ classes!("control-btn", (**sort_open).then_some("active")) }
                        onclick={ toggle_sort }
                        title="Sort — select the header tags (which tag keys are columns)"
                    >{ format!("Sort ({}) ▾", cols.len()) }</button>
                    if **sort_open {
                        <div class="control-menu">
                            <span class="facet-label">{ "header tags · columns" }</span>
                            <div class="col-chips">
                                { for ALL_COLS.into_iter().map(col_chip) }
                            </div>
                        </div>
                    }
                </div>

                <div class="control-pop">
                    <button
                        class={ classes!("control-btn", (scoped || **filter_open).then_some("active")) }
                        onclick={ toggle_filter }
                        title="Filter — an SPO query: pick a predicate (key), then its objects (values)"
                    >{ if scoped { "Filter ● ▾" } else { "Filter ▾" } }</button>
                    if **filter_open {
                        <div class="control-menu">
                            <span class="facet-label">{ "filter — predicate (key), stackable" }</span>
                            <div class="facet-tabs">
                                {
                                    // Every predicate present in the SPO triples is picked up
                                    // automatically — zero per-predicate code.
                                    for all_predicates(triples).into_iter().map(pred_tab)
                                }
                            </div>
                            <span class="facet-label">
                                { format!("{} — objects (values)", pred_label(active_pred)) }
                            </span>
                            <div class="col-chips">
                                { {
                                    let pred = (**active_pred).clone();
                                    // Scoped to the other active constraints, so the
                                    // drill-down narrows (order=Triad ⇒ triadic terms).
                                    let vals = pred_objects_scoped(triples, &pred, &spo_constraints);
                                    if vals.is_empty() {
                                        html! { <span class="facet-hint">{ "no values in the current scope" }</span> }
                                    } else {
                                        html! { for vals.into_iter().map(|v| val_chip(pred.clone(), v)) }
                                    }
                                } }
                            </div>
                        </div>
                    }
                </div>

                <span class="ref-count">{ format!("{} shown", rows.len()) }</span>
                if sel_count > 0 {
                    <button class="row-delete-btn" onclick={ on_delete_selected }
                        title="Delete the selected systems / monads / references">
                        { format!("🗑 Delete {sel_count} selected") }
                    </button>
                }
            </div>

            // Data-entry plane — folds down under the control bar when New is open.
            { editor_form }

            // Reciprocal traversal — the pinned subject's quads (S → P·O·source).
            { inspect_panel }

            <div class="ref-table-wrap">
                if cols.is_empty() {
                    <p class="ref-empty">{ "No columns — pick header tags under Sort ▾." }</p>
                } else {
                    <table class="ref-table">
                        <thead>
                            <tr>
                                <th class="ref-th row-select" />
                                { for cols.iter().map(|k| html!{ <th class="ref-th">{ k.label() }</th> }) }
                            </tr>
                        </thead>
                        <tbody>
                            { for rows.iter().map(|r| {
                                let addr = row_addr(r);
                                let checked = addr.as_ref().is_some_and(|a| selected.contains(a));
                                let on_toggle = {
                                    let selected = selected.clone();
                                    let addr = addr.clone();
                                    Callback::from(move |_: MouseEvent| {
                                        if let Some(a) = &addr {
                                            let mut next = (*selected).clone();
                                            if !next.remove(a) { next.insert(a.clone()); }
                                            selected.set(next);
                                        }
                                    })
                                };
                                html!{
                                    <tr class={ if checked { "row-selected" } else { "" } }>
                                        <td class="row-select">
                                            if addr.is_some() {
                                                <input type="checkbox" checked={ checked } onclick={ on_toggle } />
                                            }
                                        </td>
                                        { for cols.iter().map(|k| html!{ <td>{ cell(*k, r) }</td> }) }
                                    </tr>
                                }
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
