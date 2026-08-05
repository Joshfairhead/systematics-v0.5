use crate::api::client::{GraphQLClient, InstanceSystem, ReferenceView};
use crate::components::graph_view::ApiGraphView;
use crate::components::reference_browser::{
    AuthorRequest, ExtractRequest, ReferenceBrowser, SystemTemplate,
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

/// The header's selectable system keys, in order 1→12. Order 0 is **Nullad**
/// (key `"nullad"`), prepended in the selector — the unbounded "all", which has
/// no single system to render or filter to.
const ORDER_KEYS: [&str; 12] = [
    "monad", "dyad", "triad", "tetrad", "pentad", "hexad", "heptad", "octad",
    "ennead", "decad", "undecad", "dodecad",
];

/// The order a header key filters/selects, or `None` for Nullad ("all").
fn order_for_key(key: &str) -> Option<i32> {
    ORDER_KEYS
        .iter()
        .position(|k| *k == key)
        .map(|i| i as i32 + 1)
}

/// The header key for a system of the given order (inverse of `order_for_key`);
/// falls back to Nullad for out-of-range orders.
fn key_for_order(order: i32) -> String {
    ORDER_KEYS
        .get((order - 1) as usize)
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
    ReferencesLoaded(Vec<ReferenceView>),
    SetMode(ViewMode),
    AllReferencesLoaded(Vec<ReferenceView>),
    InstanceSystemsLoaded(Vec<InstanceSystem>),
    LoadInstance(String),
    ToggleCanonical,
    /// Extract the current data-view selection into a Monad (Nullad → Monad).
    ExtractMonad(ExtractRequest),
    /// Result feedback from the last Extract / author.
    MonadExtracted(String),
    /// Author a new System from custom values (the in-app editor).
    AuthorSystem(AuthorRequest),
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
    /// Drives both which graph renders and which order the data view filters to.
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
    /// Feedback from the last Extract (Nullad → Monad), shown in the data view.
    extract_note: Option<String>,
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

        // Default entry point is the Nullad table (all elements), so load every
        // reference up front rather than making the user click into it.
        let link3 = ctx.link().clone();
        let client3 = graphql_client.clone();
        spawn_local(async move {
            let refs = client3.fetch_all_references().await.unwrap_or_default();
            link3.send_message(ApiAppMsg::AllReferencesLoaded(refs));
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
            extract_note: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ApiAppMsg::SelectSystem(name) => {
                // The header buttons drive both views: which graph to render, and
                // which order the data view filters to. Set the key optimistically
                // so the highlight and the data filter update immediately.
                self.selected_key = name.clone();
                self.breadcrumbs.clear();
                self.error = None;

                if name == "nullad" {
                    // Nullad = the unbounded "all". No single system to render:
                    // a blank canvas in graph mode (future: an all-and-everything
                    // undirected graph), no order filter in data mode.
                    self.selected_system = None;
                    self.system_references = vec![];
                    self.loading = false;
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
                        &format!("  - order {} ({})", sys.order, sys.display_name()).into(),
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
                // Sync the header highlight to the order now on the canvas.
                self.selected_key = key_for_order(system.order);
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
            ApiAppMsg::ReferencesLoaded(refs) => {
                self.system_references = refs;
                true
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
                // Load is the ELT triad's edge on the data view, so switch to the
                // graph mode to reveal what was loaded.
                self.mode = ViewMode::Graph;
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
            ApiAppMsg::MonadExtracted(note) => {
                self.extract_note = Some(note);
                true
            }
            ApiAppMsg::AuthorSystem(req) => {
                self.extract_note = Some(format!("Authoring “{}”…", req.name));
                let link = ctx.link().clone();
                let client = self.graphql_client.clone();
                spawn_local(async move {
                    match client
                        .author_system(&req.name, req.order, req.terms, req.connectives)
                        .await
                    {
                        Ok(sys) => {
                            link.send_message(ApiAppMsg::MonadExtracted(format!(
                                "Authored system “{}” → {} (order {})",
                                sys.name, sys.id, sys.order
                            )));
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
        // Canonical term/connective values per order — the editor's prefill source.
        let templates: Vec<SystemTemplate> = self
            .systems
            .iter()
            .map(|s| SystemTemplate {
                order: s.order,
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
                order: s.order,
            })
            .collect();
        all_systems.extend(self.instance_systems.iter().cloned());

        // Data · Graph · Table: the Data (content the header scopes) has two views;
        // this switch chooses Graph or Table.
        let on_set_mode = ctx.link().callback(ApiAppMsg::SetMode);
        // The selected header key scopes the data view (None = Nullad = all).
        let filter_order = order_for_key(&self.selected_key);

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
                            />
                        } else if self.selected_key == "nullad" {
                            // Nullad in graph mode: a blank canvas standing in for
                            // the future all-and-everything undirected graph.
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
                                    <ApiGraphView
                                        system={ system.clone() }
                                        on_navigate={ Some(on_navigate) }
                                        show_edge_labels={ self.show_edge_labels }
                                        on_toggle_edge_labels={ Some(on_toggle_edge_labels.clone()) }
                                        references={ self.system_references.clone() }
                                        show_canonical={ self.show_canonical }
                                        on_toggle_canonical={ Some(on_toggle_canonical.clone()) }
                                    />
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

