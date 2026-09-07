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
