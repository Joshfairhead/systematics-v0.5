//! Sequence: an ordered series of member addresses.
//!
//! The `+` pole of the container triad **System · Sequence · Perspective** (see
//! `docs/design-intent.md`). A System is one `K_n`; a Perspective is the
//! reconciling web; a Sequence is the bare *ordered skeleton* — an ordered list
//! of member **addresses** (`system:<id>`, `perspective:<id>`, or `sequence:<id>`
//! so sequences nest). Pure structure: it carries no references or excerpts of
//! its own (those live on a Perspective).

use serde::{Deserialize, Serialize};

/// An ordered series of member addresses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sequence {
    pub id: String,
    pub name: String,
    /// Ordered member addresses. OrderCardinality = Vec ordinality.
    #[serde(default)]
    pub members: Vec<String>,
}

impl Sequence {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        members: Vec<String>,
    ) -> Self {
        Self { id: id.into(), name: name.into(), members }
    }

    /// Build with an id derived from the name: `sequence_<slug>`.
    pub fn with_auto_id(name: impl Into<String>, members: Vec<String>) -> Self {
        let name = name.into();
        let slug: String = name
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '_' })
            .collect();
        Self::new(format!("sequence_{slug}"), name, members)
    }

    pub fn len(&self) -> usize {
        self.members.len()
    }

    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_id_slugs_the_name() {
        let s = Sequence::with_auto_id(
            "Architecture Run",
            vec!["system:a".into(), "system:b".into()],
        );
        assert_eq!(s.id, "sequence_architecture_run");
        assert_eq!(s.len(), 2);
        assert!(!s.is_empty());
    }

    #[test]
    fn members_keep_order() {
        let s = Sequence::new("sequence_x", "X", vec!["c".into(), "a".into(), "b".into()]);
        assert_eq!(s.members, vec!["c", "a", "b"]);
    }
}
