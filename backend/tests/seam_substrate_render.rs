//! Seam regression guard: the serving path (`resolve_system`) composes the model via
//! the **substrate** (`compose_system`) and resolves the view from it — the
//! "compose, don't load" convergence. This pins the view byte-identical so the
//! substrate-composed render can't silently drift from the expected output.

use std::sync::Arc;

use systematics_backend::{create_schema, data};
use tokio::sync::RwLock;

fn make_schema() -> systematics_backend::SystematicsSchema {
    let shared_graph = Arc::new(RwLock::new(data::build_graph()));
    create_schema(shared_graph)
}

#[tokio::test]
async fn substrate_composed_triad_view_is_byte_identical() {
    let schema = make_schema();
    let q = r#"query {
        system(order: 3) {
            terms { position value }
            connectives { id basePosition targetPosition characterValue }
        }
    }"#;
    let resp = schema.execute(q).await;
    assert!(resp.errors.is_empty(), "errors: {:?}", resp.errors);
    let d = resp.data.into_json().unwrap();
    let sys = &d["system"];

    // Terms — composed by the substrate, in ordinality order.
    let terms = sys["terms"].as_array().unwrap();
    assert_eq!(terms.len(), 3);
    assert_eq!((terms[0]["position"].as_i64().unwrap(), terms[0]["value"].as_str().unwrap()), (1, "Will"));
    assert_eq!((terms[1]["position"].as_i64().unwrap(), terms[1]["value"].as_str().unwrap()), (2, "Function"));
    assert_eq!((terms[2]["position"].as_i64().unwrap(), terms[2]["value"].as_str().unwrap()), (3, "Being"));

    // Connectives — canonical edge order, with the legacy `line_{n}_{a}_{b}` ids the
    // frontend depends on, and the character values on the right edges.
    let conns = sys["connectives"].as_array().unwrap();
    let got: Vec<(&str, i64, i64, &str)> = conns
        .iter()
        .map(|c| {
            (
                c["id"].as_str().unwrap(),
                c["basePosition"].as_i64().unwrap(),
                c["targetPosition"].as_i64().unwrap(),
                c["characterValue"].as_str().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        got,
        vec![
            ("line_3_1_2", 1, 2, "Generation"),
            ("line_3_1_3", 1, 3, "Decision"),
            ("line_3_2_3", 2, 3, "Consent"),
        ]
    );
}

#[tokio::test]
async fn substrate_composes_higher_orders() {
    // The substrate composes any K_n, not just the triad: the hexad has 6 terms and
    // C(6,2)=15 connectives, with canonical edge ids.
    let schema = make_schema();
    let q = r#"query { system(order: 6) { terms { position } connectives { id targetPosition } } }"#;
    let resp = schema.execute(q).await;
    assert!(resp.errors.is_empty(), "errors: {:?}", resp.errors);
    let d = resp.data.into_json().unwrap();
    assert_eq!(d["system"]["terms"].as_array().unwrap().len(), 6);
    let conns = d["system"]["connectives"].as_array().unwrap();
    assert_eq!(conns.len(), 15);
    assert_eq!(conns[0]["id"].as_str().unwrap(), "line_6_1_2");
}
