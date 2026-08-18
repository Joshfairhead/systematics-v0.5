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

    // DU1 now owns a single system — its **Coherence Dodecad** (the 12 categories)
    // — and references the canonical systems by address (it was re-declared onto
    // canonical, dropping its 12 empty per-order shells). The one owned system must
    // still survive re-export (the bundled/canonical split guarantee).
    assert_eq!(
        exported.systems.len(),
        1,
        "DU1 must re-export its own Coherence Dodecad, got {}",
        exported.systems.len()
    );
    assert!(
        exported
            .systems
            .iter()
            .any(|s| s.id == "system_du1_coherence_dodecad_12"),
        "the DU1 Coherence Dodecad must survive re-export"
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

#[test]
fn du1_module_round_trips_losslessly() {
    // Export DU1 from the fully assembled graph, apply it onto a fresh seed-only
    // graph, then re-export: the module must be byte-for-byte identical. This is
    // the durability guarantee — a module written to disk reloads to itself.
    let assembled = assembled_graph();
    let exported = assembled.export_perspective("perspective_dramatic_universe_vol_1");

    // DU1 now composes by address (like DU3): it references the canonical systems,
    // so its manifest records those canonical deps rather than being empty.
    assert!(
        exported
            .manifest
            .contains(&"system:system_canonical_heptad_7".to_string()),
        "DU1 now cites canonical systems; manifest must record them, got {:?}",
        exported.manifest
    );

    let mut fresh = data::build_graph();
    fresh.apply_content(&exported);
    let re_exported = fresh.export_perspective("perspective_dramatic_universe_vol_1");

    assert_eq!(
        exported, re_exported,
        "DU1 module must round-trip identically through apply_content"
    );
}

#[test]
fn referential_module_records_canonical_deps_in_manifest() {
    let graph = assembled_graph();
    let exported = graph.export_perspective("perspective_dramatic_universe_vol_3");

    // DU3 owns no systems; every system it cites is canonical, so each must be
    // recorded in the manifest as an external address (composition by address).
    assert!(exported.systems.is_empty(), "DU3 owns no systems");
    assert!(
        exported
            .manifest
            .contains(&"system:system_canonical_tetrad_4".to_string()),
        "DU3's manifest must record the canonical tetrad it cites, got {:?}",
        exported.manifest
    );
    assert!(
        exported.manifest.iter().all(|a| a.starts_with("system:")),
        "DU3 only depends on canonical systems by address, got {:?}",
        exported.manifest
    );
}

#[test]
fn perspective_of_perspectives_lists_children_in_manifest() {
    // The Qualsystems course composes purely via `hasModule` links to sibling
    // perspectives. Export must preserve the links AND record the 8 children as
    // external dependencies — not inline them (referential, no copy).
    let graph = assembled_graph();
    let exported = graph.export_perspective("perspective_qualsystems_course");

    // Only the course perspective itself is exported, with its 8 links intact.
    assert_eq!(exported.perspectives.len(), 1, "children are not inlined");
    let course = &exported.perspectives[0];
    assert_eq!(course.links.len(), 8, "the 8 hasModule links are preserved");

    let child_deps: Vec<&String> = exported
        .manifest
        .iter()
        .filter(|a| a.starts_with("perspective:"))
        .collect();
    assert_eq!(
        child_deps.len(),
        8,
        "all 8 child perspectives must be recorded as manifest dependencies, got {:?}",
        exported.manifest
    );
    assert!(
        exported
            .manifest
            .contains(&"perspective:perspective_qsm1_number_qualities_perception".to_string()),
        "manifest must name the first module by address"
    );
}

#[test]
fn manifest_serializes_additively() {
    // The manifest is an additive field, omitted when empty (serde
    // skip_serializing_if). This keeps the 14 pre-existing module files and full
    // snapshots byte-compatible, and is exactly what the exportPerspective
    // GraphQL query returns.
    let graph = assembled_graph();

    // A self-contained module (owns everything it cites) has an empty manifest,
    // which must be omitted from JSON. The Architecture Pentad owns + cites only
    // its own pentad, so it is such a module. (DU1 no longer is — it now cites
    // canonical by address.)
    let pentad = graph.export_perspective("perspective_architecture_pentad");
    let pentad_json = serde_json::to_string(&pentad).unwrap();
    assert!(
        pentad.manifest.is_empty(),
        "the Architecture Pentad owns everything it cites, got {:?}",
        pentad.manifest
    );
    assert!(
        !pentad_json.contains("manifest"),
        "an empty manifest must be omitted from JSON, so old files stay identical"
    );

    let du3 = graph.export_perspective("perspective_dramatic_universe_vol_3");
    let du3_json = serde_json::to_string(&du3).unwrap();
    assert!(
        du3_json.contains("\"manifest\""),
        "a non-empty manifest must serialize"
    );

    // And it survives a serde round-trip.
    let reparsed: systematics_backend::core::GraphContent =
        serde_json::from_str(&du3_json).unwrap();
    assert_eq!(reparsed.manifest, du3.manifest);
}
