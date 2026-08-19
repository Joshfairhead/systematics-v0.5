//! GraphQL types and schema for the Systematics property graph API.

use std::path::PathBuf;
use std::sync::Arc;

use async_graphql::*;
use tokio::sync::RwLock;

use crate::core::{
    Artefact, Character, Coordinate, Entry, Functor, Geometry, Template, Graph, Line,
    Lookup, Order, Perspective, PerspectiveLink, Point, Position, Reference, Sequence, Vocabulary,
    Segment, Source, System, Topology,
};

/// Shared, mutable graph passed to the GraphQL schema as context data.
pub type SharedGraph = Arc<RwLock<Graph>>;

/// Writable-store path in context. `None` disables persistence (used by tests).
#[derive(Clone, Default)]
pub struct StorePath(pub Option<PathBuf>);

/// Take a cheap snapshot of the shared graph for read-only resolvers.
async fn graph_snapshot(ctx: &Context<'_>) -> Graph {
    ctx.data_unchecked::<SharedGraph>().read().await.clone()
}

/// Access the shared graph for mutation resolvers.
fn shared_graph<'a>(ctx: &'a Context<'_>) -> &'a SharedGraph {
    ctx.data_unchecked::<SharedGraph>()
}

/// Persist the graph's user slice, if a store path is configured. Called at the
/// end of each mutation while the write lock is still held.
fn persist(ctx: &Context<'_>, graph: &Graph) {
    if let Some(path) = ctx.data_unchecked::<StorePath>().0.as_ref() {
        if let Err(e) = crate::persistence::save(graph, path) {
            tracing::error!("failed to persist user store: {}", e);
        }
    }
}

// ============================================================================
// Root types
// ============================================================================

#[derive(Clone, Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // -------- substrate anchors --------

    async fn order(&self, ctx: &Context<'_>, value: i32) -> Option<GqlOrder> {
        if !(1..=12).contains(&value) {
            return None;
        }
        let g = graph_snapshot(ctx).await;
        g.order(value as u8).map(|o| GqlOrder::new(o.clone()))
    }

    async fn orders(&self, ctx: &Context<'_>) -> Vec<GqlOrder> {
        let g = graph_snapshot(ctx).await;
        g.orders().into_iter().map(|o| GqlOrder::new(o.clone())).collect()
    }

    async fn position(&self, ctx: &Context<'_>, value: i32) -> Option<GqlPosition> {
        if !(1..=12).contains(&value) {
            return None;
        }
        let g = graph_snapshot(ctx).await;
        g.position(value as u8).map(|p| GqlPosition::new(p.clone()))
    }

    async fn positions(&self, ctx: &Context<'_>) -> Vec<GqlPosition> {
        let g = graph_snapshot(ctx).await;
        g.positions()
            .into_iter()
            .map(|p| GqlPosition::new(p.clone()))
            .collect()
    }

    async fn point(&self, ctx: &Context<'_>, order: i32, position: i32) -> Option<GqlPoint> {
        let g = graph_snapshot(ctx).await;
        g.point(order as u8, position as u8)
            .map(|p| GqlPoint::new(p.clone()))
    }

    async fn points(&self, ctx: &Context<'_>, order: Option<i32>) -> Vec<GqlPoint> {
        let g = graph_snapshot(ctx).await;
        g.points(order.map(|o| o as u8))
            .into_iter()
            .map(|p| GqlPoint::new(p.clone()))
            .collect()
    }

    async fn line(
        &self,
        ctx: &Context<'_>,
        order: i32,
        p1: i32,
        p2: i32,
    ) -> Option<GqlLine> {
        let g = graph_snapshot(ctx).await;
        g.line(order as u8, p1 as u8, p2 as u8)
            .map(|l| GqlLine::new(l.clone()))
    }

    async fn lines_of(&self, ctx: &Context<'_>, order: Option<i32>) -> Vec<GqlLine> {
        let g = graph_snapshot(ctx).await;
        g.lines_of(order.map(|o| o as u8))
            .into_iter()
            .map(|l| GqlLine::new(l.clone()))
            .collect()
    }

    async fn coordinate(
        &self,
        ctx: &Context<'_>,
        order: i32,
        position: i32,
    ) -> Option<GqlCoordinate> {
        let g = graph_snapshot(ctx).await;
        g.coordinate(order as u8, position as u8)
            .map(|c| GqlCoordinate::new(c.clone()))
    }

    async fn coordinates(&self, ctx: &Context<'_>, order: Option<i32>) -> Vec<GqlCoordinate> {
        let g = graph_snapshot(ctx).await;
        g.coordinates(order.map(|o| o as u8))
            .into_iter()
            .map(|c| GqlCoordinate::new(c.clone()))
            .collect()
    }

    async fn segment(
        &self,
        ctx: &Context<'_>,
        order: i32,
        p1: i32,
        p2: i32,
    ) -> Option<GqlSegment> {
        let g = graph_snapshot(ctx).await;
        g.segment(order as u8, p1 as u8, p2 as u8)
            .map(|s| GqlSegment::new(s.clone()))
    }

    async fn character(&self, ctx: &Context<'_>, id: String) -> Option<GqlCharacter> {
        let g = graph_snapshot(ctx).await;
        g.character(&id).map(|c| GqlCharacter::new(c.clone()))
    }

    async fn characters(&self, ctx: &Context<'_>, kind: Option<String>) -> Vec<GqlCharacter> {
        let g = graph_snapshot(ctx).await;
        g.characters(kind.as_deref())
            .into_iter()
            .map(|c| GqlCharacter::new(c.clone()))
            .collect()
    }

    // -------- vocabularies and perspective --------

    async fn topology(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Option<GqlTopology> {
        let g = graph_snapshot(ctx).await;
        g.topology(&id)
            .map(|v| GqlTopology::new(v.clone()))
    }

    async fn topology_for_order(
        &self,
        ctx: &Context<'_>,
        order: i32,
    ) -> Option<GqlTopology> {
        let g = graph_snapshot(ctx).await;
        g.topology_for_order(order as u8)
            .map(|v| GqlTopology::new(v.clone()))
    }

    async fn geometry(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Option<GqlGeometry> {
        let g = graph_snapshot(ctx).await;
        g.geometry(&id)
            .map(|v| GqlGeometry::new(v.clone()))
    }

    async fn geometry_for_order(
        &self,
        ctx: &Context<'_>,
        order: i32,
    ) -> Option<GqlGeometry> {
        let g = graph_snapshot(ctx).await;
        g.geometry_for_order(order as u8)
            .map(|v| GqlGeometry::new(v.clone()))
    }

    async fn vocabulary(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Option<GqlVocabulary> {
        let g = graph_snapshot(ctx).await;
        g.vocabulary(&id)
            .map(|v| GqlVocabulary::new(v.clone()))
    }

    async fn vocabularies_for_order(
        &self,
        ctx: &Context<'_>,
        order: i32,
    ) -> Vec<GqlVocabulary> {
        let g = graph_snapshot(ctx).await;
        g.vocabularies_for_order(order as u8)
            .into_iter()
            .map(|v| GqlVocabulary::new(v.clone()))
            .collect()
    }

    // -------- templates (structure) and systems (reconcilers) --------

    async fn template_for_order(&self, ctx: &Context<'_>, order: i32) -> Option<GqlTemplate> {
        let g = graph_snapshot(ctx).await;
        g.template_for_order(order as u8).map(|gr| GqlTemplate::new(gr.clone()))
    }

    async fn system_by_id(&self, ctx: &Context<'_>, id: String) -> Option<GqlSystem> {
        let g = graph_snapshot(ctx).await;
        g.system(&id).map(|s| GqlSystem::new(s.clone()))
    }

    async fn systems_for_order(&self, ctx: &Context<'_>, order: i32) -> Vec<GqlSystem> {
        let g = graph_snapshot(ctx).await;
        g.systems_for_order(order as u8)
            .into_iter()
            .map(|s| GqlSystem::new(s.clone()))
            .collect()
    }

    /// Instance systems: every System that is *not* the canonical archetype for
    /// its order (DU1/DU2/ES systems, the Citation triad, the Architecture
    /// Pentad, …). These are what the Load control browses — the canonical
    /// categories are reached by the normal order navigation.
    async fn instance_systems(&self, ctx: &Context<'_>) -> Vec<GqlSystem> {
        let g = graph_snapshot(ctx).await;
        g.systems
            .iter()
            .filter(|s| s.id != canonical_system_id(s.order))
            // Hide empty/unlabelled systems (e.g. un-scraped DU1) so they don't
            // give misleading blank representations.
            .filter(|s| g.vocabulary(&s.vocabulary_ref).is_some_and(|v| !v.terms.is_empty()))
            .map(|s| GqlSystem::new(s.clone()))
            .collect()
    }

    // -------- joins --------

    async fn character_at_point(
        &self,
        ctx: &Context<'_>,
        vocabulary_id: String,
        point_id: String,
    ) -> Option<GqlCharacter> {
        let g = graph_snapshot(ctx).await;
        g.character_at_point(&vocabulary_id, &point_id)
            .map(|c| GqlCharacter::new(c.clone()))
    }

    async fn character_at_line(
        &self,
        ctx: &Context<'_>,
        vocabulary_id: String,
        line_id: String,
    ) -> Option<GqlCharacter> {
        let g = graph_snapshot(ctx).await;
        g.character_at_line(&vocabulary_id, &line_id)
            .map(|c| GqlCharacter::new(c.clone()))
    }

    async fn validate_system(&self, ctx: &Context<'_>, id: String) -> Vec<String> {
        let g = graph_snapshot(ctx).await;
        g.validate_system(&id).err().unwrap_or_default()
    }

    // -------- Functors (same-grammar morphisms between Systems) --------

    /// A single stored Functor by id.
    async fn functor(&self, ctx: &Context<'_>, id: String) -> Option<GqlFunctor> {
        let g = graph_snapshot(ctx).await;
        g.functor(&id).cloned().map(GqlFunctor::new)
    }

    /// All stored Functors.
    async fn functors(&self, ctx: &Context<'_>) -> Vec<GqlFunctor> {
        let g = graph_snapshot(ctx).await;
        g.functors().iter().cloned().map(GqlFunctor::new).collect()
    }

    /// Advisory validation of a Functor (permutation laws + resolved systems).
    /// Empty vec = valid; otherwise the reasons it is not a functor.
    async fn validate_functor(&self, ctx: &Context<'_>, id: String) -> Vec<String> {
        let g = graph_snapshot(ctx).await;
        g.validate_functor(&id).err().unwrap_or_default()
    }

    // -------- Sequences (ordered series of member addresses) --------

    async fn sequence(&self, ctx: &Context<'_>, id: String) -> Option<GqlSequence> {
        let g = graph_snapshot(ctx).await;
        g.sequence(&id).cloned().map(GqlSequence::new)
    }

    async fn sequences(&self, ctx: &Context<'_>) -> Vec<GqlSequence> {
        let g = graph_snapshot(ctx).await;
        g.sequences().iter().cloned().map(GqlSequence::new).collect()
    }

    // -------- resolved RenderedSystem (a System resolved into a bound K-graph) --------

    /// Resolve any System (canonical or user) into its complete RenderedSystem.
    async fn render_system(&self, ctx: &Context<'_>, system_id: String) -> Option<GqlRenderedSystem> {
        let g = graph_snapshot(ctx).await;
        resolve_system(&g, &system_id).map(GqlRenderedSystem::new)
    }

    /// The Citation triad (Source / Artefact / Lookup) as a resolved K3 — the
    /// referencing layer dogfooded as a (non-canonical) systematics triad.
    async fn citation_triad(&self, ctx: &Context<'_>) -> Option<GqlRenderedSystem> {
        let g = graph_snapshot(ctx).await;
        resolve_system(&g, "system_citation_3").map(GqlRenderedSystem::new)
    }

    /// Convenience: the canonical RenderedSystem for an order (resolves the
    /// seed's `Canonical <Name>` system).
    async fn system(&self, ctx: &Context<'_>, order: i32) -> Option<GqlRenderedSystem> {
        if !(1..=12).contains(&order) {
            return None;
        }
        let g = graph_snapshot(ctx).await;
        resolve_system(&g, &canonical_system_id(order as u8)).map(GqlRenderedSystem::new)
    }

    async fn system_by_name(&self, ctx: &Context<'_>, name: String) -> Option<GqlRenderedSystem> {
        let order = match name.to_lowercase().as_str() {
            "monad" => 1,
            "dyad" => 2,
            "triad" => 3,
            "tetrad" => 4,
            "pentad" => 5,
            "hexad" => 6,
            "heptad" => 7,
            "octad" => 8,
            "ennead" => 9,
            "decad" => 10,
            "undecad" => 11,
            "dodecad" => 12,
            _ => return None,
        };
        let g = graph_snapshot(ctx).await;
        resolve_system(&g, &canonical_system_id(order)).map(GqlRenderedSystem::new)
    }

    async fn all_systems(&self, ctx: &Context<'_>) -> Vec<GqlRenderedSystem> {
        let g = graph_snapshot(ctx).await;
        (1..=12u8)
            .filter_map(|o| resolve_system(&g, &canonical_system_id(o)).map(GqlRenderedSystem::new))
            .collect()
    }

    // -------- referencing layer: perspectives (webs) + citations --------

    async fn perspective(&self, ctx: &Context<'_>, id: String) -> Option<GqlPerspective> {
        let g = graph_snapshot(ctx).await;
        g.perspective(&id).map(|p| GqlPerspective::new(p.clone()))
    }

    async fn perspectives(&self, ctx: &Context<'_>) -> Vec<GqlPerspective> {
        let g = graph_snapshot(ctx).await;
        g.perspectives().iter().map(|p| GqlPerspective::new(p.clone())).collect()
    }

    async fn source(&self, ctx: &Context<'_>, id: String) -> Option<GqlSource> {
        let g = graph_snapshot(ctx).await;
        g.source(&id).map(|s| GqlSource::new(s.clone()))
    }

    async fn artefact(&self, ctx: &Context<'_>, id: String) -> Option<GqlArtefact> {
        let g = graph_snapshot(ctx).await;
        g.artefact(&id).map(|a| GqlArtefact::new(a.clone()))
    }

    async fn lookup(&self, ctx: &Context<'_>, id: String) -> Option<GqlLookup> {
        let g = graph_snapshot(ctx).await;
        g.lookup(&id).map(|l| GqlLookup::new(l.clone()))
    }

    async fn reference(&self, ctx: &Context<'_>, id: String) -> Option<GqlReference> {
        let g = graph_snapshot(ctx).await;
        g.reference(&id).map(|r| GqlReference::new(r.clone()))
    }

    /// Every reference in the graph — for the reference browser (sort/filter by
    /// source · artefact · system · lookup client-side).
    async fn all_references(&self, ctx: &Context<'_>) -> Vec<GqlReference> {
        let g = graph_snapshot(ctx).await;
        g.references.iter().map(|r| GqlReference::new(r.clone())).collect()
    }

    /// A Perspective exported as a self-contained, loadable JSON module
    /// (`GraphContent` shape) — "turn a perspective into a file".
    async fn export_perspective(&self, ctx: &Context<'_>, id: String) -> String {
        let g = graph_snapshot(ctx).await;
        serde_json::to_string_pretty(&g.export_perspective(&id)).unwrap_or_else(|_| "{}".into())
    }

    /// References citing this exact address. Set `includeDescendants: true` to
    /// roll up everything nested under a container address (e.g. a whole system
    /// sums its node/edge citations).
    async fn references_for(
        &self,
        ctx: &Context<'_>,
        address: String,
        #[graphql(default = false)] include_descendants: bool,
    ) -> Vec<GqlReference> {
        let g = graph_snapshot(ctx).await;
        let refs = if include_descendants {
            g.references_under(&address)
        } else {
            g.references_for(&address)
        };
        refs.into_iter().map(|r| GqlReference::new(r.clone())).collect()
    }

    /// All References citing anything within a System — prefetch for tooltips.
    async fn references_for_system(
        &self,
        ctx: &Context<'_>,
        system_id: String,
    ) -> Vec<GqlReference> {
        let g = graph_snapshot(ctx).await;
        g.references_for_system(&system_id)
            .into_iter()
            .map(|r| GqlReference::new(r.clone()))
            .collect()
    }

    /// All Links (across every Perspective) touching the given address.
    async fn links_for(&self, ctx: &Context<'_>, address: String) -> Vec<GqlLink> {
        let g = graph_snapshot(ctx).await;
        g.links_for(&address)
            .into_iter()
            .map(|l| GqlLink::new(l.clone()))
            .collect()
    }
}

// ============================================================================
// Compat: computed Template for the current SVG frontend renderer.
// ============================================================================

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

pub struct RenderedSystemData {
    pub order: u8,
    pub system_id: String,
    /// The order's display label (e.g. "Triad") — what the nav navigates by.
    pub name: String,
    /// The System's own name (e.g. "Citation", "Canonical Triad").
    pub system_name: String,
    pub coherence: String,
    pub term_designation: String,
    pub connective_designation: String,
    pub terms: Vec<GrammarTermData>,
    pub coordinates: Vec<Coordinate>,
    pub colours: Vec<GrammarColourData>,
    pub lines: Vec<GrammarLineData>,
    pub connectives: Vec<GrammarConnectiveData>,
}

pub struct GrammarTermData {
    pub position: i32,
    pub character_id: String,
    pub value: String,
}

pub struct GrammarColourData {
    pub position: i32,
    pub value: String,
}

pub struct GrammarLineData {
    pub id: String,
    pub base_position: i32,
    pub target_position: i32,
}

pub struct GrammarConnectiveData {
    pub id: String,
    pub base_position: i32,
    pub target_position: i32,
    pub character_value: String,
}

/// Canonical System id for an order (the seed's `Canonical <Name>`).
fn canonical_system_id(order: u8) -> String {
    format!(
        "system_canonical_{}_{}",
        canonical_system_name(order).to_lowercase(),
        order
    )
}

/// Resolve a System (by id) into a complete bound RenderedSystem — terms bound
/// to points, connectives to lines, coordinates + colours per position, metadata
/// applied. This is "resolve the wiring into a renderable K-graph".
fn resolve_system(graph: &Graph, system_id: &str) -> Option<RenderedSystemData> {
    let system = graph.system(system_id)?;
    let order = system.order;
    // Display label is the order's system name (e.g. "Triad"), not the
    // system's name ("Canonical Triad"). The frontend navigates by this
    // (systemByName expects "triad", not "canonical triad").
    let name = canonical_system_name(order).to_string();
    let system_name = system.name.clone();
    let grammar = graph.template(&system.grammar_ref)?;
    let topology = graph.topology(&grammar.topological_vocab_ref)?;
    let semantic = graph.vocabulary(&system.vocabulary_ref)?;
    let colour_vocab = graph.canonical_colour_vocab_for_order(order);

    // Terms (word Characters at each Point in position order).
    let mut terms = Vec::new();
    for (idx, char_id) in semantic.terms.iter().enumerate() {
        let position = (idx + 1) as i32;
        if let Some(c) = graph.character(char_id) {
            terms.push(GrammarTermData {
                position,
                character_id: c.id.clone(),
                value: c.value.clone(),
            });
        }
    }

    // Coordinates in position order.
    let mut coordinates = Vec::new();
    for pos in 1..=order {
        if let Some(c) = graph.coordinate(order, pos) {
            coordinates.push(c.clone());
        }
    }

    // Colours from the canonical colour vocab.
    let mut colours = Vec::new();
    if let Some(cvocab) = colour_vocab {
        for (idx, char_id) in cvocab.terms.iter().enumerate() {
            if let Some(c) = graph.character(char_id) {
                colours.push(GrammarColourData {
                    position: (idx + 1) as i32,
                    value: c.value.clone(),
                });
            }
        }
    }

    // Lines: every canonical position pair.
    let mut lines = Vec::new();
    for p1 in 1..=order {
        for p2 in (p1 + 1)..=order {
            lines.push(GrammarLineData {
                id: format!("line_{}_{}_{}", order, p1, p2),
                base_position: p1 as i32,
                target_position: p2 as i32,
            });
        }
    }

    // Connectives: pair Perspective's topology.lines[i] with semantic.connectives[i].
    let mut connectives = Vec::new();
    for (idx, line_id) in topology.lines.iter().enumerate() {
        if let Some(line) = graph.get_entry(line_id).and_then(|e| match e {
            Entry::Line(l) => Some(l),
            _ => None,
        }) {
            let base = line.position_value().unwrap_or(0) as i32;
            let target = line.position_secondary_value().unwrap_or(0) as i32;
            let char_id = semantic.connectives.get(idx).cloned().unwrap_or_default();
            let value = graph
                .character(&char_id)
                .map(|c| c.value.clone())
                .unwrap_or_default();
            connectives.push(GrammarConnectiveData {
                id: line.id.clone(),
                base_position: base,
                target_position: target,
                character_value: value,
            });
        }
    }

    Some(RenderedSystemData {
        order,
        system_id: system.id.clone(),
        name,
        system_name,
        coherence: system.coherence.clone(),
        term_designation: system.term_designation.clone(),
        connective_designation: system.connective_designation.clone(),
        terms,
        coordinates,
        colours,
        lines,
        connectives,
    })
}

pub struct GqlRenderedSystem {
    inner: RenderedSystemData,
}

impl GqlRenderedSystem {
    pub fn new(inner: RenderedSystemData) -> Self {
        Self { inner }
    }
}

#[Object]
impl GqlRenderedSystem {
    async fn order(&self) -> i32 {
        self.inner.order as i32
    }
    async fn system_id(&self) -> &str {
        &self.inner.system_id
    }
    async fn name(&self) -> &str {
        &self.inner.name
    }
    /// The System's own name (e.g. "Citation") — distinct from the order label.
    async fn system_name(&self) -> &str {
        &self.inner.system_name
    }
    async fn coherence(&self) -> &str {
        &self.inner.coherence
    }
    async fn term_designation(&self) -> &str {
        &self.inner.term_designation
    }
    async fn connective_designation(&self) -> &str {
        &self.inner.connective_designation
    }
    async fn terms(&self) -> Vec<GqlGrammarTerm> {
        self.inner
            .terms
            .iter()
            .map(|t| GqlGrammarTerm {
                position: t.position,
                character_id: t.character_id.clone(),
                value: t.value.clone(),
            })
            .collect()
    }
    async fn coordinates(&self) -> Vec<GqlCoordinate> {
        self.inner
            .coordinates
            .iter()
            .cloned()
            .map(GqlCoordinate::new)
            .collect()
    }
    async fn colours(&self) -> Vec<GqlGrammarColour> {
        self.inner
            .colours
            .iter()
            .map(|c| GqlGrammarColour {
                position: c.position,
                value: c.value.clone(),
            })
            .collect()
    }
    async fn lines(&self) -> Vec<GqlGrammarLine> {
        self.inner
            .lines
            .iter()
            .map(|l| GqlGrammarLine {
                id: l.id.clone(),
                base_position: l.base_position,
                target_position: l.target_position,
            })
            .collect()
    }
    async fn connectives(&self) -> Vec<GqlGrammarConnective> {
        self.inner
            .connectives
            .iter()
            .map(|c| GqlGrammarConnective {
                id: c.id.clone(),
                base_position: c.base_position,
                target_position: c.target_position,
                character_value: c.character_value.clone(),
            })
            .collect()
    }

    /// The canonical *class* this system instantiates — the canonical system of
    /// the same order. `None` when this system already *is* that canonical one.
    /// Both share the K_n structure, so terms/connectives pair by position — the
    /// frontend's "Canonical override" toggle flips instance labels to these.
    async fn canonical_class(&self, ctx: &Context<'_>) -> Option<GqlRenderedSystem> {
        let canonical_id = canonical_system_id(self.inner.order);
        if self.inner.system_id == canonical_id {
            return None;
        }
        let g = graph_snapshot(ctx).await;
        resolve_system(&g, &canonical_id).map(GqlRenderedSystem::new)
    }
}

#[derive(SimpleObject)]
pub struct GqlGrammarTerm {
    pub position: i32,
    pub character_id: String,
    pub value: String,
}

#[derive(SimpleObject)]
pub struct GqlGrammarColour {
    pub position: i32,
    pub value: String,
}

#[derive(SimpleObject)]
pub struct GqlGrammarLine {
    pub id: String,
    pub base_position: i32,
    pub target_position: i32,
}

#[derive(SimpleObject)]
pub struct GqlGrammarConnective {
    pub id: String,
    pub base_position: i32,
    pub target_position: i32,
    pub character_value: String,
}

/// A term/connective character with its **topological position** — a node index
/// (`"1"`) for a term, or an edge (`"1-2"`) for a connective. This is the
/// graph-derived position the inspector/triple-store consumes so the frontend
/// never guesses (terms→nodes, connectives→edges of the adjacency matrix).
#[derive(SimpleObject)]
pub struct GqlPositionedChar {
    pub value: String,
    pub position: String,
}

/// The `index`-th edge of a `K_order` in the canonical lexicographic order
/// `(1,2),(1,3),…,(2,3),…` — matches the render path's line enumeration, so a
/// connective at vocabulary index `i` sits on edge `i`.
fn nth_edge(order: u8, index: usize) -> String {
    let mut k = 0;
    for p1 in 1..=order {
        for p2 in (p1 + 1)..=order {
            if k == index {
                return format!("{p1}-{p2}");
            }
            k += 1;
        }
    }
    String::new()
}

#[derive(Clone, Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_character(
        &self,
        ctx: &Context<'_>,
        input: CharacterInput,
    ) -> async_graphql::Result<GqlCharacter> {
        let character = input.into_character();
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        if graph.character(&character.id).is_some() {
            return Err(Error::new(format!(
                "Character '{}' already exists",
                character.id
            )));
        }
        graph.add_entry(Entry::Character(character.clone()));
        persist(ctx, &graph);
        Ok(GqlCharacter::new(character))
    }

    async fn delete_character(&self, ctx: &Context<'_>, id: String) -> bool {
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        let before = graph.entries.len();
        graph
            .entries
            .retain(|e| !matches!(e, Entry::Character(c) if c.id == id));
        let changed = graph.entries.len() != before;
        if changed {
            persist(ctx, &graph);
        }
        changed
    }

    async fn create_vocabulary(
        &self,
        ctx: &Context<'_>,
        input: VocabularyInput,
    ) -> async_graphql::Result<GqlVocabulary> {
        let sv = input.into_vocabulary();
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        if graph.vocabulary(&sv.id).is_some() {
            return Err(Error::new(format!(
                "Vocabulary '{}' already exists",
                sv.id
            )));
        }
        graph.add_vocabulary(sv.clone());
        persist(ctx, &graph);
        Ok(GqlVocabulary::new(sv))
    }

    async fn update_vocabulary(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: VocabularyInput,
    ) -> async_graphql::Result<GqlVocabulary> {
        let mut sv = input.into_vocabulary();
        sv.id = id.clone();
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        if graph.update_vocabulary(sv.clone()).is_none() {
            return Err(Error::new(format!("Vocabulary '{}' not found", id)));
        }
        persist(ctx, &graph);
        Ok(GqlVocabulary::new(sv))
    }

    async fn delete_vocabulary(&self, ctx: &Context<'_>, id: String) -> bool {
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        let removed = graph.delete_vocabulary(&id).is_some();
        if removed {
            persist(ctx, &graph);
        }
        removed
    }

    async fn create_system(
        &self,
        ctx: &Context<'_>,
        input: SystemInput,
    ) -> async_graphql::Result<GqlSystem> {
        let sys = input.into_system();
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        if graph.system(&sys.id).is_some() {
            return Err(Error::new(format!("System '{}' already exists", sys.id)));
        }
        graph.add_system(sys.clone());
        persist(ctx, &graph);
        Ok(GqlSystem::new(sys))
    }

    /// Author a whole System from custom term/connective **values** — the in-app
    /// editor path (build characters + vocabulary + system at runtime, persisted
    /// to the user store), instead of defining it in Rust seed tables.
    async fn author_system(
        &self,
        ctx: &Context<'_>,
        input: AuthorSystemInput,
    ) -> async_graphql::Result<GqlSystem> {
        let order = input.order as u8;
        if !(1..=12).contains(&order) {
            return Err(Error::new(format!("order {order} out of range 1..=12")));
        }
        let grammar = Template::for_order(order);
        let expected_conn = grammar.expected_connectives();
        if input.terms.len() != order as usize {
            return Err(Error::new(format!(
                "expected {order} terms, got {}",
                input.terms.len()
            )));
        }
        if input.connectives.len() != expected_conn {
            return Err(Error::new(format!(
                "order {order} expects {expected_conn} connectives, got {}",
                input.connectives.len()
            )));
        }

        // Match the id scheme used by `with_auto_id` so char ids stay unique.
        let slug = input.name.to_lowercase().replace(' ', "_");
        let sys_id = format!("system_{slug}_{order}");
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        if graph.system(&sys_id).is_some() {
            return Err(Error::new(format!("System '{sys_id}' already exists")));
        }

        let mut characters = Vec::new();
        let mut term_ids = Vec::new();
        for (i, value) in input.terms.iter().enumerate() {
            let id = format!("char_word_{slug}_t{}", i + 1);
            characters.push(Character::new(id.clone(), "word", value.clone()));
            term_ids.push(id);
        }
        let mut conn_ids = Vec::new();
        for (j, value) in input.connectives.iter().enumerate() {
            let id = format!("char_word_{slug}_c{}", j + 1);
            characters.push(Character::new(id.clone(), "word", value.clone()));
            conn_ids.push(id);
        }
        let vocab = Vocabulary::with_auto_id(&input.name, order, term_ids, conn_ids);
        let vocab_id = vocab.id.clone();
        let system = System::with_auto_id(
            &input.name,
            order,
            input.coherence.unwrap_or_else(|| "Custom".to_string()),
            input.term_designation.unwrap_or_else(|| "Terms".to_string()),
            input.connective_designation.unwrap_or_else(|| "Connectives".to_string()),
            format!("grammar_{order}"),
            &vocab_id,
        );
        let content = crate::core::content::GraphContent {
            characters,
            vocabularies: vec![vocab],
            systems: vec![system.clone()],
            ..Default::default()
        };
        graph.apply_content(&content);
        persist(ctx, &graph);
        Ok(GqlSystem::new(system))
    }

    async fn update_system(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: SystemInput,
    ) -> async_graphql::Result<GqlSystem> {
        let mut sys = input.into_system();
        sys.id = id.clone();
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        if graph.update_system(sys.clone()).is_none() {
            return Err(Error::new(format!("System '{}' not found", id)));
        }
        persist(ctx, &graph);
        Ok(GqlSystem::new(sys))
    }

    async fn delete_system(&self, ctx: &Context<'_>, id: String) -> bool {
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        let removed = graph.delete_system(&id).is_some();
        if removed {
            persist(ctx, &graph);
        }
        removed
    }

    // -------- Functors (same-grammar morphisms between Systems) --------

    async fn create_functor(
        &self,
        ctx: &Context<'_>,
        input: FunctorInput,
    ) -> async_graphql::Result<GqlFunctor> {
        let functor = input.into_functor();
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        if graph.functor(&functor.id).is_some() {
            return Err(Error::new(format!("Functor '{}' already exists", functor.id)));
        }
        graph.add_functor(functor.clone());
        persist(ctx, &graph);
        Ok(GqlFunctor::new(functor))
    }

    async fn update_functor(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: FunctorInput,
    ) -> async_graphql::Result<GqlFunctor> {
        let mut functor = input.into_functor();
        functor.id = id.clone();
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        if graph.update_functor(functor.clone()).is_none() {
            return Err(Error::new(format!("Functor '{}' not found", id)));
        }
        persist(ctx, &graph);
        Ok(GqlFunctor::new(functor))
    }

    async fn delete_functor(&self, ctx: &Context<'_>, id: String) -> bool {
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        let removed = graph.delete_functor(&id).is_some();
        if removed {
            persist(ctx, &graph);
        }
        removed
    }

    // -------- Sequences (ordered series of member addresses) --------

    async fn create_sequence(
        &self,
        ctx: &Context<'_>,
        input: SequenceInput,
    ) -> async_graphql::Result<GqlSequence> {
        // An explicit id must be respected (collision = error); an auto-generated
        // id (no id supplied) is disambiguated so repeated Extracts don't collide.
        let explicit_id = input.id.is_some();
        let mut sequence = input.into_sequence();
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        if graph.sequence(&sequence.id).is_some() {
            if explicit_id {
                return Err(Error::new(format!("Sequence '{}' already exists", sequence.id)));
            }
            sequence.id = graph.unique_sequence_id(&sequence.id);
        }
        graph.add_sequence(sequence.clone());
        persist(ctx, &graph);
        Ok(GqlSequence::new(sequence))
    }

    async fn update_sequence(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: SequenceInput,
    ) -> async_graphql::Result<GqlSequence> {
        let mut sequence = input.into_sequence();
        sequence.id = id.clone();
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        if graph.update_sequence(sequence.clone()).is_none() {
            return Err(Error::new(format!("Sequence '{}' not found", id)));
        }
        persist(ctx, &graph);
        Ok(GqlSequence::new(sequence))
    }

    async fn delete_sequence(&self, ctx: &Context<'_>, id: String) -> bool {
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        let removed = graph.delete_sequence(&id).is_some();
        if removed {
            persist(ctx, &graph);
        }
        removed
    }

    /// Transform a Perspective by a Functor, materialising and storing a new
    /// Perspective whose addresses over the functor's source system are remapped
    /// to its target system. Returns the new Perspective.
    async fn apply_functor(
        &self,
        ctx: &Context<'_>,
        functor_ref: String,
        perspective_ref: String,
        new_id: Option<String>,
        new_name: String,
    ) -> async_graphql::Result<GqlPerspective> {
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;

        let new_id = new_id.unwrap_or_else(|| {
            let slug: String = new_name
                .to_lowercase()
                .chars()
                .map(|c| if c.is_alphanumeric() { c } else { '_' })
                .collect();
            format!("perspective_{slug}")
        });
        if graph.perspective(&new_id).is_some() {
            return Err(Error::new(format!("Perspective '{new_id}' already exists")));
        }

        let new_persp = graph
            .apply_functor(&functor_ref, &perspective_ref, new_id, new_name)
            .ok_or_else(|| {
                Error::new(format!(
                    "apply_functor: functor '{functor_ref}' or perspective '{perspective_ref}' not found"
                ))
            })?;

        graph.add_perspective(new_persp.clone());
        persist(ctx, &graph);
        Ok(GqlPerspective::new(new_persp))
    }

    /// Load (E/L/T's receptive impulse): merge an exported perspective module
    /// bundle (the JSON `exportPerspective` returns) into the working graph.
    /// The merged content becomes user content (persisted to the writable store,
    /// so it survives a restart) — export it to a `data/perspectives/*.json` file
    /// to make it a permanent, bundled module. Manifest dependencies that don't
    /// resolve are returned as warnings, not errors (dangling is tolerated; load
    /// their home modules to resolve them).
    async fn load_perspective(
        &self,
        ctx: &Context<'_>,
        bundle: String,
    ) -> async_graphql::Result<GqlLoadReport> {
        let content: crate::core::GraphContent = serde_json::from_str(&bundle)
            .map_err(|e| Error::new(format!("invalid perspective bundle: {e}")))?;
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        graph.apply_content(&content);
        persist(ctx, &graph);

        let loaded = content.perspectives.iter().map(|p| p.id.clone()).collect();
        let unresolved = content
            .manifest
            .iter()
            .filter(|a| !graph.resolves(a))
            .cloned()
            .collect();
        Ok(GqlLoadReport { loaded, unresolved })
    }

    // -------- referencing layer: perspectives (webs) + citations --------

    async fn create_perspective(
        &self,
        ctx: &Context<'_>,
        name: String,
    ) -> async_graphql::Result<GqlPerspective> {
        let p = Perspective::with_auto_id(name, vec![]);
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        if graph.perspective(&p.id).is_some() {
            return Err(Error::new(format!("Perspective '{}' already exists", p.id)));
        }
        graph.add_perspective(p.clone());
        persist(ctx, &graph);
        Ok(GqlPerspective::new(p))
    }

    async fn delete_perspective(&self, ctx: &Context<'_>, id: String) -> bool {
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        let removed = graph.delete_perspective(&id).is_some();
        if removed {
            persist(ctx, &graph);
        }
        removed
    }

    /// Add an AD4M `{source, predicate, target}` link to a Perspective.
    async fn add_link(
        &self,
        ctx: &Context<'_>,
        perspective_id: String,
        source: String,
        predicate: String,
        target: String,
    ) -> async_graphql::Result<GqlLink> {
        let link = PerspectiveLink::new(source, predicate, target);
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        let p = graph
            .perspective_mut(&perspective_id)
            .ok_or_else(|| Error::new(format!("Perspective '{}' not found", perspective_id)))?;
        p.add_link(link.clone());
        persist(ctx, &graph);
        Ok(GqlLink::new(link))
    }

    async fn remove_link(&self, ctx: &Context<'_>, perspective_id: String, link_id: String) -> bool {
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        let removed = graph
            .perspective_mut(&perspective_id)
            .map(|p| p.remove_link(&link_id))
            .unwrap_or(false);
        if removed {
            persist(ctx, &graph);
        }
        removed
    }

    async fn create_source(&self, ctx: &Context<'_>, input: SourceInput) -> GqlSource {
        let s = Source::with_auto_id(input.name, input.kind, input.description);
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        graph.upsert_source(s.clone());
        persist(ctx, &graph);
        GqlSource::new(s)
    }

    async fn create_artefact(&self, ctx: &Context<'_>, input: ArtefactInput) -> GqlArtefact {
        let a = Artefact::with_auto_id(input.source_ref, input.title, input.kind, input.url);
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        graph.upsert_artefact(a.clone());
        persist(ctx, &graph);
        GqlArtefact::new(a)
    }

    async fn create_lookup(&self, ctx: &Context<'_>, input: LookupInput) -> GqlLookup {
        let l = Lookup::with_auto_id(input.artefact_ref, input.locator);
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        graph.upsert_lookup(l.clone());
        persist(ctx, &graph);
        GqlLookup::new(l)
    }

    /// Convenience: cite `target` within a Perspective in one call. Creates the
    /// Source (and optional Artefact + Lookup) inline, stores the Reference, and
    /// wires the citation links into the Perspective web.
    async fn create_reference(
        &self,
        ctx: &Context<'_>,
        input: CreateReferenceInput,
    ) -> async_graphql::Result<GqlReference> {
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;

        if graph.perspective(&input.perspective_ref).is_none() {
            return Err(Error::new(format!(
                "Perspective '{}' not found",
                input.perspective_ref
            )));
        }

        // Source (required).
        let source = Source::with_auto_id(input.source_name, input.source_kind, None);
        let source_id = source.id.clone();
        graph.upsert_source(source);

        // Artefact (optional).
        let artefact_id = input.artefact_title.as_ref().map(|title| {
            let a = Artefact::with_auto_id(&source_id, title.clone(), None, input.artefact_url.clone());
            let id = a.id.clone();
            graph.upsert_artefact(a);
            id
        });

        // Lookup (optional, requires an artefact).
        let lookup_id = match (&artefact_id, &input.lookup_locator) {
            (Some(art_id), Some(locator)) => {
                let l = Lookup::with_auto_id(art_id.clone(), locator.clone());
                let id = l.id.clone();
                graph.upsert_lookup(l);
                Some(id)
            }
            _ => None,
        };

        // The Reference edge.
        let reference = Reference::with_auto_id(
            input.perspective_ref.clone(),
            input.target.clone(),
            source_id.clone(),
            artefact_id.clone(),
            lookup_id.clone(),
            input.note,
        );
        graph.upsert_reference(reference.clone());

        // Wire the citation chain into the Perspective web as AD4M links.
        {
            let mut links = vec![PerspectiveLink::new(
                input.target.clone(),
                "cites",
                crate::core::perspectives::address::source(&source_id),
            )];
            if let Some(art_id) = &artefact_id {
                links.push(PerspectiveLink::new(
                    crate::core::perspectives::address::source(&source_id),
                    "recordedIn",
                    crate::core::perspectives::address::artefact(art_id),
                ));
            }
            if let (Some(art_id), Some(lk_id)) = (&artefact_id, &lookup_id) {
                links.push(PerspectiveLink::new(
                    crate::core::perspectives::address::artefact(art_id),
                    "atLocation",
                    crate::core::perspectives::address::lookup(lk_id),
                ));
            }
            if let Some(p) = graph.perspective_mut(&input.perspective_ref) {
                for l in links {
                    p.add_link(l);
                }
            }
        }

        persist(ctx, &graph);
        Ok(GqlReference::new(reference))
    }

    async fn delete_reference(&self, ctx: &Context<'_>, id: String) -> bool {
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        let removed = graph.delete_reference(&id).is_some();
        if removed {
            persist(ctx, &graph);
        }
        removed
    }
}

// ============================================================================
// Wrapper types
// ============================================================================

pub struct GqlOrder {
    inner: Order,
}
impl GqlOrder {
    pub fn new(inner: Order) -> Self {
        Self { inner }
    }
}
#[Object]
impl GqlOrder {
    async fn id(&self) -> &str {
        &self.inner.id
    }
    async fn value(&self) -> i32 {
        self.inner.value as i32
    }
    async fn standard_name(&self) -> Option<&str> {
        self.inner.standard_name()
    }
}

pub struct GqlPosition {
    inner: Position,
}
impl GqlPosition {
    pub fn new(inner: Position) -> Self {
        Self { inner }
    }
}
#[Object]
impl GqlPosition {
    async fn id(&self) -> &str {
        &self.inner.id
    }
    async fn value(&self) -> i32 {
        self.inner.value as i32
    }
}

pub struct GqlPoint {
    inner: Point,
}
impl GqlPoint {
    pub fn new(inner: Point) -> Self {
        Self { inner }
    }
}
#[Object]
impl GqlPoint {
    async fn id(&self) -> &str {
        &self.inner.id
    }
    async fn order(&self) -> Option<i32> {
        self.inner.order_value().map(|v| v as i32)
    }
    async fn position(&self) -> Option<i32> {
        self.inner.position_value().map(|v| v as i32)
    }
    async fn order_ref(&self) -> &str {
        &self.inner.order
    }
    async fn position_ref(&self) -> &str {
        &self.inner.position
    }
}

pub struct GqlLine {
    inner: Line,
}
impl GqlLine {
    pub fn new(inner: Line) -> Self {
        Self { inner }
    }
}
#[Object]
impl GqlLine {
    async fn id(&self) -> &str {
        &self.inner.id
    }
    async fn order(&self) -> Option<i32> {
        self.inner.order_value().map(|v| v as i32)
    }
    async fn position(&self) -> Option<i32> {
        self.inner.position_value().map(|v| v as i32)
    }
    async fn position_secondary(&self) -> Option<i32> {
        self.inner.position_secondary_value().map(|v| v as i32)
    }
}

pub struct GqlCoordinate {
    inner: Coordinate,
}
impl GqlCoordinate {
    pub fn new(inner: Coordinate) -> Self {
        Self { inner }
    }
}
#[Object]
impl GqlCoordinate {
    async fn id(&self) -> &str {
        &self.inner.id
    }
    async fn point_ref(&self) -> &str {
        &self.inner.point_ref
    }
    async fn order(&self) -> Option<i32> {
        self.inner.order_value().map(|v| v as i32)
    }
    async fn position(&self) -> Option<i32> {
        self.inner.position_value().map(|v| v as i32)
    }
    async fn x(&self) -> f64 {
        self.inner.x
    }
    async fn y(&self) -> f64 {
        self.inner.y
    }
    async fn z(&self) -> f64 {
        self.inner.z
    }
}

pub struct GqlSegment {
    inner: Segment,
}
impl GqlSegment {
    pub fn new(inner: Segment) -> Self {
        Self { inner }
    }
}
#[Object]
impl GqlSegment {
    async fn id(&self) -> &str {
        &self.inner.id
    }
    async fn line_ref(&self) -> &str {
        &self.inner.line_ref
    }
    async fn start_coord_ref(&self) -> &str {
        &self.inner.start_coord_ref
    }
    async fn end_coord_ref(&self) -> &str {
        &self.inner.end_coord_ref
    }
}

pub struct GqlCharacter {
    inner: Character,
}
impl GqlCharacter {
    pub fn new(inner: Character) -> Self {
        Self { inner }
    }
}
#[Object]
impl GqlCharacter {
    async fn id(&self) -> &str {
        &self.inner.id
    }
    async fn kind(&self) -> &str {
        &self.inner.kind
    }
    async fn value(&self) -> &str {
        &self.inner.value
    }
}

pub struct GqlTopology {
    inner: Topology,
}
impl GqlTopology {
    pub fn new(inner: Topology) -> Self {
        Self { inner }
    }
}
#[Object]
impl GqlTopology {
    async fn id(&self) -> &str {
        &self.inner.id
    }
    async fn order(&self) -> i32 {
        self.inner.order as i32
    }
    async fn points(&self) -> &[String] {
        &self.inner.points
    }
    async fn lines(&self) -> &[String] {
        &self.inner.lines
    }
    async fn validation_errors(&self) -> Vec<String> {
        self.inner.validate().err().unwrap_or_default()
    }
}

pub struct GqlGeometry {
    inner: Geometry,
}
impl GqlGeometry {
    pub fn new(inner: Geometry) -> Self {
        Self { inner }
    }
}
#[Object]
impl GqlGeometry {
    async fn id(&self) -> &str {
        &self.inner.id
    }
    async fn order(&self) -> i32 {
        self.inner.order as i32
    }
    async fn coordinates(&self) -> &[String] {
        &self.inner.coordinates
    }
    async fn segments(&self) -> &[String] {
        &self.inner.segments
    }
    async fn validation_errors(&self) -> Vec<String> {
        self.inner.validate().err().unwrap_or_default()
    }
}

pub struct GqlVocabulary {
    inner: Vocabulary,
}
impl GqlVocabulary {
    pub fn new(inner: Vocabulary) -> Self {
        Self { inner }
    }
}
#[Object]
impl GqlVocabulary {
    async fn id(&self) -> &str {
        &self.inner.id
    }
    async fn name(&self) -> &str {
        &self.inner.name
    }
    async fn order(&self) -> i32 {
        self.inner.order as i32
    }
    async fn terms(&self) -> &[String] {
        &self.inner.terms
    }
    async fn connectives(&self) -> &[String] {
        &self.inner.connectives
    }
    async fn validation_errors(&self) -> Vec<String> {
        self.inner.validate().err().unwrap_or_default()
    }
}

/// Structural Template wrapper (the K_n graph + arity rules for an Order).
pub struct GqlTemplate {
    inner: Template,
}
impl GqlTemplate {
    pub fn new(inner: Template) -> Self {
        Self { inner }
    }
}
#[Object]
impl GqlTemplate {
    async fn id(&self) -> &str {
        &self.inner.id
    }
    async fn order(&self) -> i32 {
        self.inner.order as i32
    }
    async fn topological_vocab_ref(&self) -> &str {
        &self.inner.topological_vocab_ref
    }
    async fn geometric_vocab_ref(&self) -> &str {
        &self.inner.geometric_vocab_ref
    }
    async fn expected_terms(&self) -> i32 {
        self.inner.expected_terms() as i32
    }
    async fn expected_connectives(&self) -> i32 {
        self.inner.expected_connectives() as i32
    }
    /// Degree — the connectivity of every vertex in K_n (`n − 1`); a constraint-value.
    async fn degree(&self) -> i32 {
        self.inner.degree() as i32
    }
    /// Size — the number of edges, `C(order, 2)`; a constraint-value.
    async fn size(&self) -> i32 {
        self.inner.size() as i32
    }
    /// Adjacency matrix (n × n) — the Structural Topology as a matrix.
    async fn adjacency_matrix(&self) -> Vec<Vec<i32>> {
        to_i32_matrix(self.inner.adjacency_matrix())
    }
    /// Incidence matrix (n × size) — vertices × edges (the Semantic Projection's anchoring).
    async fn incidence_matrix(&self) -> Vec<Vec<i32>> {
        to_i32_matrix(self.inner.incidence_matrix())
    }
    /// Line-graph adjacency (size × size) — reconciles adjacency and incidence.
    async fn line_graph_adjacency(&self) -> Vec<Vec<i32>> {
        to_i32_matrix(self.inner.line_graph_adjacency())
    }
}

/// Widen a `u8` matrix to `i32` for GraphQL (`[[Int!]!]`).
fn to_i32_matrix(m: Vec<Vec<u8>>) -> Vec<Vec<i32>> {
    m.into_iter()
        .map(|row| row.into_iter().map(|v| v as i32).collect())
        .collect()
}

/// System wrapper (metadata + Template/Vocabulary reconciliation).
pub struct GqlSystem {
    inner: System,
}
impl GqlSystem {
    pub fn new(inner: System) -> Self {
        Self { inner }
    }
}
#[Object]
impl GqlSystem {
    async fn id(&self) -> &str {
        &self.inner.id
    }
    async fn name(&self) -> &str {
        &self.inner.name
    }
    async fn order(&self) -> i32 {
        self.inner.order as i32
    }
    async fn coherence(&self) -> &str {
        &self.inner.coherence
    }
    async fn term_designation(&self) -> &str {
        &self.inner.term_designation
    }
    async fn connective_designation(&self) -> &str {
        &self.inner.connective_designation
    }
    async fn grammar_ref(&self) -> &str {
        &self.inner.grammar_ref
    }
    async fn vocabulary_ref(&self) -> &str {
        &self.inner.vocabulary_ref
    }
    /// The system's **term characters with their node position** (1..n) — resolved
    /// from the vocabulary in order, so the frontend anchors each term to its node.
    async fn terms(&self, ctx: &Context<'_>) -> Vec<GqlPositionedChar> {
        let g = graph_snapshot(ctx).await;
        g.vocabulary(&self.inner.vocabulary_ref)
            .map(|v| {
                v.terms
                    .iter()
                    .enumerate()
                    .filter_map(|(i, c)| {
                        g.character(c).map(|c| GqlPositionedChar {
                            value: c.value.clone(),
                            position: (i + 1).to_string(),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
    /// The system's **connective characters with their edge** (`base-target`) — the
    /// connective at vocabulary index `i` sits on the `i`-th canonical edge.
    async fn connectives(&self, ctx: &Context<'_>) -> Vec<GqlPositionedChar> {
        let g = graph_snapshot(ctx).await;
        let order = self.inner.order;
        g.vocabulary(&self.inner.vocabulary_ref)
            .map(|v| {
                v.connectives
                    .iter()
                    .enumerate()
                    .filter_map(|(i, c)| {
                        g.character(c).map(|c| GqlPositionedChar {
                            value: c.value.clone(),
                            position: nth_edge(order, i),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}

pub struct GqlSequence {
    inner: Sequence,
}
impl GqlSequence {
    pub fn new(inner: Sequence) -> Self {
        Self { inner }
    }
}
#[Object]
impl GqlSequence {
    async fn id(&self) -> &str {
        &self.inner.id
    }
    async fn name(&self) -> &str {
        &self.inner.name
    }
    /// Ordered member addresses.
    async fn members(&self) -> Vec<String> {
        self.inner.members.clone()
    }
}

pub struct GqlFunctor {
    inner: Functor,
}
impl GqlFunctor {
    pub fn new(inner: Functor) -> Self {
        Self { inner }
    }
}
#[Object]
impl GqlFunctor {
    async fn id(&self) -> &str {
        &self.inner.id
    }
    async fn name(&self) -> &str {
        &self.inner.name
    }
    async fn order(&self) -> i32 {
        self.inner.order as i32
    }
    async fn source_ref(&self) -> &str {
        &self.inner.source_ref
    }
    async fn target_ref(&self) -> &str {
        &self.inner.target_ref
    }
    /// The object map: `permutation[i]` is the 1-based target position of source
    /// position `i + 1`.
    async fn permutation(&self) -> Vec<i32> {
        self.inner.permutation.iter().map(|&p| p as i32).collect()
    }
}

/// Outcome of `loadPerspective`: what merged, and which manifest dependencies
/// remain unresolved (dangling — tolerated, load their home modules to resolve).
#[derive(SimpleObject)]
pub struct GqlLoadReport {
    /// Perspective ids merged into the graph.
    pub loaded: Vec<String>,
    /// Manifest addresses that don't resolve in the current graph.
    pub unresolved: Vec<String>,
}

// ============================================================================
// Input types
// ============================================================================

#[derive(InputObject)]
pub struct CharacterInput {
    pub id: Option<String>,
    pub kind: String,
    pub value: String,
}

impl CharacterInput {
    fn into_character(self) -> Character {
        match self.id {
            Some(id) => Character::new(id, self.kind, self.value),
            None => Character::with_auto_id(self.kind, self.value),
        }
    }
}

#[derive(InputObject)]
pub struct VocabularyInput {
    pub id: Option<String>,
    pub name: String,
    pub order: i32,
    pub terms: Vec<String>,
    pub connectives: Vec<String>,
}

impl VocabularyInput {
    fn into_vocabulary(self) -> Vocabulary {
        match self.id {
            Some(id) => Vocabulary::new(
                id,
                self.name,
                self.order as u8,
                self.terms,
                self.connectives,
            ),
            None => Vocabulary::with_auto_id(
                self.name,
                self.order as u8,
                self.terms,
                self.connectives,
            ),
        }
    }
}

#[derive(InputObject)]
pub struct SystemInput {
    pub id: Option<String>,
    pub name: String,
    pub order: i32,
    pub coherence: String,
    pub term_designation: String,
    pub connective_designation: String,
    pub grammar_ref: String,
    pub vocabulary_ref: String,
}

/// Author a System from custom **values** — the in-app editor input. `terms` has
/// `order` entries; `connectives` has C(order,2). Metadata is optional (defaults).
#[derive(InputObject)]
pub struct AuthorSystemInput {
    pub name: String,
    pub order: i32,
    pub terms: Vec<String>,
    pub connectives: Vec<String>,
    pub coherence: Option<String>,
    pub term_designation: Option<String>,
    pub connective_designation: Option<String>,
}

impl SystemInput {
    fn into_system(self) -> System {
        match self.id {
            Some(id) => System::new(
                id,
                self.name,
                self.order as u8,
                self.coherence,
                self.term_designation,
                self.connective_designation,
                self.grammar_ref,
                self.vocabulary_ref,
            ),
            None => System::with_auto_id(
                self.name,
                self.order as u8,
                self.coherence,
                self.term_designation,
                self.connective_designation,
                self.grammar_ref,
                self.vocabulary_ref,
            ),
        }
    }
}

#[derive(InputObject)]
pub struct SequenceInput {
    pub id: Option<String>,
    pub name: String,
    /// Ordered member addresses (`system:<id>`, `perspective:<id>`, `sequence:<id>`).
    pub members: Vec<String>,
}

impl SequenceInput {
    fn into_sequence(self) -> Sequence {
        match self.id {
            Some(id) => Sequence::new(id, self.name, self.members),
            None => Sequence::with_auto_id(self.name, self.members),
        }
    }
}

#[derive(InputObject)]
pub struct FunctorInput {
    pub id: Option<String>,
    pub name: String,
    pub order: i32,
    pub source_ref: String,
    pub target_ref: String,
    /// The object map: `permutation[i]` is the 1-based target position of source
    /// position `i + 1`. Validate with `validateFunctor` after creating.
    pub permutation: Vec<i32>,
}

impl FunctorInput {
    fn into_functor(self) -> Functor {
        let permutation: Vec<u8> = self.permutation.iter().map(|&p| p as u8).collect();
        let id = self.id.unwrap_or_else(|| {
            let slug: String = self
                .name
                .to_lowercase()
                .chars()
                .map(|c| if c.is_alphanumeric() { c } else { '_' })
                .collect();
            format!("functor_{slug}")
        });
        Functor::new(
            id,
            self.name,
            self.order as u8,
            self.source_ref,
            self.target_ref,
            permutation,
        )
    }
}

// ============================================================================
// Referencing layer wrappers (Perspective / Link / Source / Artefact / Lookup /
// Reference)
// ============================================================================

pub struct GqlLink {
    inner: PerspectiveLink,
}
impl GqlLink {
    pub fn new(inner: PerspectiveLink) -> Self {
        Self { inner }
    }
}
#[Object]
impl GqlLink {
    async fn id(&self) -> &str {
        &self.inner.id
    }
    async fn source(&self) -> &str {
        &self.inner.source
    }
    async fn predicate(&self) -> &str {
        &self.inner.predicate
    }
    async fn target(&self) -> &str {
        &self.inner.target
    }
}

pub struct GqlPerspective {
    inner: Perspective,
}
impl GqlPerspective {
    pub fn new(inner: Perspective) -> Self {
        Self { inner }
    }
}
#[Object]
impl GqlPerspective {
    async fn id(&self) -> &str {
        &self.inner.id
    }
    async fn name(&self) -> &str {
        &self.inner.name
    }
    async fn links(&self) -> Vec<GqlLink> {
        self.inner.links.iter().cloned().map(GqlLink::new).collect()
    }
}

pub struct GqlSource {
    inner: Source,
}
impl GqlSource {
    pub fn new(inner: Source) -> Self {
        Self { inner }
    }
}
#[Object]
impl GqlSource {
    async fn id(&self) -> &str {
        &self.inner.id
    }
    async fn name(&self) -> &str {
        &self.inner.name
    }
    async fn kind(&self) -> Option<&str> {
        self.inner.kind.as_deref()
    }
    async fn description(&self) -> Option<&str> {
        self.inner.description.as_deref()
    }
}

pub struct GqlArtefact {
    inner: Artefact,
}
impl GqlArtefact {
    pub fn new(inner: Artefact) -> Self {
        Self { inner }
    }
}
#[Object]
impl GqlArtefact {
    async fn id(&self) -> &str {
        &self.inner.id
    }
    async fn source_ref(&self) -> &str {
        &self.inner.source_ref
    }
    async fn title(&self) -> &str {
        &self.inner.title
    }
    async fn kind(&self) -> Option<&str> {
        self.inner.kind.as_deref()
    }
    async fn url(&self) -> Option<&str> {
        self.inner.url.as_deref()
    }
}

pub struct GqlLookup {
    inner: Lookup,
}
impl GqlLookup {
    pub fn new(inner: Lookup) -> Self {
        Self { inner }
    }
}
#[Object]
impl GqlLookup {
    async fn id(&self) -> &str {
        &self.inner.id
    }
    async fn artefact_ref(&self) -> &str {
        &self.inner.artefact_ref
    }
    async fn locator(&self) -> &str {
        &self.inner.locator
    }
}

pub struct GqlReference {
    inner: Reference,
}
impl GqlReference {
    pub fn new(inner: Reference) -> Self {
        Self { inner }
    }
}
#[Object]
impl GqlReference {
    async fn id(&self) -> &str {
        &self.inner.id
    }
    async fn perspective_ref(&self) -> &str {
        &self.inner.perspective_ref
    }
    async fn target(&self) -> &str {
        &self.inner.target
    }
    async fn source_ref(&self) -> &str {
        &self.inner.source_ref
    }
    async fn artefact_ref(&self) -> Option<&str> {
        self.inner.artefact_ref.as_deref()
    }
    async fn lookup_ref(&self) -> Option<&str> {
        self.inner.lookup_ref.as_deref()
    }
    async fn note(&self) -> Option<&str> {
        self.inner.note.as_deref()
    }
    /// The **object** of the SPO assertion — the *value* the source affixes to the
    /// target predicate (e.g. target `…#coherence`, object `"Relatedness"`). #25.
    async fn object(&self) -> Option<&str> {
        self.inner.object.as_deref()
    }
    /// Resolve the cited Source (provenance origin).
    async fn source(&self, ctx: &Context<'_>) -> Option<GqlSource> {
        let g = graph_snapshot(ctx).await;
        g.source(&self.inner.source_ref).map(|s| GqlSource::new(s.clone()))
    }
    /// Resolve the Artefact, if any.
    async fn artefact(&self, ctx: &Context<'_>) -> Option<GqlArtefact> {
        let id = self.inner.artefact_ref.as_ref()?;
        let g = graph_snapshot(ctx).await;
        g.artefact(id).map(|a| GqlArtefact::new(a.clone()))
    }
    /// Resolve the Lookup, if any.
    async fn lookup(&self, ctx: &Context<'_>) -> Option<GqlLookup> {
        let id = self.inner.lookup_ref.as_ref()?;
        let g = graph_snapshot(ctx).await;
        g.lookup(id).map(|l| GqlLookup::new(l.clone()))
    }

    /// Human-readable name of the owning Perspective (the reference web this
    /// citation belongs to) — for grouping/filtering the reference browser.
    async fn perspective_name(&self, ctx: &Context<'_>) -> Option<String> {
        let g = graph_snapshot(ctx).await;
        g.perspective(&self.inner.perspective_ref)
            .map(|p| p.name.clone())
    }

    /// The fragment of the target address after `#` — `coherence`, `term:2`,
    /// `conn:1-2`, `term-designation`, … — or empty when the whole System is
    /// cited. Lets the browser filter by what is being cited.
    async fn target_fragment(&self) -> String {
        self.inner
            .target
            .split_once('#')
            .map(|(_, frag)| frag.to_string())
            .unwrap_or_default()
    }

    /// Resolve the System behind the target address (`system:<id>[#…]`) — carries
    /// the order, name and coherence/designation values, so the browser can
    /// group references by order and show each perspective's value side by side.
    async fn target_system(&self, ctx: &Context<'_>) -> Option<GqlSystem> {
        let rest = self.inner.target.strip_prefix("system:")?;
        let id = rest.split('#').next().unwrap_or(rest);
        let g = graph_snapshot(ctx).await;
        g.system(id).map(|s| GqlSystem::new(s.clone()))
    }
}

#[derive(InputObject)]
pub struct SourceInput {
    pub name: String,
    pub kind: Option<String>,
    pub description: Option<String>,
}

#[derive(InputObject)]
pub struct ArtefactInput {
    pub source_ref: String,
    pub title: String,
    pub kind: Option<String>,
    pub url: Option<String>,
}

#[derive(InputObject)]
pub struct LookupInput {
    pub artefact_ref: String,
    pub locator: String,
}

/// One-call citation: perspective + cited target + inline Source (and optional
/// Artefact + Lookup).
#[derive(InputObject)]
pub struct CreateReferenceInput {
    pub perspective_ref: String,
    pub target: String,
    pub source_name: String,
    pub source_kind: Option<String>,
    pub artefact_title: Option<String>,
    pub artefact_url: Option<String>,
    pub lookup_locator: Option<String>,
    pub note: Option<String>,
}

// ============================================================================
// Schema
// ============================================================================

pub type SystematicsSchema =
    async_graphql::Schema<QueryRoot, MutationRoot, async_graphql::EmptySubscription>;

/// Build the schema with a shared graph and no persistence (tests, ephemeral).
pub fn create_schema(graph: SharedGraph) -> SystematicsSchema {
    create_schema_with_store(graph, None)
}

/// Build the schema with a shared graph and an optional writable store path.
/// When `store` is `Some`, mutations persist the user slice after each change.
pub fn create_schema_with_store(graph: SharedGraph, store: Option<PathBuf>) -> SystematicsSchema {
    async_graphql::Schema::build(QueryRoot, MutationRoot, async_graphql::EmptySubscription)
        .data(graph)
        .data(StorePath(store))
        .finish()
}
