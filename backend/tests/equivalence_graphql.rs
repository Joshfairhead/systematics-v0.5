//! The **archetype equivalence** surface: `equivalenceHexad` exposes the two book-matched
//! hexads (topology + vocabulary) and the six paired dimensions relating them, and
//! `validateSystemEquivalence` checks a stored system instance against its archetype. See
//! `docs/v0.6-rebuild/validation.md`.

use std::sync::Arc;

use systematics_backend::{create_schema, data};
use tokio::sync::RwLock;

fn make_schema() -> systematics_backend::SystematicsSchema {
    create_schema(Arc::new(RwLock::new(data::build_graph())))
}

#[tokio::test]
async fn equivalence_hexad_book_matches_the_two_hexads() {
    let schema = make_schema();
    let q = r#"{
        equivalenceHexad(cardinality: 3) {
            topology { cardinality eigenvalue order size vertexOrdinality edgeOrdinality }
            system { name coherence termDesignation connectiveDesignation }
            pairs { topology system topologyValue systemValue }
            mismatches
        }
    }"#;
    let resp = schema.execute(q).await;
    assert!(resp.errors.is_empty(), "query errors: {:?}", resp.errors);
    let d = resp.data.into_json().unwrap();
    let h = &d["equivalenceHexad"];

    // Topology face.
    assert_eq!(h["topology"]["cardinality"], "(3,3)");
    assert_eq!(h["topology"]["eigenvalue"], "0 (×1), 3 (×2)");
    assert_eq!(h["topology"]["order"], 3);
    assert_eq!(h["topology"]["size"], 3);
    assert_eq!(h["topology"]["vertexOrdinality"], serde_json::json!([1, 2, 3]));
    // Vocabulary face.
    assert_eq!(h["system"]["name"], "Triad");
    assert_eq!(h["system"]["coherence"], "Dynamism");
    assert_eq!(h["system"]["termDesignation"], "Impulses");
    // The faces are consistent.
    assert!(h["mismatches"].as_array().unwrap().is_empty(), "book-match: {:?}", h["mismatches"]);

    // The six paired dimensions.
    let pairs = h["pairs"].as_array().unwrap();
    assert_eq!(pairs.len(), 6, "six paired dimensions");
    let find = |topology: &str| {
        pairs
            .iter()
            .find(|p| p["topology"] == topology)
            .unwrap_or_else(|| panic!("missing dimension {topology}"))
            .clone()
    };
    // Cardinality (3,3) ↔ System (Triad).
    let card = find("Cardinality");
    assert_eq!(card["system"], "System");
    assert_eq!(card["topologyValue"], "(3,3)");
    assert_eq!(card["systemValue"], "Triad");

    // Eigenvalue ↔ Coherence (Dynamism).
    let ev = find("Eigenvalue");
    assert_eq!(ev["system"], "Coherence");
    assert_eq!(ev["systemValue"], "Dynamism");

    assert_eq!(find("Order")["systemValue"], "Impulses");
    assert_eq!(find("Size")["systemValue"], "Acts");
    assert_eq!(find("VertexOrdinality")["system"], "TermOrdinality");
    assert_eq!(find("VertexOrdinality")["systemValue"], "term1..term3");
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
