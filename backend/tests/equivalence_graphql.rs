//! The **archetype equivalence** surface: `archetypeEquivalence` exposes the topology ↔
//! vocabulary mapping for a cardinality, and `validateSystemEquivalence` checks a stored
//! system instance against its archetype. See `docs/v0.6-rebuild/validation.md`.

use std::sync::Arc;

use systematics_backend::{create_schema, data};
use tokio::sync::RwLock;

fn make_schema() -> systematics_backend::SystematicsSchema {
    create_schema(Arc::new(RwLock::new(data::build_graph())))
}

#[tokio::test]
async fn archetype_equivalence_lists_the_six_pairs() {
    let schema = make_schema();
    let q = r#"{ archetypeEquivalence(cardinality: 3) { topology system topologyValue systemValue } }"#;
    let resp = schema.execute(q).await;
    assert!(resp.errors.is_empty(), "query errors: {:?}", resp.errors);
    let d = resp.data.into_json().unwrap();
    let pairs = d["archetypeEquivalence"].as_array().unwrap();
    assert_eq!(pairs.len(), 6, "six paired dimensions");

    let find = |topology: &str| {
        pairs
            .iter()
            .find(|p| p["topology"] == topology)
            .unwrap_or_else(|| panic!("missing dimension {topology}"))
            .clone()
    };
    let graph = find("Graph");
    assert_eq!(graph["system"], "System");
    assert_eq!(graph["topologyValue"], "K3");
    assert_eq!(graph["systemValue"], "Triad");

    let card = find("Cardinality");
    assert_eq!(card["system"], "Coherence");
    assert_eq!(card["topologyValue"], "(3,3)");
    assert_eq!(card["systemValue"], "Dynamism");

    assert_eq!(find("Order")["systemValue"], "Impulses");
    assert_eq!(find("Size")["systemValue"], "Acts");
}

#[tokio::test]
async fn validate_system_equivalence_passes_for_a_canonical_triad() {
    let schema = make_schema();
    // Author a canonical triad; its coherence/designations come from the hexad tables.
    let m = r#"mutation { authorSystem(input:{ name:"Eq Triad", orderCardinality:3, terms:["A","B","C"], connectives:["x","y","z"] }){ id } }"#;
    let r = schema.execute(m).await;
    assert!(r.errors.is_empty(), "author errors: {:?}", r.errors);
    let id = r.data.into_json().unwrap()["authorSystem"]["id"].as_str().unwrap().to_string();

    let q = format!(r#"{{ validateSystemEquivalence(id: "{id}") }}"#);
    let resp = schema.execute(&q).await;
    assert!(resp.errors.is_empty(), "query errors: {:?}", resp.errors);
    let d = resp.data.into_json().unwrap();
    let mismatches = d["validateSystemEquivalence"].as_array().unwrap();
    assert!(mismatches.is_empty(), "expected a conforming triad, got: {mismatches:?}");
}

#[tokio::test]
async fn validate_system_equivalence_reports_missing_system() {
    let schema = make_schema();
    let resp = schema
        .execute(r#"{ validateSystemEquivalence(id: "system_nope_3") }"#)
        .await;
    assert!(resp.errors.is_empty(), "query errors: {:?}", resp.errors);
    let d = resp.data.into_json().unwrap();
    let mismatches = d["validateSystemEquivalence"].as_array().unwrap();
    assert_eq!(mismatches.len(), 1);
    assert!(mismatches[0].as_str().unwrap().contains("no system"));
}
