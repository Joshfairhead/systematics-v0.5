//! Export behaviour tests (composable-perspectives plan, Steps 2–3).
//!
//! Step 2 splits `bundled_ids` (durable, kept out of the user store) from
//! `canonical_ids` (the immutable seed archetypes that `export_perspective` must
//! not copy). The regression these guard against: before the split, loading a
//! module and re-marking it *canonical* meant a subsequent `export_perspective`
//! excluded the module's own systems/characters — a lossy round-trip. After the
//! split, a bundled-but-not-canonical module re-exports its own entities, while
//! shared canonical entities are still referenced by address, never copied.

use systematics_backend::core::Graph;
use systematics_backend::data;

fn assembled_graph() -> Graph {
    let mut graph = data::build_graph();
    let modules = data::load_perspective_modules(&mut graph);
    if modules > 0 {
        graph.mark_bundled();
    }
    graph
}

#[test]
fn bundled_module_reexports_its_own_systems() {
    let graph = assembled_graph();

    let exported = graph.export_perspective("perspective_dramatic_universe_vol_1");

    // DU1 owns 12 systemic-attribute systems. Before the bundled/canonical split
    // these were dropped on re-export (module ids had been marked canonical).
    assert_eq!(
        exported.systems.len(),
        12,
        "DU1 must re-export its own 12 systems, got {}",
        exported.systems.len()
    );
    assert!(
        exported
            .systems
            .iter()
            .any(|s| s.id == "system_dramatic_universe_i_triad_3"),
        "the DU1 triad system must survive re-export"
    );
}

#[test]
fn referential_module_copies_no_canonical_system() {
    let graph = assembled_graph();

    // DU3 composes purely by address: it cites the canonical systems but owns
    // none, so its export must not drag a copy of any canonical system into the
    // module file.
    let exported = graph.export_perspective("perspective_dramatic_universe_vol_3");

    assert!(
        exported.systems.is_empty(),
        "DU3 owns no systems; export must copy none, got {:?}",
        exported.systems.iter().map(|s| &s.id).collect::<Vec<_>>()
    );
    assert!(
        !exported
            .systems
            .iter()
            .any(|s| s.id == "system_canonical_tetrad_4"),
        "the shared canonical tetrad must never be copied into a module"
    );
}
