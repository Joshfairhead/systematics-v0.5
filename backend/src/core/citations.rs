//! Citation triad: Source / Artefact / Lookup + Reference.
//!
//! Provenance is modelled as a Bennett triad of concrete entities:
//! - `Source`   — the origin/author of information (e.g. "J.G. Bennett").
//! - `Artefact` — the work the source produced (a book, paper, url).
//! - `Lookup`   — the specific location within the artefact (page, chapter…).
//!
//! A `Reference` is an edge (belonging to a Perspective) that ties a cited
//! **target** (any Expression address — a term, connective, System, or series)
//! to its Source, and optionally the Artefact + Lookup that locate it. Every
//! entity carries its own id, so a Reference is itself addressable/linkable.

use serde::{Deserialize, Serialize};

fn slug(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect()
}

/// The origin/author of information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Source {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

impl Source {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        kind: Option<String>,
        description: Option<String>,
    ) -> Self {
        Self { id: id.into(), name: name.into(), kind, description }
    }

    pub fn with_auto_id(name: impl Into<String>, kind: Option<String>, description: Option<String>) -> Self {
        let name = name.into();
        Self::new(format!("source_{}", slug(&name)), name, kind, description)
    }
}

/// The referenced work (book/paper/url) produced by a Source.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Artefact {
    pub id: String,
    pub source_ref: String,
    pub title: String,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

impl Artefact {
    pub fn new(
        id: impl Into<String>,
        source_ref: impl Into<String>,
        title: impl Into<String>,
        kind: Option<String>,
        url: Option<String>,
    ) -> Self {
        Self { id: id.into(), source_ref: source_ref.into(), title: title.into(), kind, url }
    }

    pub fn with_auto_id(
        source_ref: impl Into<String>,
        title: impl Into<String>,
        kind: Option<String>,
        url: Option<String>,
    ) -> Self {
        let title = title.into();
        Self::new(format!("artefact_{}", slug(&title)), source_ref, title, kind, url)
    }
}

/// A specific location within an Artefact (page, chapter, verse, timestamp…).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lookup {
    pub id: String,
    pub artefact_ref: String,
    pub locator: String,
}

impl Lookup {
    pub fn new(
        id: impl Into<String>,
        artefact_ref: impl Into<String>,
        locator: impl Into<String>,
    ) -> Self {
        Self { id: id.into(), artefact_ref: artefact_ref.into(), locator: locator.into() }
    }

    pub fn with_auto_id(artefact_ref: impl Into<String>, locator: impl Into<String>) -> Self {
        let artefact_ref = artefact_ref.into();
        let locator = locator.into();
        Self::new(
            format!("lookup_{}_{}", slug(&artefact_ref), slug(&locator)),
            artefact_ref,
            locator,
        )
    }
}

/// A citation edge within a Perspective: ties a cited `target` address to its
/// provenance (Source, and optionally Artefact + Lookup).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reference {
    pub id: String,
    /// The Perspective (web) this reference belongs to.
    pub perspective_ref: String,
    /// The cited Expression address (term / connective / system / series…).
    pub target: String,
    pub source_ref: String,
    #[serde(default)]
    pub artefact_ref: Option<String>,
    #[serde(default)]
    pub lookup_ref: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
    /// The **object** of the SPO triple this reference asserts: the *value* the
    /// source affixes to `target` (the predicate). E.g. target `…#coherence`,
    /// object `"Relatedness"` (per DU1). `None` = the reference only points, it
    /// does not (yet) carry its asserted value. (The #25 reference-tuple field.)
    #[serde(default)]
    pub object: Option<String>,
}

impl Reference {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: impl Into<String>,
        perspective_ref: impl Into<String>,
        target: impl Into<String>,
        source_ref: impl Into<String>,
        artefact_ref: Option<String>,
        lookup_ref: Option<String>,
        note: Option<String>,
    ) -> Self {
        Self {
            id: id.into(),
            perspective_ref: perspective_ref.into(),
            target: target.into(),
            source_ref: source_ref.into(),
            artefact_ref,
            lookup_ref,
            note,
            object: None,
        }
    }

    /// Set the asserted **object** (value) of this reference (the #25 field).
    pub fn with_object(mut self, object: Option<String>) -> Self {
        self.object = object;
        self
    }

    /// Deterministic id from perspective + target + source.
    pub fn with_auto_id(
        perspective_ref: impl Into<String>,
        target: impl Into<String>,
        source_ref: impl Into<String>,
        artefact_ref: Option<String>,
        lookup_ref: Option<String>,
        note: Option<String>,
    ) -> Self {
        let perspective_ref = perspective_ref.into();
        let target = target.into();
        let source_ref = source_ref.into();
        let id = format!("reference_{}_{}_{}", slug(&perspective_ref), slug(&target), slug(&source_ref));
        Self::new(id, perspective_ref, target, source_ref, artefact_ref, lookup_ref, note)
    }
}
