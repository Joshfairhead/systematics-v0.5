//! Three parallel Vocabulary types over the K_n substrate.
//!
//! Each Vocabulary is a per-OrderCardinality first-class container whose ordered lists
//! align by index: a term-shape slot at index `i` in one vocabulary
//! corresponds to the term-shape slot at index `i` in another.
//!
//! - `Topology` — ordered `points` and `lines` for one OrderCardinality.
//! - `Geometry` — ordered `coordinates` and `segments`.
//! - `Vocabulary` — ordered `terms` and `connectives` (Character refs).
//!
//! The recursive framing: Systematics is a *language*, Perspective is the *rules*,
//! and every content layer — topology, geometry, semantics — is a Vocabulary.

use serde::{Deserialize, Serialize};

/// A vocabulary of topological anchors for one OrderCardinality (K_n).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Topology {
    pub id: String,          // "topvocab_{order_cardinality}"
    #[serde(rename = "order")]
    pub order_cardinality: u8,           // K_n size
    pub points: Vec<String>, // ordered PointRef IDs, len == order_cardinality
    pub lines: Vec<String>,  // ordered LineRef IDs, len == C(order_cardinality, 2)
}

impl Topology {
    pub fn new(order_cardinality: u8, points: Vec<String>, lines: Vec<String>) -> Self {
        Self {
            id: format!("topvocab_{}", order_cardinality),
            order_cardinality,
            points,
            lines,
        }
    }

    /// Build the canonical topology for an order_cardinality: points 1..=order_cardinality,
    /// lines by every (p1, p2) with p1 < p2.
    pub fn canonical_for(order_cardinality: u8) -> Self {
        let points: Vec<String> = (1..=order_cardinality)
            .map(|p| format!("point_{}_{}", order_cardinality, p))
            .collect();
        let mut lines = Vec::new();
        for p1 in 1..=order_cardinality {
            for p2 in (p1 + 1)..=order_cardinality {
                lines.push(format!("line_{}_{}_{}", order_cardinality, p1, p2));
            }
        }
        Self::new(order_cardinality, points, lines)
    }

    /// Arity checks: correct number of point and line entries.
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errs = Vec::new();
        let expected_points = self.order_cardinality as usize;
        let expected_lines = expected_points * (expected_points.saturating_sub(1)) / 2;
        if self.points.len() != expected_points {
            errs.push(format!(
                "Topology {}: expected {} points, got {}",
                self.id,
                expected_points,
                self.points.len()
            ));
        }
        if self.lines.len() != expected_lines {
            errs.push(format!(
                "Topology {}: expected {} lines, got {}",
                self.id,
                expected_lines,
                self.lines.len()
            ));
        }
        if errs.is_empty() {
            Ok(())
        } else {
            Err(errs)
        }
    }
}

/// A vocabulary of geometric anchors for one OrderCardinality.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Geometry {
    pub id: String,               // "geovocab_{order_cardinality}"
    #[serde(rename = "order")]
    pub order_cardinality: u8,                // K_n size
    pub coordinates: Vec<String>, // ordered CoordinateRef IDs, len == order_cardinality
    pub segments: Vec<String>,    // ordered SegmentRef IDs, len == C(order_cardinality, 2)
}

impl Geometry {
    pub fn new(order_cardinality: u8, coordinates: Vec<String>, segments: Vec<String>) -> Self {
        Self {
            id: format!("geovocab_{}", order_cardinality),
            order_cardinality,
            coordinates,
            segments,
        }
    }

    /// Build the canonical geometry vocabulary for an order_cardinality (ID lists
    /// paralleling the canonical topology).
    pub fn canonical_for(order_cardinality: u8) -> Self {
        let coordinates: Vec<String> = (1..=order_cardinality)
            .map(|p| format!("coord_{}_{}", order_cardinality, p))
            .collect();
        let mut segments = Vec::new();
        for p1 in 1..=order_cardinality {
            for p2 in (p1 + 1)..=order_cardinality {
                segments.push(format!("seg_{}_{}_{}", order_cardinality, p1, p2));
            }
        }
        Self::new(order_cardinality, coordinates, segments)
    }

    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errs = Vec::new();
        let expected_coords = self.order_cardinality as usize;
        let expected_segments = expected_coords * (expected_coords.saturating_sub(1)) / 2;
        if self.coordinates.len() != expected_coords {
            errs.push(format!(
                "Geometry {}: expected {} coordinates, got {}",
                self.id,
                expected_coords,
                self.coordinates.len()
            ));
        }
        if self.segments.len() != expected_segments {
            errs.push(format!(
                "Geometry {}: expected {} segments, got {}",
                self.id,
                expected_segments,
                self.segments.len()
            ));
        }
        if errs.is_empty() {
            Ok(())
        } else {
            Err(errs)
        }
    }
}

/// A vocabulary of semantic content — ordered term and connective Characters
/// for one OrderCardinality. Multiple Vocabularies per OrderCardinality coexist (Canonical
/// Triad, Theology Triad, …).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vocabulary {
    pub id: String,              // "vocab_{slug}_{order_cardinality}"
    pub name: String,            // e.g. "Canonical Triad", "Theology Triad"
    #[serde(rename = "order")]
    pub order_cardinality: u8,               // K_n size
    pub terms: Vec<String>,      // ordered CharacterRef IDs, [i] inhabits topology.points[i]
    pub connectives: Vec<String>, // ordered CharacterRef IDs, [i] inhabits topology.lines[i]
}

impl Vocabulary {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        order_cardinality: u8,
        terms: Vec<String>,
        connectives: Vec<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            order_cardinality,
            terms,
            connectives,
        }
    }

    /// Convenience constructor that derives an ID from name + order_cardinality.
    pub fn with_auto_id(
        name: impl Into<String>,
        order_cardinality: u8,
        terms: Vec<String>,
        connectives: Vec<String>,
    ) -> Self {
        let name = name.into();
        let slug = name.to_lowercase().replace(' ', "_");
        Self {
            id: format!("vocab_{}_{}", slug, order_cardinality),
            name,
            order_cardinality,
            terms,
            connectives,
        }
    }

    /// Arity check against a paired Topology.
    pub fn validate_against(&self, topology: &Topology) -> Result<(), Vec<String>> {
        let mut errs = Vec::new();
        if self.order_cardinality != topology.order_cardinality {
            errs.push(format!(
                "Vocabulary {}: order_cardinality {} doesn't match topology order_cardinality {}",
                self.id, self.order_cardinality, topology.order_cardinality
            ));
        }
        if self.terms.len() != topology.points.len() {
            errs.push(format!(
                "Vocabulary {}: expected {} terms (topology.points), got {}",
                self.id,
                topology.points.len(),
                self.terms.len()
            ));
        }
        if self.connectives.len() != topology.lines.len() {
            errs.push(format!(
                "Vocabulary {}: expected {} connectives (topology.lines), got {}",
                self.id,
                topology.lines.len(),
                self.connectives.len()
            ));
        }
        if errs.is_empty() {
            Ok(())
        } else {
            Err(errs)
        }
    }

    /// Standalone arity check (order_cardinality-only), useful when no topology is yet
    /// available to compare against.
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errs = Vec::new();
        let expected_terms = self.order_cardinality as usize;
        let expected_connectives = expected_terms * (expected_terms.saturating_sub(1)) / 2;
        if self.terms.len() != expected_terms {
            errs.push(format!(
                "Vocabulary {}: expected {} terms, got {}",
                self.id,
                expected_terms,
                self.terms.len()
            ));
        }
        if self.connectives.len() != expected_connectives {
            errs.push(format!(
                "Vocabulary {}: expected {} connectives, got {}",
                self.id,
                expected_connectives,
                self.connectives.len()
            ));
        }
        if errs.is_empty() {
            Ok(())
        } else {
            Err(errs)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topological_canonical_triad() {
        let t = Topology::canonical_for(3);
        assert_eq!(t.id, "topvocab_3");
        assert_eq!(t.points, vec!["point_3_1", "point_3_2", "point_3_3"]);
        assert_eq!(
            t.lines,
            vec!["line_3_1_2", "line_3_1_3", "line_3_2_3"]
        );
        assert!(t.validate().is_ok());
    }

    #[test]
    fn test_geometric_canonical_triad() {
        let g = Geometry::canonical_for(3);
        assert_eq!(g.id, "geovocab_3");
        assert_eq!(g.coordinates, vec!["coord_3_1", "coord_3_2", "coord_3_3"]);
        assert_eq!(g.segments, vec!["seg_3_1_2", "seg_3_1_3", "seg_3_2_3"]);
        assert!(g.validate().is_ok());
    }

    #[test]
    fn test_semantic_validate_against_topology() {
        let t = Topology::canonical_for(3);
        let s = Vocabulary::with_auto_id(
            "Canonical Triad",
            3,
            vec![
                "char_word_will".into(),
                "char_word_function".into(),
                "char_word_being".into(),
            ],
            vec![
                "char_word_generation".into(),
                "char_word_consent".into(),
                "char_word_decision".into(),
            ],
        );
        assert_eq!(s.id, "vocab_canonical_triad_3");
        assert!(s.validate().is_ok());
        assert!(s.validate_against(&t).is_ok());
    }

    #[test]
    fn test_semantic_validate_catches_arity_mismatch() {
        let t = Topology::canonical_for(3);
        let s = Vocabulary::with_auto_id(
            "Bad Triad",
            3,
            vec!["char_a".into(), "char_b".into()], // only 2 terms — should be 3
            vec![
                "char_c".into(),
                "char_d".into(),
                "char_e".into(),
            ],
        );
        let errs = s.validate_against(&t).unwrap_err();
        assert!(errs.iter().any(|e| e.contains("expected 3 terms")));
    }

    #[test]
    fn test_topological_arity_by_order() {
        let dodecad = Topology::canonical_for(12);
        assert_eq!(dodecad.points.len(), 12);
        assert_eq!(dodecad.lines.len(), 66); // C(12,2)
        assert!(dodecad.validate().is_ok());
    }
}
