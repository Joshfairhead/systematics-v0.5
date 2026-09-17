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

#[tokio::test]
async fn edit_system_is_id_stable_and_keeps_sequence_refs() {
    // The id-stable rename: editing a system (even its name) keeps its id, so a sequence
    // that references it is not orphaned. authorSystem would fork a new id and strand the ref.
    let schema = make_schema();
    // A monad, added to a sequence.
    let a = schema
        .execute(r#"mutation { authorSystem(input:{name:"Monad CT", orderCardinality:1, terms:["Monad CT"], connectives:[]}){ id } }"#)
        .await;
    assert!(a.errors.is_empty(), "author: {:?}", a.errors);
    let id = a.data.into_json().unwrap()["authorSystem"]["id"].as_str().unwrap().to_string();
    let seq = format!(
        r#"mutation {{ createSequence(input:{{name:"CT", members:["system:{id}"]}}){{ id }} }}"#
    );
    let s = schema.execute(&seq).await;
    assert!(s.errors.is_empty(), "createSequence: {:?}", s.errors);
    let seq_id = s.data.into_json().unwrap()["createSequence"]["id"].as_str().unwrap().to_string();

    // Rename the monad to "Monoid" via editSystem — id must NOT change.
    let e = format!(
        r#"mutation {{ editSystem(id:"{id}", input:{{name:"Monoid", orderCardinality:1, terms:["Monoid"], connectives:[]}}){{ id name }} }}"#
    );
    let r = schema.execute(&e).await;
    assert!(r.errors.is_empty(), "editSystem: {:?}", r.errors);
    let d = r.data.into_json().unwrap();
    assert_eq!(d["editSystem"]["id"], id, "id must be stable across a rename");
    assert_eq!(d["editSystem"]["name"], "Monoid", "name updated in place");

    // The system still resolves under the same id, now named Monoid…
    let rr = schema.execute(&format!(r#"{{ renderSystem(systemId:"{id}"){{ terms {{ value }} }} }}"#)).await;
    let dd = rr.data.into_json().unwrap();
    assert_eq!(dd["renderSystem"]["terms"][0]["value"], "Monoid");

    // …and the sequence still references it (not orphaned).
    let sq = schema.execute(r#"{ sequences { id members } }"#).await;
    let sd = sq.data.into_json().unwrap();
    let members: Vec<String> = sd["sequences"].as_array().unwrap().iter()
        .find(|s| s["id"] == seq_id).unwrap()["members"].as_array().unwrap()
        .iter().map(|m| m.as_str().unwrap().to_string()).collect();
    assert!(members.contains(&format!("system:{id}")), "sequence ref survives the rename: {members:?}");

    // No dangling threads: the old (name-derived) vocabulary + character are cleaned up.
    let old = schema
        .execute(r#"{ v: vocabulary(id: "vocab_monad_ct_1") { id } c: character(id: "char_word_monad_ct_t1") { id } }"#)
        .await;
    assert!(old.errors.is_empty(), "cleanup query: {:?}", old.errors);
    let od = old.data.into_json().unwrap();
    assert!(od["v"].is_null(), "old vocabulary should be cleaned up after a rename");
    assert!(od["c"].is_null(), "old character should be cleaned up after a rename");
}
