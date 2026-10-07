//! The **Load** stage of ELT: `loadMonad` lands an extracted selection raw in a named
//! Monad — authoring a K₁ head (a single graph node) and a bucket Sequence whose members
//! are `[head, …selected]`, persisted to the store. The Extract stage is client-side
//! selection; Transform is the in-monad operations (join/decompose/sequence).

use std::sync::Arc;

use systematics_backend::{create_schema, data};
use tokio::sync::RwLock;

fn make_schema() -> systematics_backend::SystematicsSchema {
    create_schema(Arc::new(RwLock::new(data::build_graph())))
}

#[tokio::test]
async fn load_monad_authors_a_head_and_buckets_the_selection() {
    let schema = make_schema();
    // Two things to extract (any systems — here two authored monads).
    for (n, t) in [("Visual", "Visual"), ("Auditory", "Auditory")] {
        let m = format!(
            r#"mutation {{ authorSystem(input:{{name:"{n}",orderCardinality:1,terms:["{t}"],connectives:[]}}){{id}} }}"#
        );
        assert!(schema.execute(m).await.errors.is_empty(), "author {n}");
    }

    // Load: land the selection in a monad named "Senses".
    let m = r#"mutation {
        loadMonad(input:{ name:"Senses", members:["system:system_visual_1","system:system_auditory_1"] }) {
            id name members
        }
    }"#;
    let r = schema.execute(m).await;
    assert!(r.errors.is_empty(), "loadMonad errors: {:?}", r.errors);
    let d = r.data.into_json().unwrap();
    let seq = &d["loadMonad"];

    // The bucket is named after the monad; its head is a K₁ system named likewise and sits
    // first, followed by the extracted members (order preserved).
    assert_eq!(seq["name"], "Senses");
    let members: Vec<String> = seq["members"]
        .as_array()
        .unwrap()
        .iter()
        .map(|m| m.as_str().unwrap().to_string())
        .collect();
    assert_eq!(
        members,
        vec![
            "system:system_senses_1",
            "system:system_visual_1",
            "system:system_auditory_1",
        ],
        "head first, then the extracted members"
    );

    // The K₁ head resolves as a real single-node system (it plots as the monad's node).
    let q = r#"{ renderSystem(systemId:"system_senses_1") { orderCardinality terms { value } } }"#;
    let rr = schema.execute(q).await;
    assert!(rr.errors.is_empty(), "render head: {:?}", rr.errors);
    let dd = rr.data.into_json().unwrap();
    assert_eq!(dd["renderSystem"]["orderCardinality"], 1);
    assert_eq!(dd["renderSystem"]["terms"][0]["value"], "Senses");
}

#[tokio::test]
async fn load_monad_rejects_an_empty_selection() {
    let schema = make_schema();
    let m = r#"mutation { loadMonad(input:{ name:"Empty", members:[] }) { id } }"#;
    let r = schema.execute(m).await;
    assert!(!r.errors.is_empty(), "an empty selection must error");
}
