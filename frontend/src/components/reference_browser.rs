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

use crate::api::client::{InstanceSystem, ReferenceView, SequenceView};

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
    /// Feedback from the last Extract / author (shown in the Editor).
    #[prop_or_default]
    pub extract_note: Option<String>,
    /// Author a new System from custom term/connective values (the editor).
    pub on_author: Callback<AuthorRequest>,
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
            // A monad's order = its member count (a scoped "all" of N members).
            Row::Seq(s) => Some(s.members.len() as i32),
        }
    }
    fn kind(&self) -> CiteKind {
        match self {
            Row::Sys(_) => CiteKind::System,
            // Monads/sequences are system-like containers → show with Systems.
            Row::Seq(_) => CiteKind::System,
            Row::Ref(r) => cite_kind(r),
            Row::Raw(e) => if e.is_edge { CiteKind::Connective } else { CiteKind::Term },
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

/// The **degree** of what a reference cites (its target fragment) — the data
/// categorised by number, per the schema: 1 term-designation · 2 connective-
/// designation · 3 coherence · 4 term (character) · 5 connective (character) ·
/// 6 system (their coalescence). Filtering by degree lets you see only systems
/// (a *manifold* = a System not yet placed in a higher order), only terms, etc.
#[derive(Clone, Copy, PartialEq)]
enum CiteKind {
    TermDesignation,
    ConnectiveDesignation,
    Coherence,
    Term,
    Connective,
    System,
}

const ALL_KINDS: [CiteKind; 6] = [
    CiteKind::TermDesignation,
    CiteKind::ConnectiveDesignation,
    CiteKind::Coherence,
    CiteKind::Term,
    CiteKind::Connective,
    CiteKind::System,
];

impl CiteKind {
    fn label(self) -> &'static str {
        match self {
            CiteKind::TermDesignation => "Term designation",
            CiteKind::ConnectiveDesignation => "Connective designation",
            CiteKind::Coherence => "Coherence",
            CiteKind::Term => "Term",
            CiteKind::Connective => "Connective",
            CiteKind::System => "System",
        }
    }
}

/// Classify a reference by what it cites (its `#fragment`), i.e. its degree.
fn cite_kind(r: &ReferenceView) -> CiteKind {
    let f = frag(r);
    if f.is_empty() {
        CiteKind::System
    } else if f == "coherence" {
        CiteKind::Coherence
    } else if f == "term-designation" {
        CiteKind::TermDesignation
    } else if f == "connective-designation" {
        CiteKind::ConnectiveDesignation
    } else if f.starts_with("term:") {
        CiteKind::Term
    } else if f.starts_with("conn:") {
        CiteKind::Connective
    } else {
        CiteKind::System
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
/// Whether a **row** (System or Reference) is in the current selection: header
/// **order** (Sort's scope), the **Filter** by cite-degree, and **search**.
/// Shared by the table (what to show) and Extract (what to materialize).
fn passes_row(row: Row, filter_order: Option<i32>, active_kinds: &[CiteKind], needle: &str) -> bool {
    filter_order.is_none_or(|o| row.order() == Some(o))
        && active_kinds.contains(&row.kind())
        && (needle.is_empty() || row.hay().contains(needle))
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
    let search = use_state(String::new);
    // Sort (=) selects the header tags (which tag keys are columns).
    let sort_open = use_state(|| false);
    let visible_cols = use_state(|| vec![ColKey::Order, ColKey::Name, ColKey::Citation]);
    // Filter (−) scopes the data returned, by cite-degree. Default: Systems only —
    // coherence/designations/terms/connectives are opt-in.
    let filter_open = use_state(|| false);
    let active_kinds = use_state(|| vec![CiteKind::System]);
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
    // The order filter comes from the header (Nullad = None = all).
    let filter_order = props.filter_order;

    // Caption for the active header scope.
    let scope_label = match filter_order {
        Some(o) => format!("{} {} only", o, order_name(o)),
        None => "Nullad — all orders".to_string(),
    };

    // The Extract selection: distinct systems among the currently-filtered rows
    // (Systems directly; References via their target), as `system:<id>` members.
    let needle = search.to_lowercase();
    let mut seen = BTreeSet::new();
    let extract_members: Vec<String> = all_rows(systems, seqs, refs, raw)
        .into_iter()
        .filter(|row| passes_row(*row, filter_order, &active_kinds, &needle))
        .filter_map(|row| row.system_addr())
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

    html! {
        <div class="reference-browser">
            // Editor (the operation edge): Extract · Load · Transform + New system.
            { elt_triad(EltCtx {
                extract_members: &extract_members,
                extract_name: &extract_name,
                on_extract: &props.on_extract,
                extract_note: props.extract_note.as_deref(),
                editor_open: &editor_open,
                toggle_editor: &toggle_editor,
            }) }

            if *editor_open {
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
                <span class="ref-count">{ format!("{} systems · {} refs", systems.len(), refs.len()) }</span>
            </div>
            {
                match *tab {
                    Tab::Table => table_view(TableCtx {
                        refs,
                        systems,
                        seqs,
                        raw,
                        on_load: &props.on_load,
                        filter_order,
                        search: &search,
                        sort_open: &sort_open,
                        visible_cols: &visible_cols,
                        filter_open: &filter_open,
                        active_kinds: &active_kinds,
                    }),
                    Tab::Compare => compare_view(refs, filter_order),
                }
            }
        </div>
    }
}

/// Grouped arguments for the ELT / Editor control.
struct EltCtx<'a> {
    /// The current selection to Extract, as `system:<id>` member addresses.
    extract_members: &'a [String],
    /// Provisional name for the Monad Extract would create.
    extract_name: &'a str,
    on_extract: &'a Callback<ExtractRequest>,
    /// Feedback from the last Extract / author, if any.
    extract_note: Option<&'a str>,
    editor_open: &'a UseStateHandle<bool>,
    toggle_editor: &'a Callback<MouseEvent>,
}

/// The ELT triad control: Extract · Load · Transform.
/// - **Extract** materializes the current selection into a Monad (Nullad → Monad).
/// - **Load** opens a menu of instance systems.
/// - **Transform** (apply a Functor) is the third edge, not yet wired.
fn elt_triad(ctx: EltCtx) -> Html {
    let EltCtx {
        extract_members,
        extract_name,
        on_extract,
        extract_note,
        editor_open,
        toggle_editor,
    } = ctx;

    // Load opens the OS file browser to pick a JSON system file. (Import format is
    // not settled yet — for now we just read the chosen file's name/size.)
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
        <div class="elt-triad" title="Editor — Extract · Load · Transform + author a new system">
            <button
                class={ classes!("elt-btn", (**editor_open).then_some("active")) }
                onclick={ toggle_editor.clone() }
                title="New — author a system from custom terms/connectives"
            >{ if **editor_open { "New ▴" } else { "New ▾" } }</button>
            <button
                class="elt-btn"
                disabled={ !can_extract }
                onclick={ on_extract_click }
                title={ extract_title }
            >{ format!("Extract ({})", extract_members.len()) }</button>

            <label class="elt-btn" title="Load — open a JSON system file (import format TBD)">
                { "Load ↥" }
                <input
                    type="file"
                    accept="application/json,.json"
                    style="display:none;"
                    onchange={ on_file }
                />
            </label>

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
    /// Every system in the graph — shown as rows alongside references.
    systems: &'a [InstanceSystem],
    /// Every Sequence / Monad — shown as rows (Cites = members).
    seqs: &'a [SequenceView],
    /// The focused system's raw nodes/edges (shown when Term/Connective on).
    raw: &'a [RawElement],
    /// Click a system row to view it (loads into the graph).
    on_load: &'a Callback<String>,
    /// Order filter from the header (`None` = Nullad = all).
    filter_order: Option<i32>,
    search: &'a UseStateHandle<String>,
    /// Sort (=) popover — selecting the header tags (which keys are columns).
    sort_open: &'a UseStateHandle<bool>,
    visible_cols: &'a UseStateHandle<Vec<ColKey>>,
    /// Filter (−) popover — scoping the data by cite-degree.
    filter_open: &'a UseStateHandle<bool>,
    active_kinds: &'a UseStateHandle<Vec<CiteKind>>,
}

fn table_view(ctx: TableCtx) -> Html {
    let TableCtx {
        refs,
        systems,
        seqs,
        raw,
        on_load,
        filter_order,
        search,
        sort_open,
        visible_cols,
        filter_open,
        active_kinds,
    } = ctx;

    // Filter — same predicate Extract uses (header order + cite-degree + search).
    // Rows: every System + Monad + Reference + the focused system's raw nodes/edges.
    let needle = search.to_lowercase();
    let mut rows: Vec<Row> = all_rows(systems, seqs, refs, raw)
        .into_iter()
        .filter(|row| passes_row(*row, filter_order, active_kinds, &needle))
        .collect();
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
    // Filter (−) — scope the data returned by cite-degree.
    let kind_chip = |k: CiteKind| -> Html {
        let active_kinds = active_kinds.clone();
        let on = active_kinds.contains(&k);
        let onclick = Callback::from(move |_: MouseEvent| {
            let mut next = (*active_kinds).clone();
            if let Some(i) = next.iter().position(|c| *c == k) {
                next.remove(i);
            } else {
                next.push(k);
            }
            active_kinds.set(next);
        });
        html! {
            <button class={ classes!("facet-chip", on.then_some("active")) } onclick={ onclick }>
                { k.label() }
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
                let on_load = on_load.clone();
                let id = s.id.clone();
                let onclick = Callback::from(move |_: MouseEvent| on_load.emit(id.clone()));
                html! { <button class="tag tag-system row-open" onclick={ onclick } title="View this system">{ &s.name }</button> }
            }
            (ColKey::Name, Row::Ref(r)) => html! {
                { r.target_system.as_ref().map(|s| s.name.clone()).unwrap_or_default() }
            },
            (ColKey::Name, Row::Raw(e)) => {
                let cls = if e.is_edge { "tag tag-locator" } else { "tag tag-perspective" };
                html! { <span class={ cls }>{ &e.name }</span> }
            }
            (ColKey::Cites, Row::Raw(e)) => html! { { if e.is_edge { "edge" } else { "node" } } },
            (_, Row::Raw(_)) => html! {},
            // A Monad / Sequence row: name as a tag, members listed under Cites.
            (ColKey::Name, Row::Seq(s)) => html! { <span class="tag tag-monad">{ &s.name }</span> },
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
    let scoped = active_kinds.len() < ALL_KINDS.len();

    html! {
        <>
            // Control bar. Sort (=) selects the header tags (columns); Filter (−)
            // scopes the data returned (by cite-degree). Then free-text search.
            <div class="ref-controlbar">
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
                        title="Filter — scope the data returned, by cite degree (system / node / edge / …)"
                    >{ if scoped { "Filter ● ▾" } else { "Filter ▾" } }</button>
                    if **filter_open {
                        <div class="control-menu">
                            <span class="facet-label">{ "data · by cite degree" }</span>
                            <div class="col-chips">
                                { for ALL_KINDS.into_iter().map(kind_chip) }
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
                    <p class="ref-empty">{ "No columns — pick header tags under Sort ▾." }</p>
                } else {
                    <table class="ref-table">
                        <thead>
                            <tr>
                                { for cols.iter().map(|k| html!{ <th class="ref-th">{ k.label() }</th> }) }
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
