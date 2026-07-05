//! Graph structure and query methods.
//!
//! The Graph is the primary container for the property graph,
//! holding all entries and links with methods for querying.
//!
//! Queries are organized into two categories:
//! - **Anchor Queries**: Query the fundamental graph structure (Order, Position, Location)
//! - **Systematic Queries**: Query semantic/categorical content mapped to anchors

use serde::{Deserialize, Serialize};

use super::entries::{
    Character, CoherenceAttribute, Colour, ConnectiveDesignation, Coordinate, Entry, Location,
    Order, Position, SystemName, Term, TermDesignation,
};
use super::functors::Functor;
use super::language::Language;
use super::links::{Link, LinkType};

/// Graph is the primary container for the property graph (AD4M: Perspective).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Graph {
    pub entries: Vec<Entry>,
    pub links: Vec<Link>,
    #[serde(default)]
    pub functors: Vec<Functor>,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an entry to the graph
    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    /// Add a link to the graph
    pub fn add_link(&mut self, link: Link) {
        self.links.push(link);
    }

    /// Find an entry by ID
    pub fn get_entry(&self, id: &str) -> Option<&Entry> {
        self.entries.iter().find(|e| e.id() == id)
    }

    /// Find a link by ID
    pub fn get_link(&self, id: &str) -> Option<&Link> {
        self.links.iter().find(|l| l.id == id)
    }

    // ==========================================================================
    // Anchor Queries - Query the fundamental graph structure
    // ==========================================================================

    /// Get an Order entry by value
    pub fn order(&self, value: u8) -> Option<&Order> {
        self.entries.iter().find_map(|e| match e {
            Entry::Order(o) if o.value == value => Some(o),
            _ => None,
        })
    }

    /// Get all Order entries
    pub fn orders(&self) -> Vec<&Order> {
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Order(o) => Some(o),
                _ => None,
            })
            .collect()
    }

    /// Get a Position entry by value
    pub fn position(&self, value: u8) -> Option<&Position> {
        self.entries.iter().find_map(|e| match e {
            Entry::Position(p) if p.value == value => Some(p),
            _ => None,
        })
    }

    /// Get all Position entries
    pub fn positions(&self) -> Vec<&Position> {
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Position(p) => Some(p),
                _ => None,
            })
            .collect()
    }

    /// Get an entry-shaped Location by order and position values
    pub fn location(&self, order: u8, position: u8) -> Option<&Location> {
        let order_id = format!("order_{}", order);
        let position_id = format!("position_{}", position);
        self.entries.iter().find_map(|e| match e {
            Entry::Location(l)
                if l.order == order_id
                    && l.position == position_id
                    && l.position_secondary.is_none() =>
            {
                Some(l)
            }
            _ => None,
        })
    }

    /// Get a link-shaped Location by order and the two positions (undirected).
    /// Argument order is normalised to canonical form.
    pub fn link_location(&self, order: u8, p1: u8, p2: u8) -> Option<&Location> {
        let (lo, hi) = if p1 <= p2 { (p1, p2) } else { (p2, p1) };
        let order_id = format!("order_{}", order);
        let position_id = format!("position_{}", lo);
        let secondary_id = format!("position_{}", hi);
        self.entries.iter().find_map(|e| match e {
            Entry::Location(l)
                if l.order == order_id
                    && l.position == position_id
                    && l.position_secondary.as_deref() == Some(secondary_id.as_str()) =>
            {
                Some(l)
            }
            _ => None,
        })
    }

    /// Get all entry-shaped Location entries (bind a single position).
    /// Use `link_locations` for link-shaped ones.
    pub fn locations(&self) -> Vec<&Location> {
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Location(l) if l.is_entry() => Some(l),
                _ => None,
            })
            .collect()
    }

    /// Get all entry-shaped Locations for a given order.
    pub fn locations_for_order(&self, order: u8) -> Vec<&Location> {
        let order_id = format!("order_{}", order);
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Location(l) if l.order == order_id && l.is_entry() => Some(l),
                _ => None,
            })
            .collect()
    }

    /// Get all entry-shaped Locations for a given position (across all orders).
    pub fn locations_for_position(&self, position: u8) -> Vec<&Location> {
        let position_id = format!("position_{}", position);
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Location(l) if l.position == position_id && l.is_entry() => Some(l),
                _ => None,
            })
            .collect()
    }

    /// Get all link-shaped Locations (bind two positions).
    pub fn link_locations(&self) -> Vec<&Location> {
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Location(l) if l.is_link() => Some(l),
                _ => None,
            })
            .collect()
    }

    /// Get all link-shaped Locations for a given order.
    pub fn link_locations_for_order(&self, order: u8) -> Vec<&Location> {
        let order_id = format!("order_{}", order);
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Location(l) if l.order == order_id && l.is_link() => Some(l),
                _ => None,
            })
            .collect()
    }

    // ==========================================================================
    // Systematic Queries - Query semantic/categorical content mapped to anchors
    // ==========================================================================

    // -------------------- Order-Level Systematic Queries --------------------

    /// Get all entries for a given order (everything mapped to that order)
    pub fn system(&self, order: u8) -> Vec<&Entry> {
        self.entries
            .iter()
            .filter(|e| e.order() == Some(order))
            .collect()
    }

    /// Get the system name for an order
    pub fn system_name(&self, order: u8) -> Option<&SystemName> {
        let order_id = format!("order_{}", order);
        self.entries.iter().find_map(|e| match e {
            Entry::SystemName(s) if s.order == order_id => Some(s),
            _ => None,
        })
    }

    /// Get the coherence attribute for an order
    pub fn coherence(&self, order: u8) -> Option<&CoherenceAttribute> {
        let order_id = format!("order_{}", order);
        self.entries.iter().find_map(|e| match e {
            Entry::CoherenceAttribute(c) if c.order == order_id => Some(c),
            _ => None,
        })
    }

    /// Get the term designation for an order
    pub fn term_designation(&self, order: u8) -> Option<&TermDesignation> {
        let order_id = format!("order_{}", order);
        self.entries.iter().find_map(|e| match e {
            Entry::TermDesignation(t) if t.order == order_id => Some(t),
            _ => None,
        })
    }

    /// Get the connective designation for an order
    pub fn connective_designation(&self, order: u8) -> Option<&ConnectiveDesignation> {
        let order_id = format!("order_{}", order);
        self.entries.iter().find_map(|e| match e {
            Entry::ConnectiveDesignation(c) if c.order == order_id => Some(c),
            _ => None,
        })
    }

    // -------------------- Location-Level Systematic Queries --------------------

    /// Get all entry-shaped terms for an order, optionally filtered by language of their character.
    /// Excludes terms attached to link-shaped Locations; use `terms_for_order_links` for those.
    pub fn terms(&self, order: u8, language: Option<Language>) -> Vec<&Term> {
        let terms: Vec<&Term> = self
            .entries
            .iter()
            .filter_map(|e| match e {
                Entry::Term(t) if t.order_value() == Some(order) && !t.is_link_term() => Some(t),
                _ => None,
            })
            .collect();

        if let Some(lang) = language {
            terms
                .into_iter()
                .filter(|t| {
                    self.get_character(&t.character)
                        .map(|c| c.language == lang)
                        .unwrap_or(false)
                })
                .collect()
        } else {
            terms
        }
    }

    /// Get a specific term by order and position (entry-shaped Location)
    pub fn term(&self, order: u8, position: u8) -> Option<&Term> {
        let location_id = format!("loc_{}_{}", order, position);
        self.entries.iter().find_map(|e| match e {
            Entry::Term(t) if t.location == location_id => Some(t),
            _ => None,
        })
    }

    /// Get the term attached to a link-shaped Location (a connective vocabulary).
    /// Argument order is normalised to canonical form.
    pub fn term_at_link_location(&self, order: u8, p1: u8, p2: u8) -> Option<&Term> {
        let (lo, hi) = if p1 <= p2 { (p1, p2) } else { (p2, p1) };
        let location_id = format!("loc_{}_{}_{}", order, lo, hi);
        self.entries.iter().find_map(|e| match e {
            Entry::Term(t) if t.location == location_id => Some(t),
            _ => None,
        })
    }

    /// Get all terms attached to link-shaped Locations for a given order.
    pub fn terms_for_order_links(&self, order: u8) -> Vec<&Term> {
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Term(t) if t.order_value() == Some(order) && t.is_link_term() => Some(t),
                _ => None,
            })
            .collect()
    }

    /// Get all terms at a specific location
    pub fn terms_at_location(&self, location_id: &str) -> Vec<&Term> {
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Term(t) if t.location == location_id => Some(t),
                _ => None,
            })
            .collect()
    }

    /// Get the (first/canonical) term at a specific location
    pub fn term_at_location(&self, location_id: &str) -> Option<&Term> {
        self.entries.iter().find_map(|e| match e {
            Entry::Term(t) if t.location == location_id => Some(t),
            _ => None,
        })
    }

    /// Get the character of the term at a location (for simplex-anchored connective rendering)
    pub fn term_character_at(&self, location_id: &str) -> Option<&Character> {
        let term = self.term_at_location(location_id)?;
        self.get_character(&term.character)
    }

    /// Get all coordinates for an order
    pub fn coordinates(&self, order: u8) -> Vec<&Coordinate> {
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Coordinate(c) if c.order_value() == Some(order) => Some(c),
                _ => None,
            })
            .collect()
    }

    /// Get a specific coordinate by order and position
    pub fn coordinate(&self, order: u8, position: u8) -> Option<&Coordinate> {
        let location_id = format!("loc_{}_{}", order, position);
        self.entries.iter().find_map(|e| match e {
            Entry::Coordinate(c) if c.location == location_id => Some(c),
            _ => None,
        })
    }

    /// Get all colours for an order
    pub fn colours(&self, order: u8) -> Vec<&Colour> {
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Colour(c) if c.order_value() == Some(order) => Some(c),
                _ => None,
            })
            .collect()
    }

    /// Get a specific colour by order, position, and language
    pub fn colour(&self, order: u8, position: u8, language: Language) -> Option<&Colour> {
        let location_id = format!("loc_{}_{}", order, position);
        self.entries.iter().find_map(|e| match e {
            Entry::Colour(c) if c.location == location_id && c.language == language => Some(c),
            _ => None,
        })
    }

    // -------------------- Character Queries --------------------

    /// Get all characters for a language
    pub fn characters(&self, language: Language) -> Vec<&Character> {
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Character(c) if c.language == language => Some(c),
                _ => None,
            })
            .collect()
    }

    /// Get a character by ID
    pub fn get_character(&self, id: &str) -> Option<&Character> {
        self.entries.iter().find_map(|e| match e {
            Entry::Character(c) if c.id == id => Some(c),
            _ => None,
        })
    }

    // -------------------- Cross-Cutting Systematic Queries --------------------

    /// Get all entries at a specific order+position (the "slice" / fiber).
    /// Excludes link-shaped Locations and Terms; a slice describes one position, not a connective.
    pub fn slice(&self, order: u8, position: u8) -> Vec<&Entry> {
        self.entries
            .iter()
            .filter(|e| {
                if e.order() != Some(order) || e.position() != Some(position) {
                    return false;
                }
                match e {
                    Entry::Location(l) => l.is_entry(),
                    Entry::Term(t) => !t.is_link_term(),
                    _ => true,
                }
            })
            .collect()
    }

    /// Get all terms at the same position across different languages
    pub fn isomorphic_terms(&self, order: u8, position: u8) -> Vec<(&Term, &Character)> {
        let location_id = format!("loc_{}_{}", order, position);
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Term(t) if t.location == location_id => {
                    self.get_character(&t.character).map(|c| (t, c))
                }
                _ => None,
            })
            .collect()
    }

    // ==========================================================================
    // Link Queries
    // ==========================================================================

    /// Get connective links, optionally filtered by order and/or base/target positions
    /// Note: Connectives now reference Locations (simplex-anchored), not Terms
    pub fn connectives(
        &self,
        order: u8,
        base_position: Option<u8>,
        target_position: Option<u8>,
    ) -> Vec<&Link> {
        self.links
            .iter()
            .filter(|l| {
                if !l.is_connective() {
                    return false;
                }

                // Get the locations for base and target
                let base_id = match l.base_single() {
                    Some(id) => id,
                    None => return false,
                };
                let target_id = match l.target_single() {
                    Some(id) => id,
                    None => return false,
                };

                let base_loc = self.entries.iter().find_map(|e| match e {
                    Entry::Location(loc) if loc.id == base_id => Some(loc),
                    _ => None,
                });
                let target_loc = self.entries.iter().find_map(|e| match e {
                    Entry::Location(loc) if loc.id == target_id => Some(loc),
                    _ => None,
                });

                // Both locations must exist and be in the specified order
                match (base_loc, target_loc) {
                    (Some(bl), Some(tl))
                        if bl.order_value() == Some(order) && tl.order_value() == Some(order) =>
                    {
                        let base_match = base_position
                            .map(|p| bl.position_value() == Some(p))
                            .unwrap_or(true);
                        let target_match = target_position
                            .map(|p| tl.position_value() == Some(p))
                            .unwrap_or(true);
                        base_match && target_match
                    }
                    _ => false,
                }
            })
            .collect()
    }

    /// Get all connectives involving a specific location
    pub fn connectives_for_location(&self, location_id: &str) -> Vec<&Link> {
        self.links
            .iter()
            .filter(|l| {
                l.is_connective()
                    && (l.base_single() == Some(location_id)
                        || l.target_single() == Some(location_id))
            })
            .collect()
    }

    /// Get all connectives involving a specific term (by resolving term to location)
    pub fn connectives_for_term(&self, term_id: &str) -> Vec<&Link> {
        // Find the term's location
        let term = self.entries.iter().find_map(|e| match e {
            Entry::Term(t) if t.id == term_id => Some(t),
            _ => None,
        });
        match term {
            Some(t) => self.connectives_for_location(&t.location),
            None => vec![],
        }
    }

    // ==========================================================================
    // Functor Queries and Mutations
    // ==========================================================================

    /// Get a Functor by ID
    pub fn functor(&self, id: &str) -> Option<&Functor> {
        self.functors.iter().find(|f| f.id == id)
    }

    /// Get a Functor by name (first match)
    pub fn functor_by_name(&self, name: &str) -> Option<&Functor> {
        self.functors.iter().find(|f| f.name == name)
    }

    /// All stored Functors
    pub fn functors(&self) -> &[Functor] {
        &self.functors
    }

    /// Add a Functor to the graph.
    pub fn add_functor(&mut self, functor: Functor) {
        self.functors.push(functor);
    }

    /// Replace an existing Functor by ID. Returns the replaced Functor if it
    /// existed, or None if no Functor with that ID was found.
    pub fn update_functor(&mut self, functor: Functor) -> Option<Functor> {
        let idx = self.functors.iter().position(|f| f.id == functor.id)?;
        Some(std::mem::replace(&mut self.functors[idx], functor))
    }

    /// Remove a Functor by ID. Returns the removed Functor if it existed.
    pub fn delete_functor(&mut self, id: &str) -> Option<Functor> {
        let idx = self.functors.iter().position(|f| f.id == id)?;
        Some(self.functors.remove(idx))
    }

    /// Apply a stored Functor by ID: materialise Terms into `entries` and
    /// return the produced Terms. Returns None if no Functor with that ID exists.
    ///
    /// Idempotency: existing Terms with the same ID are replaced; duplicates
    /// are not accumulated. This lets `applyFunctor` be called safely more
    /// than once.
    pub fn apply_functor(&mut self, id: &str) -> Option<Vec<Term>> {
        let functor = self.functor(id)?.clone();
        let produced = functor.apply();
        for term in &produced {
            let term_id = term.id.clone();
            self.entries
                .retain(|e| !matches!(e, Entry::Term(t) if t.id == term_id));
            self.entries.push(Entry::Term(term.clone()));
        }
        Some(produced)
    }

    // ==========================================================================
    // Line Link Queries
    // ==========================================================================

    /// Get all line links for an order
    pub fn lines(&self, order: u8) -> Vec<&Link> {
        self.links
            .iter()
            .filter(|l| {
                if !matches!(l.link_type, LinkType::Line) {
                    return false;
                }

                // Check that base coordinate is in the specified order
                let base_id = match l.base_single() {
                    Some(id) => id,
                    None => return false,
                };

                self.entries.iter().any(|e| match e {
                    Entry::Coordinate(c) if c.id == base_id => c.order_value() == Some(order),
                    _ => false,
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::entries::Point3d;

    fn create_test_graph() -> Graph {
        let mut graph = Graph::new();

        // Add anchor entries
        graph.add_entry(Entry::Order(Order::new(3)));
        graph.add_entry(Entry::Position(Position::new(1)));
        graph.add_entry(Entry::Position(Position::new(2)));
        graph.add_entry(Entry::Position(Position::new(3)));
        graph.add_entry(Entry::Location(Location::new(3, 1)));
        graph.add_entry(Entry::Location(Location::new(3, 2)));
        graph.add_entry(Entry::Location(Location::new(3, 3)));

        // Add order-level metadata
        graph.add_entry(Entry::SystemName(SystemName::with_auto_id(3, "Triad")));
        graph.add_entry(Entry::CoherenceAttribute(CoherenceAttribute::with_auto_id(
            3, "Dynamism",
        )));
        graph.add_entry(Entry::TermDesignation(TermDesignation::with_auto_id(
            3, "Impulses",
        )));
        graph.add_entry(Entry::ConnectiveDesignation(
            ConnectiveDesignation::with_auto_id(3, "Acts"),
        ));

        // Add characters
        graph.add_entry(Entry::Character(Character::with_auto_id(
            Language::Canonical,
            "Will",
        )));
        graph.add_entry(Entry::Character(Character::with_auto_id(
            Language::Canonical,
            "Function",
        )));
        graph.add_entry(Entry::Character(Character::with_auto_id(
            Language::Canonical,
            "Being",
        )));

        // Add terms (reference locations)
        graph.add_entry(Entry::Term(Term::with_auto_id(3, 1, "char_canonical_will")));
        graph.add_entry(Entry::Term(Term::with_auto_id(
            3,
            2,
            "char_canonical_function",
        )));
        graph.add_entry(Entry::Term(Term::with_auto_id(
            3,
            3,
            "char_canonical_being",
        )));

        // Add coordinates (reference locations)
        graph.add_entry(Entry::Coordinate(Coordinate::with_auto_id(
            3,
            1,
            Point3d::new(0.0, 1.0, 0.0),
        )));
        graph.add_entry(Entry::Coordinate(Coordinate::with_auto_id(
            3,
            2,
            Point3d::new(-0.866, -0.5, 0.0),
        )));
        graph.add_entry(Entry::Coordinate(Coordinate::with_auto_id(
            3,
            3,
            Point3d::new(0.866, -0.5, 0.0),
        )));

        // Add colours (reference locations)
        graph.add_entry(Entry::Colour(Colour::with_auto_id(
            3,
            1,
            Language::Hex,
            "#FF0000",
        )));
        graph.add_entry(Entry::Colour(Colour::with_auto_id(
            3,
            2,
            Language::Hex,
            "#0000FF",
        )));
        graph.add_entry(Entry::Colour(Colour::with_auto_id(
            3,
            3,
            Language::Hex,
            "#E6E600",
        )));

        graph
    }

    #[test]
    fn test_anchor_queries() {
        let graph = create_test_graph();

        // Order queries
        let order = graph.order(3);
        assert!(order.is_some());
        assert_eq!(order.unwrap().value, 3);

        // Position queries
        let position = graph.position(1);
        assert!(position.is_some());
        assert_eq!(position.unwrap().value, 1);

        // Location queries
        let location = graph.location(3, 1);
        assert!(location.is_some());
        assert_eq!(location.unwrap().id, "loc_3_1");

        // Locations for order
        let locs = graph.locations_for_order(3);
        assert_eq!(locs.len(), 3);

        // Locations for position (cross-order)
        let locs = graph.locations_for_position(1);
        assert_eq!(locs.len(), 1); // Only one order in test graph
    }

    #[test]
    fn test_system_queries() {
        let graph = create_test_graph();

        assert!(graph.system_name(3).is_some());
        assert_eq!(graph.system_name(3).unwrap().value, "Triad");

        assert!(graph.coherence(3).is_some());
        assert_eq!(graph.coherence(3).unwrap().value, "Dynamism");

        assert!(graph.term_designation(3).is_some());
        assert_eq!(graph.term_designation(3).unwrap().value, "Impulses");
    }

    #[test]
    fn test_term_queries() {
        let graph = create_test_graph();

        let terms = graph.terms(3, None);
        assert_eq!(terms.len(), 3);

        let term = graph.term(3, 1);
        assert!(term.is_some());
        assert_eq!(term.unwrap().character, "char_canonical_will");

        // Terms at location
        let terms = graph.terms_at_location("loc_3_1");
        assert_eq!(terms.len(), 1);
    }

    #[test]
    fn test_slice_query() {
        let graph = create_test_graph();

        let slice = graph.slice(3, 1);
        // Should contain: Location, Term, Coordinate, Colour at position 1
        assert_eq!(slice.len(), 4);
    }

    #[test]
    fn test_character_lookup() {
        let graph = create_test_graph();

        let char = graph.get_character("char_canonical_will");
        assert!(char.is_some());
        assert_eq!(char.unwrap().value, "Will");
    }

    #[test]
    fn test_isomorphic_terms() {
        let graph = create_test_graph();

        let iso = graph.isomorphic_terms(3, 1);
        assert_eq!(iso.len(), 1);
        assert_eq!(iso[0].1.value, "Will");
    }

    #[test]
    fn test_functor_crud_and_apply() {
        use super::super::functors::{Functor, FunctorMapping};

        let mut graph = create_test_graph();
        let functor = Functor::new(
            "f_alt_triad",
            "alt-triad",
            Some(Language::Canonical),
            vec![
                FunctorMapping::new("char_canonical_generation", "loc_3_1_2"),
                FunctorMapping::new("char_canonical_consent", "loc_3_2_3"),
            ],
        );
        graph.add_functor(functor);
        assert_eq!(graph.functors().len(), 1);
        assert!(graph.functor("f_alt_triad").is_some());
        assert!(graph.functor_by_name("alt-triad").is_some());

        let produced = graph.apply_functor("f_alt_triad").unwrap();
        assert_eq!(produced.len(), 2);
        // Apply is idempotent — running again shouldn't duplicate.
        let again = graph.apply_functor("f_alt_triad").unwrap();
        assert_eq!(again.len(), 2);
        let link_terms = graph.terms_for_order_links(3);
        assert_eq!(link_terms.len(), 2);

        let removed = graph.delete_functor("f_alt_triad").unwrap();
        assert_eq!(removed.id, "f_alt_triad");
        assert!(graph.functor("f_alt_triad").is_none());
    }
}
