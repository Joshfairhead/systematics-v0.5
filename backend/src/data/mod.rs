//! Seed the property graph.
//!
//! Emits substrate entries (Order, Position, Point, Line, Coordinate, Segment,
//! Character) followed by the canonical Topological / Geometric / Semantic
//! Vocabularies and one Canonical Perspective per Order.

use crate::core::{
    Entry, Geometry, Template, GraphContent, Graph, Line, Order, Point, Position,
    Segment, Topology,
};

/// The canonical seed content (coordinates, characters, semantic vocabularies,
/// perspectives), bundled as JSON. Source of truth for canonical data; the Rust
/// tables that generated it live under `#[cfg(test)] mod regen`.
const CANONICAL_JSON: &str = include_str!("../../../data/canonical.json");

/// Parse the embedded canonical seed content.
pub fn canonical_content() -> GraphContent {
    serde_json::from_str(CANONICAL_JSON).expect("data/canonical.json is valid GraphContent")
}

/// The Citation triad seed — a separate bundled module (Source / Artefact /
/// Lookup as a K3). Non-canonical in spirit (not one of the 12 archetypes) but
/// ships bundled. Regenerate with the ignored `regenerate_citation_seed` test.
const CITATION_JSON: &str = include_str!("../../../data/citation.json");

/// Parse the embedded Citation triad seed content.
pub fn citation_content() -> GraphContent {
    serde_json::from_str(CITATION_JSON).expect("data/citation.json is valid GraphContent")
}

/// The **fragments** seed — author-contributed triads + the determining-conditions
/// tetrad (see `docs/fragments.md`). Tracked raw material, seeded so it resolves.
const FRAGMENTS_JSON: &str = include_str!("../../../data/fragments.json");

/// Parse the embedded fragments seed content.
pub fn fragments_content() -> GraphContent {
    serde_json::from_str(FRAGMENTS_JSON).expect("data/fragments.json is valid GraphContent")
}

/// The perspective module files, embedded into the binary at build time (like
/// the canonical/citation seed). This is what a deployed build serves — no
/// runtime filesystem, volume or working-directory assumption required.
static EMBEDDED_MODULES: include_dir::Dir<'static> =
    include_dir::include_dir!("$CARGO_MANIFEST_DIR/data/perspectives");

/// Load every perspective module (`*.json`) onto the graph. Each file is an
/// exported Perspective bundle — a durable, modular alternative to the single
/// ephemeral user store; one source = one file. Follow with `mark_bundled` so
/// the modules are kept out of the user store yet stay editable / re-exportable
/// (unlike the canonical seed, which is marked with `mark_canonical`).
///
/// Source resolution, in order:
///   1. `$SYSTEMATICS_DATA/perspectives` if that env var is set,
///   2. the source tree (`$CARGO_MANIFEST_DIR/data/perspectives`) — present when
///      the binary runs on its build machine (local dev, `cargo test`, CI), so
///      edits are picked up live without a rebuild,
///   3. otherwise the copy embedded at build time — the deployed path.
///
/// Returns the number of modules applied.
pub fn load_perspective_modules(graph: &mut Graph) -> usize {
    let fs_dir = std::env::var_os("SYSTEMATICS_DATA")
        .map(|root| std::path::PathBuf::from(root).join("perspectives"))
        .unwrap_or_else(|| {
            std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/data/perspectives"))
        });

    match std::fs::read_dir(&fs_dir) {
        Ok(rd) => {
            let mut paths: Vec<std::path::PathBuf> = rd
                .filter_map(|e| e.ok().map(|e| e.path()))
                .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("json"))
                .collect();
            paths.sort();
            let mut loaded = 0;
            for p in paths {
                match std::fs::read_to_string(&p)
                    .ok()
                    .and_then(|s| serde_json::from_str::<GraphContent>(&s).ok())
                {
                    Some(content) => {
                        graph.apply_content(&content);
                        loaded += 1;
                    }
                    None => eprintln!("skipped invalid perspective module {}", p.display()),
                }
            }
            loaded
        }
        // Deployed build: no source tree on disk — use the embedded copy.
        Err(_) => load_embedded_modules(graph),
    }
}

/// Apply the perspective modules embedded into the binary at build time.
fn load_embedded_modules(graph: &mut Graph) -> usize {
    let mut files: Vec<&include_dir::File> = EMBEDDED_MODULES
        .files()
        .filter(|f| f.path().extension().and_then(|x| x.to_str()) == Some("json"))
        .collect();
    files.sort_by(|a, b| a.path().cmp(b.path()));
    let mut loaded = 0;
    for f in files {
        match f
            .contents_utf8()
            .and_then(|s| serde_json::from_str::<GraphContent>(s).ok())
        {
            Some(content) => {
                graph.apply_content(&content);
                loaded += 1;
            }
            None => eprintln!("skipped invalid embedded perspective module {}", f.path().display()),
        }
    }
    loaded
}

/// Build the complete graph with all systems (1-12).
///
/// The combinatoric substrate (Order, Position, Point, Line, Segment, and the
/// topological/geometric vocabulary ref-lists) is computed here; the data layer
/// (coordinates, characters, semantic vocabularies, perspectives) is applied from
/// the canonical seed and then marked canonical so later user additions can be
/// told apart for persistence.
pub fn build_graph() -> Graph {
    let mut graph = Graph::new();

    add_orders(&mut graph);
    add_positions(&mut graph);
    add_substrate_combinatorics(&mut graph);

    graph.apply_content(&canonical_content());
    // The Citation triad ships as its own bundle, applied before marking
    // canonical so it counts as bundled (not user) content.
    graph.apply_content(&citation_content());
    // The fragments (author triads + determining-conditions tetrad) ship the same
    // way — durable seed, resolvable in the graph (see docs/fragments.md).
    graph.apply_content(&fragments_content());
    graph.mark_canonical();

    graph
}

// =============================================================================
// Substrate creation
// =============================================================================

fn add_orders(graph: &mut Graph) {
    for i in 1..=12u8 {
        graph.add_entry(Entry::Order(Order::new(i)));
    }
}

fn add_positions(graph: &mut Graph) {
    for i in 1..=12u8 {
        graph.add_entry(Entry::Position(Position::new(i)));
    }
}

/// Emit the combinatoric substrate: Points, Lines, Segments, and the
/// topological/geometric vocabulary ref-lists. All derivable from the Order,
/// so this stays in code. Coordinates (the geometric *values*) are data and
/// come from the canonical seed.
fn add_substrate_combinatorics(graph: &mut Graph) {
    for order in 1..=12u8 {
        for position in 1..=order {
            graph.add_entry(Entry::Point(Point::new(order, position)));
        }
        for p1 in 1..=order {
            for p2 in (p1 + 1)..=order {
                graph.add_entry(Entry::Line(Line::new(order, p1, p2)));
            }
        }
        for p1 in 1..=order {
            for p2 in (p1 + 1)..=order {
                graph.add_entry(Entry::Segment(Segment::new(order, p1, p2)));
            }
        }
        graph.add_topological_vocab(Topology::canonical_for(order));
        graph.add_geometric_vocab(Geometry::canonical_for(order));
        // The complete-graph structure for this Order (deterministic).
        graph.add_template(Template::for_order(order));
    }
}


// =============================================================================
// Canonical content generator + data tables (test-only).
//
// These build `data/canonical.json`. At runtime the JSON is loaded via
// `canonical_content()`; the tables compile only under `#[cfg(test)]`.
// =============================================================================

#[cfg(test)]
mod regen {
    use crate::core::{Character, Coordinate, GraphContent, Point3d, Vocabulary, System};

/// Build the canonical data layer from the Rust tables. This is the generator
/// behind `data/canonical.json`; at runtime the JSON is loaded instead.
pub fn build_canonical_from_tables() -> GraphContent {
    let mut content = GraphContent::default();
    let mut have_char = std::collections::HashSet::new();

    for order in 1..=12u8 {
        // Coordinates (the geometric values).
        for (idx, coord) in get_coordinates(order).iter().enumerate() {
            content
                .coordinates
                .push(Coordinate::from_point3d(order, (idx + 1) as u8, *coord));
        }

        // Word triad: term/connective Characters + Vocabulary + System.
        push_triadic_system(
            &mut content,
            &mut have_char,
            &format!("Canonical {}", canonical_system_name(order)),
            order,
            &get_term_character_slugs(order),
            &get_canonical_connective_slugs(order),
        );

        // Hex colour characters + the canonical colour Vocabulary.
        let hex_codes = get_colours(order);
        let mut colour_ids = Vec::new();
        for hex in &hex_codes {
            let id = format!("char_hex_{}", hex.trim_start_matches('#').to_lowercase());
            if have_char.insert(id.clone()) {
                content
                    .characters
                    .push(Character::new(id.clone(), "hex", hex.to_string()));
            }
            colour_ids.push(id);
        }
        content.vocabularies.push(Vocabulary::with_auto_id(
            format!("Canonical Colours {}", canonical_system_name(order)),
            order,
            colour_ids,
            vec![],
        ));
    }

    content
}

/// Shared construction primitive: push term/connective word Characters, then a
/// `Vocabulary` and a `System` over `grammar_{order}`, with metadata inherited
/// from the order. Reused for every canonical order AND the Citation triad — the
/// homoiconic bootstrap: the same builder that makes any system makes the system
/// that *describes* citation.
fn push_triadic_system(
    content: &mut GraphContent,
    have_char: &mut std::collections::HashSet<String>,
    name: &str,
    order: u8,
    term_slugs: &[String],
    connective_slugs: &[String],
) {
    for slug in term_slugs.iter().chain(connective_slugs.iter()) {
        let id = format!("char_word_{}", slug);
        if have_char.insert(id.clone()) {
            content
                .characters
                .push(Character::new(id, "word", word_from_slug(slug)));
        }
    }
    let term_char_ids: Vec<String> = term_slugs.iter().map(|s| format!("char_word_{}", s)).collect();
    let connective_char_ids: Vec<String> = connective_slugs
        .iter()
        .map(|s| format!("char_word_{}", s))
        .collect();
    let vocab = Vocabulary::with_auto_id(name, order, term_char_ids, connective_char_ids);
    let vocab_id = vocab.id.clone();
    content.vocabularies.push(vocab);
    content.systems.push(System::with_auto_id(
        name,
        order,
        canonical_coherence(order),
        canonical_term_designation(order),
        canonical_connective_designation(order),
        format!("grammar_{}", order),
        &vocab_id,
    ));
}

/// Build the Citation triad as its own content bundle (`data/citation.json`):
/// **Source / Artefact / Lookup** as the three impulses of a K3, with the
/// **citation-query** edges (user, 2026-08-18): the three connectives are
/// **Search · Sort · Filter**, in canonical edge order over nodes
/// 1=source · 2=artefact · 3=lookup — (1,2) source–artefact = **Sort**,
/// (1,3) source–lookup = **Search**, (2,3) artefact–lookup = **Filter**.
/// Metadata inherits order 3 (Dynamism / Impulses / Acts).
pub fn build_citation_from_tables() -> GraphContent {
    let mut content = GraphContent::default();
    let mut have_char = std::collections::HashSet::new();
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Citation",
        3,
        &["source".to_string(), "artefact".to_string(), "lookup".to_string()],
        &["sort".to_string(), "search".to_string(), "filter".to_string()],
    );
    content
}

/// Small helper: `Vec<String>` from string literals, for readability below.
fn slugs(xs: &[&str]) -> Vec<String> {
    xs.iter().map(|s| s.to_string()).collect()
}

/// `C(order, 2)` placeholder edge slugs for a `K_order` whose connectives aren't
/// yet named (used by the architecture/graph-theory scaffold systems).
fn edge_slugs(prefix: &str, order: usize) -> Vec<String> {
    let count = order * order.saturating_sub(1) / 2;
    (1..=count).map(|i| format!("{prefix}_edge_{i}")).collect()
}

/// Build the author-contributed and systematics-core **fragments** as their own
/// bundle (`data/fragments.json`). Source of the author triads: **Josh Fairhead**
/// (see `docs/fragments.md`). Each is a real K_n system (terms + named/placeholder
/// connectives) so it resolves in the graph; metadata inherits the canonical order.
///
/// Maths triad term positions (theorems=1, lemmas=2, proofs=3) fix the named edges
/// onto the K3 lines (1,2)(1,3)(2,3): bili · φ · Fibonacci. The Determining-
/// Conditions tetrad is K4 (4 dimensions, 6 laws as its 6 edges — assignment TBD).
pub fn build_fragments_from_tables() -> GraphContent {
    let mut content = GraphContent::default();
    let mut have_char = std::collections::HashSet::new();

    // Impulse → position convention: **pos1 = + (affirming), pos2 = − (denying),
    // pos3 = = (reconciling)** — fixed by the confirmed-correct Maths sub-triad.

    // Root triad: Aesthetics (+) · Maths (−) · Harmony (=).
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Aesthetics Maths Harmony",
        3,
        &slugs(&["aesthetics", "maths", "harmony"]),
        &slugs(&["ahm_edge_1", "ahm_edge_2", "ahm_edge_3"]),
    );
    // Aesthetics → Unity (+) · Variety (−) · Form (=).
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Aesthetics",
        3,
        &slugs(&["unity", "variety", "form"]),
        &slugs(&["aesthetics_edge_1", "aesthetics_edge_2", "aesthetics_edge_3"]),
    );
    // Harmony → Inevitability (+) · Economy (−) · Proportion (=).
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Harmony",
        3,
        &slugs(&["inevitability", "economy", "proportion"]),
        &slugs(&["harmony_edge_1", "harmony_edge_2", "harmony_edge_3"]),
    );
    // Maths → Theorems (+) · Lemmas (−) · Proofs (=); named edges on lines
    // (1,2)=bili, (1,3)=φ, (2,3)=Fibonacci.
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Maths",
        3,
        &slugs(&["theorems", "lemmas", "proofs"]),
        &slugs(&["bili", "phi", "fibonacci"]),
    );
    // Life triad (not architectural; perspective/tag = "life") — Health (+) ·
    // Wealth (−) · Wisdom (=). Source: Josh Fairhead.
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Life",
        3,
        &slugs(&["health", "wealth", "wisdom"]),
        &slugs(&["life_edge_1", "life_edge_2", "life_edge_3"]),
    );
    // Determining Conditions of science — a K4 tetrad: Time (Chronos) · Hyparxis ·
    // Eternity (Aionios) · Space; the 6 laws as its 6 edges. (Positioning WIP.)
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Determining Conditions",
        4,
        &slugs(&["time", "hyparxis", "eternity", "space"]),
        &slugs(&[
            "statistical",
            "correspondence",
            "classification",
            "conservation",
            "irreversibility",
            "coexistence",
        ]),
    );

    // ---- Architecture-project WIP fragments (documented in-graph) ----

    // AD4M Perspective tetrad [WIP]: Perspective · Subject · Predicate · Object.
    push_triadic_system(
        &mut content,
        &mut have_char,
        "AD4M Perspective",
        4,
        &slugs(&["perspective", "subject", "predicate", "object"]),
        &slugs(&["ad4m_edge_1", "ad4m_edge_2", "ad4m_edge_3", "ad4m_edge_4", "ad4m_edge_5", "ad4m_edge_6"]),
    );
    // Perspective triad [WIP] — subject · predicate · object are the three EDGES
    // (a triple's roles); the three nodes are TBD (perspective parts).
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Perspective",
        3,
        &slugs(&["perspective_node_1", "perspective_node_2", "perspective_node_3"]),
        &slugs(&["subject", "predicate", "object"]),
    );
    // Order · Position · Location triad [WIP] — the topological anchor. `order ×
    // position = location`; most triples attach to a location (e.g. Location ·
    // Term-character · Source; Lines = Location · Connection · Location). May fold
    // into a pentad.
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Order Position Location",
        3,
        &slugs(&["order", "position", "location"]),
        &slugs(&["opl_edge_1", "opl_edge_2", "opl_edge_3"]),
    );
    // Data Object triad [WIP] — the reference triple: key (+) · value (−) ·
    // reference (=). A key holds many values, each independently referenced (the
    // basis of #25/#28). Likely a Perspective once sequenced +=− / 132 interaction.
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Data Object",
        3,
        &slugs(&["key", "value", "reference"]),
        &slugs(&["do_edge_1", "do_edge_2", "do_edge_3"]),
    );
    // Class · Instantiation · Instance triad [WIP] — e.g. K₄ (class) · Tetrad
    // (instantiation) · Canonical Tetrad (instance).
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Class Instantiation Instance",
        3,
        &slugs(&["class", "instantiation", "instance"]),
        &slugs(&["cii_edge_1", "cii_edge_2", "cii_edge_3"]),
    );
    // Operations triad [WIP] — ELT: Extract · Load · Transform as the three NODES;
    // edges TBD. ELT ≅ RAG (data-warehouse ↔ generative-AI): the same isomorphic
    // triangle; both are *composition*. The 3!=6 permutations = the six laws of three.
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Operations",
        3,
        &slugs(&["extract", "load", "transform"]),
        &slugs(&["op_edge_1", "op_edge_2", "op_edge_3"]),
    );
    // Composition triad [WIP] — RAG: Retrieve · Augment · Generate as the nodes;
    // the generative-AI isomorph of ELT (Operations). Both are composition.
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Composition",
        3,
        &slugs(&["retrieve", "augment", "generate"]),
        &slugs(&["comp_edge_1", "comp_edge_2", "comp_edge_3"]),
    );
    // The Data progression (monad → dyad → triad) [WIP]. The monad "Data" (order 1);
    // the dyad (key · value) is ALSO called "Data" (data = a key:value tag) and
    // links to the Data monad. (Two "Data" systems, orders 1 and 2.)
    push_triadic_system(&mut content, &mut have_char, "Data", 1, &slugs(&["data"]), &slugs(&[]));
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Data",
        2,
        &slugs(&["key", "value"]),
        &slugs(&["key_value_edge"]),
    );

    // The Data progression as a Sequence: monad → dyad → triad (Data → Data →
    // ELT). Links the Data monad and dyad (in-graph navigation is design-TBD).
    content.sequences.push(crate::core::Sequence::new(
        "sequence_data_progression",
        "Data",
        vec![
            "system:system_data_1".to_string(),
            "system:system_data_2".to_string(),
            "system:system_operations_3".to_string(),
        ],
    ));

    // The CT monad as a sequence: Monad (1) → {Container · Operations} dyad (2) →
    // {Identity · Associativity · Composition} triad (3). A monad needs structure
    // (container) AND process (operations), governed by the three CT axioms.
    push_triadic_system(&mut content, &mut have_char, "Monad", 1, &slugs(&["monad"]), &slugs(&[]));
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Container Operations",
        2,
        &slugs(&["container", "operations"]),
        &slugs(&["container_operations_edge"]),
    );
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Identity Associativity Composition",
        3,
        &slugs(&["identity", "associativity", "composition"]),
        &slugs(&["iac_edge_1", "iac_edge_2", "iac_edge_3"]),
    );
    content.sequences.push(crate::core::Sequence::new(
        "sequence_ct_monad",
        "Monad (CT)",
        vec![
            "system:system_monad_1".to_string(),
            "system:system_container_operations_2".to_string(),
            "system:system_identity_associativity_composition_3".to_string(),
        ],
    ));

    // ---- Author-contributed triads (perspective/tag, not architectural) ----

    // Requests triad [Josh Fairhead] — Ask (+) · Context (−) · Urgency (=): the
    // shape of a request (what is asked, in what context, at what urgency).
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Requests",
        3,
        &slugs(&["ask", "context", "urgency"]),
        &slugs(&["requests_edge_1", "requests_edge_2", "requests_edge_3"]),
    );
    // Identification triad [Ouspensky, The Fourth Way] — Like (+) · Dislike (−) ·
    // Identification (=): like and dislike are both identification.
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Identification",
        3,
        &slugs(&["like", "dislike", "identification"]),
        &slugs(&["identification_edge_1", "identification_edge_2", "identification_edge_3"]),
    );
    // Negative-emotion triad [Ouspensky, The Fourth Way] — Negative emotion (+) ·
    // Identification (−) · Imagination (=). Shares the `identification` character
    // with the Identification triad (same term, reused across systems).
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Negative Emotion",
        3,
        &slugs(&["negative_emotion", "identification", "imagination"]),
        &slugs(&["ne_edge_1", "ne_edge_2", "ne_edge_3"]),
    );

    // ---- Graph-theory / topology scaffold (user, 2026-08-13) [architecture] ----
    // The construction triad — how we build a system for examining property graphs:
    // Semantic Projection (+) · Structural Topology (−) · Graph Template (=). Alt
    // graph-theory names: Incidence graph (+) · Adjacency graph (−) · Line graph (=).
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Graph Construction",
        3,
        &slugs(&["semantic_projection", "structural_topology", "graph_template"]),
        &edge_slugs("gc", 3),
    );
    // The dyad on each construction node: what that impulse defines.
    // Graph Template (=) → order · size.
    push_triadic_system(&mut content, &mut have_char, "Order Size", 2, &slugs(&["order", "size"]), &edge_slugs("os", 2));
    // Structural Topology (−) → vertex · edge (the adjacency pair).
    push_triadic_system(&mut content, &mut have_char, "Vertex Edge", 2, &slugs(&["vertex", "edge"]), &edge_slugs("ve", 2));
    // Semantic Projection (+) → term · connective (the characters).
    push_triadic_system(&mut content, &mut have_char, "Term Connective", 2, &slugs(&["term", "connective"]), &edge_slugs("tc", 2));
    // Navigation dyad [architecture / graph-theory monad]: path · distance.
    push_triadic_system(&mut content, &mut have_char, "Path Distance", 2, &slugs(&["path", "distance"]), &edge_slugs("pd", 2));
    // Graph-theory holding heptad — terms to sort later (order=#nodes, size=#edges).
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Graph Theory",
        7,
        &slugs(&["order", "degree", "index", "adjacency", "neighbourhood", "location", "position"]),
        &edge_slugs("gt", 7),
    );
    // Topology · Geometry · Position · Location tetrad — the topology/geometry split
    // (position = combinatorial/topological adjacency; location = geometric).
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Topology Geometry",
        4,
        &slugs(&["topology", "geometry", "position", "location"]),
        &edge_slugs("tg", 4),
    );
    // Provenance pentad (generalises the citation triad): Source · Distributer ·
    // Expression · Artefact · Lookup (user swapped artefact/expression order).
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Provenance",
        5,
        &slugs(&["source", "distributer", "expression", "artefact", "lookup"]),
        &edge_slugs("prov", 5),
    );

    // The six laws of three as a Hexad — the Controller laid out as DATA (the laws
    // named as nodes). Terms in Hexad-position order (1 identity · 2 expansion ·
    // 3 order · 4 freedom · 5 interaction[SPO] · 6 concentration), matching
    // `core::laws::Law::HEXAD`. Edges = the S₃ group relations between laws, TBD →
    // placeholder for now. The morphism logic (permutation/read/compose) lives in
    // `core/laws.rs`; this seed is the in-graph, addressable form of the same six.
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Six Laws of Three",
        6,
        &slugs(&[
            "identity",
            "expansion",
            "order",
            "freedom",
            "interaction",
            "concentration",
        ]),
        &edge_slugs("law", 6),
    );

    // Harness triad — what a harness *is* (the Controller-as-harness). Impulse →
    // position: pos1 = + (tools), pos2 = − (constraints), pos3 = = (environment).
    // Edges TBD → placeholder.
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Harness",
        3,
        &slugs(&["tools", "constraints", "environment"]),
        &edge_slugs("harness", 3),
    );

    // Interface triad — how the Controller's morphisms are expressed in Rust:
    // struct (+, instance) · enum (−, kinds) · trait (=, interface/behaviour).
    // Edges TBD → placeholder. (Impulse assignment tentative.)
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Interface",
        3,
        &slugs(&["struct", "enum", "trait"]),
        &edge_slugs("interface", 3),
    );

    // Metadata dodecads (user, 2026-08-20) — the 12 values per metadata dimension made
    // explicit as order-12 systems, to fill the view's metadata sections (the first
    // stage of the systemic product). Values are the canonical metadata per order
    // (cf. `canonical_coherence` / `_term_designation` / `_connective_designation`);
    // designations 9–12 are still "needs research". Edges TBD → placeholder.
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Coherence Attributes",
        12,
        &slugs(&[
            "universality", "complementarity", "dynamism", "activity_field",
            "significance_and_potential", "coalescence", "generation", "self_sufficiency",
            "transformation", "intrinsic_harmony", "articulate_symmetry", "perfection",
        ]),
        &edge_slugs("coh", 12),
    );
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Term Designations",
        12,
        &slugs(&[
            "totality", "poles", "impulses", "sources", "limits", "laws", "states", "elements",
            "term_designation_9_needs_research", "term_designation_10_needs_research",
            "term_designation_11_needs_research", "term_designation_12_needs_research",
        ]),
        &edge_slugs("tdes", 12),
    );
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Connective Designations",
        12,
        &slugs(&[
            "unity", "force", "acts", "interplays", "mutualities", "steps", "intervals",
            "components", "connective_designation_9_needs_research",
            "connective_designation_10_needs_research", "connective_designation_11_needs_research",
            "connective_designation_12_needs_research",
        ]),
        &edge_slugs("cdes", 12),
    );

    content
}

// =============================================================================
// Canonical vocabulary data
// =============================================================================

/// Term-character values in position order for each canonical system.
fn get_term_characters(order: u8) -> Vec<&'static str> {
    match order {
        1 => vec!["Unity"],
        2 => vec!["Essence", "Existence"],
        3 => vec!["Will", "Function", "Being"],
        4 => vec!["Ideal", "Ground", "Directive", "Instrumental"],
        5 => vec![
            "Quintessence",
            "Source",
            "Higher Potential",
            "Lower Potential",
            "Purpose",
        ],
        6 => vec![
            "Priorities",
            "Criteria",
            "Values",
            "Resources",
            "Options",
            "Facts",
        ],
        7 => vec![
            "Insight",
            "Application",
            "Design",
            "Research",
            "Synthesis",
            "Delivery",
            "Value",
        ],
        8 => vec![
            "Inherent Values",
            "Critical Functions",
            "Organisational Modes",
            "Necessary Resourcing",
            "Intrinsic Nature",
            "Smallest Significant Holon",
            "Integrative Totality",
            "Supportive Platform",
        ],
        9 => vec![
            "Term 1", "Term 2", "Term 3", "Term 4", "Term 5", "Term 6", "Term 7", "Term 8",
            "Term 9",
        ],
        10 => vec![
            "Term 1", "Term 2", "Term 3", "Term 4", "Term 5", "Term 6", "Term 7", "Term 8",
            "Term 9", "Term 10",
        ],
        11 => vec![
            "Term 1", "Term 2", "Term 3", "Term 4", "Term 5", "Term 6", "Term 7", "Term 8",
            "Term 9", "Term 10", "Term 11",
        ],
        12 => vec![
            "Term 1", "Term 2", "Term 3", "Term 4", "Term 5", "Term 6", "Term 7", "Term 8",
            "Term 9", "Term 10", "Term 11", "Term 12",
        ],
        _ => vec![],
    }
}

fn get_term_character_slugs(order: u8) -> Vec<String> {
    get_term_characters(order).into_iter().map(slugify).collect()
}

/// Canonical connective slugs in canonical topological order (p1 < p2).
fn get_canonical_connective_slugs(order: u8) -> Vec<String> {
    match order {
        1 => vec![],
        2 => vec!["force_1_needs_research".into()],
        3 => vec![
            "generation".into(),
            "decision".into(),
            "consent".into(),
        ],
        4 => vec![
            "motivational_imperative".into(),
            "receptive_regard".into(),
            "effectual_compatibility".into(),
            "material_mastery".into(),
            "technical_power".into(),
            "demonstrable_activity".into(),
        ],
        5 => vec![
            "quantitative_match".into(),
            "aspiration".into(),
            "operation".into(),
            "qualitative_match".into(),
            "function".into(),
            "input".into(),
            "range_of_significance".into(),
            "range_of_potential".into(),
            "output".into(),
            "form".into(),
        ],
        _ => {
            let prefix = match order {
                6 => "step",
                7 => "interval",
                8 => "component",
                9 => "transmutation",
                10 => "progression",
                11 => "correlation",
                12 => "harmony",
                _ => return vec![],
            };
            let n = order as usize;
            let count = n * (n - 1) / 2;
            (1..=count)
                .map(|i| format!("{}_{}_needs_research", prefix, i))
                .collect()
        }
    }
}

fn slugify(s: &str) -> String {
    s.to_lowercase().replace(' ', "_")
}

fn word_from_slug(slug: &str) -> String {
    slug.split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn canonical_system_name(order: u8) -> &'static str {
    match order {
        1 => "Monad",
        2 => "Dyad",
        3 => "Triad",
        4 => "Tetrad",
        5 => "Pentad",
        6 => "Hexad",
        7 => "Heptad",
        8 => "Octad",
        9 => "Ennead",
        10 => "Decad",
        11 => "Undecad",
        12 => "Dodecad",
        _ => "Unknown",
    }
}

// The canonical metadata is single-sourced in `core::systematics` (the hexad model);
// these seed-side helpers delegate to it.
fn canonical_coherence(order: u8) -> &'static str {
    crate::core::systematics::coherence(order)
}

fn canonical_term_designation(order: u8) -> &'static str {
    crate::core::systematics::term_designation(order)
}

fn canonical_connective_designation(order: u8) -> &'static str {
    crate::core::systematics::connective_designation(order)
}

// =============================================================================
// Coordinate data — same geometry as the pre-refactor codebase.
// =============================================================================

fn get_coordinates(order: u8) -> Vec<Point3d> {
    match order {
        1 => vec![Point3d::new(0.0, 0.0, 0.0)],
        2 => vec![
            Point3d::new(-1.0, 0.0, 0.0),
            Point3d::new(1.0, 0.0, 0.0),
        ],
        3 => vec![
            Point3d::new(0.0, 1.0, 0.0),
            Point3d::new(0.0, -1.0, 0.0),
            Point3d::new(1.0, 0.0, 0.0),
        ],
        4 => vec![
            Point3d::new(0.0, 1.0, 0.0),
            Point3d::new(0.0, -1.0, 0.0),
            Point3d::new(1.0, 0.0, 0.0),
            Point3d::new(-1.0, 0.0, 0.0),
        ],
        5 => vec![
            Point3d::new(-0.75, 0.0, 0.0),
            Point3d::new(1.0, -0.75, 0.0),
            Point3d::new(0.0, 0.5, 0.0),
            Point3d::new(0.0, -0.5, 0.0),
            Point3d::new(1.0, 0.75, 0.0),
        ],
        6 => vec![
            Point3d::new(-0.866, -0.5, 0.0),
            Point3d::new(0.866, -0.5, 0.0),
            Point3d::new(0.0, 1.0, 0.0),
            Point3d::new(-0.866, 0.5, 0.0),
            Point3d::new(0.866, 0.5, 0.0),
            Point3d::new(0.0, -1.0, 0.0),
        ],
        7 => vec![
            Point3d::new(0.0, 1.0, 0.0),
            Point3d::new(-0.433884, -0.900969, 0.0),
            Point3d::new(0.974370, -0.222521, 0.0),
            Point3d::new(0.781831, 0.623489, 0.0),
            Point3d::new(0.433884, -0.900969, 0.0),
            Point3d::new(-0.974370, -0.222521, 0.0),
            Point3d::new(-0.781831, 0.623489, 0.0),
        ],
        8 => vec![
            Point3d::new(
                -std::f64::consts::FRAC_1_SQRT_2,
                std::f64::consts::FRAC_1_SQRT_2,
                0.0,
            ),
            Point3d::new(
                std::f64::consts::FRAC_1_SQRT_2,
                -std::f64::consts::FRAC_1_SQRT_2,
                0.0,
            ),
            Point3d::new(
                std::f64::consts::FRAC_1_SQRT_2,
                std::f64::consts::FRAC_1_SQRT_2,
                0.0,
            ),
            Point3d::new(
                -std::f64::consts::FRAC_1_SQRT_2,
                -std::f64::consts::FRAC_1_SQRT_2,
                0.0,
            ),
            Point3d::new(0.0, 1.0, 0.0),
            Point3d::new(1.0, 0.0, 0.0),
            Point3d::new(-1.0, 0.0, 0.0),
            Point3d::new(0.0, -1.0, 0.0),
        ],
        9 => vec![
            Point3d::new(-0.64278760968, 0.76604444311, 0.0),
            Point3d::new(0.86602540378, -0.5, 0.0),
            Point3d::new(0.64278760968, 0.76604444311, 0.0),
            Point3d::new(-0.34202014333, -0.93969262079, 0.0),
            Point3d::new(0.0, 1.0, 0.0),
            Point3d::new(0.98480775301, 0.17364817767, 0.0),
            Point3d::new(-0.98480775301, 0.17364817767, 0.0),
            Point3d::new(0.34202014333, -0.93969262079, 0.0),
            Point3d::new(-0.86602540378, -0.5, 0.0),
        ],
        10 => vec![
            Point3d::new(-0.80901699437, 0.58778525229, 0.0),
            Point3d::new(0.80901699437, -0.58778525229, 0.0),
            Point3d::new(0.30901699437, 0.95105651630, 0.0),
            Point3d::new(-0.30901699437, -0.95105651630, 0.0),
            Point3d::new(-0.30901699437, 0.95105651630, 0.0),
            Point3d::new(0.80901699437, 0.58778525229, 0.0),
            Point3d::new(-1.0, 0.0, 0.0),
            Point3d::new(0.30901699437, -0.95105651630, 0.0),
            Point3d::new(1.0, 0.0, 0.0),
            Point3d::new(-0.80901699437, -0.58778525229, 0.0),
        ],
        11 => vec![
            Point3d::new(-0.909632, 0.415415, 0.0),
            Point3d::new(0.755750, -0.654861, 0.0),
            Point3d::new(0.54064081745, 0.84125353283, 0.0),
            Point3d::new(-0.281733, -0.959493, 0.0),
            Point3d::new(-0.54064081745, 0.84125353283, 0.0),
            Point3d::new(0.909632, 0.415415, 0.0),
            Point3d::new(-0.989821, -0.142315, 0.0),
            Point3d::new(0.281733, -0.959493, 0.0),
            Point3d::new(0.989821, -0.142315, 0.0),
            Point3d::new(-0.755750, -0.654861, 0.0),
            Point3d::new(0.0, 1.0, 0.0),
        ],
        12 => vec![
            Point3d::new(-0.5, 0.86602540378, 0.0),
            Point3d::new(0.86602540378, -0.5, 0.0),
            Point3d::new(0.86602540378, 0.5, 0.0),
            Point3d::new(-0.86602540378, -0.5, 0.0),
            Point3d::new(1.0, 0.0, 0.0),
            Point3d::new(0.5, 0.86602540378, 0.0),
            Point3d::new(0.0, -1.0, 0.0),
            Point3d::new(-0.5, -0.86602540378, 0.0),
            Point3d::new(0.0, 1.0, 0.0),
            Point3d::new(0.5, -0.86602540378, 0.0),
            Point3d::new(-1.0, 0.0, 0.0),
            Point3d::new(-0.86602540378, 0.5, 0.0),
        ],
        _ => vec![],
    }
}

fn get_colours(order: u8) -> Vec<&'static str> {
    const RED: &str = "#FF0000";
    const BLUE: &str = "#0000FF";
    const YELLOW: &str = "#FFFF00";
    const GREEN: &str = "#099902";
    const PURPLE: &str = "#9900FF";
    const ORANGE: &str = "#FFA500";
    const LIGHT_BLUE: &str = "#00FFFF";
    const BROWN: &str = "#8B4513";
    const MAGENTA: &str = "#FF00FF";
    const WHITE: &str = "#FFFFFF";
    const SILVER: &str = "#C0C0C0";
    const GOLD: &str = "#FFD700";

    match order {
        1 => vec![RED],
        2 => vec![RED, BLUE],
        3 => vec![RED, BLUE, YELLOW],
        4 => vec![RED, BLUE, YELLOW, GREEN],
        5 => vec![RED, BLUE, YELLOW, GREEN, PURPLE],
        6 => vec![RED, BLUE, YELLOW, GREEN, PURPLE, ORANGE],
        7 => vec![RED, BLUE, YELLOW, GREEN, PURPLE, ORANGE, LIGHT_BLUE],
        8 => vec![RED, BLUE, YELLOW, GREEN, PURPLE, ORANGE, LIGHT_BLUE, BROWN],
        9 => vec![RED, BLUE, YELLOW, GREEN, PURPLE, ORANGE, LIGHT_BLUE, BROWN, MAGENTA],
        10 => vec![RED, BLUE, YELLOW, GREEN, PURPLE, ORANGE, LIGHT_BLUE, BROWN, MAGENTA, WHITE],
        11 => vec![
            RED, BLUE, YELLOW, GREEN, PURPLE, ORANGE, LIGHT_BLUE, BROWN, MAGENTA, WHITE, SILVER,
        ],
        12 => vec![
            RED, BLUE, YELLOW, GREEN, PURPLE, ORANGE, LIGHT_BLUE, BROWN, MAGENTA, WHITE, SILVER,
            GOLD,
        ],
        _ => vec![],
    }
}
} // mod regen

#[cfg(test)]
mod tests {
    use super::*;
    use super::regen::{build_canonical_from_tables, build_citation_from_tables, canonical_system_name};

    #[test]
    fn test_build_graph_substrate() {
        let g = build_graph();
        assert!(g.order(1).is_some());
        assert!(g.order(12).is_some());
        assert!(g.point(3, 1).is_some());
        assert!(g.line(3, 1, 2).is_some());
        assert!(g.coordinate(3, 1).is_some());
        assert!(g.segment(3, 1, 2).is_some());
        assert_eq!(g.coordinate(3, 1).unwrap().point_ref, "point_3_1");
    }

    #[test]
    fn test_build_graph_canonical_systems() {
        let g = build_graph();
        for order in 1..=12u8 {
            assert!(g.topology_for_order(order).is_some());
            assert!(g.geometry_for_order(order).is_some());
            assert!(g.template_for_order(order).is_some());
            let name = format!("Canonical {}", canonical_system_name(order));
            let vocab_id = format!(
                "vocab_{}_{}",
                name.to_lowercase().replace(' ', "_"),
                order
            );
            assert!(g.vocabulary(&vocab_id).is_some());
            let system_id = format!(
                "system_{}_{}",
                name.to_lowercase().replace(' ', "_"),
                order
            );
            assert!(g.system(&system_id).is_some());
            assert!(
                g.validate_system(&system_id).is_ok(),
                "System {} failed validation: {:?}",
                system_id,
                g.validate_system(&system_id)
            );
        }
    }

    #[test]
    fn test_character_at_point_canonical_triad() {
        let g = build_graph();
        assert_eq!(
            g.character_at_point("vocab_canonical_triad_3", "point_3_1")
                .unwrap()
                .value,
            "Will"
        );
        assert_eq!(
            g.character_at_line("vocab_canonical_triad_3", "line_3_1_2")
                .unwrap()
                .value,
            "Generation"
        );
    }

    /// Regenerate `data/canonical.json` from the Rust tables. Ignored by
    /// default — run explicitly after changing canonical data:
    /// `cargo test -p systematics-backend regenerate_canonical_seed -- --ignored`
    #[test]
    #[ignore]
    fn regenerate_canonical_seed() {
        let content = build_canonical_from_tables();
        let json = serde_json::to_string_pretty(&content).unwrap();
        // CARGO_MANIFEST_DIR is backend/, so the workspace-root data dir is ../data.
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../data/canonical.json");
        std::fs::create_dir_all(concat!(env!("CARGO_MANIFEST_DIR"), "/../data")).unwrap();
        std::fs::write(path, json).unwrap();
        eprintln!(
            "wrote {} ({} chars, {} coords, {} vocabularies, {} systems)",
            path,
            content.characters.len(),
            content.coordinates.len(),
            content.vocabularies.len(),
            content.systems.len()
        );
    }

    #[test]
    fn test_citation_triad_seeded() {
        let g = build_graph();
        assert!(g.template_for_order(3).is_some());
        assert!(g.vocabulary("vocab_citation_3").is_some());
        assert!(g.system("system_citation_3").is_some());
        assert!(
            g.validate_system("system_citation_3").is_ok(),
            "citation triad failed validation: {:?}",
            g.validate_system("system_citation_3")
        );
    }

    /// Regenerate `data/citation.json` from the Rust tables. Ignored by default:
    /// `cargo test -p systematics-backend regenerate_citation_seed -- --ignored`
    #[test]
    #[ignore]
    fn regenerate_citation_seed() {
        let content = build_citation_from_tables();
        let json = serde_json::to_string_pretty(&content).unwrap();
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../data/citation.json");
        std::fs::write(path, json).unwrap();
        eprintln!(
            "wrote {} ({} chars, {} vocabularies, {} systems)",
            path,
            content.characters.len(),
            content.vocabularies.len(),
            content.systems.len()
        );
    }

    /// Regenerate `data/fragments.json` from the Rust tables. Ignored by default:
    /// `cargo test -p systematics-backend regenerate_fragments_seed -- --ignored`
    #[test]
    #[ignore]
    fn regenerate_fragments_seed() {
        let content = super::regen::build_fragments_from_tables();
        let json = serde_json::to_string_pretty(&content).unwrap();
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../data/fragments.json");
        std::fs::write(path, json).unwrap();
        eprintln!(
            "wrote {} ({} chars, {} vocabularies, {} systems)",
            path,
            content.characters.len(),
            content.vocabularies.len(),
            content.systems.len()
        );
    }
}
