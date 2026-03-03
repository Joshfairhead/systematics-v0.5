//! Data module for populating the property graph.
//!
//! This module provides functions to build the complete graph with all
//! system data including anchor types, Characters, Terms, Coordinates,
//! Colours, and Links.
//!
//! Data loading follows the category-theoretic structure:
//! 1. Create anchor entries first (Order, Position, Location)
//! 2. Add geometry (Coordinates, Colours, Lines) - invariant structure
//! 3. Add order-level metadata (SystemName, Coherence, Designations)
//! 4. Add vocabulary-specific content (Characters, Terms, Connectives)

use crate::core::{
    Character, CoherenceAttribute, Colour, ConnectiveDesignation, Coordinate, Entry, Graph,
    Language, Link, Location, Order, Point3d, Position, SystemName, Term, TermDesignation,
};

/// Build the complete graph with all systems (1-12)
pub fn build_graph() -> Graph {
    let mut graph = Graph::new();

    // 1. Create anchor entries first (invariant structure)
    add_orders(&mut graph);
    add_positions(&mut graph);
    add_locations(&mut graph);

    // 2. Add geometry (invariant, references Location)
    for order in 1..=12 {
        add_coordinates(&mut graph, order);
        add_colours(&mut graph, order);
    }

    // 3. Add order-level metadata (references Order)
    add_system_metadata(&mut graph);

    // 4. Add vocabulary-specific content (references Location)
    add_canonical_characters(&mut graph);
    for order in 1..=12 {
        add_terms(&mut graph, order);
    }

    // 5. Add links (connectives and lines)
    for order in 1..=12 {
        add_system_links(&mut graph, order);
    }

    graph
}

// =============================================================================
// Anchor Creation - Fundamental structure
// =============================================================================

/// Add all Order entries (1-12)
fn add_orders(graph: &mut Graph) {
    for i in 1..=12 {
        graph.add_entry(Entry::Order(Order::new(i)));
    }
}

/// Add all Position entries (1-12)
fn add_positions(graph: &mut Graph) {
    for i in 1..=12 {
        graph.add_entry(Entry::Position(Position::new(i)));
    }
}

/// Add all Location entries (pullback of Order × Position)
fn add_locations(graph: &mut Graph) {
    for order in 1..=12u8 {
        for position in 1..=order {
            graph.add_entry(Entry::Location(Location::new(order, position)));
        }
    }
}

// =============================================================================
// Geometry - Invariant structure mapped to Locations
// =============================================================================

/// Add coordinates for a specific system order
fn add_coordinates(graph: &mut Graph, order: u8) {
    let coords = get_coordinates(order);
    for (idx, coord) in coords.iter().enumerate() {
        let position = (idx + 1) as u8;
        graph.add_entry(Entry::Coordinate(Coordinate::with_auto_id(
            order, position, *coord,
        )));
    }
}

/// Add colours for a specific system order
fn add_colours(graph: &mut Graph, order: u8) {
    let colours = get_colours(order);
    for (idx, colour) in colours.iter().enumerate() {
        let position = (idx + 1) as u8;
        graph.add_entry(Entry::Colour(Colour::with_auto_id(
            order,
            position,
            Language::Hex,
            *colour,
        )));
    }
}

// =============================================================================
// Order-Level Metadata - References Order anchor
// =============================================================================

/// Add system-level metadata for all orders
fn add_system_metadata(graph: &mut Graph) {
    // System names
    let names = [
        (1, "Monad"),
        (2, "Dyad"),
        (3, "Triad"),
        (4, "Tetrad"),
        (5, "Pentad"),
        (6, "Hexad"),
        (7, "Heptad"),
        (8, "Octad"),
        (9, "Ennead"),
        (10, "Decad"),
        (11, "Undecad"),
        (12, "Dodecad"),
    ];

    for (order, name) in names {
        graph.add_entry(Entry::SystemName(SystemName::with_auto_id(order, name)));
    }

    // Coherence attributes
    let coherences = [
        (1, "Universality"),
        (2, "Complementarity"),
        (3, "Dynamism"),
        (4, "Activity Field"),
        (5, "Significance and Potential"),
        (6, "Coalescence"),
        (7, "Generation"),
        (8, "Self-Sufficiency"),
        (9, "Transformation"),
        (10, "Intrinsic Harmony"),
        (11, "Articulate Symmetry"),
        (12, "Perfection"),
    ];

    for (order, coherence) in coherences {
        graph.add_entry(Entry::CoherenceAttribute(CoherenceAttribute::with_auto_id(
            order, coherence,
        )));
    }

    // Term designations
    let term_designations = [
        (1, "Totality"),
        (2, "Poles"),
        (3, "Impulses"),
        (4, "Sources"),
        (5, "Limits"),
        (6, "Laws"),
        (7, "States"),
        (8, "Elements"),
        (9, "Needs Research"),
        (10, "Needs Research"),
        (11, "Needs Research"),
        (12, "Needs Research"),
    ];

    for (order, designation) in term_designations {
        graph.add_entry(Entry::TermDesignation(TermDesignation::with_auto_id(
            order,
            designation,
        )));
    }

    // Connective designations
    let connective_designations = [
        (1, "Unity"),
        (2, "Force"),
        (3, "Acts"),
        (4, "Interplays"),
        (5, "Mutualities"),
        (6, "Steps"),
        (7, "Intervals"),
        (8, "Components"),
        (9, "Needs Research"),
        (10, "Needs Research"),
        (11, "Needs Research"),
        (12, "Needs Research"),
    ];

    for (order, designation) in connective_designations {
        graph.add_entry(Entry::ConnectiveDesignation(
            ConnectiveDesignation::with_auto_id(order, designation),
        ));
    }
}

// =============================================================================
// Vocabulary-Specific Content - Characters and Terms
// =============================================================================

/// Add canonical vocabulary characters
fn add_canonical_characters(graph: &mut Graph) {
    let characters = [
        // Monad
        "Unity",
        // Dyad
        "Essence",
        "Existence",
        // Triad
        "Will",
        "Function",
        "Being",
        // Tetrad
        "Ideal",
        "Directive",
        "Instrumental",
        "Ground",
        // Pentad
        "Purpose",
        "Higher Potential",
        "Quintessence",
        "Lower Potential",
        "Source",
        // Hexad
        "Resources",
        "Values",
        "Options",
        "Criteria",
        "Facts",
        "Priorities",
        // Heptad
        "Insight",
        "Research",
        "Design",
        "Synthesis",
        "Application",
        "Delivery",
        "Value",
        // Octad
        "Smallest Significant Holon",
        "Critical Functions",
        "Supportive Platform",
        "Necessary Resourcing",
        "Integrative Totality",
        "Inherent Values",
        "Intrinsic Nature",
        "Organisational Modes",
    ];

    for value in characters {
        graph.add_entry(Entry::Character(Character::with_auto_id(
            Language::Canonical,
            value,
        )));
    }

    // Connective characters for Triad (Acts)
    for value in ["Generation", "Consent", "Decision"] {
        graph.add_entry(Entry::Character(Character::with_auto_id(
            Language::Canonical,
            value,
        )));
    }

    // Connective characters for Tetrad (Interplays)
    for value in [
        "Receptive Regard",
        "Effectual Compatibility",
        "Motivational Imperative",
        "Demonstrable Activity",
        "Material Mastery",
        "Technical Power",
    ] {
        graph.add_entry(Entry::Character(Character::with_auto_id(
            Language::Canonical,
            value,
        )));
    }

    // Connective characters for Pentad (Mutualities)
    for value in [
        "Range of Potential",
        "Range of Significance",
        "Aspiration",
        "Operation",
        "Output",
        "Input",
        "Qualitative Match",
        "Quantitative Match",
        "Form",
        "Function",
    ] {
        graph.add_entry(Entry::Character(Character::with_auto_id(
            Language::Canonical,
            value,
        )));
    }

    // Connective characters for Hexad (Steps) - placeholders
    for i in 1..=15 {
        let value = format!("Step {} Needs Research", i);
        graph.add_entry(Entry::Character(Character::with_auto_id(
            Language::Canonical,
            &value,
        )));
    }

    // Connective characters for Heptad (Intervals) - placeholders
    for i in 1..=21 {
        let value = format!("Interval {} Needs Research", i);
        graph.add_entry(Entry::Character(Character::with_auto_id(
            Language::Canonical,
            &value,
        )));
    }

    // Connective characters for Octad (Components) - placeholders
    for i in 1..=28 {
        let value = format!("Component {} Needs Research", i);
        graph.add_entry(Entry::Character(Character::with_auto_id(
            Language::Canonical,
            &value,
        )));
    }

    // Connective characters for Ennead (Transmutations) - placeholders
    for i in 1..=36 {
        let value = format!("Transmutation {} Needs Research", i);
        graph.add_entry(Entry::Character(Character::with_auto_id(
            Language::Canonical,
            &value,
        )));
    }

    // Connective characters for Decad (Progressions) - placeholders
    for i in 1..=45 {
        let value = format!("Progression {} Needs Research", i);
        graph.add_entry(Entry::Character(Character::with_auto_id(
            Language::Canonical,
            &value,
        )));
    }

    // Connective characters for Undecad (Correlations) - placeholders
    for i in 1..=55 {
        let value = format!("Correlation {} Needs Research", i);
        graph.add_entry(Entry::Character(Character::with_auto_id(
            Language::Canonical,
            &value,
        )));
    }

    // Connective characters for Dodecad (Harmonies) - placeholders
    for i in 1..=66 {
        let value = format!("Harmony {} Needs Research", i);
        graph.add_entry(Entry::Character(Character::with_auto_id(
            Language::Canonical,
            &value,
        )));
    }

    // Generic terms for orders 9-12
    for i in 1..=12 {
        let value = format!("Term {}", i);
        graph.add_entry(Entry::Character(Character::with_auto_id(
            Language::Canonical,
            &value,
        )));
    }

    // Index-based terms for orders 9-12
    for i in 1..=12 {
        let value = format!("Index {}", i);
        graph.add_entry(Entry::Character(Character::with_auto_id(
            Language::Canonical,
            &value,
        )));
    }
}

/// Add terms for a specific order (references Location)
fn add_terms(graph: &mut Graph, order: u8) {
    let term_chars = get_term_characters(order);

    for (idx, char_name) in term_chars.iter().enumerate() {
        let position = (idx + 1) as u8;
        let char_id = format!(
            "char_canonical_{}",
            char_name.to_lowercase().replace(' ', "_")
        );
        graph.add_entry(Entry::Term(Term::with_auto_id(order, position, &char_id)));
    }
}

// =============================================================================
// Links - Connectives and Lines
// =============================================================================

/// Add links (connectives and lines) for a system
fn add_system_links(graph: &mut Graph, order: u8) {
    // Add connective links for specific orders
    match order {
        3 => {
            // Triad: Acts between locations (simplex-anchored)
            let acts = [
                ("loc_3_1", "loc_3_2", "generation"),
                ("loc_3_2", "loc_3_3", "consent"),
                ("loc_3_3", "loc_3_1", "decision"),
            ];
            for (from, to, act) in acts {
                let char_id = format!("char_canonical_{}", act);
                graph.add_link(Link::connective(from, to).with_tag(&char_id));
            }
        }
        4 => {
            // Tetrad: Interplays between locations (simplex-anchored)
            // The structural edges are invariant; vocabulary-coupling happens
            // via dynamic Term lookup at render time (future Option B)
            let interplays = [
                ("loc_4_1", "loc_4_2", "motivational_imperative"), // Position 1 → Position 2
                ("loc_4_3", "loc_4_4", "demonstrable_activity"),   // Position 3 → Position 4
                ("loc_4_4", "loc_4_1", "effectual_compatibility"), // Position 4 → Position 1
                ("loc_4_3", "loc_4_1", "receptive_regard"),        // Position 3 → Position 1
                ("loc_4_3", "loc_4_2", "material_mastery"),        // Position 3 → Position 2
                ("loc_4_4", "loc_4_2", "technical_power"),         // Position 4 → Position 2
            ];
            for (from, to, name) in interplays {
                let char_id = format!("char_canonical_{}", name);
                graph.add_link(Link::connective(from, to).with_tag(&char_id));
            }
        }
        5 => {
            // Pentad: Mutualities between locations (simplex-anchored)
            // Structural positions: 1=Quintessence, 2=Source, 3=Higher Potential, 4=Lower Potential, 5=Purpose
            // Vocabulary-coupling happens via dynamic Term lookup at render time (future Option B)
            let mutualities = [
                ("loc_5_3", "loc_5_4", "range_of_potential"), // Position 3 → Position 4
                ("loc_5_5", "loc_5_2", "range_of_significance"), // Position 5 → Position 2
                ("loc_5_1", "loc_5_3", "aspiration"),         // Position 1 → Position 3
                ("loc_5_1", "loc_5_4", "operation"),          // Position 1 → Position 4
                ("loc_5_3", "loc_5_5", "output"),             // Position 3 → Position 5
                ("loc_5_4", "loc_5_2", "input"),              // Position 4 → Position 2
                ("loc_5_1", "loc_5_5", "qualitative_match"),  // Position 1 → Position 5
                ("loc_5_1", "loc_5_2", "quantitative_match"), // Position 1 → Position 2
                ("loc_5_4", "loc_5_5", "form"),               // Position 4 → Position 5
                ("loc_5_3", "loc_5_2", "function"),           // Position 3 → Position 2
            ];
            for (from, to, name) in mutualities {
                let char_id = format!("char_canonical_{}", name);
                graph.add_link(Link::connective(from, to).with_tag(&char_id));
            }
        }
        6..=12 => {
            // Higher orders: Add placeholder connective links for all term pairs
            add_placeholder_connectives(graph, order);
        }
        _ => {}
    }

    // Add line links between all coordinates (complete graph)
    for i in 1..=order {
        for j in (i + 1)..=order {
            graph.add_link(Link::line(
                format!("coord_{}_{}", order, i),
                format!("coord_{}_{}", order, j),
            ));
        }
    }
}

/// Add placeholder connective links for orders 6-12 (simplex-anchored)
fn add_placeholder_connectives(graph: &mut Graph, order: u8) {
    let (prefix, _designation) = match order {
        6 => ("step", "Steps"),
        7 => ("interval", "Intervals"),
        8 => ("component", "Components"),
        9 => ("transmutation", "Transmutations"),
        10 => ("progression", "Progressions"),
        11 => ("correlation", "Correlations"),
        12 => ("harmony", "Harmonies"),
        _ => return,
    };

    let mut idx = 1;
    for i in 1..=order {
        for j in (i + 1)..=order {
            let from = format!("loc_{}_{}", order, i);
            let to = format!("loc_{}_{}", order, j);
            let char_id = format!("char_canonical_{}_{}_needs_research", prefix, idx);
            graph.add_link(Link::connective(&from, &to).with_tag(&char_id));
            idx += 1;
        }
    }
}

// =============================================================================
// Data Helpers
// =============================================================================

/// Get term characters for an order
fn get_term_characters(order: u8) -> Vec<&'static str> {
    match order {
        1 => vec!["Unity"],
        2 => vec!["Essence", "Existence"],
        3 => vec!["Will", "Function", "Being"],
        4 => vec!["Ideal", "Ground", "Directive", "Instrumental"],
        // Pentad: 1=Quintessence, 2=Source, 3=Higher Potential, 4=Lower Potential, 5=Purpose
        5 => vec![
            "Quintessence",
            "Source",
            "Higher Potential",
            "Lower Potential",
            "Purpose",
        ],
        // Hexad: 1=Priorities, 2=Criteria, 3=Values, 4=Resources, 5=Options, 6=Facts
        6 => vec![
            "Priorities",
            "Criteria",
            "Values",
            "Resources",
            "Options",
            "Facts",
        ],
        // Heptad: 1=Insight, 2=Application, 3=Design, 4=Research, 5=Synthesis, 6=Delivery, 7=Value
        7 => vec![
            "Insight",
            "Application",
            "Design",
            "Research",
            "Synthesis",
            "Delivery",
            "Value",
        ],
        // Octad: Position N = Term mapping
        // 1=Inherent Values, 2=Critical Functions, 3=Organisational Modes, 4=Necessary Resourcing
        // 5=Intrinsic Nature, 6=Smallest Holon, 7=Integrative Totality, 8=Supportive Platform
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
        // Ennead: Sequential positions 1-9
        9 => vec![
            "Term 1", "Term 2", "Term 3", "Term 4", "Term 5", "Term 6", "Term 7", "Term 8",
            "Term 9",
        ],
        // Decad: Sequential positions 1-10
        10 => vec![
            "Term 1", "Term 2", "Term 3", "Term 4", "Term 5", "Term 6", "Term 7", "Term 8",
            "Term 9", "Term 10",
        ],
        // Undecad: Sequential positions 1-11
        11 => vec![
            "Term 1", "Term 2", "Term 3", "Term 4", "Term 5", "Term 6", "Term 7", "Term 8",
            "Term 9", "Term 10", "Term 11",
        ],
        // Dodecad: Sequential positions 1-12
        12 => vec![
            "Term 1", "Term 2", "Term 3", "Term 4", "Term 5", "Term 6", "Term 7", "Term 8",
            "Term 9", "Term 10", "Term 11", "Term 12",
        ],
        _ => vec![],
    }
}

/// Get coordinates for an order (from curated data files)
fn get_coordinates(order: u8) -> Vec<Point3d> {
    match order {
        1 => vec![Point3d::new(0.0, 0.0, 0.0)],
        2 => vec![
            Point3d::new(-1.0, 0.0, 0.0), // Essence (left)
            Point3d::new(1.0, 0.0, 0.0),  // Existence (right)
        ],
        3 => vec![
            Point3d::new(0.0, 1.0, 0.0),  // Will (top left)
            Point3d::new(0.0, -1.0, 0.0), // Function (bottom left)
            Point3d::new(1.0, 0.0, 0.0),  // Being (right, midpoint vertically)
        ],
        4 => vec![
            Point3d::new(0.0, 1.0, 0.0),  // Ideal (top)
            Point3d::new(0.0, -1.0, 0.0), // Ground (bottom)
            Point3d::new(1.0, 0.0, 0.0),  // Directive (right)
            Point3d::new(-1.0, 0.0, 0.0), // Instrumental (left)
        ],
        5 => vec![
            Point3d::new(-0.75, 0.0, 0.0), // Quintessence (left-center, middle)
            Point3d::new(1.0, -0.75, 0.0), // Source (right, bottom)
            Point3d::new(0.0, 0.5, 0.0),   // Higher Potential (center, upper)
            Point3d::new(0.0, -0.5, 0.0),  // Lower Potential (center, lower)
            Point3d::new(1.0, 0.75, 0.0),  // Purpose (right, top)
        ],
        6 => vec![
            Point3d::new(-0.866, -0.5, 0.0), // Priorities (lower left)
            Point3d::new(0.866, -0.5, 0.0),  // Criteria (lower right)
            Point3d::new(0.0, 1.0, 0.0),     // Values (top)
            Point3d::new(-0.866, 0.5, 0.0),  // Resources (upper left)
            Point3d::new(0.866, 0.5, 0.0),   // Options (upper right)
            Point3d::new(0.0, -1.0, 0.0),    // Facts (bottom)
        ],
        7 => vec![
            Point3d::new(0.0, 1.0, 0.0),             // Insight (top center)
            Point3d::new(-0.433884, -0.900969, 0.0), // Application
            Point3d::new(0.974370, -0.222521, 0.0),  // Design
            Point3d::new(0.781831, 0.623489, 0.0),   // Research
            Point3d::new(0.433884, -0.900969, 0.0),  // Synthesis
            Point3d::new(-0.974370, -0.222521, 0.0), // Delivery
            Point3d::new(-0.781831, 0.623489, 0.0),  // Value
        ],
        8 => vec![
            Point3d::new(
                -std::f64::consts::FRAC_1_SQRT_2,
                std::f64::consts::FRAC_1_SQRT_2,
                0.0,
            ), // Inherent Values (upper left)
            Point3d::new(
                std::f64::consts::FRAC_1_SQRT_2,
                -std::f64::consts::FRAC_1_SQRT_2,
                0.0,
            ), // Critical Functions (lower right)
            Point3d::new(
                std::f64::consts::FRAC_1_SQRT_2,
                std::f64::consts::FRAC_1_SQRT_2,
                0.0,
            ), // Organisational Modes (upper right)
            Point3d::new(
                -std::f64::consts::FRAC_1_SQRT_2,
                -std::f64::consts::FRAC_1_SQRT_2,
                0.0,
            ), // Necessary Resourcing (lower left)
            Point3d::new(0.0, 1.0, 0.0),  // Intrinsic Nature (top)
            Point3d::new(1.0, 0.0, 0.0),  // Smallest Significant Holon (right)
            Point3d::new(-1.0, 0.0, 0.0), // Integrative Totality (left)
            Point3d::new(0.0, -1.0, 0.0), // Supportive Platform (bottom)
        ],
        // Ennead: 9 points arranged in a circle
        9 => vec![
            Point3d::new(-0.64278760968, 0.76604444311, 0.0), // Position 1
            Point3d::new(0.86602540378, -0.5, 0.0),           // Position 2
            Point3d::new(0.64278760968, 0.76604444311, 0.0),  // Position 3
            Point3d::new(-0.34202014333, -0.93969262079, 0.0), // Position 4
            Point3d::new(0.0, 1.0, 0.0),                      // Position 5
            Point3d::new(0.98480775301, 0.17364817767, 0.0),  // Position 6
            Point3d::new(-0.98480775301, 0.17364817767, 0.0), // Position 7
            Point3d::new(0.34202014333, -0.93969262079, 0.0), // Position 8
            Point3d::new(-0.86602540378, -0.5, 0.0),          // Position 9
        ],
        // Decad: 10 points arranged in a circle
        10 => vec![
            Point3d::new(-0.80901699437, 0.58778525229, 0.0), // Position 1
            Point3d::new(0.80901699437, -0.58778525229, 0.0), // Position 2
            Point3d::new(0.30901699437, 0.95105651630, 0.0),  // Position 3
            Point3d::new(-0.30901699437, -0.95105651630, 0.0), // Position 4
            Point3d::new(-0.30901699437, 0.95105651630, 0.0), // Position 5
            Point3d::new(0.80901699437, 0.58778525229, 0.0),  // Position 6
            Point3d::new(-1.0, 0.0, 0.0),                     // Position 7
            Point3d::new(0.30901699437, -0.95105651630, 0.0), // Position 8
            Point3d::new(1.0, 0.0, 0.0),                      // Position 9
            Point3d::new(-0.80901699437, -0.58778525229, 0.0), // Position 10
        ],
        // Undecad: 11 points arranged in a circle
        11 => vec![
            Point3d::new(-0.909632, 0.415415, 0.0),           // Position 1
            Point3d::new(0.755750, -0.654861, 0.0),           // Position 2
            Point3d::new(0.54064081745, 0.84125353283, 0.0),  // Position 3
            Point3d::new(-0.281733, -0.959493, 0.0),          // Position 4
            Point3d::new(-0.54064081745, 0.84125353283, 0.0), // Position 5
            Point3d::new(0.909632, 0.415415, 0.0),            // Position 6
            Point3d::new(-0.989821, -0.142315, 0.0),          // Position 7
            Point3d::new(0.281733, -0.959493, 0.0),           // Position 8
            Point3d::new(0.989821, -0.142315, 0.0),           // Position 9
            Point3d::new(-0.755750, -0.654861, 0.0),          // Position 10
            Point3d::new(0.0, 1.0, 0.0),                      // Position 11
        ],
        // Dodecad: 12 points arranged in a circle
        12 => vec![
            Point3d::new(-0.5, 0.86602540378, 0.0),  // Position 1
            Point3d::new(0.86602540378, -0.5, 0.0),  // Position 2
            Point3d::new(0.86602540378, 0.5, 0.0),   // Position 3
            Point3d::new(-0.86602540378, -0.5, 0.0), // Position 4
            Point3d::new(1.0, 0.0, 0.0),             // Position 5
            Point3d::new(0.5, 0.86602540378, 0.0),   // Position 6
            Point3d::new(0.0, -1.0, 0.0),            // Position 7
            Point3d::new(-0.5, -0.86602540378, 0.0), // Position 8
            Point3d::new(0.0, 1.0, 0.0),             // Position 9
            Point3d::new(0.5, -0.86602540378, 0.0),  // Position 10
            Point3d::new(-1.0, 0.0, 0.0),            // Position 11
            Point3d::new(-0.86602540378, 0.5, 0.0),  // Position 12
        ],
        _ => vec![],
    }
}

/// Get position colours for an order
fn get_colours(order: u8) -> Vec<&'static str> {
    // Color palette
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
        9 => vec![
            RED, BLUE, YELLOW, GREEN, PURPLE, ORANGE, LIGHT_BLUE, BROWN, MAGENTA,
        ],
        10 => vec![
            RED, BLUE, YELLOW, GREEN, PURPLE, ORANGE, LIGHT_BLUE, BROWN, MAGENTA, WHITE,
        ],
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_graph_has_anchors() {
        let graph = build_graph();

        // Should have Order entries
        assert!(graph.order(1).is_some());
        assert!(graph.order(12).is_some());

        // Should have Position entries
        assert!(graph.position(1).is_some());
        assert!(graph.position(12).is_some());

        // Should have Location entries
        assert!(graph.location(3, 1).is_some());
        assert!(graph.location(12, 12).is_some());
    }

    #[test]
    fn test_build_graph_has_metadata() {
        let graph = build_graph();

        // Should have entries for all 12 systems
        assert!(graph.system_name(1).is_some());
        assert!(graph.system_name(12).is_some());

        // Check triad
        assert_eq!(graph.system_name(3).unwrap().value, "Triad");
        assert_eq!(graph.coherence(3).unwrap().value, "Dynamism");
        assert_eq!(graph.term_designation(3).unwrap().value, "Impulses");
    }

    #[test]
    fn test_build_graph_has_terms() {
        let graph = build_graph();

        // Check terms exist
        let triad_terms = graph.terms(3, None);
        assert_eq!(triad_terms.len(), 3);

        // Verify term references location
        let term = graph.term(3, 1).unwrap();
        assert_eq!(term.location, "loc_3_1");
    }

    #[test]
    fn test_coordinates_reference_location() {
        let graph = build_graph();

        let coord = graph.coordinate(3, 1).unwrap();
        assert_eq!(coord.location, "loc_3_1");
    }

    #[test]
    fn test_slices_include_location() {
        let graph = build_graph();

        // Slice at (3, 1) should include Location, Term, Coordinate, Colour
        let slice = graph.slice(3, 1);
        assert!(slice.len() >= 4);

        // Verify Location is in the slice
        let has_location = slice.iter().any(|e| matches!(e, Entry::Location(_)));
        assert!(has_location);
    }

    #[test]
    fn test_locations_for_order() {
        let graph = build_graph();

        let triad_locs = graph.locations_for_order(3);
        assert_eq!(triad_locs.len(), 3);

        let dodecad_locs = graph.locations_for_order(12);
        assert_eq!(dodecad_locs.len(), 12);
    }

    #[test]
    fn test_locations_for_position() {
        let graph = build_graph();

        // Position 1 exists in all 12 orders
        let pos1_locs = graph.locations_for_position(1);
        assert_eq!(pos1_locs.len(), 12);

        // Position 12 only exists in Dodecad
        let pos12_locs = graph.locations_for_position(12);
        assert_eq!(pos12_locs.len(), 1);
    }
}
