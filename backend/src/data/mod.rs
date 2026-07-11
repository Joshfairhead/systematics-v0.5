//! Seed the property graph.
//!
//! Emits substrate entries (Order, Position, Point, Line, Coordinate, Segment,
//! Character) followed by the canonical Topological / Geometric / Semantic
//! Vocabularies and one Canonical Grammar per Order.

use crate::core::{
    Character, Coordinate, Entry, GeometricVocabulary, Grammar, Graph, Line, Link, Order, Point,
    Point3d, Position, SemanticVocabulary, Segment, TopologicalVocabulary,
};

/// Build the complete graph with all systems (1-12).
pub fn build_graph() -> Graph {
    let mut graph = Graph::new();

    add_orders(&mut graph);
    add_positions(&mut graph);
    add_substrate(&mut graph);
    add_canonical_vocabularies_and_grammars(&mut graph);
    add_canonical_colour_vocabularies(&mut graph);
    add_rendering_line_links(&mut graph);

    graph
}

/// Seed one canonical hex-colour Character per unique colour + one Canonical
/// Colour SemanticVocabulary per Order (terms only, no connectives).
fn add_canonical_colour_vocabularies(graph: &mut Graph) {
    for order in 1..=12u8 {
        let hex_codes = get_colours(order);
        // Ensure each hex code has a Character entry (content-addressed).
        for hex in &hex_codes {
            let id = format!("char_hex_{}", hex.trim_start_matches('#').to_lowercase());
            if graph.character(&id).is_none() {
                graph.add_entry(Entry::Character(Character::new(&id, "hex", *hex)));
            }
        }
        let terms: Vec<String> = hex_codes
            .iter()
            .map(|hex| format!("char_hex_{}", hex.trim_start_matches('#').to_lowercase()))
            .collect();
        let name = format!("Canonical Colours {}", canonical_system_name(order));
        graph.add_semantic_vocab(SemanticVocabulary::with_auto_id(
            name,
            order,
            terms,
            vec![], // colours have no connectives; validate() is not called on this vocab.
        ));
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

fn add_substrate(graph: &mut Graph) {
    for order in 1..=12u8 {
        // Points at every position.
        for position in 1..=order {
            graph.add_entry(Entry::Point(Point::new(order, position)));
        }
        // Lines at every canonical pair (p1 < p2).
        for p1 in 1..=order {
            for p2 in (p1 + 1)..=order {
                graph.add_entry(Entry::Line(Line::new(order, p1, p2)));
            }
        }
        // Coordinates at every Point.
        let coords = get_coordinates(order);
        for (idx, coord) in coords.iter().enumerate() {
            let position = (idx + 1) as u8;
            graph.add_entry(Entry::Coordinate(Coordinate::from_point3d(
                order, position, *coord,
            )));
        }
        // Segments at every Line.
        for p1 in 1..=order {
            for p2 in (p1 + 1)..=order {
                graph.add_entry(Entry::Segment(Segment::new(order, p1, p2)));
            }
        }
        // Per-order topological and geometric vocabularies.
        graph.add_topological_vocab(TopologicalVocabulary::canonical_for(order));
        graph.add_geometric_vocab(GeometricVocabulary::canonical_for(order));
    }
}

fn add_canonical_vocabularies_and_grammars(graph: &mut Graph) {
    for order in 1..=12u8 {
        let term_slugs = get_term_character_slugs(order);
        let connective_slugs = get_canonical_connective_slugs(order);

        // Emit Character entries for every referenced slug (word Characters
        // are content-addressed by `char_word_{slug}`).
        for slug in term_slugs.iter().chain(connective_slugs.iter()) {
            let id = format!("char_word_{}", slug);
            if graph.character(&id).is_none() {
                let value = word_from_slug(slug);
                graph.add_entry(Entry::Character(Character::new(&id, "word", value)));
            }
        }

        let term_char_ids: Vec<String> = term_slugs
            .iter()
            .map(|s| format!("char_word_{}", s))
            .collect();
        let connective_char_ids: Vec<String> = connective_slugs
            .iter()
            .map(|s| format!("char_word_{}", s))
            .collect();

        let semvocab = SemanticVocabulary::with_auto_id(
            format!("Canonical {}", canonical_system_name(order)),
            order,
            term_char_ids,
            connective_char_ids,
        );
        let semvocab_id = semvocab.id.clone();
        graph.add_semantic_vocab(semvocab);

        let topvocab_id = format!("topvocab_{}", order);
        let geovocab_id = format!("geovocab_{}", order);

        let gram = Grammar::with_auto_id(
            format!("Canonical {}", canonical_system_name(order)),
            order,
            canonical_coherence(order),
            canonical_term_designation(order),
            canonical_connective_designation(order),
            &topvocab_id,
            &geovocab_id,
            &semvocab_id,
        );
        graph.add_grammar(gram);
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

fn canonical_system_name(order: u8) -> &'static str {
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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_build_graph_canonical_grammars() {
        let g = build_graph();
        for order in 1..=12u8 {
            assert!(g.topological_vocab_for_order(order).is_some());
            assert!(g.geometric_vocab_for_order(order).is_some());
            let name = format!("Canonical {}", canonical_system_name(order));
            let semvocab_id = format!(
                "semvocab_{}_{}",
                name.to_lowercase().replace(' ', "_"),
                order
            );
            assert!(g.semantic_vocab(&semvocab_id).is_some());
            let grammar_id = format!(
                "grammar_{}_{}",
                name.to_lowercase().replace(' ', "_"),
                order
            );
            assert!(g.grammar(&grammar_id).is_some());
            assert!(
                g.validate_grammar(&grammar_id).is_ok(),
                "Grammar {} failed validation: {:?}",
                grammar_id,
                g.validate_grammar(&grammar_id)
            );
        }
    }

    #[test]
    fn test_character_at_point_canonical_triad() {
        let g = build_graph();
        assert_eq!(
            g.character_at_point("semvocab_canonical_triad_3", "point_3_1")
                .unwrap()
                .value,
            "Will"
        );
        assert_eq!(
            g.character_at_line("semvocab_canonical_triad_3", "line_3_1_2")
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
}
