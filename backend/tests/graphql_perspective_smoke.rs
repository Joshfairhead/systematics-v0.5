//! End-to-end smoke test for the System / Vocabulary / Character surface.
//!
//! Verifies that a user can build a full Theology Triad from scratch and
//! query it back through the schema.

use std::sync::Arc;

use serde_json::json;
use systematics_backend::{create_schema, data};
use tokio::sync::RwLock;

fn make_schema() -> systematics_backend::SystematicsSchema {
    let shared_graph = Arc::new(RwLock::new(data::build_graph()));
    create_schema(shared_graph)
}

#[tokio::test]
async fn create_theology_triad_end_to_end() {
    let schema = make_schema();

    // Create three Characters.
    let create_chars = r#"
        mutation {
            imm: createCharacter(input: { kind: "word", value: "Immanent" }) { id kind value }
            omn: createCharacter(input: { kind: "word", value: "Omniscient" }) { id }
            trans: createCharacter(input: { kind: "word", value: "Transcendental" }) { id }
        }
    "#;
    let resp = schema.execute(create_chars).await;
    assert!(resp.errors.is_empty(), "create chars errors: {:?}", resp.errors);
    let data = resp.data.into_json().unwrap();
    assert_eq!(data["imm"]["id"], "char_word_immanent");
    assert_eq!(data["omn"]["id"], "char_word_omniscient");
    assert_eq!(data["trans"]["id"], "char_word_transcendental");

    // Create the Vocabulary (Vocabulary), reusing canonical connective
    // Characters (Generation / Decision / Consent are seeded).
    let create_sv = r#"
        mutation {
            createVocabulary(input: {
                name: "Theology Triad",
                orderCardinality: 3,
                terms: [
                    "char_word_immanent",
                    "char_word_omniscient",
                    "char_word_transcendental"
                ],
                connectives: [
                    "char_word_generation",
                    "char_word_decision",
                    "char_word_consent"
                ]
            }) { id name orderCardinality terms connectives validationErrors }
        }
    "#;
    let resp = schema.execute(create_sv).await;
    assert!(resp.errors.is_empty(), "create sv errors: {:?}", resp.errors);
    let data = resp.data.into_json().unwrap();
    let sv_id = data["createVocabulary"]["id"].as_str().unwrap().to_string();
    assert_eq!(data["createVocabulary"]["name"], "Theology Triad");
    assert_eq!(data["createVocabulary"]["terms"].as_array().unwrap().len(), 3);
    assert!(data["createVocabulary"]["validationErrors"]
        .as_array()
        .unwrap()
        .is_empty());

    // Create the System reconciling the canonical triad Grammar with the vocab.
    let create_system = r#"
        mutation Create($sv: String!) {
            createSystem(input: {
                name: "Theology Triad",
                orderCardinality: 3,
                coherence: "Trinity",
                termDesignation: "Persons",
                connectiveDesignation: "Perichoresis",
                grammarRef: "grammar_3",
                vocabularyRef: $sv
            }) { id name coherence grammarRef vocabularyRef }
        }
    "#;
    let req = async_graphql::Request::new(create_system)
        .variables(async_graphql::Variables::from_json(json!({ "sv": sv_id })));
    let resp = schema.execute(req).await;
    assert!(resp.errors.is_empty(), "create system errors: {:?}", resp.errors);
    let data = resp.data.into_json().unwrap();
    let system_id = data["createSystem"]["id"].as_str().unwrap().to_string();
    assert_eq!(data["createSystem"]["name"], "Theology Triad");
    assert_eq!(data["createSystem"]["coherence"], "Trinity");
    assert_eq!(data["createSystem"]["grammarRef"], "grammar_3");

    // Validate returns no errors.
    let validate = r#"query Validate($id: String!) { validateSystem(id: $id) }"#;
    let req = async_graphql::Request::new(validate)
        .variables(async_graphql::Variables::from_json(json!({ "id": system_id })));
    let resp = schema.execute(req).await;
    assert!(resp.errors.is_empty(), "validate errors: {:?}", resp.errors);
    let data = resp.data.into_json().unwrap();
    let errs = data["validateSystem"].as_array().unwrap();
    assert!(errs.is_empty(), "validation returned: {:?}", errs);

    // Character-at-point join uses the new vocabulary.
    let cap = r#"
        query Cap($sv: String!) {
            characterAtPoint(vocabularyId: $sv, pointId: "point_3_2") { id value }
        }
    "#;
    let req = async_graphql::Request::new(cap)
        .variables(async_graphql::Variables::from_json(json!({ "sv": sv_id })));
    let resp = schema.execute(req).await;
    let data = resp.data.into_json().unwrap();
    assert_eq!(data["characterAtPoint"]["value"], "Omniscient");

    // Clean up.
    let cleanup = r#"
        mutation Cleanup($system: String!, $sv: String!) {
            g: deleteSystem(id: $system)
            s: deleteVocabulary(id: $sv)
        }
    "#;
    let req = async_graphql::Request::new(cleanup).variables(
        async_graphql::Variables::from_json(json!({ "system": system_id, "sv": sv_id })),
    );
    let resp = schema.execute(req).await;
    let data = resp.data.into_json().unwrap();
    assert_eq!(data["g"], true);
    assert_eq!(data["s"], true);
}

#[tokio::test]
async fn mutation_root_exposes_only_new_shape_mutations() {
    let schema = make_schema();
    let introspect = r#"
        query { __type(name: "MutationRoot") { fields { name } } }
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

    // Language-layer mutations (Character / Vocabulary / System) plus the
    // referencing layer (Perspective / Link / Source / Artefact / Lookup /
    // Reference). Structural anchors and grammars remain immutable.
    let allowed = [
        "Character",
        "Vocabulary",
        "System",
        "Functor",
        "Sequence",
        "Perspective",
        "Link",
        "Source",
        "Artefact",
        "Lookup",
        "Reference",
    ];
    for f in &fields {
        assert!(
            allowed.iter().any(|a| f.contains(a)),
            "unexpected mutation `{}` — structural anchors and grammars are immutable",
            f
        );
    }

    for expected in [
        "createCharacter",
        "deleteCharacter",
        "createVocabulary",
        "updateVocabulary",
        "deleteVocabulary",
        "createSystem",
        "updateSystem",
        "deleteSystem",
    ] {
        assert!(
            fields.iter().any(|f| f == expected),
            "missing expected mutation `{}` in {:?}",
            expected,
            fields
        );
    }
}
