//! End-to-end test for the Functor surface (composable-perspectives plan,
//! Step 5). Exercises create → validate → applyFunctor through the schema, plus
//! rejection of a non-functor (a relabelling table that is not a bijection).
//!
//! Same-grammar functors are `S_n` permutations; the connective map is derived
//! from the object map, so a valid permutation transforms a perspective's
//! addresses (terms and connectives) onto the target system.

use std::sync::Arc;

use serde_json::json;
use systematics_backend::{create_schema, data};
use tokio::sync::RwLock;

fn make_schema() -> systematics_backend::SystematicsSchema {
    let shared_graph = Arc::new(RwLock::new(data::build_graph()));
    create_schema(shared_graph)
}

#[tokio::test]
async fn functor_create_validate_apply() {
    let schema = make_schema();

    // A valid S_3 permutation between two existing order_cardinality-3 systems: 1→2, 2→3,
    // 3→1, from the canonical triad to the citation triad.
    let create = r#"
        mutation {
            createFunctor(input: {
                id: "functor_rot3",
                name: "Rotate triad",
                orderCardinality: 3,
                sourceRef: "system_canonical_triad_3",
                targetRef: "system_citation_3",
                permutation: [2, 3, 1]
            }) { id orderCardinality sourceRef targetRef permutation }
        }
    "#;
    let resp = schema.execute(create).await;
    assert!(resp.errors.is_empty(), "createFunctor errors: {:?}", resp.errors);
    let data = resp.data.into_json().unwrap();
    assert_eq!(data["createFunctor"]["id"], "functor_rot3");
    assert_eq!(data["createFunctor"]["permutation"], json!([2, 3, 1]));

    // It validates (permutation laws hold + both systems exist at order_cardinality 3).
    let validate = r#"query { validateFunctor(id: "functor_rot3") }"#;
    let resp = schema.execute(validate).await;
    let data = resp.data.into_json().unwrap();
    assert!(
        data["validateFunctor"].as_array().unwrap().is_empty(),
        "expected no validation errors, got {:?}",
        data["validateFunctor"]
    );

    // Build a small source perspective with a term link and a connective link.
    let persp = r#"
        mutation {
            createPerspective(name: "Triad web") { id }
        }
    "#;
    let resp = schema.execute(persp).await;
    let data = resp.data.into_json().unwrap();
    let pid = data["createPerspective"]["id"].as_str().unwrap().to_string();

    let add_links = format!(
        r#"
        mutation {{
            a: addLink(perspectiveId: "{pid}", source: "system:system_canonical_triad_3#term:1", predicate: "notes", target: "system:system_canonical_triad_3#term:2") {{ id }}
            b: addLink(perspectiveId: "{pid}", source: "system:system_canonical_triad_3#conn:2-3", predicate: "notes", target: "system:system_canonical_triad_3") {{ id }}
        }}
    "#
    );
    let resp = schema.execute(add_links.as_str()).await;
    assert!(resp.errors.is_empty(), "addLink errors: {:?}", resp.errors);

    // Transform it by the functor → a new perspective whose addresses are
    // remapped onto the target system.
    let apply = format!(
        r#"
        mutation {{
            applyFunctor(
                functorRef: "functor_rot3",
                perspectiveRef: "{pid}",
                newId: "perspective_triad_web_rotated",
                newName: "Triad web (rotated)"
            ) {{ id links {{ source predicate target }} }}
        }}
    "#
    );
    let resp = schema.execute(apply.as_str()).await;
    assert!(resp.errors.is_empty(), "applyFunctor errors: {:?}", resp.errors);
    let data = resp.data.into_json().unwrap();
    let links = data["applyFunctor"]["links"].as_array().unwrap();
    assert_eq!(links.len(), 2);

    // term:1 → term:f(1)=2 ; term:2 → term:f(2)=3, all onto the target system.
    let term_link = links.iter().find(|l| l["predicate"] == "notes" && l["source"].as_str().unwrap().contains("term:")).unwrap();
    assert_eq!(term_link["source"], "system:system_citation_3#term:2");
    assert_eq!(term_link["target"], "system:system_citation_3#term:3");

    // conn:2-3 → conn between f(2)=3, f(3)=1 → canonicalized 1-3 ; bare system
    // retargeted.
    let conn_link = links.iter().find(|l| l["source"].as_str().unwrap().contains("conn:")).unwrap();
    assert_eq!(conn_link["source"], "system:system_citation_3#conn:1-3");
    assert_eq!(conn_link["target"], "system:system_citation_3");
}

#[tokio::test]
async fn non_bijection_fails_validation() {
    let schema = make_schema();

    // permutation [1, 1, 3] collapses two source positions and never hits 2 —
    // total but not a bijection, so not a functor.
    let create = r#"
        mutation {
            createFunctor(input: {
                id: "functor_bad",
                name: "Collapse",
                orderCardinality: 3,
                sourceRef: "system_canonical_triad_3",
                targetRef: "system_citation_3",
                permutation: [1, 1, 3]
            }) { id }
        }
    "#;
    let resp = schema.execute(create).await;
    assert!(resp.errors.is_empty(), "createFunctor errors: {:?}", resp.errors);

    let validate = r#"query { validateFunctor(id: "functor_bad") }"#;
    let resp = schema.execute(validate).await;
    let data = resp.data.into_json().unwrap();
    assert!(
        !data["validateFunctor"].as_array().unwrap().is_empty(),
        "a non-bijection must fail validation"
    );
}
