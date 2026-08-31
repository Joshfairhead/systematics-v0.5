//! The **ingest bridge** — the substrate as a DATA SOURCE. Load the graph (from the
//! legacy JSON), ingest every character into the substrate store as a raw,
//! content-addressed element, then compose a real seeded system **from the store** —
//! its term/connective values sourced from the substrate, not from the JSON.

use systematics_backend::core::substrate::{
    compose_system_from_store, ingest_from_graph, SubstrateStore,
};
use systematics_backend::data;

#[test]
fn ingest_graph_then_compose_the_triad_from_the_store() {
    let graph = data::build_graph();
    let store = ingest_from_graph(&graph);

    // Characters are in the store, content-addressed (id = "{kind}:{value}").
    assert!(store.get(&SubstrateStore::content_id("word", "Will")).is_some());
    assert!(store.get(&SubstrateStore::content_id("word", "Generation")).is_some());
    assert!(!store.elements.is_empty());
    // Content-addressing DEDUPS: one element per distinct (kind, value), fewer than the
    // total character entries (e.g. the four "Needs Research" placeholders collapse).
    let n_chars = graph.characters(None).len();
    let distinct: std::collections::HashSet<(String, String)> = graph
        .characters(None)
        .iter()
        .map(|c| (c.kind.clone(), c.value.clone()))
        .collect();
    assert_eq!(store.elements.len(), distinct.len(), "one element per distinct content");
    assert!(store.elements.len() < n_chars, "content-addressing deduped shared values");

    // Compose the canonical triad FROM the store — values come from the substrate.
    let hg = compose_system_from_store(&graph, &store, "system_canonical_triad_3")
        .expect("canonical triad composes from the ingested store");
    assert_eq!(hg.topology.elements.len(), 3);
    assert_eq!(hg.topology.links.len(), 3);
    assert_eq!(hg.data.iter().find(|d| d.id == "term_3_1").unwrap().character, "Will");
    assert_eq!(hg.data.iter().find(|d| d.id == "term_3_3").unwrap().character, "Being");
    assert_eq!(hg.data.iter().find(|d| d.id == "conn_3_1_2").unwrap().character, "Generation");
}
