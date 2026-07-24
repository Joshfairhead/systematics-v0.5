//! End-to-end test for the Load primitive (E/L/T's receptive impulse).
//!
//! Closes the Extract → Load loop through the schema: `exportPerspective`
//! serialises a module bundle; `loadPerspective` merges it into a fresh graph
//! and reports any unresolved manifest dependencies. Uses DU3, which owns no
//! systems and depends on the canonical systems purely by address — so a bundle
//! loaded into a graph that has the canonical seed resolves cleanly, and its
//! manifest is reported when those homes are absent.

use std::sync::Arc;

use systematics_backend::{create_schema, data};
use tokio::sync::RwLock;

fn make_schema() -> systematics_backend::SystematicsSchema {
    let shared = Arc::new(RwLock::new({
        let mut g = data::build_graph();
        let n = data::load_perspective_modules(&mut g);
        if n > 0 {
            g.mark_bundled();
        }
        g
    }));
    create_schema(shared)
}

/// A seed-only schema (canonical + citation, no perspective modules) — the
/// "fresh graph" a bundle gets loaded into.
fn seed_only_schema() -> systematics_backend::SystematicsSchema {
    create_schema(Arc::new(RwLock::new(data::build_graph())))
}

#[tokio::test]
async fn export_then_load_round_trips_through_the_schema() {
    // 1) Export DU3 from a fully assembled graph.
    let full = make_schema();
    let resp = full
        .execute(r#"{ exportPerspective(id: "perspective_dramatic_universe_vol_3") }"#)
        .await;
    assert!(resp.errors.is_empty(), "export errors: {:?}", resp.errors);
    let data = resp.data.into_json().unwrap();
    let bundle = data["exportPerspective"].as_str().unwrap().to_string();

    // 2) Load it into a fresh seed-only graph (which lacks DU3 but has the
    //    canonical systems DU3 cites).
    let fresh = seed_only_schema();
    let req = async_graphql::Request::new(
        r#"mutation Load($b: String!) { loadPerspective(bundle: $b) { loaded unresolved } }"#,
    )
    .variables(async_graphql::Variables::from_json(
        serde_json::json!({ "b": bundle }),
    ));
    let resp = fresh.execute(req).await;
    assert!(resp.errors.is_empty(), "load errors: {:?}", resp.errors);
    let data = resp.data.into_json().unwrap();

    let loaded = data["loadPerspective"]["loaded"].as_array().unwrap();
    assert!(
        loaded.iter().any(|v| v == "perspective_dramatic_universe_vol_3"),
        "DU3 must be reported as loaded, got {loaded:?}"
    );
    // The canonical systems DU3 depends on are present in the seed → nothing
    // dangling.
    let unresolved = data["loadPerspective"]["unresolved"].as_array().unwrap();
    assert!(
        unresolved.is_empty(),
        "canonical deps are seeded, so nothing should dangle; got {unresolved:?}"
    );

    // 3) The loaded perspective is now queryable, and its references resolve.
    let resp = fresh
        .execute(r#"{ referencesForSystem(systemId: "system_canonical_tetrad_4") { id } }"#)
        .await;
    let data = resp.data.into_json().unwrap();
    assert!(
        !data["referencesForSystem"].as_array().unwrap().is_empty(),
        "DU3's citations onto the canonical tetrad must resolve after load"
    );
}

#[tokio::test]
async fn load_reports_dangling_manifest_dependencies() {
    // A hand-built bundle: a perspective plus a manifest naming a system that
    // does not exist in the target graph. Load merges it and warns.
    let bundle = serde_json::json!({
        "perspectives": [{ "id": "perspective_orphan", "name": "Orphan", "links": [] }],
        "manifest": ["system:system_does_not_exist_9"]
    })
    .to_string();

    let fresh = seed_only_schema();
    let req = async_graphql::Request::new(
        r#"mutation Load($b: String!) { loadPerspective(bundle: $b) { loaded unresolved } }"#,
    )
    .variables(async_graphql::Variables::from_json(
        serde_json::json!({ "b": bundle }),
    ));
    let resp = fresh.execute(req).await;
    assert!(resp.errors.is_empty(), "load errors: {:?}", resp.errors);
    let data = resp.data.into_json().unwrap();

    assert_eq!(data["loadPerspective"]["loaded"][0], "perspective_orphan");
    assert_eq!(
        data["loadPerspective"]["unresolved"][0],
        "system:system_does_not_exist_9",
        "an unresolved manifest dep must be reported (tolerated, not an error)"
    );
}
