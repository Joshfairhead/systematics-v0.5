use std::sync::Arc;

/// Self-contained GraphiQL page, pinned to a compatible GraphiQL 2.x + React 17
/// (async-graphql's bundled `GraphiQLSource` loads an *unpinned* graphiql that
/// now resolves to v3+, which is incompatible with the React 17 UMD globals it
/// also loads — the page then spins forever). Pinning fixes that.
const GRAPHIQL_HTML: &str = r#"<!DOCTYPE html>
<html>
  <head>
    <title>Systematics GraphQL</title>
    <style>body { height: 100%; margin: 0; width: 100%; overflow: hidden; } #graphiql { height: 100vh; }</style>
    <link rel="stylesheet" href="https://unpkg.com/graphiql@2.4.7/graphiql.min.css" />
  </head>
  <body>
    <div id="graphiql">Loading GraphiQL…</div>
    <script crossorigin src="https://unpkg.com/react@17.0.2/umd/react.production.min.js"></script>
    <script crossorigin src="https://unpkg.com/react-dom@17.0.2/umd/react-dom.production.min.js"></script>
    <script crossorigin src="https://unpkg.com/graphiql@2.4.7/graphiql.min.js"></script>
    <script>
      const fetcher = GraphiQL.createFetcher({ url: '/graphql' });
      const defaultQuery = `# Systematics GraphQL. Press the ▶ button to run.
{
  systemByName(name: "triad") {
    name
    coherence
    termDesignation
    connectiveDesignation
    terms { position value }
    connectives { basePosition targetPosition characterValue }
  }
}`;
      ReactDOM.render(
        React.createElement(GraphiQL, { fetcher, defaultQuery, defaultEditorToolsVisibility: true }),
        document.getElementById('graphiql'),
      );
    </script>
  </body>
</html>"#;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use systematics_backend::{create_schema_with_store, data, persistence};
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[cfg(not(feature = "shuttle"))]
use std::net::SocketAddr;

use tower_http::services::{ServeDir, ServeFile};

async fn graphql_handler(
    State(schema): State<systematics_backend::SystematicsSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(GRAPHIQL_HTML)
}

/// Initialize tracing subscriber
fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "systematics_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

/// Build the GraphQL API router (shared between local and Shuttle).
///
/// The graph is seeded once at startup and shared behind an `Arc<RwLock<_>>`
/// so mutations (Functor CRUD, applyFunctor) persist across requests within
/// a single process. Restarting the process resets to seed state.
fn build_api_router() -> Router {
    // Build the canonical graph, then merge the writable user store over it.
    let mut graph = data::build_graph();
    let store_path = persistence::resolve_store_path();
    if let Err(e) = persistence::load_into(&mut graph, &store_path) {
        tracing::error!("failed to load user store from {}: {}", store_path.display(), e);
    }
    tracing::info!("User store: {}", store_path.display());

    let shared_graph = Arc::new(RwLock::new(graph));
    let schema = create_schema_with_store(shared_graph, Some(store_path));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .layer(cors)
        .with_state(schema)
}

// Local development runtime (tokio)
#[cfg(not(feature = "shuttle"))]
#[tokio::main]
async fn main() {
    init_tracing();

    // Build API routes
    let api_router = build_api_router();

    // Serve static files from frontend/dist
    // Fallback to index.html for SPA routing
    let static_files = ServeDir::new("frontend/dist")
        .not_found_service(ServeFile::new("frontend/dist/index.html"));

    // Combine routes: API takes precedence, then static files
    let app = Router::new()
        .nest("/", api_router)
        .fallback_service(static_files);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(8000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("GraphQL API configured at /graphql");
    tracing::info!("Static files served from frontend/dist");
    tracing::info!("Server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Production deployment runtime (Shuttle)
#[cfg(feature = "shuttle")]
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    init_tracing();

    // Build API routes
    let api_router = build_api_router();

    // Serve static files from frontend/dist
    // Fallback to index.html for SPA routing
    let static_files = ServeDir::new("frontend/dist")
        .not_found_service(ServeFile::new("frontend/dist/index.html"));

    // Combine routes: API takes precedence, then static files
    let app = Router::new()
        .nest("/", api_router)
        .fallback_service(static_files);

    tracing::info!("GraphQL API configured at /graphql");
    tracing::info!("Static files served from frontend/dist");

    Ok(app.into())
}
