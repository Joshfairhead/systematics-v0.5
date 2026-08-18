//! The reference-browser design enriches GqlReference with resolved fields so a
//! single `allReferences` query drives both the table and the compare-by-order
//! matrix. This pins those resolvers against real module data.

use std::sync::Arc;

use systematics_backend::{create_schema, data};
use tokio::sync::RwLock;

fn assembled_schema() -> systematics_backend::SystematicsSchema {
    let shared = Arc::new(RwLock::new({
        let mut g = data::build_graph();
        let n = data::load_perspective_modules(&mut g);
        if n > 0 {
            g.mark_bundled();
        }
        g
    }));
    create_schema(shared)
}

#[tokio::test]
async fn all_references_exposes_resolved_browser_fields() {
    let schema = assembled_schema();

    let query = r#"
        {
            allReferences {
                perspectiveName
                target
                targetFragment
                object
                targetSystem { order name coherence }
            }
        }
    "#;
    let resp = schema.execute(query).await;
    assert!(resp.errors.is_empty(), "query errors: {:?}", resp.errors);
    let data = resp.data.into_json().unwrap();
    let refs = data["allReferences"].as_array().unwrap();
    assert_eq!(refs.len(), 59, "all 59 references present");

    // DU1 now asserts its heptad coherence onto the CANONICAL heptad, carrying the
    // value ("Structure") as the reference's SPO `object`.
    let du1_heptad = refs
        .iter()
        .find(|r| r["target"] == "system:system_canonical_heptad_7#coherence"
            && r["perspectiveName"] == "Dramatic Universe Vol 1")
        .expect("DU1 heptad coherence citation onto canonical present");

    assert_eq!(du1_heptad["targetFragment"], "coherence");
    assert_eq!(du1_heptad["targetSystem"]["order"], 7);
    // The value DU1 affixes is now the reference `object` (not a field on a shell).
    assert_eq!(du1_heptad["object"], "Structure");
}

#[tokio::test]
async fn whole_system_citation_has_empty_fragment() {
    let schema = assembled_schema();
    let query = r#"
        {
            allReferences {
                target
                targetFragment
                targetSystem { order }
            }
        }
    "#;
    let resp = schema.execute(query).await;
    let data = resp.data.into_json().unwrap();
    let refs = data["allReferences"].as_array().unwrap();

    // A whole-system citation (no '#') resolves an empty fragment but still a system.
    // DU1's whole-system citation now points at its Coherence Dodecad (order 12).
    let whole = refs
        .iter()
        .find(|r| r["target"] == "system:system_du1_coherence_dodecad_12")
        .expect("DU1 whole-dodecad citation present");
    assert_eq!(whole["targetFragment"], "");
    assert_eq!(whole["targetSystem"]["order"], 12);
}
