//! Graph structure and query methods.
//!
//! The Graph holds substrate entries + the four higher-level tables
//! (topological, geometric, semantic vocabularies, and grammars).

use serde::{Deserialize, Serialize};

use super::entries::{Character, Coordinate, Entry, Line, Order, Point, Position, Segment};
use super::grammars::Grammar;
use super::links::{Link, LinkType};
use super::vocabularies::{GeometricVocabulary, SemanticVocabulary, TopologicalVocabulary};

/// The primary container. Holds substrate entries, edge Links, and the four
/// higher-level tables.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Graph {
    pub entries: Vec<Entry>,
    pub links: Vec<Link>,
    #[serde(default)]
    pub topological_vocabs: Vec<TopologicalVocabulary>,
    #[serde(default)]
    pub geometric_vocabs: Vec<GeometricVocabulary>,
    #[serde(default)]
    pub semantic_vocabs: Vec<SemanticVocabulary>,
    #[serde(default)]
    pub grammars: Vec<Grammar>,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    pub fn add_link(&mut self, link: Link) {
        self.links.push(link);
    }

    pub fn get_entry(&self, id: &str) -> Option<&Entry> {
        self.entries.iter().find(|e| e.id() == id)
    }

    pub fn get_link(&self, id: &str) -> Option<&Link> {
        self.links.iter().find(|l| l.id == id)
    }

    // ==========================================================================
    // Substrate queries: Order, Position, Point, Line, Coordinate, Segment,
    // Character
    // ==========================================================================

    pub fn order(&self, value: u8) -> Option<&Order> {
        self.entries.iter().find_map(|e| match e {
            Entry::Order(o) if o.value == value => Some(o),
            _ => None,
        })
    }

    pub fn orders(&self) -> Vec<&Order> {
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Order(o) => Some(o),
                _ => None,
            })
            .collect()
    }

    pub fn position(&self, value: u8) -> Option<&Position> {
        self.entries.iter().find_map(|e| match e {
            Entry::Position(p) if p.value == value => Some(p),
            _ => None,
        })
    }

    pub fn positions(&self) -> Vec<&Position> {
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Position(p) => Some(p),
                _ => None,
            })
            .collect()
    }

    pub fn point(&self, order: u8, position: u8) -> Option<&Point> {
        let id = format!("point_{}_{}", order, position);
        self.entries.iter().find_map(|e| match e {
            Entry::Point(p) if p.id == id => Some(p),
            _ => None,
        })
    }

    pub fn points(&self, order: Option<u8>) -> Vec<&Point> {
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Point(p) if order.map(|o| p.order_value() == Some(o)).unwrap_or(true) => {
                    Some(p)
                }
                _ => None,
            })
            .collect()
    }

    pub fn line(&self, order: u8, p1: u8, p2: u8) -> Option<&Line> {
        let (lo, hi) = if p1 <= p2 { (p1, p2) } else { (p2, p1) };
        let id = format!("line_{}_{}_{}", order, lo, hi);
        self.entries.iter().find_map(|e| match e {
            Entry::Line(l) if l.id == id => Some(l),
            _ => None,
        })
    }

    pub fn lines_of(&self, order: Option<u8>) -> Vec<&Line> {
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Line(l) if order.map(|o| l.order_value() == Some(o)).unwrap_or(true) => {
                    Some(l)
                }
                _ => None,
            })
            .collect()
    }

    pub fn coordinate(&self, order: u8, position: u8) -> Option<&Coordinate> {
        let id = format!("coord_{}_{}", order, position);
        self.entries.iter().find_map(|e| match e {
            Entry::Coordinate(c) if c.id == id => Some(c),
            _ => None,
        })
    }

    pub fn coordinates(&self, order: Option<u8>) -> Vec<&Coordinate> {
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Coordinate(c)
                    if order.map(|o| c.order_value() == Some(o)).unwrap_or(true) =>
                {
                    Some(c)
                }
                _ => None,
            })
            .collect()
    }

    pub fn segment(&self, order: u8, p1: u8, p2: u8) -> Option<&Segment> {
        let (lo, hi) = if p1 <= p2 { (p1, p2) } else { (p2, p1) };
        let id = format!("seg_{}_{}_{}", order, lo, hi);
        self.entries.iter().find_map(|e| match e {
            Entry::Segment(s) if s.id == id => Some(s),
            _ => None,
        })
    }

    pub fn character(&self, id: &str) -> Option<&Character> {
        self.entries.iter().find_map(|e| match e {
            Entry::Character(c) if c.id == id => Some(c),
            _ => None,
        })
    }

    pub fn characters(&self, kind: Option<&str>) -> Vec<&Character> {
        self.entries
            .iter()
            .filter_map(|e| match e {
                Entry::Character(c) if kind.map(|k| c.kind == k).unwrap_or(true) => Some(c),
                _ => None,
            })
            .collect()
    }

    // ==========================================================================
    // Vocabulary and Grammar queries + mutations
    // ==========================================================================

    pub fn topological_vocab(&self, id: &str) -> Option<&TopologicalVocabulary> {
        self.topological_vocabs.iter().find(|v| v.id == id)
    }

    pub fn topological_vocab_for_order(&self, order: u8) -> Option<&TopologicalVocabulary> {
        self.topological_vocabs.iter().find(|v| v.order == order)
    }

    pub fn geometric_vocab(&self, id: &str) -> Option<&GeometricVocabulary> {
        self.geometric_vocabs.iter().find(|v| v.id == id)
    }

    pub fn geometric_vocab_for_order(&self, order: u8) -> Option<&GeometricVocabulary> {
        self.geometric_vocabs.iter().find(|v| v.order == order)
    }

    pub fn semantic_vocab(&self, id: &str) -> Option<&SemanticVocabulary> {
        self.semantic_vocabs.iter().find(|v| v.id == id)
    }

    pub fn semantic_vocabs_for_order(&self, order: u8) -> Vec<&SemanticVocabulary> {
        self.semantic_vocabs
            .iter()
            .filter(|v| v.order == order)
            .collect()
    }

    pub fn add_topological_vocab(&mut self, vocab: TopologicalVocabulary) {
        self.topological_vocabs.push(vocab);
    }

    pub fn add_geometric_vocab(&mut self, vocab: GeometricVocabulary) {
        self.geometric_vocabs.push(vocab);
    }

    pub fn add_semantic_vocab(&mut self, vocab: SemanticVocabulary) {
        self.semantic_vocabs.push(vocab);
    }

    pub fn update_semantic_vocab(
        &mut self,
        vocab: SemanticVocabulary,
    ) -> Option<SemanticVocabulary> {
        let idx = self.semantic_vocabs.iter().position(|v| v.id == vocab.id)?;
        Some(std::mem::replace(&mut self.semantic_vocabs[idx], vocab))
    }

    pub fn delete_semantic_vocab(&mut self, id: &str) -> Option<SemanticVocabulary> {
        let idx = self.semantic_vocabs.iter().position(|v| v.id == id)?;
        Some(self.semantic_vocabs.remove(idx))
    }

    pub fn grammar(&self, id: &str) -> Option<&Grammar> {
        self.grammars.iter().find(|g| g.id == id)
    }

    pub fn grammars_for_order(&self, order: u8) -> Vec<&Grammar> {
        self.grammars.iter().filter(|g| g.order == order).collect()
    }

    pub fn add_grammar(&mut self, grammar: Grammar) {
        self.grammars.push(grammar);
    }

    pub fn update_grammar(&mut self, grammar: Grammar) -> Option<Grammar> {
        let idx = self.grammars.iter().position(|g| g.id == grammar.id)?;
        Some(std::mem::replace(&mut self.grammars[idx], grammar))
    }

    pub fn delete_grammar(&mut self, id: &str) -> Option<Grammar> {
        let idx = self.grammars.iter().position(|g| g.id == id)?;
        Some(self.grammars.remove(idx))
    }

    /// Look up which Character inhabits a given Point through a
    /// SemanticVocabulary's paired TopologicalVocabulary.
    pub fn character_at_point(
        &self,
        semantic_vocab_id: &str,
        point_id: &str,
    ) -> Option<&Character> {
        let sv = self.semantic_vocab(semantic_vocab_id)?;
        let topology = self.topological_vocab_for_order(sv.order)?;
        let idx = topology.points.iter().position(|p| p == point_id)?;
        let char_id = sv.terms.get(idx)?;
        self.character(char_id)
    }

    /// Look up which Character inhabits a given Line through a
    /// SemanticVocabulary's paired TopologicalVocabulary.
    pub fn character_at_line(
        &self,
        semantic_vocab_id: &str,
        line_id: &str,
    ) -> Option<&Character> {
        let sv = self.semantic_vocab(semantic_vocab_id)?;
        let topology = self.topological_vocab_for_order(sv.order)?;
        let idx = topology.lines.iter().position(|l| l == line_id)?;
        let char_id = sv.connectives.get(idx)?;
        self.character(char_id)
    }

    /// Validate a Grammar by resolving all three referenced vocabularies.
    pub fn validate_grammar(&self, grammar_id: &str) -> Result<(), Vec<String>> {
        let g = self
            .grammar(grammar_id)
            .ok_or_else(|| vec![format!("Grammar '{}' not found", grammar_id)])?;
        let t = self
            .topological_vocab(&g.topological_vocab_ref)
            .ok_or_else(|| {
                vec![format!(
                    "TopologicalVocabulary '{}' not found",
                    g.topological_vocab_ref
                )]
            })?;
        let geo = self
            .geometric_vocab(&g.geometric_vocab_ref)
            .ok_or_else(|| {
                vec![format!(
                    "GeometricVocabulary '{}' not found",
                    g.geometric_vocab_ref
                )]
            })?;
        let s = self
            .semantic_vocab(&g.semantic_vocab_ref)
            .ok_or_else(|| {
                vec![format!(
                    "SemanticVocabulary '{}' not found",
                    g.semantic_vocab_ref
                )]
            })?;
        g.validate_with(t, geo, s)
    }

    /// Look up the Canonical SemanticVocabulary containing hex colours for
    /// the given order (created by seed as "Canonical Colours {name}").
    pub fn canonical_colour_vocab_for_order(&self, order: u8) -> Option<&SemanticVocabulary> {
        self.semantic_vocabs
            .iter()
            .find(|v| v.order == order && v.name.starts_with("Canonical Colours"))
    }

    // ==========================================================================
    // Line-link queries (rendering shim; `Line` here means the edge Link
    // between coordinates, retained until frontend consumes `Segment`).
    // ==========================================================================

    pub fn lines(&self, order: u8) -> Vec<&Link> {
        self.links
            .iter()
            .filter(|l| {
                if !matches!(l.link_type, LinkType::Line) {
                    return false;
                }
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
    use crate::core::vocabularies::{
        GeometricVocabulary, SemanticVocabulary, TopologicalVocabulary,
    };

    fn triad_test_graph() -> Graph {
        let mut g = Graph::new();
        g.add_entry(Entry::Order(Order::new(3)));
        for pos in 1..=3 {
            g.add_entry(Entry::Position(Position::new(pos)));
            g.add_entry(Entry::Point(Point::new(3, pos)));
            g.add_entry(Entry::Coordinate(Coordinate::new(3, pos, 0.0, 0.0, 0.0)));
        }
        for p1 in 1..=3 {
            for p2 in (p1 + 1)..=3 {
                g.add_entry(Entry::Line(Line::new(3, p1, p2)));
                g.add_entry(Entry::Segment(Segment::new(3, p1, p2)));
            }
        }
        for value in ["Will", "Function", "Being", "Generation", "Decision", "Consent"] {
            g.add_entry(Entry::Character(Character::with_auto_id("word", value)));
        }
        g.add_topological_vocab(TopologicalVocabulary::canonical_for(3));
        g.add_geometric_vocab(GeometricVocabulary::canonical_for(3));
        g.add_semantic_vocab(SemanticVocabulary::with_auto_id(
            "Canonical Triad",
            3,
            vec![
                "char_word_will".into(),
                "char_word_function".into(),
                "char_word_being".into(),
            ],
            vec![
                "char_word_generation".into(),
                "char_word_decision".into(),
                "char_word_consent".into(),
            ],
        ));
        g.add_grammar(Grammar::with_auto_id(
            "Canonical Triad",
            3,
            "Dynamism",
            "Impulses",
            "Acts",
            "topvocab_3",
            "geovocab_3",
            "semvocab_canonical_triad_3",
        ));
        g
    }

    #[test]
    fn test_substrate_queries() {
        let g = triad_test_graph();
        assert!(g.order(3).is_some());
        assert!(g.position(2).is_some());
        assert_eq!(g.point(3, 1).unwrap().id, "point_3_1");
        assert_eq!(g.line(3, 1, 2).unwrap().id, "line_3_1_2");
        assert_eq!(g.coordinate(3, 1).unwrap().point_ref, "point_3_1");
        assert_eq!(g.segment(3, 2, 3).unwrap().line_ref, "line_3_2_3");
        assert_eq!(g.character("char_word_will").unwrap().value, "Will");
    }

    #[test]
    fn test_character_at_point_and_line_via_join() {
        let g = triad_test_graph();
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
    fn test_grammar_validate_via_graph() {
        let g = triad_test_graph();
        assert!(g.validate_grammar("grammar_canonical_triad_3").is_ok());
    }
}
