use crate::api::client::{
    GraphQLClient, InstanceSystem, PositionedChar, ReferenceView, SequenceView,
};
use crate::components::graph_view::{ApiGraphView, GraphEdit};
use crate::components::reference_browser::{
    AuthorRequest, DecomposeRequest, ExtractRequest, JoinRequest, RawElement, ReferenceBrowser,
    SystemTemplate,
};
use crate::components::system_selector::{SystemDisplay, SystemSelector};
use systematics_middleware::RenderedSystem;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

/// How the **Data** (the content the header scopes) is represented. Data · Graph
/// · Table is a triad: Data is the whole; Graph and Table are its two views.
/// - **Graph**: the K-graph canvas — a scoped system, or (for Nullad) a blank
///   canvas standing in for the future all-and-everything graph.
/// - **Table**: the reference browser — every element (all "cites"), scoped by
///   the header's selected system button.
#[derive(Clone, Copy, PartialEq)]
pub enum ViewMode {
    Graph,
    Table,
}

/// The canvas **interface state model** — a small, data-model-independent state machine
/// governing view vs edit (the data model will change later; this stays interface-side).
///
/// - **Viewing** — read-only. The Canonical sub-toggle chooses which labels show (a
///   canonical seed / an instance's class vs its own values). A seed is *only* viewable.
/// - **Editing** — on-graph editing of an *editable instance* (its nodes/edges/name
///   overwrite in place). Create/fill/rename all live here.
///
/// Transitions (centralised so the flags can't drift into illegal combos, e.g. editing a
/// canonical view):
/// - navigate / load a system         → **Viewing** (`enter_viewing`)
/// - Update on  (Viewing → Editing)   → drop the canonical view; if the current system is
///   a canonical *seed*, first author a fresh `sketchNN` instance and edit that
/// - Update off (Editing → Viewing)   → Viewing
/// - Canonical on while Editing        → Viewing (canonical is read-only)
#[derive(Clone, Copy, PartialEq)]
pub enum CanvasMode {
    Viewing,
    Editing,
}

/// The header's selectable system keys, in order_cardinality 1→12. OrderCardinality 0 is **Nullad**
/// (key `"nullad"`), prepended in the selector — the unbounded "all", which has
/// no single system to render or filter to.
const ORDER_KEYS: [&str; 12] = [
    "monad", "dyad", "triad", "tetrad", "pentad", "hexad", "heptad", "octad",
    "ennead", "decad", "undecad", "dodecad",
];

/// The index of edge `(base,target)` in canonical K_n edge order (1,2),(1,3),(2,3),…
/// Used to place a single on-graph connective edit into the flat connectives vector.
fn edge_index(order: i32, base: i32, target: i32) -> Option<usize> {
    let (a, b) = (base.min(target), base.max(target));
    let mut idx = 0usize;
    for p1 in 1..=order {
        for p2 in (p1 + 1)..=order {
            if p1 == a && p2 == b {
                return Some(idx);
            }
            idx += 1;
        }
    }
    None
}

/// The order_cardinality a header key filters/selects, or `None` for Nullad ("all").
fn order_for_key(key: &str) -> Option<i32> {
    ORDER_KEYS
        .iter()
        .position(|k| *k == key)
        .map(|i| i as i32 + 1)
}

/// The header key for a system of the given order_cardinality (inverse of `order_for_key`);
/// falls back to Nullad for out-of-range orders.
fn key_for_order(order_cardinality: i32) -> String {
    ORDER_KEYS
        .get((order_cardinality - 1) as usize)
        .map(|s| s.to_string())
        .unwrap_or_else(|| "nullad".to_string())
}

/// Detect GraphQL endpoint based on current browser location
/// - Development (localhost:8080): Points to http://localhost:8000/graphql
/// - Production (any other domain): Uses relative /graphql (same origin)
fn get_graphql_endpoint() -> String {
    use web_sys::window;

    // In WASM, access the browser's location
    if let Some(window) = window() {
        if let Ok(location) = window.location().href() {
            // If we're on Trunk dev server (port 8080), use backend port 8000
            if location.contains("localhost:8080") || location.contains("127.0.0.1:8080") {
                return "http://localhost:8000/graphql".to_string();
            }
            // Otherwise, we're deployed - use relative path (same origin)
            return "/graphql".to_string();
        }
    }

    // Fallback to relative path (production-like)
    "/graphql".to_string()
}

#[derive(Clone, Debug, PartialEq)]
pub struct Breadcrumb {
    pub system_name: String,
}

pub enum ApiAppMsg {
    SelectSystem(String),
    SystemsLoaded(Vec<RenderedSystem>),
    SystemLoaded(Box<RenderedSystem>),
    LoadError(String),
    NavigateToSystem(String),
    NavigateBack,
    ToggleEdgeLabels,
    /// Toggle showing every system's raw nodes/edges in the list (all-or-none).
    ToggleRaw,
    ReferencesLoaded(Vec<ReferenceView>),
    SetMode(ViewMode),
    /// Toggle Graph ↔ Table (the `v` hotkey).
    ToggleView,
    AllReferencesLoaded(Vec<ReferenceView>),
    InstanceSystemsLoaded(Vec<InstanceSystem>),
    LoadInstance(String),
    ToggleCanonical,
    /// Extract the current data-view selection into a Monad (Nullad → Monad).
    ExtractMonad(ExtractRequest),
    /// Join (addition) the selected systems into a new K_k (the assembly operation).
    JoinSystems(JoinRequest),
    /// Decompose a system into its faces (the inverse of Join).
    DecomposeSystem(DecomposeRequest),
    /// Result feedback from the last Extract / author.
    MonadExtracted(String),
    /// Author a new System from custom values (the in-app editor).
    AuthorSystem(AuthorRequest),
    /// Toggle Update (on-graph editing) mode.
    ToggleEditing,
    /// Commit an on-graph value edit (term/connective) → re-author the whole system
    /// with that value applied (overwrite). Also updates the list view.
    EditValue(GraphEdit),
    /// All sequences/monads loaded.
    SequencesLoaded(Vec<SequenceView>),
    /// Enter a monad/sequence (member addresses) — navigate its members by order_cardinality.
    ViewSequence(Vec<String>),
    /// Step the canvas up (-1) / down (+1) through the current scope's filtered member list
    /// (the members the list shows) — the canvas arrows while inside a monad.
    StepInScope(i32),
    /// Delete a Sequence/Monad by id (e.g. a stray Extract monad), then refresh.
    DeleteSequence(String),
    /// Delete selected rows (addresses `system:<id>` / `sequence:<id>` /
    /// `reference:<id>`), dispatching to the right mutation, then refresh.
    DeleteRows(Vec<String>),
}

pub struct ApiApp {
    systems: Vec<RenderedSystem>,
    selected_system: Option<RenderedSystem>,
    loading: bool,
    error: Option<String>,
    graphql_client: GraphQLClient,
    breadcrumbs: Vec<Breadcrumb>,
    show_edge_labels: bool,
    /// All citations within the current system, keyed by their target address —
    /// prefetched so nodes can show references as a hover tooltip.
    system_references: Vec<ReferenceView>,
    /// How the main pane represents the selection (graph canvas vs data browser).
    mode: ViewMode,
    /// The header's currently selected system key (`"nullad"`, `"monad"`, …).
    /// Drives both which graph renders and which order_cardinality the data view filters to.
    selected_key: String,
    /// Every citation in the graph (enriched), loaded lazily on first entry to
    /// the References view — powers the browser's table + compare matrix.
    all_references: Vec<ReferenceView>,
    /// Non-canonical instance systems the Load control browses (e.g. the
    /// Architecture Pentad).
    instance_systems: Vec<InstanceSystem>,
    /// When true, node/edge labels show the canonical *class* (from the loaded
    /// system's `canonicalClass`) instead of its instance values.
    show_canonical: bool,
    /// When true, the list also shows the **raw nodes/edges of *every* system** (all-or-none,
    /// not just the focused one). Off by default — see `docs`/task "Raw nodes/edges".
    show_raw: bool,
    /// Feedback from the last Extract (Nullad → Monad), shown in the data view.
    extract_note: Option<String>,
    /// The canvas interface state (Viewing / Editing) — see `CanvasMode`.
    canvas_mode: CanvasMode,
    /// Every Sequence / Monad in the graph — shown as rows in the data view.
    sequences: Vec<SequenceView>,
    /// **The navigation scope** (`None` = Nullad / whole registry; `Some` = inside a monad,
    /// its member addresses). The single owner of scope: *both* views read it — the graph
    /// (Chronos) steps its members by order, the list (Eternity) is scoped to the same members
    /// (the candidate set / Hyparxis). Survives a view toggle. See docs/v0.6-rebuild/view-model.md.
    active_sequence: Option<Vec<String>>,
}

impl ApiApp {
    /// State-model transition: enter the read-only Viewing state. Called on every
    /// navigation/load so Update never lingers on a newly-shown system.
    fn enter_viewing(&mut self) {
        self.canvas_mode = CanvasMode::Viewing;
    }
    /// The next free "sketchNN" name — a starter name for a fresh instance so the user
    /// can go straight to editing nodes/edges without naming the system first.
    fn next_sketch_name(&self) -> String {
        let existing: std::collections::HashSet<String> = self
            .systems
            .iter()
            .map(|s| s.system_name.to_lowercase())
            .chain(self.instance_systems.iter().map(|s| s.name.to_lowercase()))
            .collect();
        (1..=999)
            .map(|i| format!("sketch{i:02}"))
            .find(|n| !existing.contains(&n.to_lowercase()))
            .unwrap_or_else(|| "sketch".to_string())
    }
    /// The order_cardinality of a system by id (canonical `self.systems` or `instance_systems`).
    fn system_order(&self, id: &str) -> Option<i32> {
        self.systems
            .iter()
            .find(|s| s.system_id == id)
            .map(|s| s.order_cardinality)
            .or_else(|| self.instance_systems.iter().find(|s| s.id == id).map(|s| s.order_cardinality))
    }
    /// The sequence member (system id) whose resolved order_cardinality matches, if any.
    fn sequence_member_for_order(&self, members: &[String], order_cardinality: i32) -> Option<String> {
        members
            .iter()
            .filter_map(|m| m.strip_prefix("system:"))
            .find(|id| self.system_order(id) == Some(order_cardinality))
            .map(|id| id.to_string())
    }
    /// The distinct orders of a sequence's *system* members, in ascending order_cardinality.
    fn member_orders(&self, members: &[String]) -> Vec<i32> {
        let mut orders: Vec<i32> = members
            .iter()
            .filter_map(|m| m.strip_prefix("system:"))
            .filter_map(|id| self.system_order(id))
            .collect();
        orders.sort_unstable();
        orders.dedup();
        orders
    }
    /// The header keys reachable in the current context. `None` when not in a monad
    /// (canonical: all enabled). Inside a monad, the orders **present** in its members are
    /// enabled — one *or several* members per order alike (a bucket enables its orders too; the
    /// graph shows the first candidate at an order, the scoped list shows the rest). Nullad exits.
    fn enabled_order_keys(&self) -> Option<Vec<String>> {
        let members = self.active_sequence.as_ref()?;
        let mut orders = self.member_orders(members);
        // The **monad (order 1) is the container itself** — always reachable, so you can
        // always return to the whole-contents list even when no K1 member has been authored
        // (e.g. the Architectural Monad, whose lowest member is a dyad).
        if !orders.contains(&1) {
            orders.insert(0, 1);
        }
        Some(orders.into_iter().map(key_for_order).collect())
    }
    /// The system ids (no `system:` prefix) of the current scope's members of the **selected
    /// order** (a *strict* order match — the same-order candidates), in member order. This is
    /// what the canvas up/down arrows step through, so a lone system of an order (e.g. a
    /// sequence's single monad) shows no arrows, while three dyads do.
    fn scoped_filtered_ids(&self) -> Vec<String> {
        let Some(members) = self.active_sequence.as_ref() else {
            return Vec::new();
        };
        let want = order_for_key(&self.selected_key);
        members
            .iter()
            .filter_map(|m| m.strip_prefix("system:"))
            .filter(|id| want.is_none_or(|o| self.system_order(id) == Some(o)))
            .map(|s| s.to_string())
            .collect()
    }
}

impl Component for ApiApp {
    type Message = ApiAppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // GraphQL endpoint - auto-detected based on environment
        let graphql_endpoint = get_graphql_endpoint();
        let graphql_client = GraphQLClient::new(graphql_endpoint);

        // Load all systems on initialization
        let link = ctx.link().clone();
        let client = graphql_client.clone();

        spawn_local(async move {
            match client.fetch_all_systems().await {
                Ok(systems) => {
                    link.send_message(ApiAppMsg::SystemsLoaded(systems));
                }
                Err(e) => {
                    link.send_message(ApiAppMsg::LoadError(e.to_string()));
                }
            }
        });

        // Instance systems for the Load control (best-effort).
        let link2 = ctx.link().clone();
        let client2 = graphql_client.clone();
        spawn_local(async move {
            if let Ok(instances) = client2.fetch_instance_systems().await {
                link2.send_message(ApiAppMsg::InstanceSystemsLoaded(instances));
            }
        });

        // Global `v` hotkey → toggle the View (Graph ↔ Table), ignored while typing.
        {
            use wasm_bindgen::JsCast;
            let link_kb = ctx.link().clone();
            let closure = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(
                move |e: web_sys::KeyboardEvent| {
                    if let Some(t) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok()) {
                        let tag = t.tag_name().to_lowercase();
                        if tag == "input" || tag == "textarea" || t.is_content_editable() {
                            return;
                        }
                    }
                    if e.key().eq_ignore_ascii_case("v") {
                        link_kb.send_message(ApiAppMsg::ToggleView);
                    }
                },
            );
            if let Some(win) = web_sys::window() {
                let _ = win.add_event_listener_with_callback(
                    "keydown",
                    closure.as_ref().unchecked_ref(),
                );
            }
            closure.forget();
        }

        // Default entry point is the Nullad table (all elements), so load every
        // reference up front rather than making the user click into it.
        let link3 = ctx.link().clone();
        let client3 = graphql_client.clone();
        spawn_local(async move {
            let refs = client3.fetch_all_references().await.unwrap_or_default();
            link3.send_message(ApiAppMsg::AllReferencesLoaded(refs));
        });

        // Sequences / Monads — shown as rows in the data view.
        let link4 = ctx.link().clone();
        let client4 = graphql_client.clone();
        spawn_local(async move {
            let seqs = client4.fetch_sequences().await.unwrap_or_default();
            link4.send_message(ApiAppMsg::SequencesLoaded(seqs));
        });

        Self {
            systems: vec![],
            selected_system: None,
            loading: true,
            error: None,
            graphql_client,
            breadcrumbs: vec![],
            show_edge_labels: false,
            system_references: vec![],
            // Nullad, as a Table, is the default entry point (see all).
            mode: ViewMode::Table,
            selected_key: "nullad".to_string(),
            all_references: vec![],
            instance_systems: vec![],
            show_canonical: false,
            show_raw: false,
            extract_note: None,
            canvas_mode: CanvasMode::Viewing,
            sequences: vec![],
            active_sequence: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ApiAppMsg::SelectSystem(name) => {
                // Navigating to a system returns to the read-only Viewing state.
                self.enter_viewing();
                // The header buttons drive both views: which graph to render, and
                // which order_cardinality the data view filters to. Set the key optimistically
                // so the highlight and the data filter update immediately.
                self.selected_key = name.clone();
                self.breadcrumbs.clear();
                self.error = None;

                if name == "nullad" {
                    // Nullad = the unbounded "all", and the reset: clicking it drops
                    // any monad context and scope, back to the whole registry. No
                    // single system to render (blank canvas in graph mode), no order_cardinality
                    // filter and no member-scope in data mode.
                    self.active_sequence = None; // leave any monad context (exit to Nullad)
                    self.selected_system = None;
                    self.system_references = vec![];
                    self.loading = false;
                    return true;
                }

                // Inside a monad: the header steps its members by order (e.g. Monad(CT)→Dyad =
                // Container·Operations, not canonical). Where several members share an order (a
                // bucket), the graph shows the first candidate at that order and the scoped list
                // shows the rest. On a miss we stay in the monad (don't fall back to canonical).
                if let Some(members) = self.active_sequence.clone() {
                    if let Some(order_cardinality) = order_for_key(&name) {
                        if let Some(id) = self.sequence_member_for_order(&members, order_cardinality) {
                            // Focus this order: load its system onto the canvas (monad→K1,
                            // dyad→K2, …). Stay in the current view — in the list the header
                            // filters (monad = all, order k = its members), in the graph the
                            // canvas shows this order's system. Each tab always resolves the
                            // correct order.
                            self.loading = true;
                            let link = ctx.link().clone();
                            let client = self.graphql_client.clone();
                            spawn_local(async move {
                                match client.fetch_rendered_by_id(&id).await {
                                    Ok(system) => link.send_message(ApiAppMsg::SystemLoaded(Box::new(system))),
                                    Err(e) => link.send_message(ApiAppMsg::LoadError(e.to_string())),
                                }
                            });
                            return true;
                        }
                    }
                    // No member at this order in scope → keep the sequence, don't navigate away.
                    return true;
                }

                self.loading = true;
                // Fetch the selected system (keeps the canvas in sync with the key
                // even while browsing in data mode).
                let link = ctx.link().clone();
                let client = self.graphql_client.clone();

                spawn_local(async move {
                    match client.fetch_system(&name).await {
                        Ok(system) => {
                            link.send_message(ApiAppMsg::SystemLoaded(Box::new(system)));
                        }
                        Err(e) => {
                            link.send_message(ApiAppMsg::LoadError(e.to_string()));
                        }
                    }
                });

                true
            }
            ApiAppMsg::NavigateToSystem(name) => {
                self.enter_viewing();
                // Add current system to breadcrumbs before navigating
                if let Some(ref current) = self.selected_system {
                    self.breadcrumbs.push(Breadcrumb {
                        system_name: current.name.clone(),
                    });
                }

                self.loading = true;
                self.error = None;

                // Fetch the target system
                let link = ctx.link().clone();
                let client = self.graphql_client.clone();

                spawn_local(async move {
                    match client.fetch_system(&name).await {
                        Ok(system) => {
                            link.send_message(ApiAppMsg::SystemLoaded(Box::new(system)));
                        }
                        Err(e) => {
                            link.send_message(ApiAppMsg::LoadError(e.to_string()));
                        }
                    }
                });

                true
            }
            ApiAppMsg::NavigateBack => {
                self.enter_viewing();
                if let Some(breadcrumb) = self.breadcrumbs.pop() {
                    self.loading = true;
                    self.error = None;

                    // Fetch the previous system
                    let link = ctx.link().clone();
                    let client = self.graphql_client.clone();
                    let name = breadcrumb.system_name;

                    spawn_local(async move {
                        match client.fetch_system(&name).await {
                            Ok(system) => {
                                link.send_message(ApiAppMsg::SystemLoaded(Box::new(system)));
                            }
                            Err(e) => {
                                link.send_message(ApiAppMsg::LoadError(e.to_string()));
                            }
                        }
                    });
                }

                true
            }
            ApiAppMsg::SystemsLoaded(systems) => {
                self.loading = false;

                web_sys::console::log_1(
                    &format!("ApiApp received {} systems", systems.len()).into(),
                );
                for sys in &systems {
                    web_sys::console::log_1(
                        &format!("  - order_cardinality {} ({})", sys.order_cardinality, sys.display_name()).into(),
                    );
                }

                // Default entry point is the Nullad, so do NOT auto-select a
                // system — the header stays on Nullad until the user picks one.
                self.systems = systems;
                true
            }
            ApiAppMsg::SystemLoaded(system) => {
                self.loading = false;
                let system_id = system.system_id.clone();
                // Default view is canonical: a canonical system (no canonical_class)
                // loads with the Canonical switch ON; a custom/instance system shows
                // its own values (switch off).
                self.show_canonical = system.canonical_class.is_none();
                // Sync the header highlight to the order_cardinality now on the canvas.
                self.selected_key = key_for_order(system.order_cardinality);
                self.selected_system = Some(*system);
                // Prefetch citations for the new system (hover tooltips).
                self.system_references = vec![];
                let link = ctx.link().clone();
                let client = self.graphql_client.clone();
                spawn_local(async move {
                    let refs = client
                        .fetch_references_for_system(&system_id)
                        .await
                        .unwrap_or_default();
                    link.send_message(ApiAppMsg::ReferencesLoaded(refs));
                });
                true
            }
            ApiAppMsg::LoadError(error) => {
                self.loading = false;
                self.error = Some(error);
                true
            }
            ApiAppMsg::ToggleEdgeLabels => {
                self.show_edge_labels = !self.show_edge_labels;
                true
            }
            ApiAppMsg::ToggleRaw => {
                self.show_raw = !self.show_raw;
                true
            }
            ApiAppMsg::ReferencesLoaded(refs) => {
                self.system_references = refs;
                true
            }
            ApiAppMsg::ToggleView => {
                let target = if self.mode == ViewMode::Graph {
                    ViewMode::Table
                } else {
                    ViewMode::Graph
                };
                ctx.link().send_message(ApiAppMsg::SetMode(target));
                false
            }
            ApiAppMsg::SetMode(mode) => {
                self.mode = mode;
                // Lazily load all references the first time the data view opens.
                if mode == ViewMode::Table && self.all_references.is_empty() {
                    let link = ctx.link().clone();
                    let client = self.graphql_client.clone();
                    spawn_local(async move {
                        let refs = client.fetch_all_references().await.unwrap_or_default();
                        link.send_message(ApiAppMsg::AllReferencesLoaded(refs));
                    });
                }
                true
            }
            ApiAppMsg::AllReferencesLoaded(refs) => {
                self.all_references = refs;
                true
            }
            ApiAppMsg::InstanceSystemsLoaded(instances) => {
                self.instance_systems = instances;
                true
            }
            ApiAppMsg::LoadInstance(id) => {
                // Replace the canvas with the loaded instance system (single canvas).
                // Load is the ELT triad's edge on the data view, so switch to the graph mode
                // to reveal what was loaded. If the loaded system is a member of the monad
                // we're in, STAY in that monad (viewing a candidate as a graph — Hyparxis);
                // only a standalone system leaves the monad context.
                self.enter_viewing();
                self.mode = ViewMode::Graph;
                let in_scope = self
                    .active_sequence
                    .as_ref()
                    .is_some_and(|m| m.iter().any(|a| a == &format!("system:{id}")));
                if !in_scope {
                    self.active_sequence = None;
                }
                self.breadcrumbs.clear();
                self.loading = true;
                self.error = None;
                let link = ctx.link().clone();
                let client = self.graphql_client.clone();
                spawn_local(async move {
                    match client.fetch_rendered_by_id(&id).await {
                        Ok(system) => link.send_message(ApiAppMsg::SystemLoaded(Box::new(system))),
                        Err(e) => link.send_message(ApiAppMsg::LoadError(e.to_string())),
                    }
                });
                true
            }
            ApiAppMsg::ToggleCanonical => {
                self.show_canonical = !self.show_canonical;
                // Canonical view is read-only, so turning it on returns to Viewing.
                if self.show_canonical {
                    self.canvas_mode = CanvasMode::Viewing;
                }
                true
            }
            ApiAppMsg::ExtractMonad(req) => {
                // Extract (Nullad → Monad): materialize the current selection as a
                // persisted Sequence via GraphQL. The scope becomes a real graph
                // object rather than a transient client-side key.
                if req.members.is_empty() {
                    self.extract_note = Some("Nothing selected to extract.".to_string());
                    return true;
                }
                self.extract_note = Some(format!("Extracting {}…", req.name));
                let link = ctx.link().clone();
                let client = self.graphql_client.clone();
                let count = req.members.len();
                spawn_local(async move {
                    let note = match client.create_sequence(&req.name, req.members).await {
                        Ok(seq) => format!(
                            "Extracted Monad “{}” ({} systems) → {}",
                            seq.name, count, seq.id
                        ),
                        Err(e) => format!("Extract failed: {e}"),
                    };
                    link.send_message(ApiAppMsg::MonadExtracted(note));
                });
                true
            }
            ApiAppMsg::JoinSystems(req) => {
                // Join (addition): combine the selected systems into a new K_k. If we're
                // inside a monad (bucket), append the joined system to that monad's
                // sequence (its components) — resolve the sequence id by matching the
                // active members against the known sequences.
                if req.members.len() < 2 {
                    self.extract_note = Some("Select at least two systems to join.".to_string());
                    return true;
                }
                let sequence_ref = self.active_sequence.as_ref().and_then(|members| {
                    self.sequences
                        .iter()
                        .find(|s| &s.members == members)
                        .map(|s| s.id.clone())
                });
                self.extract_note = Some(format!("Joining {}…", req.name));
                let link = ctx.link().clone();
                let client = self.graphql_client.clone();
                spawn_local(async move {
                    match client.join_systems(&req.name, req.members, sequence_ref).await {
                        Ok(sys) => {
                            link.send_message(ApiAppMsg::MonadExtracted(format!(
                                "Joined “{}” → {} (order_cardinality {})",
                                sys.name, sys.id, sys.order_cardinality
                            )));
                            // Load the joined system + refresh the registry lists.
                            if let Ok(system) = client.fetch_rendered_by_id(&sys.id).await {
                                link.send_message(ApiAppMsg::SystemLoaded(Box::new(system)));
                            }
                            if let Ok(instances) = client.fetch_instance_systems().await {
                                link.send_message(ApiAppMsg::InstanceSystemsLoaded(instances));
                            }
                            if let Ok(seqs) = client.fetch_sequences().await {
                                link.send_message(ApiAppMsg::SequencesLoaded(seqs));
                            }
                        }
                        Err(e) => link
                            .send_message(ApiAppMsg::MonadExtracted(format!("Join failed: {e}"))),
                    }
                });
                true
            }
            ApiAppMsg::DecomposeSystem(req) => {
                // Decompose (subtraction): break the system into its faces. If inside a
                // monad (bucket), append the faces to that monad's sequence (its components).
                let sequence_ref = self.active_sequence.as_ref().and_then(|members| {
                    self.sequences
                        .iter()
                        .find(|s| &s.members == members)
                        .map(|s| s.id.clone())
                });
                self.extract_note = Some("Decomposing…".to_string());
                let link = ctx.link().clone();
                let client = self.graphql_client.clone();
                spawn_local(async move {
                    match client.decompose_system(&req.system_ref, sequence_ref).await {
                        Ok(faces) => {
                            link.send_message(ApiAppMsg::MonadExtracted(format!(
                                "Decomposed into {} face(s).",
                                faces.len()
                            )));
                            if let Ok(instances) = client.fetch_instance_systems().await {
                                link.send_message(ApiAppMsg::InstanceSystemsLoaded(instances));
                            }
                            if let Ok(seqs) = client.fetch_sequences().await {
                                link.send_message(ApiAppMsg::SequencesLoaded(seqs));
                            }
                        }
                        Err(e) => link
                            .send_message(ApiAppMsg::MonadExtracted(format!("Decompose failed: {e}"))),
                    }
                });
                true
            }
            ApiAppMsg::MonadExtracted(note) => {
                self.extract_note = Some(note);
                true
            }
            ApiAppMsg::ViewSequence(members) => {
                // Enter a sequence: scope both views to its members and **land on the list** of
                // all its contents (the container). Load the sequence's **monad (K1)** onto the
                // canvas so switching to graph shows the monad vertex (its head, named after the
                // sequence). Order tabs then focus each order (list: filter — monad = all, order
                // k = its members; graph: that order's system); Nullad exits.
                self.enter_viewing();
                self.breadcrumbs.clear();
                self.mode = ViewMode::Table;
                self.selected_key = "monad".to_string();
                self.selected_system = None;
                let monad = self.sequence_member_for_order(&members, 1);
                self.active_sequence = Some(members);
                if let Some(id) = monad {
                    self.loading = true;
                    let link = ctx.link().clone();
                    let client = self.graphql_client.clone();
                    spawn_local(async move {
                        match client.fetch_rendered_by_id(&id).await {
                            Ok(system) => link.send_message(ApiAppMsg::SystemLoaded(Box::new(system))),
                            Err(e) => link.send_message(ApiAppMsg::LoadError(e.to_string())),
                        }
                    });
                } else {
                    self.loading = false;
                }
                true
            }
            ApiAppMsg::StepInScope(delta) => {
                // Move the canvas up/down through the current scope's filtered member list
                // (wraps around). No-op unless there are at least two candidates.
                let ids = self.scoped_filtered_ids();
                if ids.len() < 2 {
                    return false;
                }
                let idx = self
                    .selected_system
                    .as_ref()
                    .and_then(|s| ids.iter().position(|i| i == &s.system_id))
                    .unwrap_or(0) as i32;
                let n = ids.len() as i32;
                let id = ids[((((idx + delta) % n) + n) % n) as usize].clone();
                self.mode = ViewMode::Graph;
                self.loading = true;
                let link = ctx.link().clone();
                let client = self.graphql_client.clone();
                spawn_local(async move {
                    match client.fetch_rendered_by_id(&id).await {
                        Ok(system) => link.send_message(ApiAppMsg::SystemLoaded(Box::new(system))),
                        Err(e) => link.send_message(ApiAppMsg::LoadError(e.to_string())),
                    }
                });
                true
            }
            ApiAppMsg::ToggleEditing => {
                // Toggle Viewing ↔ Editing. Entering Editing drops the read-only canonical
                // view; a canonical seed becomes a blank editable template. The instance is
                // created **lazily**, on the first data input (a name or a node/edge value)
                // — see the `is_blank` branch of `EditValue`. So toggling Update and backing
                // out leaves no stray system.
                let entering = self.canvas_mode == CanvasMode::Viewing;
                self.canvas_mode = if entering { CanvasMode::Editing } else { CanvasMode::Viewing };
                if entering {
                    self.show_canonical = false;
                }
                true
            }
            ApiAppMsg::SequencesLoaded(seqs) => {
                self.sequences = seqs;
                true
            }
            ApiAppMsg::DeleteSequence(id) => {
                // If the deleted monad is the one we're inside, leave its context.
                self.active_sequence = None;
                self.extract_note = Some("Deleting monad…".to_string());
                let link = ctx.link().clone();
                let client = self.graphql_client.clone();
                spawn_local(async move {
                    let note = match client.delete_sequence(&id).await {
                        Ok(true) => format!("Deleted monad {id}."),
                        Ok(false) => format!("No monad {id} to delete."),
                        Err(e) => format!("Delete failed: {e}"),
                    };
                    // Refresh the sequence list so the row disappears.
                    if let Ok(seqs) = client.fetch_sequences().await {
                        link.send_message(ApiAppMsg::SequencesLoaded(seqs));
                    }
                    link.send_message(ApiAppMsg::MonadExtracted(note));
                });
                true
            }
            ApiAppMsg::DeleteRows(addrs) => {
                // Delete each selected row by its address prefix, then refresh all
                // three collections so the table reflects the CRUD.
                self.active_sequence = None;
                // If the focused system was just deleted, drop it — otherwise its
                // nodes/edges (derived from `selected_system`) linger as list rows.
                if let Some(sys) = &self.selected_system {
                    if addrs.iter().any(|a| a == &format!("system:{}", sys.system_id)) {
                        self.selected_system = None;
                        self.breadcrumbs.clear();
                    }
                }
                self.extract_note = Some(format!("Deleting {} item(s)…", addrs.len()));
                let link = ctx.link().clone();
                let client = self.graphql_client.clone();
                spawn_local(async move {
                    let mut ok = 0usize;
                    let mut failed = 0usize;
                    for addr in &addrs {
                        let res = if let Some(id) = addr.strip_prefix("system:") {
                            client.delete_system(id).await
                        } else if let Some(id) = addr.strip_prefix("sequence:") {
                            client.delete_sequence(id).await
                        } else if let Some(id) = addr.strip_prefix("reference:") {
                            client.delete_reference(id).await
                        } else {
                            Ok(false)
                        };
                        match res {
                            Ok(true) => ok += 1,
                            _ => failed += 1,
                        }
                    }
                    // Refresh systems, references, and sequences.
                    if let Ok(instances) = client.fetch_instance_systems().await {
                        link.send_message(ApiAppMsg::InstanceSystemsLoaded(instances));
                    }
                    if let Ok(refs) = client.fetch_all_references().await {
                        link.send_message(ApiAppMsg::AllReferencesLoaded(refs));
                    }
                    if let Ok(seqs) = client.fetch_sequences().await {
                        link.send_message(ApiAppMsg::SequencesLoaded(seqs));
                    }
                    let note = if failed == 0 {
                        format!("Deleted {ok} item(s).")
                    } else {
                        format!("Deleted {ok}, {failed} failed/absent.")
                    };
                    link.send_message(ApiAppMsg::MonadExtracted(note));
                });
                true
            }
            ApiAppMsg::AuthorSystem(req) => {
                self.extract_note = Some(format!("Authoring “{}”…", req.name));
                let link = ctx.link().clone();
                let client = self.graphql_client.clone();
                spawn_local(async move {
                    match client
                        .author_system(&req.name, req.order_cardinality, req.terms, req.connectives)
                        .await
                    {
                        Ok(sys) => {
                            link.send_message(ApiAppMsg::MonadExtracted(format!(
                                "Authored system “{}” → {} (order_cardinality {})",
                                sys.name, sys.id, sys.order_cardinality
                            )));
                            // Load the authored system so the graph updates to it.
                            match client.fetch_rendered_by_id(&sys.id).await {
                                Ok(system) => link.send_message(ApiAppMsg::SystemLoaded(Box::new(system))),
                                Err(e) => link.send_message(ApiAppMsg::LoadError(e.to_string())),
                            }
                            // Refetch so the new system appears in the Nullad table.
                            if let Ok(instances) = client.fetch_instance_systems().await {
                                link.send_message(ApiAppMsg::InstanceSystemsLoaded(instances));
                            }
                        }
                        Err(e) => link
                            .send_message(ApiAppMsg::MonadExtracted(format!("Author failed: {e}"))),
                    }
                });
                true
            }
            ApiAppMsg::EditValue(edit) => {
                // On-graph edit: take the loaded system's current values, apply the
                // single change, and re-author the WHOLE system (overwrite). The name
                // comes from `system_name` so the edit targets the same id (no fork).
                let Some(system) = self.selected_system.as_ref() else {
                    return false;
                };

                // Blank mode (a canonical seed with Canonical switched off): the graph is a
                // blank NEW instance created **lazily** on the first data input. A name →
                // that name; a node/edge value → an auto "sketchNN" name with that value
                // applied. The seed is never touched; then it's a normal editable system.
                let is_blank = system.canonical_class.is_none() && !self.show_canonical;
                if is_blank {
                    let order = system.order_cardinality;
                    let n = order.max(0) as usize;
                    let conn_count = n * n.saturating_sub(1) / 2;
                    let mut terms = vec![String::new(); n];
                    let mut connectives = vec![String::new(); conn_count];
                    let name = match &edit {
                        GraphEdit::Name { value } => value.clone(),
                        GraphEdit::Term { ordinality, value } => {
                            let i = (*ordinality as usize).saturating_sub(1);
                            if i < terms.len() {
                                terms[i] = value.clone();
                            }
                            self.next_sketch_name()
                        }
                        GraphEdit::Connective { base, target, value } => {
                            if let Some(i) = edge_index(order, *base, *target) {
                                if i < connectives.len() {
                                    connectives[i] = value.clone();
                                }
                            }
                            self.next_sketch_name()
                        }
                    };
                    let link = ctx.link().clone();
                    let client = self.graphql_client.clone();
                    spawn_local(async move {
                        match client.author_system(&name, order, terms, connectives).await {
                            Ok(sys) => {
                                if let Ok(system) = client.fetch_rendered_by_id(&sys.id).await {
                                    link.send_message(ApiAppMsg::SystemLoaded(Box::new(system)));
                                }
                                if let Ok(instances) = client.fetch_instance_systems().await {
                                    link.send_message(ApiAppMsg::InstanceSystemsLoaded(instances));
                                }
                            }
                            Err(e) => link
                                .send_message(ApiAppMsg::MonadExtracted(format!("Create failed: {e}"))),
                        }
                    });
                    return false;
                }

                let mut terms: Vec<(i32, String)> = system
                    .terms
                    .iter()
                    .map(|t| (t.ordinality, t.value.clone()))
                    .collect();
                terms.sort_by_key(|(o, _)| *o);
                let mut conns: Vec<(i32, i32, String)> = system
                    .connectives
                    .iter()
                    .map(|c| (c.base_ordinality, c.target_ordinality, c.character_value.clone()))
                    .collect();
                conns.sort_by_key(|(b, t, _)| (*b, *t));

                let mut new_name = if system.system_name.is_empty() {
                    system.name.clone()
                } else {
                    system.system_name.clone()
                };

                match &edit {
                    GraphEdit::Term { ordinality, value } => {
                        for slot in terms.iter_mut() {
                            if slot.0 == *ordinality {
                                slot.1 = value.clone();
                                break;
                            }
                        }
                    }
                    GraphEdit::Connective { base, target, value } => {
                        let (lo, hi) = ((*base).min(*target), (*base).max(*target));
                        for slot in conns.iter_mut() {
                            if slot.0.min(slot.1) == lo && slot.0.max(slot.1) == hi {
                                slot.2 = value.clone();
                                break;
                            }
                        }
                    }
                    GraphEdit::Name { value } => {
                        new_name = value.clone();
                    }
                }

                let order = system.order_cardinality;
                let terms: Vec<String> = terms.into_iter().map(|(_, v)| v).collect();
                let connectives: Vec<String> = conns.into_iter().map(|(_, _, v)| v).collect();

                // Instance systems edit **in place, keeping their id** (id-stable rename/edit),
                // so any sequence that references them stays valid — a system's id is derived
                // from its name, so re-authoring under a new name would fork a new id and orphan
                // the sequence. A CANONICAL seed instead **forks** a custom copy (author under
                // the new name; the seed is never touched).
                let old_id = system.system_id.clone();
                let is_instance = system.canonical_class.is_some();
                let link = ctx.link().clone();
                let client = self.graphql_client.clone();
                spawn_local(async move {
                    let result = if is_instance {
                        client.edit_system(&old_id, &new_name, order, terms, connectives).await
                    } else {
                        client.author_system(&new_name, order, terms, connectives).await
                    };
                    match result {
                        Ok(sys) => {
                            if let Ok(system) = client.fetch_rendered_by_id(&sys.id).await {
                                link.send_message(ApiAppMsg::SystemLoaded(Box::new(system)));
                            }
                            if let Ok(instances) = client.fetch_instance_systems().await {
                                link.send_message(ApiAppMsg::InstanceSystemsLoaded(instances));
                            }
                        }
                        Err(e) => {
                            link.send_message(ApiAppMsg::MonadExtracted(format!("Update failed: {e}")))
                        }
                    }
                });
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_select = ctx.link().callback(ApiAppMsg::SelectSystem);
        let on_navigate = ctx.link().callback(ApiAppMsg::NavigateToSystem);
        let on_back = ctx.link().callback(|_| ApiAppMsg::NavigateBack);
        let on_toggle_edge_labels = ctx.link().callback(|_| ApiAppMsg::ToggleEdgeLabels);
        let on_load = ctx.link().callback(ApiAppMsg::LoadInstance);
        let on_toggle_canonical = ctx.link().callback(|_| ApiAppMsg::ToggleCanonical);
        let on_extract = ctx.link().callback(ApiAppMsg::ExtractMonad);
        let on_author = ctx.link().callback(ApiAppMsg::AuthorSystem);
        let on_view_sequence = ctx.link().callback(ApiAppMsg::ViewSequence);
        let on_delete_sequence = ctx.link().callback(ApiAppMsg::DeleteSequence);
        let on_delete_rows = ctx.link().callback(ApiAppMsg::DeleteRows);
        let on_join = ctx.link().callback(ApiAppMsg::JoinSystems);
        let on_decompose = ctx.link().callback(ApiAppMsg::DecomposeSystem);
        let on_edit_value = ctx.link().callback(ApiAppMsg::EditValue);
        // Canonical term/connective values per order_cardinality — the editor's prefill source.
        let templates: Vec<SystemTemplate> = self
            .systems
            .iter()
            .map(|s| SystemTemplate {
                order_cardinality: s.order_cardinality,
                terms: s.terms.iter().map(|t| t.value.clone()).collect(),
                connectives: s.connectives.iter().map(|c| c.character_value.clone()).collect(),
            })
            .collect();
        // ALL systems as table rows: canonical (from the loaded set) + instance
        // (non-canonical). Both clickable to view.
        let mut all_systems: Vec<InstanceSystem> = self
            .systems
            .iter()
            .map(|s| InstanceSystem {
                id: s.system_id.clone(),
                name: s.name.clone(),
                order_cardinality: s.order_cardinality,
                // Canonical systems carry graph positions: term.ordinality (node) and
                // connective base–target (edge) — the same source the graph view uses.
                terms: s
                    .terms
                    .iter()
                    .map(|t| PositionedChar { value: t.value.clone(), ordinality: t.ordinality.to_string() })
                    .collect(),
                connectives: s
                    .connectives
                    .iter()
                    .map(|c| PositionedChar {
                        value: c.character_value.clone(),
                        ordinality: format!("{}-{}", c.base_ordinality, c.target_ordinality),
                    })
                    .collect(),
            })
            .collect();
        all_systems.extend(self.instance_systems.iter().cloned());
        // Raw nodes/edges as data rows — **all systems or none** (the `show_raw` toggle),
        // rather than only the focused system's (which was confusing). Off by default.
        let raw_elements: Vec<RawElement> = if self.show_raw {
            all_systems
                .iter()
                .flat_map(|s| {
                    let o = s.order_cardinality;
                    s.terms
                        .iter()
                        .map(move |t| RawElement { name: t.value.clone(), order_cardinality: o, is_edge: false })
                        .chain(s.connectives.iter().map(move |c| RawElement {
                            name: c.value.clone(),
                            order_cardinality: o,
                            is_edge: true,
                        }))
                })
                .collect()
        } else {
            Vec::new()
        };

        // Data · Graph · Table: the Data (content the header scopes) has two views;
        // this switch chooses Graph or Table.
        let on_set_mode = ctx.link().callback(ApiAppMsg::SetMode);
        let on_toggle_editing = ctx.link().callback(|_| ApiAppMsg::ToggleEditing);
        // List filter (Eternity over the scope): inside a monad the **head/monad** selection
        // shows the whole container (all members); a higher order (dyad, triad, …) filters the
        // scoped list to just that order — e.g. Blue Earth + dyad = its three dyads. `scope_ids`
        // restricts rows to the monad's members either way. At the Nullad the selected order
        // filters globally by type.
        let filter_order = if self.active_sequence.is_some() {
            match order_for_key(&self.selected_key) {
                Some(1) | None => None, // monad head (or Nullad) → the whole container
                some => some,           // dyad/triad/… → that order within the scope
            }
        } else {
            order_for_key(&self.selected_key)
        };
        // Canvas up/down arrows: step the current scope's filtered member list. Shown only
        // inside a monad when the filter has more than one candidate to move between.
        let on_step_up = ctx.link().callback(|_| ApiAppMsg::StepInScope(-1));
        let on_step_down = ctx.link().callback(|_| ApiAppMsg::StepInScope(1));
        let show_canvas_nav = self.active_sequence.is_some() && self.scoped_filtered_ids().len() > 1;

        html! {
            <div class="app">
                <div class="app-content">
                    <aside class="sidebar">
                        {
                            if self.loading && self.systems.is_empty() {
                                html! { <div class="loading">{"Loading systems..."}</div> }
                            } else {
                                // Convert RenderedSystem to SystemDisplay for SystemSelector
                                let display_systems: Vec<SystemDisplay> = self.systems.iter().map(|sys| {
                                    SystemDisplay {
                                        name: sys.name.to_lowercase(),
                                        display_name: sys.display_name(),
                                        k_notation: sys.k_notation(),
                                    }
                                }).collect();

                                html! {
                                    <SystemSelector
                                        systems={ display_systems }
                                        selected={ self.selected_key.clone() }
                                        enabled={ self.enabled_order_keys() }
                                        on_select={ on_select }
                                        mode={ self.mode }
                                        on_set_mode={ on_set_mode }
                                    />
                                }
                            }
                        }
                    </aside>

                    <main class="main-view">
                        if self.mode == ViewMode::Table {
                            <ReferenceBrowser
                                references={ self.all_references.clone() }
                                instance_systems={ all_systems }
                                on_load={ on_load.clone() }
                                filter_order={ filter_order }
                                on_extract={ on_extract }
                                extract_note={ self.extract_note.clone() }
                                on_author={ on_author }
                                templates={ templates }
                                raw_elements={ raw_elements }
                                sequences={ self.sequences.clone() }
                                on_view_sequence={ on_view_sequence }
                                on_delete_sequence={ on_delete_sequence }
                                on_delete_rows={ on_delete_rows }
                                on_join={ on_join }
                                on_decompose={ on_decompose }
                                scope_ids={ self.active_sequence.clone() }
                                show_raw={ self.show_raw }
                                on_toggle_raw={ ctx.link().callback(|_| ApiAppMsg::ToggleRaw) }
                            />
                        } else if self.selected_key == "nullad" && self.active_sequence.is_none() {
                            // Nullad in graph mode: a blank canvas standing in for
                            // the future all-and-everything undirected graph. (Only when NOT
                            // in a monad — in a monad the graph shows the current member.)
                            <div class="nullad-blank">
                                <p class="nullad-blank-title">{ "Nullad — all & everything" }</p>
                                <p class="nullad-blank-hint">
                                    { "An undirected graph of everything will live here. Blank for now." }
                                </p>
                            </div>
                        } else {
                        // Breadcrumb trail
                        if !self.breadcrumbs.is_empty() {
                            <nav class="breadcrumbs">
                                { for self.breadcrumbs.iter().map(|crumb| {
                                    html! {
                                        <span class="breadcrumb">
                                            { &crumb.system_name }
                                            { " > " }
                                        </span>
                                    }
                                })}
                                if let Some(ref system) = self.selected_system {
                                    <span class="breadcrumb-current">
                                        { system.display_name() }
                                    </span>
                                }
                                <button class="breadcrumb-back" onclick={ on_back }>
                                    { "← Back" }
                                </button>
                            </nav>
                        }

                        {
                            if let Some(ref error) = self.error {
                                html! {
                                    <div class="error">
                                        <h2>{"Error"}</h2>
                                        <p>{ error }</p>
                                    </div>
                                }
                            } else if self.loading {
                                html! { <div class="loading">{"Loading system..."}</div> }
                            } else if let Some(ref system) = self.selected_system {
                                html! {
                                    <div class="graph-with-editor">
                                        if show_canvas_nav {
                                            <button class="canvas-nav canvas-nav-up"
                                                title="Previous in list"
                                                onclick={ on_step_up.clone() }>{ "▲" }</button>
                                        }
                                        <ApiGraphView
                                            system={ system.clone() }
                                            on_navigate={ Some(on_navigate) }
                                            show_edge_labels={ self.show_edge_labels }
                                            on_toggle_edge_labels={ Some(on_toggle_edge_labels.clone()) }
                                            references={ self.system_references.clone() }
                                            show_canonical={ self.show_canonical }
                                            on_toggle_canonical={ Some(on_toggle_canonical.clone()) }
                                            editing={ self.canvas_mode == CanvasMode::Editing }
                                            on_toggle_editing={ Some(on_toggle_editing.clone()) }
                                            on_edit_value={ Some(on_edit_value) }
                                            name_hint={ self.next_sketch_name() }
                                        />
                                        if show_canvas_nav {
                                            <button class="canvas-nav canvas-nav-down"
                                                title="Next in list"
                                                onclick={ on_step_down.clone() }>{ "▼" }</button>
                                        }
                                    </div>
                                }
                            } else {
                                html! { <div class="loading">{"Select a system"}</div> }
                            }
                        }
                        } // end Graph view
                    </main>
                </div>
            </div>
        }
    }
}

