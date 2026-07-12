//! GraphQL types and schema for the Systematics property graph API.

use std::path::PathBuf;
use std::sync::Arc;

use async_graphql::*;
use tokio::sync::RwLock;

use crate::core::{
    Character, Coordinate, Entry, GeometricVocabulary, Perspective, Graph, Line, Order, Point,
    Position, SemanticVocabulary, Segment, TopologicalVocabulary,
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

    async fn topological_vocab(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Option<GqlTopologicalVocabulary> {
        let g = graph_snapshot(ctx).await;
        g.topological_vocab(&id)
            .map(|v| GqlTopologicalVocabulary::new(v.clone()))
    }

    async fn topological_vocab_for_order(
        &self,
        ctx: &Context<'_>,
        order: i32,
    ) -> Option<GqlTopologicalVocabulary> {
        let g = graph_snapshot(ctx).await;
        g.topological_vocab_for_order(order as u8)
            .map(|v| GqlTopologicalVocabulary::new(v.clone()))
    }

    async fn geometric_vocab(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Option<GqlGeometricVocabulary> {
        let g = graph_snapshot(ctx).await;
        g.geometric_vocab(&id)
            .map(|v| GqlGeometricVocabulary::new(v.clone()))
    }

    async fn geometric_vocab_for_order(
        &self,
        ctx: &Context<'_>,
        order: i32,
    ) -> Option<GqlGeometricVocabulary> {
        let g = graph_snapshot(ctx).await;
        g.geometric_vocab_for_order(order as u8)
            .map(|v| GqlGeometricVocabulary::new(v.clone()))
    }

    async fn semantic_vocab(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Option<GqlSemanticVocabulary> {
        let g = graph_snapshot(ctx).await;
        g.semantic_vocab(&id)
            .map(|v| GqlSemanticVocabulary::new(v.clone()))
    }

    async fn semantic_vocabs_for_order(
        &self,
        ctx: &Context<'_>,
        order: i32,
    ) -> Vec<GqlSemanticVocabulary> {
        let g = graph_snapshot(ctx).await;
        g.semantic_vocabs_for_order(order as u8)
            .into_iter()
            .map(|v| GqlSemanticVocabulary::new(v.clone()))
            .collect()
    }

    async fn perspective(&self, ctx: &Context<'_>, id: String) -> Option<GqlPerspective> {
        let g = graph_snapshot(ctx).await;
        g.perspective(&id).map(|gr| GqlPerspective::new(gr.clone()))
    }

    async fn perspectives_for_order(&self, ctx: &Context<'_>, order: i32) -> Vec<GqlPerspective> {
        let g = graph_snapshot(ctx).await;
        g.perspectives_for_order(order as u8)
            .into_iter()
            .map(|gr| GqlPerspective::new(gr.clone()))
            .collect()
    }

    // -------- joins --------

    async fn character_at_point(
        &self,
        ctx: &Context<'_>,
        semantic_vocab_id: String,
        point_id: String,
    ) -> Option<GqlCharacter> {
        let g = graph_snapshot(ctx).await;
        g.character_at_point(&semantic_vocab_id, &point_id)
            .map(|c| GqlCharacter::new(c.clone()))
    }

    async fn character_at_line(
        &self,
        ctx: &Context<'_>,
        semantic_vocab_id: String,
        line_id: String,
    ) -> Option<GqlCharacter> {
        let g = graph_snapshot(ctx).await;
        g.character_at_line(&semantic_vocab_id, &line_id)
            .map(|c| GqlCharacter::new(c.clone()))
    }

    async fn validate_perspective(&self, ctx: &Context<'_>, id: String) -> Vec<String> {
        let g = graph_snapshot(ctx).await;
        g.validate_perspective(&id).err().unwrap_or_default()
    }

    // -------- resolved Grammar (a Perspective resolved into a bound K-graph) --------

    /// Resolve any Perspective (canonical or user) into its complete Grammar.
    async fn grammar(&self, ctx: &Context<'_>, perspective_id: String) -> Option<GqlGrammar> {
        let g = graph_snapshot(ctx).await;
        resolve_perspective(&g, &perspective_id).map(GqlGrammar::new)
    }

    /// Convenience: the canonical Grammar for an order (resolves the seed's
    /// `Canonical <Name>` perspective).
    async fn system(&self, ctx: &Context<'_>, order: i32) -> Option<GqlGrammar> {
        if !(1..=12).contains(&order) {
            return None;
        }
        let g = graph_snapshot(ctx).await;
        resolve_perspective(&g, &canonical_perspective_id(order as u8)).map(GqlGrammar::new)
    }

    async fn system_by_name(&self, ctx: &Context<'_>, name: String) -> Option<GqlGrammar> {
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
        resolve_perspective(&g, &canonical_perspective_id(order)).map(GqlGrammar::new)
    }

    async fn all_systems(&self, ctx: &Context<'_>) -> Vec<GqlGrammar> {
        let g = graph_snapshot(ctx).await;
        (1..=12u8)
            .filter_map(|o| resolve_perspective(&g, &canonical_perspective_id(o)).map(GqlGrammar::new))
            .collect()
    }
}

// ============================================================================
// Compat: computed Grammar for the current SVG frontend renderer.
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

pub struct GrammarData {
    pub order: u8,
    pub name: String,
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

/// Canonical perspective id for an order (the seed's `Canonical <Name>`).
fn canonical_perspective_id(order: u8) -> String {
    format!(
        "perspective_canonical_{}_{}",
        canonical_system_name(order).to_lowercase(),
        order
    )
}

/// Resolve a Perspective (by id) into a complete bound Grammar — terms bound to
/// points, connectives to lines, coordinates + colours per position, metadata
/// applied. This is "resolve the wiring into a renderable K-graph".
fn resolve_perspective(graph: &Graph, perspective_id: &str) -> Option<GrammarData> {
    let perspective = graph.perspective(perspective_id)?;
    let order = perspective.order;
    let name = perspective.name.clone();
    let topology = graph.topological_vocab(&perspective.topological_vocab_ref)?;
    let semantic = graph.semantic_vocab(&perspective.semantic_vocab_ref)?;
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

    Some(GrammarData {
        order,
        name,
        coherence: perspective.coherence.clone(),
        term_designation: perspective.term_designation.clone(),
        connective_designation: perspective.connective_designation.clone(),
        terms,
        coordinates,
        colours,
        lines,
        connectives,
    })
}

pub struct GqlGrammar {
    inner: GrammarData,
}

impl GqlGrammar {
    pub fn new(inner: GrammarData) -> Self {
        Self { inner }
    }
}

#[Object]
impl GqlGrammar {
    async fn order(&self) -> i32 {
        self.inner.order as i32
    }
    async fn name(&self) -> &str {
        &self.inner.name
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

    async fn create_semantic_vocab(
        &self,
        ctx: &Context<'_>,
        input: SemanticVocabInput,
    ) -> async_graphql::Result<GqlSemanticVocabulary> {
        let sv = input.into_semantic_vocab();
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        if graph.semantic_vocab(&sv.id).is_some() {
            return Err(Error::new(format!(
                "SemanticVocabulary '{}' already exists",
                sv.id
            )));
        }
        graph.add_semantic_vocab(sv.clone());
        persist(ctx, &graph);
        Ok(GqlSemanticVocabulary::new(sv))
    }

    async fn update_semantic_vocab(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: SemanticVocabInput,
    ) -> async_graphql::Result<GqlSemanticVocabulary> {
        let mut sv = input.into_semantic_vocab();
        sv.id = id.clone();
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        if graph.update_semantic_vocab(sv.clone()).is_none() {
            return Err(Error::new(format!("SemanticVocabulary '{}' not found", id)));
        }
        persist(ctx, &graph);
        Ok(GqlSemanticVocabulary::new(sv))
    }

    async fn delete_semantic_vocab(&self, ctx: &Context<'_>, id: String) -> bool {
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        let removed = graph.delete_semantic_vocab(&id).is_some();
        if removed {
            persist(ctx, &graph);
        }
        removed
    }

    async fn create_perspective(
        &self,
        ctx: &Context<'_>,
        input: PerspectiveInput,
    ) -> async_graphql::Result<GqlPerspective> {
        let gr = input.into_perspective();
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        if graph.perspective(&gr.id).is_some() {
            return Err(Error::new(format!("Perspective '{}' already exists", gr.id)));
        }
        graph.add_perspective(gr.clone());
        persist(ctx, &graph);
        Ok(GqlPerspective::new(gr))
    }

    async fn update_perspective(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: PerspectiveInput,
    ) -> async_graphql::Result<GqlPerspective> {
        let mut gr = input.into_perspective();
        gr.id = id.clone();
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        if graph.update_perspective(gr.clone()).is_none() {
            return Err(Error::new(format!("Perspective '{}' not found", id)));
        }
        persist(ctx, &graph);
        Ok(GqlPerspective::new(gr))
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

pub struct GqlTopologicalVocabulary {
    inner: TopologicalVocabulary,
}
impl GqlTopologicalVocabulary {
    pub fn new(inner: TopologicalVocabulary) -> Self {
        Self { inner }
    }
}
#[Object]
impl GqlTopologicalVocabulary {
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

pub struct GqlGeometricVocabulary {
    inner: GeometricVocabulary,
}
impl GqlGeometricVocabulary {
    pub fn new(inner: GeometricVocabulary) -> Self {
        Self { inner }
    }
}
#[Object]
impl GqlGeometricVocabulary {
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

pub struct GqlSemanticVocabulary {
    inner: SemanticVocabulary,
}
impl GqlSemanticVocabulary {
    pub fn new(inner: SemanticVocabulary) -> Self {
        Self { inner }
    }
}
#[Object]
impl GqlSemanticVocabulary {
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
    async fn topological_vocab_ref(&self) -> &str {
        &self.inner.topological_vocab_ref
    }
    async fn geometric_vocab_ref(&self) -> &str {
        &self.inner.geometric_vocab_ref
    }
    async fn semantic_vocab_ref(&self) -> &str {
        &self.inner.semantic_vocab_ref
    }
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
pub struct SemanticVocabInput {
    pub id: Option<String>,
    pub name: String,
    pub order: i32,
    pub terms: Vec<String>,
    pub connectives: Vec<String>,
}

impl SemanticVocabInput {
    fn into_semantic_vocab(self) -> SemanticVocabulary {
        match self.id {
            Some(id) => SemanticVocabulary::new(
                id,
                self.name,
                self.order as u8,
                self.terms,
                self.connectives,
            ),
            None => SemanticVocabulary::with_auto_id(
                self.name,
                self.order as u8,
                self.terms,
                self.connectives,
            ),
        }
    }
}

#[derive(InputObject)]
pub struct PerspectiveInput {
    pub id: Option<String>,
    pub name: String,
    pub order: i32,
    pub coherence: String,
    pub term_designation: String,
    pub connective_designation: String,
    pub topological_vocab_ref: String,
    pub geometric_vocab_ref: String,
    pub semantic_vocab_ref: String,
}

impl PerspectiveInput {
    fn into_perspective(self) -> Perspective {
        match self.id {
            Some(id) => Perspective::new(
                id,
                self.name,
                self.order as u8,
                self.coherence,
                self.term_designation,
                self.connective_designation,
                self.topological_vocab_ref,
                self.geometric_vocab_ref,
                self.semantic_vocab_ref,
            ),
            None => Perspective::with_auto_id(
                self.name,
                self.order as u8,
                self.coherence,
                self.term_designation,
                self.connective_designation,
                self.topological_vocab_ref,
                self.geometric_vocab_ref,
                self.semantic_vocab_ref,
            ),
        }
    }
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
