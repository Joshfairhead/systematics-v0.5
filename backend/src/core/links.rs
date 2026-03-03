//! Link types for the property graph.
//!
//! Links are explicit relationships between entries.
//! They connect entries via base (source) and target IDs.

use serde::{Deserialize, Serialize};

/// LinkType defines the kind of relationship between entries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LinkType {
    /// Line connects Coordinate → Coordinate (geometric edge)
    Line,
    /// Connective connects Location → Location (simplex-anchored)
    /// Character ID stored in Link's `tag` field
    Connective,
}

/// Link is an explicit relationship between entries.
/// Supports multiple sources and targets for future morphism types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Link {
    pub id: String,
    /// Entry ID(s) of the source(s)
    pub base: Option<Vec<String>>,
    /// Entry ID(s) of the target(s)
    pub target: Option<Vec<String>>,
    /// Type of the link
    pub link_type: LinkType,
    /// Optional payload/tag
    pub tag: Option<String>,
}

impl Link {
    /// Create a new link with optional multiple bases and targets
    pub fn new(
        id: impl Into<String>,
        base: Option<Vec<String>>,
        target: Option<Vec<String>>,
        link_type: LinkType,
    ) -> Self {
        Self {
            id: id.into(),
            base,
            target,
            link_type,
            tag: None,
        }
    }

    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }

    /// Create a Line link between two coordinates
    pub fn line(base: impl Into<String>, target: impl Into<String>) -> Self {
        let base = base.into();
        let target = target.into();
        let id = format!("line_{}_{}", base, target);
        Self::new(id, Some(vec![base]), Some(vec![target]), LinkType::Line)
    }

    /// Create a Connective link between two locations (simplex-anchored)
    /// Use `.with_tag(character_id)` to set the label character
    pub fn connective(base: impl Into<String>, target: impl Into<String>) -> Self {
        let base = base.into();
        let target = target.into();
        let id = format!("conn_{}_{}", base, target);
        Self::new(
            id,
            Some(vec![base]),
            Some(vec![target]),
            LinkType::Connective,
        )
    }

    // =========================================================================
    // Helper methods for accessing base/target
    // =========================================================================

    /// Get the first base ID (for single-base links)
    pub fn base_single(&self) -> Option<&str> {
        self.base
            .as_ref()
            .and_then(|v| v.first().map(|s| s.as_str()))
    }

    /// Get the first target ID (for single-target links)
    pub fn target_single(&self) -> Option<&str> {
        self.target
            .as_ref()
            .and_then(|v| v.first().map(|s| s.as_str()))
    }

    /// Get all base IDs
    pub fn bases(&self) -> &[String] {
        self.base.as_deref().unwrap_or(&[])
    }

    /// Get all target IDs
    pub fn targets(&self) -> &[String] {
        self.target.as_deref().unwrap_or(&[])
    }

    /// Check if this is a connective link
    pub fn is_connective(&self) -> bool {
        matches!(self.link_type, LinkType::Connective)
    }

    /// Get the character ID (from tag field) if this is a connective link
    pub fn character_id(&self) -> Option<&str> {
        if self.is_connective() {
            self.tag.as_deref()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_link() {
        let link = Link::line("coord_3_1", "coord_3_2");
        assert_eq!(link.base_single(), Some("coord_3_1"));
        assert_eq!(link.target_single(), Some("coord_3_2"));
        assert!(matches!(link.link_type, LinkType::Line));
    }

    #[test]
    fn test_connective_link() {
        let link = Link::connective("loc_3_1", "loc_3_2").with_tag("char_canonical_generation");
        assert!(link.is_connective());
        assert_eq!(link.character_id(), Some("char_canonical_generation"));
        assert_eq!(link.base_single(), Some("loc_3_1"));
        assert_eq!(link.target_single(), Some("loc_3_2"));
    }

    #[test]
    fn test_link_with_tag() {
        let link = Link::line("a", "b").with_tag("my_tag");
        assert_eq!(link.tag, Some("my_tag".to_string()));
    }

    #[test]
    fn test_bases_and_targets() {
        let link = Link::line("coord_1", "coord_2");
        assert_eq!(link.bases(), &["coord_1".to_string()]);
        assert_eq!(link.targets(), &["coord_2".to_string()]);
    }

    #[test]
    fn test_empty_base_target() {
        let link = Link::new("test_id", None, None, LinkType::Line);
        assert_eq!(link.base_single(), None);
        assert_eq!(link.target_single(), None);
        assert!(link.bases().is_empty());
        assert!(link.targets().is_empty());
    }
}
