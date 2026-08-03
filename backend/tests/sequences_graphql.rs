//! End-to-end test for the Sequence surface (the `+` pole of the container
//! triad System · Sequence · Perspective). A Sequence is an ordered list of
//! member addresses; here we create one, read it back, and confirm order is
//! preserved and delete works.

use std::sync::Arc;

use systematics_backend::{create_schema, data};
use tokio::sync::RwLock;

fn make_schema() -> systematics_backend::SystematicsSchema {
    let shared_graph = Arc::new(RwLock::new(data::build_graph()));
    create_schema(shared_graph)
}

#[tokio::test]
async fn create_read_delete_sequence() {
    let schema = make_schema();

    // Create — members reference the pentad plus two canonical placeholders,
    // in order (an "Architecture Run"-shaped sequence).
    let create = r#"
        mutation {
            createSequence(input: {
                id: "sequence_test_run",
                name: "Test Run",
                members: [
                    "system:system_canonical_monad_1",
                    "system:system_canonical_dyad_2",
                    "system:system_architecture_pentad_5"
                ]
            }) { id name members }
        }
    "#;
    let resp = schema.execute(create).await;
    assert!(resp.errors.is_empty(), "create errors: {:?}", resp.errors);
    let data = resp.data.into_json().unwrap();
    assert_eq!(data["createSequence"]["id"], "sequence_test_run");
    let members = data["createSequence"]["members"].as_array().unwrap();
    assert_eq!(members.len(), 3);
    // Order preserved.
    assert_eq!(members[2], "system:system_architecture_pentad_5");

    // Read back via the list + single queries.
    let resp = schema.execute(r#"{ sequences { id } }"#).await;
    let data = resp.data.into_json().unwrap();
    assert!(data["sequences"]
        .as_array()
        .unwrap()
        .iter()
        .any(|s| s["id"] == "sequence_test_run"));

    let resp = schema
        .execute(r#"{ sequence(id: "sequence_test_run") { name members } }"#)
        .await;
    let data = resp.data.into_json().unwrap();
    assert_eq!(data["sequence"]["name"], "Test Run");
    assert_eq!(data["sequence"]["members"][0], "system:system_canonical_monad_1");

    // Delete.
    let resp = schema
        .execute(r#"mutation { deleteSequence(id: "sequence_test_run") }"#)
        .await;
    let data = resp.data.into_json().unwrap();
    assert_eq!(data["deleteSequence"], true);
}

#[tokio::test]
async fn auto_id_extracts_disambiguate() {
    // Two Extracts of the same scope produce the same auto-name → same base id.
    // The second must NOT error "already exists"; it gets a distinct id, so each
    // Extract yields its own Monad (the Extract-collision bug fix).
    let schema = make_schema();

    let extract = r#"
        mutation {
            createSequence(input: {
                name: "Monad — 3 Triad",
                members: ["system:system_canonical_triad_3"]
            }) { id name }
        }
    "#;

    let first = schema.execute(extract).await;
    assert!(first.errors.is_empty(), "first extract errors: {:?}", first.errors);
    let first_id = first.data.into_json().unwrap()["createSequence"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    let second = schema.execute(extract).await;
    assert!(
        second.errors.is_empty(),
        "second extract must not collide: {:?}",
        second.errors
    );
    let second_id = second.data.into_json().unwrap()["createSequence"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    assert_ne!(first_id, second_id, "repeated Extract must yield distinct ids");
    assert_eq!(second_id, format!("{first_id}_2"), "disambiguation appends _2");
}

#[tokio::test]
async fn explicit_id_collision_still_errors() {
    // An explicitly-supplied id that collides is a real error (not auto-disambiguated).
    let schema = make_schema();
    let create = r#"
        mutation { createSequence(input: { id: "seq_x", name: "X", members: [] }) { id } }
    "#;
    assert!(schema.execute(create).await.errors.is_empty());
    let dup = schema.execute(create).await;
    assert!(!dup.errors.is_empty(), "explicit duplicate id must error");
}

#[tokio::test]
async fn unknown_sequence_is_none() {
    let schema = make_schema();
    let resp = schema.execute(r#"{ sequence(id: "nope") { id } }"#).await;
    let data = resp.data.into_json().unwrap();
    assert!(data["sequence"].is_null());
}
