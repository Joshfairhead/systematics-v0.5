//! Seed the property graph.
//!
//! Emits substrate entries (Order, Position, Point, Line, Coordinate, Segment,
//! Character) followed by the canonical Topological / Geometric / Semantic
//! Vocabularies and one Canonical Perspective per Order.

use crate::core::{
    Entry, GeometricVocabulary, Grammar, GraphContent, Graph, Line, Link, Order, Point, Position,
    Segment, TopologicalVocabulary,
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
    add_rendering_line_links(&mut graph);

    graph.apply_content(&canonical_content());
    // The Citation triad ships as its own bundle, applied before marking
    // canonical so it counts as bundled (not user) content.
    graph.apply_content(&citation_content());
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
        graph.add_topological_vocab(TopologicalVocabulary::canonical_for(order));
        graph.add_geometric_vocab(GeometricVocabulary::canonical_for(order));
        // The complete-graph structure for this Order (deterministic).
        graph.add_grammar(Grammar::for_order(order));
    }
}

/// Emit `Link::line` edges between coordinates (rendering shim; retained
/// until the frontend consumes `Segment` entries directly).
fn add_rendering_line_links(graph: &mut Graph) {
    for order in 1..=12u8 {
        for p1 in 1..=order {
            for p2 in (p1 + 1)..=order {
                graph.add_link(Link::line(
                    format!("coord_{}_{}", order, p1),
                    format!("coord_{}_{}", order, p2),
                ));
            }
        }
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
/// Source / Artefact / Lookup as the three impulses of a K3, with placeholder
/// acts (names TBD). Metadata inherits order 3 (Dynamism / Impulses / Acts).
pub fn build_citation_from_tables() -> GraphContent {
    let mut content = GraphContent::default();
    let mut have_char = std::collections::HashSet::new();
    push_triadic_system(
        &mut content,
        &mut have_char,
        "Citation",
        3,
        &["source".to_string(), "artefact".to_string(), "lookup".to_string()],
        &[
            "citation_act_1_needs_research".to_string(),
            "citation_act_2_needs_research".to_string(),
            "citation_act_3_needs_research".to_string(),
        ],
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

fn canonical_coherence(order: u8) -> &'static str {
    match order {
        1 => "Universality",
        2 => "Complementarity",
        3 => "Dynamism",
        4 => "Activity Field",
        5 => "Significance and Potential",
        6 => "Coalescence",
        7 => "Generation",
        8 => "Self-Sufficiency",
        9 => "Transformation",
        10 => "Intrinsic Harmony",
        11 => "Articulate Symmetry",
        12 => "Perfection",
        _ => "Unknown",
    }
}

fn canonical_term_designation(order: u8) -> &'static str {
    match order {
        1 => "Totality",
        2 => "Poles",
        3 => "Impulses",
        4 => "Sources",
        5 => "Limits",
        6 => "Laws",
        7 => "States",
        8 => "Elements",
        _ => "Needs Research",
    }
}

fn canonical_connective_designation(order: u8) -> &'static str {
    match order {
        1 => "Unity",
        2 => "Force",
        3 => "Acts",
        4 => "Interplays",
        5 => "Mutualities",
        6 => "Steps",
        7 => "Intervals",
        8 => "Components",
        _ => "Needs Research",
    }
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
            assert!(g.topological_vocab_for_order(order).is_some());
            assert!(g.geometric_vocab_for_order(order).is_some());
            assert!(g.grammar_for_order(order).is_some());
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

    #[test]
    fn test_rendering_line_links_exist() {
        let g = build_graph();
        // Triad has 3 rendering lines.
        assert_eq!(g.lines(3).len(), 3);
        // Dodecad has C(12,2) = 66.
        assert_eq!(g.lines(12).len(), 66);
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
        assert!(g.grammar_for_order(3).is_some());
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
}
