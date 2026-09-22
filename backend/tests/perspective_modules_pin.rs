//! Pin test — locks in the current perspective-module load behaviour BEFORE the
//! composable-perspectives refactor (docs/plans/composable-perspectives.md,
//! Step 1). Nothing in that plan moves until this is green, and it re-runs after
//! every subsequent step.
//!
//! It reproduces `main.rs::build_api_router` exactly — `build_graph()` →
//! `load_perspective_modules()` → second `mark_canonical()` — and asserts:
//!   * the 16 committed module files load,
//!   * all 59 references resolve to an existing System (the target's system id
//!     is present in the graph), and
//!   * the 3 module-owned systems are all present.
//!
//! Any drift here is a regression in module loading or referential composition.
//!
//! `load_perspective_modules` resolves modules from the source tree via the
//! compile-time `CARGO_MANIFEST_DIR` (cwd-independent) in dev/CI, and from the
//! copy embedded via `include_dir!` when deployed — so this test does not depend
//! on the process working directory (defect #5, fixed in Step 4).

use systematics_backend::core::Graph;
use systematics_backend::data;

/// The 3 systems that live in (are owned by) the perspective module files:
/// DU1's single **Coherence Dodecad** (its 12 categories, was 12 empty shells),
/// the Elementary Systematics triad, and the self-describing Architecture Pentad.
/// DU1 and DU2 both now compose by address (references onto canonical) like DU3 —
/// so DU2's 3 shells are gone. Canonical (12) and citation (1) are seeded separately.
const MODULE_OWNED_SYSTEMS: &[&str] = &[
    "system_du1_coherence_dodecad_12",
    "system_elementary_systematics_triad_3",
    "system_architecture_pentad_5",
];

/// Reproduce the startup sequence from `main.rs::build_api_router`, minus the
/// writable user store (absent in CI, and irrelevant to module loading).
fn assembled_graph() -> (Graph, usize) {
    let mut graph = data::build_graph();
    let modules = data::load_perspective_modules(&mut graph);
    if modules > 0 {
        // Modules are durable (kept out of the user store) but not canonical
        // archetypes — mirrors main.rs::build_api_router.
        graph.mark_bundled();
    }
    (graph, modules)
}

#[test]
fn all_modules_load() {
    let (graph, modules) = assembled_graph();
    assert_eq!(
        modules, 21,
        "expected 21 module files to load (14 sources + the Architecture Pentad \
         + the Architectural Monad, Blue Earth, Interface State Model, and Blue Earth \
         Ventures registries + the Plato and Bennett sequences); got {modules}"
    );
    assert_eq!(
        graph.perspectives().len(),
        15,
        "15 perspectives from 19 modules — the Architectural Monad, Blue Earth, Interface \
         State Model and Blue Earth Ventures modules carry only a Sequence, no perspective"
    );
}

#[test]
fn all_references_present_and_resolve() {
    let (graph, _) = assembled_graph();

    assert_eq!(
        graph.references.len(),
        59,
        "expected 59 references (DU1+DU2 re-declared onto canonical, carrying \
         coherence values as reference `object`s)"
    );

    // Every reference targets `system:<id>[#...]`; each must resolve to a System
    // that exists in the graph. A dangling target = broken referential composition.
    let mut unresolved = Vec::new();
    for r in &graph.references {
        let Some(rest) = r.target.strip_prefix("system:") else {
            panic!("reference {} has non-system target {}", r.id, r.target);
        };
        let sys_id = rest.split('#').next().unwrap_or(rest);
        if graph.system(sys_id).is_none() {
            unresolved.push((r.id.clone(), r.target.clone()));
        }
    }
    assert!(
        unresolved.is_empty(),
        "these references point at a system that isn't loaded: {unresolved:?}"
    );
}

#[test]
fn module_owned_systems_present() {
    let (graph, _) = assembled_graph();

    let missing: Vec<&str> = MODULE_OWNED_SYSTEMS
        .iter()
        .copied()
        .filter(|id| graph.system(id).is_none())
        .collect();
    assert!(
        missing.is_empty(),
        "module-owned systems missing after load: {missing:?}"
    );

    // Total systems = 12 canonical + 1 citation + 88 fragments + 3 module-owned = 104.
    // Fragments grew 39 → 55 (2026-09-01): the Elementary Systematics pentad, Potency,
    // two Levels-of-Energy dodecads (Hodgson + Bennett), Work, Workspace, the awareness
    // Monad, the Dodecads pentad, Tetrad (Hodgson), Society, Values, the four Holochain-
    // architecture systems (Holochain Zomes, Integrity/Coordinator Zome, Link), and the
    // Graph Products tetrad (Cartesian/Tensor/Strong/Lexicographic = ground/instrument/
    // directive/goal). Then 55 → 61 (2026-09-07): the Knowledge, Discourse, Common Noun
    // and Verbs triads, and the two registry tetrads (Field of Action = Store/Read/Graph/
    // List; Registry Core = Store/Read/Sort/Filter). Then 61 → 74 (2026-09-08): the
    // Import/Export, Store/Load and Semantics/Syntax dyads; the Blue Earth monad, its
    // Hosting/Facilitation, Niche/National and Deals/Deployment dyads, Delivery and Impact
    // triads, and Blue Earth Strategy tetrad; and the Fitness/Exercise and Stretching/
    // Mobility dyads plus the Physical Fitness pentad. Then 74 → 76 (2026-09-09): the
    // Interface State Model monad and the Viewing/Editing dyad. Then 76 → 77 (2026-09-09):
    // the CRUD tetrad (Create/Delete/Update/Read, order 1–4), the state model's tetrad.
    // Then 77 → 84 (2026-09-10): the Material Science, Information Science and FAIR tetrads,
    // the Convening and Proof of Stake triads, the Semiotics triad (the Interface State
    // Model's), and the Assembly/Decomposition dyad. Then 84 → 88 (2026-09-11): the Blue
    // Earth Ventures monad, its World/Businesses dyad, Backing triad, and Venture Ecosystem
    // tetrad.
    // Then 104 → 105 (2026-09-16): the Architectural Monad's own K1 monad head
    // (system_architectural_monad_1), so the sequence has a monad = its name.
    // Then 105 → 110 (2026-09-17): the Plato sequence — its monad head + 4 triads
    // (Tripartite Soul, Social Classes, Three Fates, Cave Allegory).
    // Then 110 → 112 (2026-09-17): the Bennett sequence — its monad head + the Mind triad.
    // Then 112 → 113 (2026-09-17): the Browser triad (the sort/filter module as a system —
    // Query·List·View / Sort·Search·Filter) added to the Architectural Monad registry.
    assert_eq!(
        graph.systems.len(),
        113,
        "expected 12 canonical + 1 citation + 88 fragment + 12 module systems"
    );
}

#[test]
fn du3_composes_by_address_not_copy() {
    // The plan's core-model claim, pinned: the DU3 module owns 0 systems yet its
    // citations still resolve onto the canonical tetrad by address — composition
    // is referential, not by copying the canonical system into DU3's file.
    let (graph, _) = assembled_graph();

    let du3 = graph
        .perspective("perspective_dramatic_universe_vol_3")
        .expect("DU3 perspective loaded");

    let du3_tetrad_refs: Vec<_> = graph
        .references
        .iter()
        .filter(|r| r.perspective_ref == du3.id)
        .filter(|r| r.target.starts_with("system:system_canonical_tetrad_4"))
        .collect();
    assert!(
        !du3_tetrad_refs.is_empty(),
        "DU3 should cite the canonical tetrad by address"
    );

    // And the canonical tetrad itself is owned by the seed, not by DU3.
    assert!(
        graph.system("system_canonical_tetrad_4").is_some(),
        "canonical tetrad present from the seed"
    );
}

#[test]
fn architectural_monad_registry_loads() {
    // The Architectural Monad is a Sequence registry of the declared architecture
    // systems — currently just the seeded Pentad. Undeclared orders are simply
    // absent (blank in the view). Its one member must resolve.
    let (graph, _) = assembled_graph();

    let monad = graph
        .sequence("sequence_architectural_monad")
        .expect("Architectural Monad registry loaded from its module");
    // Its head is the K1 monad (named after the sequence), then the architecture systems
    // grouped for sorting (several triads). OrderCardinality does not matter here.
    assert_eq!(
        monad.members,
        vec![
            "system:system_architectural_monad_1",
            "system:system_data_2",
            "system:system_order_position_location_3",
            "system:system_citation_3",
            "system:system_identity_associativity_composition_3",
            "system:system_architecture_pentad_5",
            "system:system_architecture_octad_8",
            "system:system_graph_construction_3",
            "system:system_order_size_2",
            "system:system_vertex_edge_2",
            "system:system_term_connective_2",
            "system:system_path_distance_2",
            "system:system_graph_theory_7",
            "system:system_topology_geometry_4",
            "system:system_provenance_5",
            "system:system_browser_3",
        ]
    );
    // Seeded members resolve; the architecture **octad** is documented but not yet
    // seeded — a *dangling* address (tracked, not yet assembled into a system).
    assert!(graph.resolves("system:system_data_2"));
    assert!(graph.resolves("system:system_architecture_pentad_5"));
    assert!(!graph.resolves("system:system_architecture_octad_8"));
    // The graph-theory / topology scaffold (seeded fragments) resolves too.
    assert!(graph.resolves("system:system_graph_construction_3"));
    assert!(graph.resolves("system:system_graph_theory_7"));
    assert!(graph.resolves("system:system_topology_geometry_4"));
    assert!(graph.resolves("system:system_provenance_5"));
}
