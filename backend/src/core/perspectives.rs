//! Perspective: an AD4M-style directed web of Links.
//!
//! Unlike a `Grammar` (a complete, undirected K_n) a `Perspective` is a
//! *not-necessarily-complete, directed* graph — a web of `Link`s. Each Link is
//! an RDF-like `{ source, predicate, target }` triple whose endpoints are
//! Expression *addresses* (URIs). Nodes can be anything addressable: a term, a
//! connective, a whole System, a Vocabulary, a Grammar, a citation entity, or
//! even another Perspective (webs of webs). See `address` for the URI scheme.
//!
//! The referencing/citation system IS a Perspective (see `citations`).

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

/// A directed edge in a Perspective: an AD4M `{ source, predicate, target }`
/// triple over Expression addresses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Link {
    pub id: String,
    pub source: String,
    pub predicate: String,
    pub target: String,
}

impl Link {
    pub fn new(
        source: impl Into<String>,
        predicate: impl Into<String>,
        target: impl Into<String>,
    ) -> Self {
        let source = source.into();
        let predicate = predicate.into();
        let target = target.into();
        let id = Self::compute_id(&source, &predicate, &target);
        Self { id, source, predicate, target }
    }

    /// Deterministic id from the triple, so the same link is idempotent and
    /// removable without a separate handle.
    fn compute_id(source: &str, predicate: &str, target: &str) -> String {
        let mut h = DefaultHasher::new();
        source.hash(&mut h);
        predicate.hash(&mut h);
        target.hash(&mut h);
        format!("link_{:016x}", h.finish())
    }
}

/// A Perspective: a named web of Links.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Perspective {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub links: Vec<Link>,
}

impl Perspective {
    pub fn new(id: impl Into<String>, name: impl Into<String>, links: Vec<Link>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            links,
        }
    }

    /// Build an id from the name: `perspective_{slug}`.
    pub fn with_auto_id(name: impl Into<String>, links: Vec<Link>) -> Self {
        let name = name.into();
        let slug = name.to_lowercase().replace(' ', "_");
        Self::new(format!("perspective_{}", slug), name, links)
    }

    /// Add a link (idempotent by triple identity).
    pub fn add_link(&mut self, link: Link) {
        if !self.links.iter().any(|l| l.id == link.id) {
            self.links.push(link);
        }
    }

    /// Remove a link by its id. Returns true if one was removed.
    pub fn remove_link(&mut self, link_id: &str) -> bool {
        let before = self.links.len();
        self.links.retain(|l| l.id != link_id);
        self.links.len() != before
    }
}

/// Expression addressing — canonical URIs so every element is individually
/// citable/linkable. Reuses the existing structured ids.
pub mod address {
    /// A whole System instance: `system:<system_id>`.
    pub fn system(system_id: &str) -> String {
        format!("system:{}", system_id)
    }

    /// An individual term (node) within a System: `system:<system_id>#term:<position>`.
    pub fn term(system_id: &str, position: u8) -> String {
        format!("system:{}#term:{}", system_id, position)
    }

    /// A connective (edge) within a System: `system:<system_id>#conn:<p1>-<p2>`.
    pub fn connective(system_id: &str, p1: u8, p2: u8) -> String {
        format!("system:{}#conn:{}-{}", system_id, p1, p2)
    }

    /// A Grammar (structure): `grammar:<order>`.
    pub fn grammar(order: u8) -> String {
        format!("grammar:{}", order)
    }

    /// A Vocabulary: `vocab:<vocab_id>`.
    pub fn vocabulary(vocab_id: &str) -> String {
        format!("vocab:{}", vocab_id)
    }

    /// A citation Source / Artefact / Lookup node.
    pub fn source(id: &str) -> String {
        format!("source:{}", id)
    }
    pub fn artefact(id: &str) -> String {
        format!("artefact:{}", id)
    }
    pub fn lookup(id: &str) -> String {
        format!("lookup:{}", id)
    }

    /// A Perspective (a web — e.g. a named series of systems): `perspective:<id>`.
    pub fn perspective(id: &str) -> String {
        format!("perspective:{}", id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_id_is_deterministic() {
        let a = Link::new("system:s#term:1", "cites", "source:x");
        let b = Link::new("system:s#term:1", "cites", "source:x");
        assert_eq!(a.id, b.id);
        let c = Link::new("system:s#term:2", "cites", "source:x");
        assert_ne!(a.id, c.id);
    }

    #[test]
    fn test_add_remove_link() {
        let mut p = Perspective::with_auto_id("Sources", vec![]);
        assert_eq!(p.id, "perspective_sources");
        let l = Link::new("system:s", "cites", "source:x");
        let lid = l.id.clone();
        p.add_link(l.clone());
        p.add_link(l); // idempotent
        assert_eq!(p.links.len(), 1);
        assert!(p.remove_link(&lid));
        assert!(p.links.is_empty());
    }

    #[test]
    fn test_addresses() {
        assert_eq!(address::term("system_canonical_triad_3", 1), "system:system_canonical_triad_3#term:1");
        assert_eq!(address::connective("s", 1, 3), "system:s#conn:1-3");
    }
}
