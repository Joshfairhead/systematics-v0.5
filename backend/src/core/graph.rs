//! Graph structure and query methods.
//!
//! The Graph holds substrate entries + the four higher-level tables
//! (topological, geometric, semantic vocabularies, and perspectives).

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::citations::{Artefact, Lookup, Reference, Source};
use super::content::GraphContent;
use super::entries::{Character, Coordinate, Entry, Line, Order, Point, Position, Segment};
use super::functors::Functor;
use super::grammar::GraphTemplate;
use super::sequences::Sequence;
use super::perspectives::Perspective;
use super::systems::System;
use super::vocabularies::{Geometry, Vocabulary, Topology};

/// The primary container. Holds substrate entries and the four higher-level tables.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Graph {
    pub entries: Vec<Entry>,
    #[serde(default)]
    pub topological_vocabs: Vec<Topology>,
    #[serde(default)]
    pub geometric_vocabs: Vec<Geometry>,
    #[serde(default)]
    pub vocabularies: Vec<Vocabulary>,
    /// Complete-graph structures, one per Order. Seeded in code (deterministic),
    /// never persisted as content.
    #[serde(default)]
    pub grammars: Vec<GraphTemplate>,
    /// Systems: metadata reconciling a GraphTemplate with a Vocabulary.
    #[serde(default)]
    pub systems: Vec<System>,
    /// Referencing layer: AD4M-style directed webs of Links.
    #[serde(default)]
    pub perspectives: Vec<Perspective>,
    /// Citation triad entities + Reference edges.
    #[serde(default)]
    pub sources: Vec<Source>,
    #[serde(default)]
    pub artefacts: Vec<Artefact>,
    #[serde(default)]
    pub lookups: Vec<Lookup>,
    #[serde(default)]
    pub references: Vec<Reference>,
    /// Same-grammar functors: position permutations between systems of one Order.
    #[serde(default)]
    pub functors: Vec<Functor>,
    /// Sequences: ordered series of member addresses (container triad `+` pole).
    #[serde(default)]
    pub sequences: Vec<Sequence>,
    /// IDs of the immutable canonical *archetypes* (the seed: the 12 per-order
    /// systems + the citation triad + their vocabularies/characters). Runtime
    /// only, never serialised. This is the "do not copy me into a module file"
    /// set used by `export_perspective` — it must NOT include module content, or
    /// re-exporting a loaded module would drop the module's own systems.
    #[serde(skip)]
    canonical_ids: HashSet<String>,
    /// IDs of everything sourced from a *file* (the canonical seed AND the loaded
    /// perspective modules). Runtime only, never serialised. This is the "already
    /// durable elsewhere, so don't write me to the user store" set used by
    /// `user_content`. A superset of `canonical_ids`. Module content is bundled
    /// (durable) but *not* canonical (still editable / re-exportable).
    #[serde(skip)]
    bundled_ids: HashSet<String>,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    pub fn get_entry(&self, id: &str) -> Option<&Entry> {
        self.entries.iter().find(|e| e.id() == id)
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
    // Vocabulary and Perspective queries + mutations
    // ==========================================================================

    pub fn topological_vocab(&self, id: &str) -> Option<&Topology> {
        self.topological_vocabs.iter().find(|v| v.id == id)
    }

    pub fn topological_vocab_for_order(&self, order: u8) -> Option<&Topology> {
        self.topological_vocabs.iter().find(|v| v.order == order)
    }

    pub fn geometric_vocab(&self, id: &str) -> Option<&Geometry> {
        self.geometric_vocabs.iter().find(|v| v.id == id)
    }

    pub fn geometric_vocab_for_order(&self, order: u8) -> Option<&Geometry> {
        self.geometric_vocabs.iter().find(|v| v.order == order)
    }

    pub fn vocabulary(&self, id: &str) -> Option<&Vocabulary> {
        self.vocabularies.iter().find(|v| v.id == id)
    }

    pub fn vocabularies_for_order(&self, order: u8) -> Vec<&Vocabulary> {
        self.vocabularies
            .iter()
            .filter(|v| v.order == order)
            .collect()
    }

    pub fn add_topological_vocab(&mut self, vocab: Topology) {
        self.topological_vocabs.push(vocab);
    }

    pub fn add_geometric_vocab(&mut self, vocab: Geometry) {
        self.geometric_vocabs.push(vocab);
    }

    pub fn add_vocabulary(&mut self, vocab: Vocabulary) {
        self.vocabularies.push(vocab);
    }

    pub fn update_vocabulary(
        &mut self,
        vocab: Vocabulary,
    ) -> Option<Vocabulary> {
        let idx = self.vocabularies.iter().position(|v| v.id == vocab.id)?;
        Some(std::mem::replace(&mut self.vocabularies[idx], vocab))
    }

    pub fn delete_vocabulary(&mut self, id: &str) -> Option<Vocabulary> {
        let idx = self.vocabularies.iter().position(|v| v.id == id)?;
        Some(self.vocabularies.remove(idx))
    }

    // -------- Grammars (K_n structure; code-seeded, deterministic) --------

    pub fn grammar(&self, id: &str) -> Option<&GraphTemplate> {
        self.grammars.iter().find(|g| g.id == id)
    }

    pub fn grammar_for_order(&self, order: u8) -> Option<&GraphTemplate> {
        self.grammars.iter().find(|g| g.order == order)
    }

    pub fn add_grammar(&mut self, grammar: GraphTemplate) {
        self.grammars.push(grammar);
    }

    // -------- Systems (metadata + GraphTemplate/Vocabulary reconciliation) --------

    pub fn system(&self, id: &str) -> Option<&System> {
        self.systems.iter().find(|s| s.id == id)
    }

    pub fn systems_for_order(&self, order: u8) -> Vec<&System> {
        self.systems.iter().filter(|s| s.order == order).collect()
    }

    pub fn add_system(&mut self, system: System) {
        self.systems.push(system);
    }

    pub fn update_system(&mut self, system: System) -> Option<System> {
        let idx = self.systems.iter().position(|s| s.id == system.id)?;
        Some(std::mem::replace(&mut self.systems[idx], system))
    }

    pub fn delete_system(&mut self, id: &str) -> Option<System> {
        let idx = self.systems.iter().position(|s| s.id == id)?;
        Some(self.systems.remove(idx))
    }

    // -------- Functors (same-grammar morphisms between Systems) --------

    pub fn functor(&self, id: &str) -> Option<&Functor> {
        self.functors.iter().find(|f| f.id == id)
    }

    pub fn functors(&self) -> &[Functor] {
        &self.functors
    }

    pub fn add_functor(&mut self, functor: Functor) {
        self.functors.push(functor);
    }

    pub fn update_functor(&mut self, functor: Functor) -> Option<Functor> {
        let idx = self.functors.iter().position(|f| f.id == functor.id)?;
        Some(std::mem::replace(&mut self.functors[idx], functor))
    }

    pub fn delete_functor(&mut self, id: &str) -> Option<Functor> {
        let idx = self.functors.iter().position(|f| f.id == id)?;
        Some(self.functors.remove(idx))
    }

    /// Advisory validation of a stored Functor, resolved against the graph
    /// (mirrors `validate_system`): the permutation laws hold, both referenced
    /// systems exist, and all three orders agree. Returns the reasons on failure.
    pub fn validate_functor(&self, id: &str) -> Result<(), Vec<String>> {
        let Some(f) = self.functor(id) else {
            return Err(vec![format!("Functor '{id}' not found")]);
        };
        let mut errs = match f.validate() {
            Ok(()) => Vec::new(),
            Err(e) => e,
        };
        for (role, sref) in [("source", &f.source_ref), ("target", &f.target_ref)] {
            match self.system(sref) {
                Some(s) if s.order != f.order => errs.push(format!(
                    "Functor {}: {} system '{}' has order {}, expected {}",
                    f.id, role, sref, s.order, f.order
                )),
                Some(_) => {}
                None => errs.push(format!(
                    "Functor {}: {} system '{}' not found",
                    f.id, role, sref
                )),
            }
        }
        if errs.is_empty() {
            Ok(())
        } else {
            Err(errs)
        }
    }

    /// Apply a functor to a source Perspective, materialising a *new* Perspective
    /// whose links are the source's with every address over the functor's source
    /// system remapped to the target system (`Transform(perspective, functor)`).
    /// Pure — computes but does not store the result; returns `None` if either
    /// the functor or the perspective is missing.
    pub fn apply_functor(
        &self,
        functor_id: &str,
        perspective_id: &str,
        new_id: impl Into<String>,
        new_name: impl Into<String>,
    ) -> Option<Perspective> {
        let f = self.functor(functor_id)?;
        let src = self.perspective(perspective_id)?;
        let new_id = new_id.into();
        let links = src
            .links
            .iter()
            .map(|l| super::perspectives::Link {
                id: format!("{}_{}", new_id, l.id),
                source: f.map_address(&l.source),
                predicate: l.predicate.clone(),
                target: f.map_address(&l.target),
            })
            .collect();
        Some(Perspective {
            id: new_id,
            name: new_name.into(),
            links,
        })
    }

    // -------- Sequences (ordered series of member addresses) --------

    pub fn sequence(&self, id: &str) -> Option<&Sequence> {
        self.sequences.iter().find(|s| s.id == id)
    }

    pub fn sequences(&self) -> &[Sequence] {
        &self.sequences
    }

    pub fn add_sequence(&mut self, sequence: Sequence) {
        self.sequences.push(sequence);
    }

    /// A sequence id free in the graph: `base` if unused, else `base_2`, `base_3`,
    /// … So repeated Extracts of the same scope (which slug to the same base id
    /// from their auto-name) each get a distinct Monad rather than colliding.
    pub fn unique_sequence_id(&self, base: &str) -> String {
        if self.sequence(base).is_none() {
            return base.to_string();
        }
        let mut n = 2;
        loop {
            let candidate = format!("{base}_{n}");
            if self.sequence(&candidate).is_none() {
                return candidate;
            }
            n += 1;
        }
    }

    pub fn update_sequence(&mut self, sequence: Sequence) -> Option<Sequence> {
        let idx = self.sequences.iter().position(|s| s.id == sequence.id)?;
        Some(std::mem::replace(&mut self.sequences[idx], sequence))
    }

    pub fn delete_sequence(&mut self, id: &str) -> Option<Sequence> {
        let idx = self.sequences.iter().position(|s| s.id == id)?;
        Some(self.sequences.remove(idx))
    }

    /// Does an address resolve to an entity currently in the graph? Used by the
    /// Load primitive to warn about dangling manifest dependencies (which are
    /// tolerated, per the referential model, not errors).
    pub fn resolves(&self, address: &str) -> bool {
        let Some((kind, rest)) = address.split_once(':') else {
            return false;
        };
        let id = rest.split('#').next().unwrap_or(rest);
        match kind {
            "system" => self.system(id).is_some(),
            "perspective" => self.perspective(id).is_some(),
            "sequence" => self.sequence(id).is_some(),
            "vocab" => self.vocabulary(id).is_some(),
            "source" => self.source(id).is_some(),
            "artefact" => self.artefact(id).is_some(),
            "lookup" => self.lookup(id).is_some(),
            "grammar" => id
                .parse::<u8>()
                .ok()
                .is_some_and(|o| self.grammar_for_order(o).is_some()),
            _ => false,
        }
    }

    // ==========================================================================
    // Referencing layer: Perspectives (webs) + citation triad
    // ==========================================================================

    pub fn perspective(&self, id: &str) -> Option<&Perspective> {
        self.perspectives.iter().find(|p| p.id == id)
    }

    pub fn perspectives(&self) -> &[Perspective] {
        &self.perspectives
    }

    pub fn add_perspective(&mut self, perspective: Perspective) {
        self.perspectives.push(perspective);
    }

    pub fn update_perspective(&mut self, perspective: Perspective) -> Option<Perspective> {
        let idx = self.perspectives.iter().position(|p| p.id == perspective.id)?;
        Some(std::mem::replace(&mut self.perspectives[idx], perspective))
    }

    pub fn delete_perspective(&mut self, id: &str) -> Option<Perspective> {
        let idx = self.perspectives.iter().position(|p| p.id == id)?;
        Some(self.perspectives.remove(idx))
    }

    /// Mutable access to a Perspective by id (for add/remove link).
    pub fn perspective_mut(&mut self, id: &str) -> Option<&mut Perspective> {
        self.perspectives.iter_mut().find(|p| p.id == id)
    }

    pub fn source(&self, id: &str) -> Option<&Source> {
        self.sources.iter().find(|s| s.id == id)
    }
    pub fn upsert_source(&mut self, source: Source) {
        let id = source.id.clone();
        self.sources.retain(|s| s.id != id);
        self.sources.push(source);
    }

    pub fn artefact(&self, id: &str) -> Option<&Artefact> {
        self.artefacts.iter().find(|a| a.id == id)
    }
    pub fn upsert_artefact(&mut self, artefact: Artefact) {
        let id = artefact.id.clone();
        self.artefacts.retain(|a| a.id != id);
        self.artefacts.push(artefact);
    }

    pub fn lookup(&self, id: &str) -> Option<&Lookup> {
        self.lookups.iter().find(|l| l.id == id)
    }
    pub fn upsert_lookup(&mut self, lookup: Lookup) {
        let id = lookup.id.clone();
        self.lookups.retain(|l| l.id != id);
        self.lookups.push(lookup);
    }

    pub fn reference(&self, id: &str) -> Option<&Reference> {
        self.references.iter().find(|r| r.id == id)
    }
    pub fn upsert_reference(&mut self, reference: Reference) {
        let id = reference.id.clone();
        self.references.retain(|r| r.id != id);
        self.references.push(reference);
    }
    pub fn delete_reference(&mut self, id: &str) -> Option<Reference> {
        let idx = self.references.iter().position(|r| r.id == id)?;
        Some(self.references.remove(idx))
    }

    /// All References whose cited target matches the given Expression address.
    pub fn references_for(&self, address: &str) -> Vec<&Reference> {
        self.references.iter().filter(|r| r.target == address).collect()
    }

    /// All References citing anything within a System (the whole system, or any
    /// of its terms/connectives) — used to prefetch hover tooltips for a graph.
    pub fn references_for_system(&self, system_id: &str) -> Vec<&Reference> {
        let exact = format!("system:{}", system_id);
        let prefix = format!("system:{}#", system_id);
        self.references
            .iter()
            .filter(|r| r.target == exact || r.target.starts_with(&prefix))
            .collect()
    }

    /// All References citing `address` *or anything nested under it* — a container
    /// rolls up its parts. Descendants are addressed by appending `#…` to the
    /// container address (e.g. `system:<id>` contains `system:<id>#term:1`), so a
    /// leaf address (a node/edge) simply returns its own exact matches.
    pub fn references_under(&self, address: &str) -> Vec<&Reference> {
        let prefix = format!("{}#", address);
        self.references
            .iter()
            .filter(|r| r.target == address || r.target.starts_with(&prefix))
            .collect()
    }

    /// A self-contained `GraphContent` module for one Perspective, built by
    /// *reference* rather than by copy:
    ///  * the perspective itself (with its links),
    ///  * the references whose `perspective_ref` is this perspective,
    ///  * the non-canonical entities those references and links *own* — systems,
    ///    vocabularies, characters, sources, artefacts, lookups, and
    ///  * a `manifest` of the external addresses the module depends on but does
    ///    not own (canonical systems, sibling perspectives, shared citation
    ///    entities).
    ///
    /// Canonical archetypes (the seed) are never copied — one expression, one
    /// home; they are recorded in the manifest instead, so composition resolves
    /// by address once those homes are loaded. A manifest entry that never
    /// resolves is a tolerated dangling citation, not an error.
    pub fn export_perspective(&self, perspective_id: &str) -> GraphContent {
        let mut out = GraphContent::default();
        let Some(persp) = self.perspectives.iter().find(|p| p.id == perspective_id) else {
            return out;
        };
        out.perspectives.push(persp.clone());

        // Own = not a canonical archetype. Applies to *every* entity kind, so a
        // shared/canonical vocabulary, source, artefact or lookup is referenced
        // by address, never duplicated into the file.
        let is_own = |id: &str| !self.canonical_ids.contains(id);
        let mut manifest: HashSet<String> = HashSet::new();

        // 1) References this perspective owns, and the entities they reach.
        let (mut src, mut art, mut lk, mut sys) =
            (HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new());
        for r in self.references.iter().filter(|r| r.perspective_ref == perspective_id) {
            out.references.push(r.clone());
            src.insert(r.source_ref.clone());
            if let Some(a) = &r.artefact_ref { art.insert(a.clone()); }
            if let Some(l) = &r.lookup_ref { lk.insert(l.clone()); }
            if let Some(rest) = r.target.strip_prefix("system:") {
                sys.insert(rest.split('#').next().unwrap_or(rest).to_string());
            }
        }

        // 2) Systems referenced by the perspective's links, too (a
        //    perspective-of-perspectives / link-composed system is not reachable
        //    through references alone).
        for l in &persp.links {
            for endpoint in [&l.source, &l.target] {
                if let Some(rest) = endpoint.strip_prefix("system:") {
                    sys.insert(rest.split('#').next().unwrap_or(rest).to_string());
                }
            }
        }

        // 3) Systems: own the non-canonical ones; record the rest (canonical or
        //    absent) as external dependencies.
        let mut vocab = HashSet::new();
        for sid in &sys {
            match self.system(sid) {
                Some(s) if is_own(sid) => {
                    if !out.systems.iter().any(|x| x.id == *sid) {
                        out.systems.push(s.clone());
                        vocab.insert(s.vocabulary_ref.clone());
                    }
                }
                _ => {
                    manifest.insert(format!("system:{sid}"));
                }
            }
        }

        // 4) Vocabularies + characters owned via the owned systems.
        let mut chars = HashSet::new();
        for v in self.vocabularies.iter().filter(|v| vocab.contains(&v.id) && is_own(&v.id)) {
            out.vocabularies.push(v.clone());
            chars.extend(v.terms.iter().chain(v.connectives.iter()).cloned());
        }
        out.characters = self
            .characters(None)
            .into_iter()
            .filter(|c| chars.contains(&c.id) && is_own(&c.id))
            .cloned()
            .collect();

        // 5) Citation entities: own the non-canonical ones; record canonical /
        //    shared ones as external dependencies.
        out.sources = self.sources.iter().filter(|s| src.contains(&s.id) && is_own(&s.id)).cloned().collect();
        out.artefacts = self.artefacts.iter().filter(|a| art.contains(&a.id) && is_own(&a.id)).cloned().collect();
        out.lookups = self.lookups.iter().filter(|l| lk.contains(&l.id) && is_own(&l.id)).cloned().collect();
        for s in &src { if self.source(s).is_none_or(|e| !is_own(&e.id)) { manifest.insert(format!("source:{s}")); } }
        for a in &art { if self.artefact(a).is_none_or(|e| !is_own(&e.id)) { manifest.insert(format!("artefact:{a}")); } }
        for l in &lk  { if self.lookup(l).is_none_or(|e| !is_own(&e.id)) { manifest.insert(format!("lookup:{l}")); } }

        // 6) Link endpoints that point outside this module (sibling perspectives,
        //    shared citation entities) are external dependencies.
        for l in &persp.links {
            for endpoint in [&l.source, &l.target] {
                let Some((kind, rest)) = endpoint.split_once(':') else { continue };
                let id = rest.split('#').next().unwrap_or(rest);
                let owned = match kind {
                    "system" => out.systems.iter().any(|s| s.id == id),
                    "perspective" => out.perspectives.iter().any(|p| p.id == id),
                    "source" => out.sources.iter().any(|s| s.id == id),
                    "artefact" => out.artefacts.iter().any(|a| a.id == id),
                    "lookup" => out.lookups.iter().any(|l| l.id == id),
                    // fragments of substrate/other kinds resolve from the seed.
                    _ => true,
                };
                if !owned {
                    manifest.insert(format!("{kind}:{id}"));
                }
            }
        }

        // Deterministic ordering: several sets above are harvested from
        // `HashSet`s, so sort every collection by id to keep exports (and the
        // module files they become) stable and diff-friendly.
        out.systems.sort_by(|a, b| a.id.cmp(&b.id));
        out.vocabularies.sort_by(|a, b| a.id.cmp(&b.id));
        out.characters.sort_by(|a, b| a.id.cmp(&b.id));
        out.sources.sort_by(|a, b| a.id.cmp(&b.id));
        out.artefacts.sort_by(|a, b| a.id.cmp(&b.id));
        out.lookups.sort_by(|a, b| a.id.cmp(&b.id));
        out.references.sort_by(|a, b| a.id.cmp(&b.id));
        out.manifest = manifest.into_iter().collect();
        out.manifest.sort();
        out
    }

    /// All Links (across every Perspective) touching the given address as
    /// either source or target.
    pub fn links_for(&self, address: &str) -> Vec<&super::perspectives::Link> {
        self.perspectives
            .iter()
            .flat_map(|p| p.links.iter())
            .filter(|l| l.source == address || l.target == address)
            .collect()
    }

    /// Look up which Character inhabits a given Point through a
    /// Vocabulary's paired Topology.
    pub fn character_at_point(
        &self,
        vocabulary_id: &str,
        point_id: &str,
    ) -> Option<&Character> {
        let sv = self.vocabulary(vocabulary_id)?;
        let topology = self.topological_vocab_for_order(sv.order)?;
        let idx = topology.points.iter().position(|p| p == point_id)?;
        let char_id = sv.terms.get(idx)?;
        self.character(char_id)
    }

    /// Look up which Character inhabits a given Line through a
    /// Vocabulary's paired Topology.
    pub fn character_at_line(
        &self,
        vocabulary_id: &str,
        line_id: &str,
    ) -> Option<&Character> {
        let sv = self.vocabulary(vocabulary_id)?;
        let topology = self.topological_vocab_for_order(sv.order)?;
        let idx = topology.lines.iter().position(|l| l == line_id)?;
        let char_id = sv.connectives.get(idx)?;
        self.character(char_id)
    }

    /// Validate a System by resolving its GraphTemplate + Vocabulary and the
    /// GraphTemplate's referenced substrate vocabularies.
    pub fn validate_system(&self, system_id: &str) -> Result<(), Vec<String>> {
        let sys = self
            .system(system_id)
            .ok_or_else(|| vec![format!("System '{}' not found", system_id)])?;
        let grammar = self
            .grammar(&sys.grammar_ref)
            .ok_or_else(|| vec![format!("GraphTemplate '{}' not found", sys.grammar_ref)])?;
        let t = self
            .topological_vocab(&grammar.topological_vocab_ref)
            .ok_or_else(|| {
                vec![format!(
                    "Topology '{}' not found",
                    grammar.topological_vocab_ref
                )]
            })?;
        let geo = self
            .geometric_vocab(&grammar.geometric_vocab_ref)
            .ok_or_else(|| {
                vec![format!(
                    "Geometry '{}' not found",
                    grammar.geometric_vocab_ref
                )]
            })?;
        let s = self
            .vocabulary(&sys.vocabulary_ref)
            .ok_or_else(|| {
                vec![format!("Vocabulary '{}' not found", sys.vocabulary_ref)]
            })?;
        grammar.validate_with(t, geo, s)
    }

    /// Look up the Canonical Vocabulary containing hex colours for
    /// the given order (created by seed as "Canonical Colours {name}").
    pub fn canonical_colour_vocab_for_order(&self, order: u8) -> Option<&Vocabulary> {
        self.vocabularies
            .iter()
            .find(|v| v.order == order && v.name.starts_with("Canonical Colours"))
    }

    // ==========================================================================
    // Content: apply / snapshot / user-vs-canonical separation
    // ==========================================================================

    /// Upsert a Character by id (replace if present, else append).
    pub fn upsert_character(&mut self, character: Character) {
        let id = character.id.clone();
        self.entries
            .retain(|e| !matches!(e, Entry::Character(c) if c.id == id));
        self.entries.push(Entry::Character(character));
    }

    /// Upsert a Coordinate by id.
    pub fn upsert_coordinate(&mut self, coordinate: Coordinate) {
        let id = coordinate.id.clone();
        self.entries
            .retain(|e| !matches!(e, Entry::Coordinate(c) if c.id == id));
        self.entries.push(Entry::Coordinate(coordinate));
    }

    /// Apply a content bundle onto the graph, upserting every item by id.
    pub fn apply_content(&mut self, content: &GraphContent) {
        for c in &content.characters {
            self.upsert_character(c.clone());
        }
        for c in &content.coordinates {
            self.upsert_coordinate(c.clone());
        }
        for v in &content.vocabularies {
            if self.update_vocabulary(v.clone()).is_none() {
                self.add_vocabulary(v.clone());
            }
        }
        for s in &content.systems {
            if self.update_system(s.clone()).is_none() {
                self.add_system(s.clone());
            }
        }
        for p in &content.perspectives {
            if self.update_perspective(p.clone()).is_none() {
                self.add_perspective(p.clone());
            }
        }
        for s in &content.sources {
            self.upsert_source(s.clone());
        }
        for a in &content.artefacts {
            self.upsert_artefact(a.clone());
        }
        for l in &content.lookups {
            self.upsert_lookup(l.clone());
        }
        for r in &content.references {
            self.upsert_reference(r.clone());
        }
        for f in &content.functors {
            if self.update_functor(f.clone()).is_none() {
                self.add_functor(f.clone());
            }
        }
        for s in &content.sequences {
            if self.update_sequence(s.clone()).is_none() {
                self.add_sequence(s.clone());
            }
        }
    }

    /// Snapshot the entire data layer as a content bundle.
    pub fn content_snapshot(&self) -> GraphContent {
        GraphContent {
            characters: self.characters(None).into_iter().cloned().collect(),
            coordinates: self.coordinates(None).into_iter().cloned().collect(),
            vocabularies: self.vocabularies.clone(),
            systems: self.systems.clone(),
            perspectives: self.perspectives.clone(),
            sources: self.sources.clone(),
            artefacts: self.artefacts.clone(),
            lookups: self.lookups.clone(),
            references: self.references.clone(),
            functors: self.functors.clone(),
            sequences: self.sequences.clone(),
            manifest: Vec::new(),
        }
    }

    /// Mark everything currently in the data layer as canonical (an immutable
    /// archetype) *and* bundled. Called once after the canonical seed is applied,
    /// before modules or any user store are loaded.
    pub fn mark_canonical(&mut self) {
        let ids: HashSet<String> = self.content_snapshot().ids().into_iter().collect();
        self.bundled_ids.extend(ids.iter().cloned());
        self.canonical_ids = ids;
    }

    /// Mark everything currently in the data layer as *bundled* (durable, sourced
    /// from a file) without marking it canonical. Called after perspective
    /// modules are loaded: their content must be kept out of the writable user
    /// store (like canonical content), but it is not an immutable archetype, so
    /// `export_perspective` may still re-emit a module's own entities losslessly.
    pub fn mark_bundled(&mut self) {
        self.bundled_ids.extend(self.content_snapshot().ids());
    }

    /// The user-added slice of the data layer (everything not already durable in
    /// a file — neither canonical seed nor a loaded module). This is what gets
    /// persisted to the user store.
    pub fn user_content(&self) -> GraphContent {
        let is_user = |id: &str| !self.bundled_ids.contains(id);
        GraphContent {
            characters: self
                .characters(None)
                .into_iter()
                .filter(|c| is_user(&c.id))
                .cloned()
                .collect(),
            coordinates: self
                .coordinates(None)
                .into_iter()
                .filter(|c| is_user(&c.id))
                .cloned()
                .collect(),
            vocabularies: self
                .vocabularies
                .iter()
                .filter(|v| is_user(&v.id))
                .cloned()
                .collect(),
            systems: self
                .systems
                .iter()
                .filter(|s| is_user(&s.id))
                .cloned()
                .collect(),
            perspectives: self
                .perspectives
                .iter()
                .filter(|p| is_user(&p.id))
                .cloned()
                .collect(),
            sources: self
                .sources
                .iter()
                .filter(|s| is_user(&s.id))
                .cloned()
                .collect(),
            artefacts: self
                .artefacts
                .iter()
                .filter(|a| is_user(&a.id))
                .cloned()
                .collect(),
            lookups: self
                .lookups
                .iter()
                .filter(|l| is_user(&l.id))
                .cloned()
                .collect(),
            references: self
                .references
                .iter()
                .filter(|r| is_user(&r.id))
                .cloned()
                .collect(),
            functors: self
                .functors
                .iter()
                .filter(|f| is_user(&f.id))
                .cloned()
                .collect(),
            sequences: self
                .sequences
                .iter()
                .filter(|s| is_user(&s.id))
                .cloned()
                .collect(),
            manifest: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::vocabularies::{
        Geometry, Vocabulary, Topology,
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
        g.add_topological_vocab(Topology::canonical_for(3));
        g.add_geometric_vocab(Geometry::canonical_for(3));
        g.add_vocabulary(Vocabulary::with_auto_id(
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
        g.add_grammar(GraphTemplate::for_order(3));
        g.add_system(System::with_auto_id(
            "Canonical Triad",
            3,
            "Dynamism",
            "Impulses",
            "Acts",
            "grammar_3",
            "vocab_canonical_triad_3",
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
    fn test_system_validate_via_graph() {
        let g = triad_test_graph();
        assert!(g.validate_system("system_canonical_triad_3").is_ok());
    }
}
