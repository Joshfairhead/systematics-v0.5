use crate::api::client::{GraphQLClient, ReferenceView};
use crate::components::graph_view::ApiGraphView;
use crate::components::reference_browser::ReferenceBrowser;
use crate::components::system_selector::{SystemDisplay, SystemSelector};
use systematics_middleware::RenderedSystem;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

/// Which top-level view the main pane shows.
#[derive(Clone, Copy, PartialEq)]
pub enum AppView {
    Graph,
    References,
}

/// Whether the (big-table) reference browser is reachable from the UI. Hidden
/// while the perspective/system/sequence selector redesign is pending; the
/// component and its wiring are retained as a detailed perspective browser.
const SHOW_REFERENCES_VIEW: bool = false;

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
    SetView(AppView),
    AllReferencesLoaded(Vec<ReferenceView>),
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
    /// Which top-level view the main pane shows (graph canvas vs reference browser).
    view: AppView,
    /// Every citation in the graph (enriched), loaded lazily on first entry to
    /// the References view — powers the browser's table + compare matrix.
    all_references: Vec<ReferenceView>,
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

        Self {
            systems: vec![],
            selected_system: None,
            loading: true,
            error: None,
            graphql_client,
            breadcrumbs: vec![],
            show_edge_labels: false,
            system_references: vec![],
            view: AppView::Graph,
            all_references: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ApiAppMsg::SelectSystem(name) => {
                // Clear breadcrumbs when manually selecting from sidebar
                self.breadcrumbs.clear();
                self.loading = true;
                self.error = None;

                // Fetch the selected system
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

                // Select the first system by default + prefetch its citations.
                if let Some(first_system) = systems.first() {
                    self.selected_system = Some(first_system.clone());
                    let system_id = first_system.system_id.clone();
                    let link = ctx.link().clone();
                    let client = self.graphql_client.clone();
                    spawn_local(async move {
                        let refs = client
                            .fetch_references_for_system(&system_id)
                            .await
                            .unwrap_or_default();
                        link.send_message(ApiAppMsg::ReferencesLoaded(refs));
                    });
                }

                self.systems = systems;
                true
            }
            ApiAppMsg::SystemLoaded(system) => {
                self.loading = false;
                let system_id = system.system_id.clone();
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
            ApiAppMsg::SetView(view) => {
                self.view = view;
                // Lazily load all references the first time the browser opens.
                if view == AppView::References && self.all_references.is_empty() {
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
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_select = ctx.link().callback(ApiAppMsg::SelectSystem);
        let on_navigate = ctx.link().callback(ApiAppMsg::NavigateToSystem);
        let on_back = ctx.link().callback(|_| ApiAppMsg::NavigateBack);
        let on_toggle_edge_labels = ctx.link().callback(|_| ApiAppMsg::ToggleEdgeLabels);
        let show_graph = ctx.link().callback(|_| ApiAppMsg::SetView(AppView::Graph));
        let show_refs = ctx.link().callback(|_| ApiAppMsg::SetView(AppView::References));

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

                                let selected_name = self.selected_system
                                    .as_ref()
                                    .map(|s| s.name.to_lowercase())
                                    .unwrap_or_else(|| "monad".to_string());

                                html! {
                                    <SystemSelector
                                        systems={ display_systems }
                                        selected={ selected_name }
                                        on_select={ on_select }
                                    />
                                }
                            }
                        }
                    </aside>

                    <main class="main-view">
                        // Top-level view switch (Graph ⇄ References). Hidden for
                        // now: the big-table reference browser is kept as code (a
                        // detailed perspective browser) but out of view, pending
                        // the perspective/system/sequence selector redesign. Flip
                        // SHOW_REFERENCES_VIEW to re-expose it.
                        if SHOW_REFERENCES_VIEW {
                            <div class="view-switch">
                                <button
                                    class={ if self.view == AppView::Graph { "view-switch-btn active" } else { "view-switch-btn" } }
                                    onclick={ show_graph }
                                >{ "Graph" }</button>
                                <button
                                    class={ if self.view == AppView::References { "view-switch-btn active" } else { "view-switch-btn" } }
                                    onclick={ show_refs }
                                >{ "References" }</button>
                            </div>
                        }

                        if SHOW_REFERENCES_VIEW && self.view == AppView::References {
                            <ReferenceBrowser references={ self.all_references.clone() } />
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

