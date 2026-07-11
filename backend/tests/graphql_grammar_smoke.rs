//! End-to-end smoke test for the Grammar / Vocabulary / Character surface.
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

    // Create the SemanticVocabulary, reusing canonical connective Characters
    // (Generation / Decision / Consent are seeded).
    let create_sv = r#"
        mutation {
            createSemanticVocab(input: {
                name: "Theology Triad",
                order: 3,
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
            }) { id name order terms connectives validationErrors }
        }
    "#;
    let resp = schema.execute(create_sv).await;
    assert!(resp.errors.is_empty(), "create sv errors: {:?}", resp.errors);
    let data = resp.data.into_json().unwrap();
    let sv_id = data["createSemanticVocab"]["id"].as_str().unwrap().to_string();
    assert_eq!(data["createSemanticVocab"]["name"], "Theology Triad");
    assert_eq!(data["createSemanticVocab"]["terms"].as_array().unwrap().len(), 3);
    assert!(data["createSemanticVocab"]["validationErrors"]
        .as_array()
        .unwrap()
        .is_empty());

    // Create the Grammar.
    let create_grammar = r#"
        mutation Create($sv: String!) {
            createGrammar(input: {
                name: "Theology Triad",
                order: 3,
                coherence: "Trinity",
                termDesignation: "Persons",
                connectiveDesignation: "Perichoresis",
                topologicalVocabRef: "topvocab_3",
                geometricVocabRef: "geovocab_3",
                semanticVocabRef: $sv
            }) { id name coherence topologicalVocabRef }
        }
    "#;
    let req = async_graphql::Request::new(create_grammar)
        .variables(async_graphql::Variables::from_json(json!({ "sv": sv_id })));
    let resp = schema.execute(req).await;
    assert!(resp.errors.is_empty(), "create grammar errors: {:?}", resp.errors);
    let data = resp.data.into_json().unwrap();
    let grammar_id = data["createGrammar"]["id"].as_str().unwrap().to_string();
    assert_eq!(data["createGrammar"]["name"], "Theology Triad");
    assert_eq!(data["createGrammar"]["coherence"], "Trinity");

    // Validate returns no errors.
    let validate = r#"query Validate($id: String!) { validateGrammar(id: $id) }"#;
    let req = async_graphql::Request::new(validate)
        .variables(async_graphql::Variables::from_json(json!({ "id": grammar_id })));
    let resp = schema.execute(req).await;
    assert!(resp.errors.is_empty(), "validate errors: {:?}", resp.errors);
    let data = resp.data.into_json().unwrap();
    let errs = data["validateGrammar"].as_array().unwrap();
    assert!(errs.is_empty(), "validation returned: {:?}", errs);

    // Character-at-point join uses the new vocabulary.
    let cap = r#"
        query Cap($sv: String!) {
            characterAtPoint(semanticVocabId: $sv, pointId: "point_3_2") { id value }
        }
    "#;
    let req = async_graphql::Request::new(cap)
        .variables(async_graphql::Variables::from_json(json!({ "sv": sv_id })));
    let resp = schema.execute(req).await;
    let data = resp.data.into_json().unwrap();
    assert_eq!(data["characterAtPoint"]["value"], "Omniscient");

    // Clean up.
    let cleanup = r#"
        mutation Cleanup($grammar: String!, $sv: String!) {
            g: deleteGrammar(id: $grammar)
            s: deleteSemanticVocab(id: $sv)
        }
    "#;
    let req = async_graphql::Request::new(cleanup).variables(
        async_graphql::Variables::from_json(json!({ "grammar": grammar_id, "sv": sv_id })),
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

    // Only Character / SemanticVocab / Grammar mutations exist.
    let allowed = ["Character", "SemanticVocab", "Grammar"];
    for f in &fields {
        assert!(
            allowed.iter().any(|a| f.contains(a)),
            "unexpected mutation `{}` — structural anchors and metadata are immutable",
            f
        );
    }

    for expected in [
        "createCharacter",
        "deleteCharacter",
        "createSemanticVocab",
        "updateSemanticVocab",
        "deleteSemanticVocab",
        "createGrammar",
        "updateGrammar",
        "deleteGrammar",
    ] {
        assert!(
            fields.iter().any(|f| f == expected),
            "missing expected mutation `{}` in {:?}",
            expected,
            fields
        );
    }
}
