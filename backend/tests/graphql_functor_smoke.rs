//! End-to-end smoke test for the Functor GraphQL CRUD surface.
//!
//! Verifies that a Functor can be created, listed, applied, and deleted
//! through the schema without going through HTTP.

use std::sync::Arc;

use serde_json::json;
use systematics_backend::{create_schema, data};
use tokio::sync::RwLock;

fn make_schema() -> systematics_backend::SystematicsSchema {
    let shared_graph = Arc::new(RwLock::new(data::build_graph()));
    create_schema(shared_graph)
}

#[tokio::test]
async fn create_apply_and_delete_functor_via_graphql() {
    let schema = make_schema();

    // Create.
    let create = r#"
        mutation Create($input: FunctorInput!) {
            createFunctor(input: $input) { id name mappings { base target } }
        }
    "#;
    let vars = json!({
        "input": {
            "name": "canonical-triad",
            "sourceLanguage": "CANONICAL",
            "mappings": [
                { "base": "char_canonical_will", "target": "loc_3_1" },
                { "base": "char_canonical_function", "target": "loc_3_2" },
                { "base": "char_canonical_being", "target": "loc_3_3" }
            ]
        }
    });
    let req = async_graphql::Request::new(create).variables(async_graphql::Variables::from_json(vars));
    let resp = schema.execute(req).await;
    assert!(resp.errors.is_empty(), "create errors: {:?}", resp.errors);
    let data = resp.data.into_json().unwrap();
    let functor_id = data["createFunctor"]["id"].as_str().unwrap().to_string();
    assert_eq!(data["createFunctor"]["name"], "canonical-triad");
    assert_eq!(data["createFunctor"]["mappings"].as_array().unwrap().len(), 3);

    // List.
    let list = r#"query { functors { id name } }"#;
    let resp = schema.execute(list).await;
    assert!(resp.errors.is_empty(), "list errors: {:?}", resp.errors);
    let listed = resp.data.into_json().unwrap();
    let names: Vec<String> = listed["functors"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f["name"].as_str().unwrap().to_string())
        .collect();
    assert!(names.contains(&"canonical-triad".to_string()));

    // Apply.
    let apply = r#"
        mutation Apply($id: String!) {
            applyFunctor(id: $id) { id locationId characterId }
        }
    "#;
    let req = async_graphql::Request::new(apply).variables(
        async_graphql::Variables::from_json(json!({ "id": functor_id })),
    );
    let resp = schema.execute(req).await;
    assert!(resp.errors.is_empty(), "apply errors: {:?}", resp.errors);
    let applied = resp.data.into_json().unwrap();
    let produced = applied["applyFunctor"].as_array().unwrap();
    assert_eq!(produced.len(), 3);
    assert!(produced
        .iter()
        .any(|t| t["locationId"] == "loc_3_1" && t["characterId"] == "char_canonical_will"));

    // Delete.
    let delete = r#"
        mutation Delete($id: String!) {
            deleteFunctor(id: $id)
        }
    "#;
    let req = async_graphql::Request::new(delete).variables(
        async_graphql::Variables::from_json(json!({ "id": functor_id })),
    );
    let resp = schema.execute(req).await;
    assert!(resp.errors.is_empty(), "delete errors: {:?}", resp.errors);
    let deleted = resp.data.into_json().unwrap();
    assert_eq!(deleted["deleteFunctor"], true);

    // Confirm gone.
    let resp = schema.execute(list).await;
    let listed = resp.data.into_json().unwrap();
    let still_there = listed["functors"]
        .as_array()
        .unwrap()
        .iter()
        .any(|f| f["name"] == "canonical-triad");
    assert!(!still_there);
}

#[tokio::test]
async fn no_mutation_exists_for_fixed_metadata() {
    // Schema introspection should show only Functor mutations.
    let schema = make_schema();
    let introspect = r#"
        query {
            __type(name: "MutationRoot") {
                fields { name }
            }
        }
    "#;
    let resp = schema.execute(introspect).await;
    assert!(resp.errors.is_empty(), "introspect errors: {:?}", resp.errors);
    let data = resp.data.into_json().unwrap();
    let fields: Vec<String> = data["__type"]["fields"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f["name"].as_str().unwrap().to_string())
        .collect();

    // Only functor mutations exist.
    for f in &fields {
        assert!(
            f.contains("Functor") || f.contains("functor"),
            "unexpected mutation `{}` — fixed metadata must remain immutable",
            f
        );
    }
    // Sanity: the four we expect are all present.
    for expected in ["createFunctor", "updateFunctor", "deleteFunctor", "applyFunctor"] {
        assert!(
            fields.iter().any(|f| f == expected),
            "missing expected mutation `{}` in {:?}",
            expected,
            fields
        );
    }
}
