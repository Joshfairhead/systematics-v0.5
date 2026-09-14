//! The in-app editor path: author a whole System from custom term/connective
//! values (build characters + vocabulary + system at runtime), instead of a
//! Rust seed table.

use std::sync::Arc;

use systematics_backend::{create_schema, data};
use tokio::sync::RwLock;

fn make_schema() -> systematics_backend::SystematicsSchema {
    create_schema(Arc::new(RwLock::new(data::build_graph())))
}

#[tokio::test]
async fn author_triad_from_values() {
    let schema = make_schema();
    let m = r#"
        mutation {
            authorSystem(input: {
                name: "My Triad",
                orderCardinality: 3,
                terms: ["Alpha", "Beta", "Gamma"],
                connectives: ["e1", "e2", "e3"]
            }) { id name orderCardinality }
        }
    "#;
    let resp = schema.execute(m).await;
    assert!(resp.errors.is_empty(), "author errors: {:?}", resp.errors);
    let d = resp.data.into_json().unwrap();
    assert_eq!(d["authorSystem"]["id"], "system_my_triad_3");

    // It renders with the custom terms + connectives.
    let r = schema
        .execute(r#"{ renderSystem(systemId: "system_my_triad_3") { terms { value } } }"#)
        .await;
    assert!(r.errors.is_empty(), "render errors: {:?}", r.errors);
    let d = r.data.into_json().unwrap();
    let terms: Vec<String> = d["renderSystem"]["terms"].as_array().unwrap().iter().map(|t| t["value"].as_str().unwrap().to_string()).collect();
    assert_eq!(terms, vec!["Alpha", "Beta", "Gamma"]);
}

#[tokio::test]
async fn join_combines_selected_systems_into_a_kn() {
    // Composition (the assembly join): two monads → a dyad on the union of their distinct
    // terms, appended to the monad's sequence (its associations).
    let schema = make_schema();
    for (n, t) in [("Essence", "Essence"), ("Existence", "Existence")] {
        let m = format!(
            r#"mutation {{ authorSystem(input:{{name:"{n}",orderCardinality:1,terms:["{t}"],connectives:[]}}){{id}} }}"#
        );
        assert!(schema.execute(m).await.errors.is_empty(), "author {n}");
    }
    let seq = r#"mutation { createSequence(input:{name:"Unity Assoc", members:["system:system_essence_1","system:system_existence_1"]}){ id } }"#;
    let r = schema.execute(seq).await;
    assert!(r.errors.is_empty(), "createSequence: {:?}", r.errors);
    let seq_id = r.data.into_json().unwrap()["createSequence"]["id"].as_str().unwrap().to_string();

    let join = format!(
        r#"mutation {{ joinSystems(input:{{ name:"Essence Existence", members:["system:system_essence_1","system:system_existence_1"], sequenceRef:"{seq_id}" }}){{ id orderCardinality }} }}"#
    );
    let r = schema.execute(join).await;
    assert!(r.errors.is_empty(), "join: {:?}", r.errors);
    let d = r.data.into_json().unwrap();
    assert_eq!(d["joinSystems"]["orderCardinality"], 2, "two distinct terms → a dyad");
    let new_id = d["joinSystems"]["id"].as_str().unwrap().to_string();

    // The joined dyad renders with the two distinct terms (order preserved).
    let r = schema
        .execute(format!(r#"{{ renderSystem(systemId:"{new_id}"){{ terms {{ value }} }} }}"#))
        .await;
    let terms: Vec<String> = r.data.into_json().unwrap()["renderSystem"]["terms"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["value"].as_str().unwrap().to_string())
        .collect();
    assert_eq!(terms, vec!["Essence", "Existence"]);

    // …and it's now an association (member) of the monad's sequence.
    let r = schema.execute(r#"{ sequences { id members } }"#).await;
    let seqs = r.data.into_json().unwrap();
    let appended = seqs["sequences"].as_array().unwrap().iter().any(|s| {
        s["id"] == serde_json::json!(seq_id)
            && s["members"]
                .as_array()
                .unwrap()
                .iter()
                .any(|m| m.as_str() == Some(&format!("system:{new_id}")))
    });
    assert!(appended, "joined system should be appended to the monad's sequence");
}

#[tokio::test]
async fn decompose_tetrad_into_dyads_and_triads() {
    // Decomposition (the inverse of join): a tetrad → its 6 dyads (2-subsets) + 4 triads
    // (3-subsets) — the complete-subgraph faces.
    let schema = make_schema();
    let m = r#"mutation { authorSystem(input:{name:"Quad",orderCardinality:4,terms:["A","B","C","D"],connectives:["e1","e2","e3","e4","e5","e6"]}){id} }"#;
    assert!(schema.execute(m).await.errors.is_empty(), "author tetrad");

    let d = schema
        .execute(r#"mutation { decomposeSystem(input:{ systemRef:"system:system_quad_4" }){ id orderCardinality } }"#)
        .await;
    assert!(d.errors.is_empty(), "decompose: {:?}", d.errors);
    let faces = d.data.into_json().unwrap()["decomposeSystem"].as_array().unwrap().clone();
    assert_eq!(faces.len(), 10, "tetrad decomposes into 10 faces");
    let dyads = faces.iter().filter(|f| f["orderCardinality"] == 2).count();
    let triads = faces.iter().filter(|f| f["orderCardinality"] == 3).count();
    assert_eq!((dyads, triads), (6, 4), "6 dyads + 4 triads");
}

#[tokio::test]
async fn author_same_name_overwrites_in_place() {
    // Store = write with overwrite: re-authoring an existing name+order UPDATES it
    // (the CRUD Update path) rather than forking or erroring.
    let schema = make_schema();
    let author = |terms: &str| {
        format!(
            r#"mutation {{ authorSystem(input: {{ name: "Edit Me", orderCardinality: 3, terms: {terms}, connectives: ["e1","e2","e3"] }}) {{ id }} }}"#
        )
    };

    let first = schema.execute(author(r#"["Alpha", "Beta", "Gamma"]"#)).await;
    assert!(first.errors.is_empty(), "first author errors: {:?}", first.errors);

    // Same name + order, different term values — must succeed (overwrite), not error.
    let second = schema.execute(author(r#"["One", "Two", "Three"]"#)).await;
    assert!(
        second.errors.is_empty(),
        "re-authoring the same system must overwrite, not error: {:?}",
        second.errors
    );

    // The rendered system reflects the NEW values, and there is exactly one system id.
    let r = schema
        .execute(r#"{ renderSystem(systemId: "system_edit_me_3") { terms { value } } }"#)
        .await;
    let d = r.data.into_json().unwrap();
    let terms: Vec<String> = d["renderSystem"]["terms"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["value"].as_str().unwrap().to_string())
        .collect();
    assert_eq!(terms, vec!["One", "Two", "Three"], "overwrite must replace the term values");
}

#[tokio::test]
async fn author_rejects_wrong_arity() {
    let schema = make_schema();
    let m = r#"mutation { authorSystem(input: { name: "Bad", orderCardinality: 3, terms: ["a","b"], connectives: ["e1","e2","e3"] }) { id } }"#;
    let resp = schema.execute(m).await;
    assert!(!resp.errors.is_empty(), "wrong term count must error");
}
