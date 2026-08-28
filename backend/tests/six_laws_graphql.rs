//! End-to-end test for the Controller surface: `runSixLaws` runs a seeded system
//! through the six laws of three (mimic form — each law reorders a triad). Also
//! exercises `positions` — interpreting a selection of nodes from a higher-order
//! system as a triad (the "semantic maths over any graph" move, e.g. an octad).

use std::sync::Arc;

use serde_json::json;
use systematics_backend::{create_schema, data};
use tokio::sync::RwLock;

fn make_schema() -> systematics_backend::SystematicsSchema {
    let shared_graph = Arc::new(RwLock::new(data::build_graph()));
    create_schema(shared_graph)
}

#[tokio::test]
async fn run_six_laws_over_canonical_triad() {
    let schema = make_schema();
    let q = r#"query {
        runSixLaws(systemId: "system_canonical_triad_3") {
            law hexadPosition colour permutation aliases reading
        }
    }"#;
    let resp = schema.execute(q).await;
    assert!(resp.errors.is_empty(), "errors: {:?}", resp.errors);
    let data = resp.data.into_json().unwrap();
    let rows = data["runSixLaws"].as_array().unwrap();
    assert_eq!(rows.len(), 6, "one reading per law");

    let find = |name: &str| rows.iter().find(|r| r["law"] == name).unwrap().clone();
    let expansion = find("expansion"); // 123 — the base order
    let interaction = find("interaction"); // 132 — SPO

    // interaction is the SPO law: purple, hexad ordinality 5, alias SPO.
    assert_eq!(interaction["aliases"], json!(["SPO"]));
    assert_eq!(interaction["colour"], "purple");
    assert_eq!(interaction["hexadPosition"], 5);
    assert_eq!(interaction["permutation"], json!([1, 3, 2]));

    // Robust vs the actual term values: interaction (132) reorders the base
    // reading [t1, t2, t3] into [t1, t3, t2].
    let base = expansion["reading"].as_array().unwrap();
    assert_eq!(base.len(), 3);
    let spo = interaction["reading"].as_array().unwrap();
    assert_eq!(spo, &vec![base[0].clone(), base[2].clone(), base[1].clone()]);
}

#[tokio::test]
async fn run_six_laws_interprets_selected_nodes_as_a_triad() {
    // The Controller operates over ANY graph: pick 3 nodes from the seeded Hexad
    // (order 6) — positions 4, 6, 1 — and read them as a triad.
    let schema = make_schema();
    let q = r#"query {
        runSixLaws(systemId: "system_six_laws_of_three_6", positions: [4, 6, 1]) {
            law reading
        }
    }"#;
    let resp = schema.execute(q).await;
    assert!(resp.errors.is_empty(), "errors: {:?}", resp.errors);
    let data = resp.data.into_json().unwrap();
    let rows = data["runSixLaws"].as_array().unwrap();
    assert_eq!(rows.len(), 6);

    // positions [4,6,1] pick a triad; expansion (123) reads it in that order, and
    // interaction (132) reorders it to [pos4, pos1, pos6] = [base[0], base[2], base[1]].
    let expansion = rows.iter().find(|r| r["law"] == "expansion").unwrap();
    let base = expansion["reading"].as_array().unwrap();
    assert_eq!(base.len(), 3);
    assert!(base[0] != base[1] && base[1] != base[2] && base[0] != base[2]);
    let interaction = rows.iter().find(|r| r["law"] == "interaction").unwrap();
    assert_eq!(
        interaction["reading"].as_array().unwrap(),
        &vec![base[0].clone(), base[2].clone(), base[1].clone()]
    );
}

#[tokio::test]
async fn run_six_laws_empty_for_bad_input() {
    let schema = make_schema();
    // Unknown system → empty.
    let q1 = r#"query { runSixLaws(systemId: "nope") { law } }"#;
    let r1 = schema.execute(q1).await;
    assert!(r1.errors.is_empty());
    assert_eq!(r1.data.into_json().unwrap()["runSixLaws"], json!([]));
    // Wrong number of positions → empty (needs exactly 3 to be a triad).
    let q2 = r#"query { runSixLaws(systemId: "system_canonical_triad_3", positions: [1,2]) { law } }"#;
    let r2 = schema.execute(q2).await;
    assert!(r2.errors.is_empty());
    assert_eq!(r2.data.into_json().unwrap()["runSixLaws"], json!([]));
}
