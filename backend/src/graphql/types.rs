//! GraphQL types and schema for the Systematics property graph API.

use std::sync::Arc;

use tokio::sync::RwLock;

use crate::core::{
    Character, CoherenceAttribute, Colour, ConnectiveDesignation, Coordinate, Entry, Functor,
    FunctorMapping, Graph, Language, Link, LinkType, Location, Order, Position, SystemName, Term,
    TermDesignation,
};
use async_graphql::*;

/// Shared, mutable graph passed to the GraphQL schema as context data.
pub type SharedGraph = Arc<RwLock<Graph>>;

/// Take a cheap snapshot of the shared graph for read-only resolvers.
async fn graph_snapshot(ctx: &Context<'_>) -> Graph {
    ctx.data_unchecked::<SharedGraph>().read().await.clone()
}

/// Access the shared graph for mutation resolvers.
fn shared_graph<'a>(ctx: &'a Context<'_>) -> &'a SharedGraph {
    ctx.data_unchecked::<SharedGraph>()
}

/// Root query object
#[derive(Clone, Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // ========================================================================
    // Graph Queries
    // ========================================================================

    /// Get the full graph with all entries and links
    async fn graph(&self, ctx: &Context<'_>) -> GqlGraph {
        GqlGraph::new(graph_snapshot(ctx).await)
    }

    // ========================================================================
    // Anchor Queries
    // ========================================================================

    /// Get an Order anchor by value (1-12)
    async fn order(&self, ctx: &Context<'_>, value: i32) -> Option<GqlOrder> {
        if !(1..=12).contains(&value) {
            return None;
        }
        let graph = graph_snapshot(ctx).await;
        graph
            .order(value as u8)
            .map(|o| GqlOrder::new(o.clone(), graph.clone()))
    }

    /// Get all Order anchors
    async fn orders(&self, ctx: &Context<'_>) -> Vec<GqlOrder> {
        let graph = graph_snapshot(ctx).await;
        graph
            .orders()
            .into_iter()
            .map(|o| GqlOrder::new(o.clone(), graph.clone()))
            .collect()
    }

    /// Get a Position anchor by value (1-12)
    async fn position(&self, ctx: &Context<'_>, value: i32) -> Option<GqlPosition> {
        if !(1..=12).contains(&value) {
            return None;
        }
        let graph = graph_snapshot(ctx).await;
        graph
            .position(value as u8)
            .map(|p| GqlPosition::new(p.clone(), graph.clone()))
    }

    /// Get all Position anchors
    async fn positions(&self, ctx: &Context<'_>) -> Vec<GqlPosition> {
        let graph = graph_snapshot(ctx).await;
        graph
            .positions()
            .into_iter()
            .map(|p| GqlPosition::new(p.clone(), graph.clone()))
            .collect()
    }

    /// Get a Location anchor by order and position
    async fn location(&self, ctx: &Context<'_>, order: i32, position: i32) -> Option<GqlLocation> {
        if !(1..=12).contains(&order) || position < 1 || position > order {
            return None;
        }
        let graph = graph_snapshot(ctx).await;
        graph
            .location(order as u8, position as u8)
            .map(|l| GqlLocation::new(l.clone(), graph.clone()))
    }

    /// Get all Location anchors
    async fn locations(&self, ctx: &Context<'_>) -> Vec<GqlLocation> {
        let graph = graph_snapshot(ctx).await;
        graph
            .locations()
            .into_iter()
            .map(|l| GqlLocation::new(l.clone(), graph.clone()))
            .collect()
    }

    /// Get all Locations for a given order
    async fn locations_for_order(&self, ctx: &Context<'_>, order: i32) -> Vec<GqlLocation> {
        let graph = graph_snapshot(ctx).await;
        graph
            .locations_for_order(order as u8)
            .into_iter()
            .map(|l| GqlLocation::new(l.clone(), graph.clone()))
            .collect()
    }

    /// Get all Locations for a given position (across all orders)
    async fn locations_for_position(&self, ctx: &Context<'_>, position: i32) -> Vec<GqlLocation> {
        let graph = graph_snapshot(ctx).await;
        graph
            .locations_for_position(position as u8)
            .into_iter()
            .map(|l| GqlLocation::new(l.clone(), graph.clone()))
            .collect()
    }

    // ========================================================================
    // System Queries
    // ========================================================================

    /// Get system by order (1-12)
    async fn system(&self, ctx: &Context<'_>, order: i32) -> Option<GqlSystemView> {
        if !(1..=12).contains(&order) {
            return None;
        }
        let graph = graph_snapshot(ctx).await;
        Some(GqlSystemView::new(order as u8, graph))
    }

    /// Get all systems (1-12)
    async fn all_systems(&self, ctx: &Context<'_>) -> Vec<GqlSystemView> {
        let graph = graph_snapshot(ctx).await;
        (1..=12)
            .map(|order| GqlSystemView::new(order, graph.clone()))
            .collect()
    }

    /// Get system by name (e.g., "Triad")
    async fn system_by_name(&self, ctx: &Context<'_>, name: String) -> Option<GqlSystemView> {
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
        let graph = graph_snapshot(ctx).await;
        Some(GqlSystemView::new(order, graph))
    }

    // ========================================================================
    // Term Queries
    // ========================================================================

    /// Get term at a specific order and position
    async fn term(&self, ctx: &Context<'_>, order: i32, position: i32) -> Option<GqlTerm> {
        let graph = graph_snapshot(ctx).await;
        graph
            .term(order as u8, position as u8)
            .map(|t| GqlTerm::new(t.clone(), &graph))
    }

    /// Get all terms for an order
    async fn terms(
        &self,
        ctx: &Context<'_>,
        order: i32,
        language: Option<GqlLanguage>,
    ) -> Vec<GqlTerm> {
        let graph = graph_snapshot(ctx).await;
        let lang = language.map(|l| l.into());
        graph
            .terms(order as u8, lang)
            .into_iter()
            .map(|t| GqlTerm::new(t.clone(), &graph))
            .collect()
    }

    // ========================================================================
    // Character Queries
    // ========================================================================

    /// Get all characters for a language
    async fn characters(&self, ctx: &Context<'_>, language: GqlLanguage) -> Vec<GqlCharacter> {
        let graph = graph_snapshot(ctx).await;
        graph
            .characters(language.into())
            .into_iter()
            .map(|c| GqlCharacter::new(c.clone()))
            .collect()
    }

    // ========================================================================
    // Slice Queries
    // ========================================================================

    /// Get slice (all entries at order+position)
    async fn slice(&self, ctx: &Context<'_>, order: i32, position: i32) -> GqlSlice {
        let graph = graph_snapshot(ctx).await;
        GqlSlice::new(order as u8, position as u8, graph)
    }

    // ========================================================================
    // Language Queries
    // ========================================================================

    /// Get all available languages
    async fn languages(&self) -> Vec<GqlLanguage> {
        vec![
            GqlLanguage::Canonical,
            GqlLanguage::Energy,
            GqlLanguage::Values,
            GqlLanguage::Society,
            GqlLanguage::Hex,
            GqlLanguage::Name,
        ]
    }

    /// Get vocabulary languages (for Character entries)
    async fn vocabulary_languages(&self) -> Vec<GqlLanguage> {
        vec![
            GqlLanguage::Canonical,
            GqlLanguage::Energy,
            GqlLanguage::Values,
            GqlLanguage::Society,
        ]
    }

    // ========================================================================
    // Functor Queries
    // ========================================================================

    /// All Functors in the graph
    async fn functors(&self, ctx: &Context<'_>) -> Vec<GqlFunctor> {
        let graph = graph_snapshot(ctx).await;
        graph
            .functors()
            .iter()
            .map(|f| GqlFunctor::new(f.clone()))
            .collect()
    }

    /// Get a Functor by ID
    async fn functor(&self, ctx: &Context<'_>, id: String) -> Option<GqlFunctor> {
        let graph = graph_snapshot(ctx).await;
        graph.functor(&id).map(|f| GqlFunctor::new(f.clone()))
    }
}

// ============================================================================
// Mutation Root
// ============================================================================

/// Root mutation object.
///
/// Deliberately exposes only Functor CRUD — the fixed order metadata
/// (`SystemName`, `CoherenceAttribute`, `TermDesignation`, `ConnectiveDesignation`)
/// is treated as schema-level and is not mutable at runtime.
#[derive(Clone, Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Create a Functor. If `id` is None, one is generated from the name.
    async fn create_functor(
        &self,
        ctx: &Context<'_>,
        input: FunctorInput,
    ) -> async_graphql::Result<GqlFunctor> {
        let functor: Functor = input.into_functor();
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        if graph.functor(&functor.id).is_some() {
            return Err(Error::new(format!(
                "Functor with id '{}' already exists",
                functor.id
            )));
        }
        graph.add_functor(functor.clone());
        Ok(GqlFunctor::new(functor))
    }

    /// Replace an existing Functor's fields by ID.
    async fn update_functor(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: FunctorInput,
    ) -> async_graphql::Result<GqlFunctor> {
        let mut functor: Functor = input.into_functor();
        functor.id = id.clone();
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        if graph.update_functor(functor.clone()).is_none() {
            return Err(Error::new(format!("Functor '{}' not found", id)));
        }
        Ok(GqlFunctor::new(functor))
    }

    /// Delete a Functor by ID. Returns true on success.
    async fn delete_functor(&self, ctx: &Context<'_>, id: String) -> bool {
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        graph.delete_functor(&id).is_some()
    }

    /// Apply a stored Functor, materialising its mappings as Terms in the graph.
    /// Returns the Terms produced (idempotent — re-application replaces).
    async fn apply_functor(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<Vec<GqlTerm>> {
        let graph_arc = shared_graph(ctx);
        let mut graph = graph_arc.write().await;
        let produced = graph
            .apply_functor(&id)
            .ok_or_else(|| Error::new(format!("Functor '{}' not found", id)))?;
        let snapshot = graph.clone();
        Ok(produced
            .into_iter()
            .map(|t| GqlTerm::new(t, &snapshot))
            .collect())
    }
}

// ============================================================================
// GraphQL Enums
// ============================================================================

/// Language enum for vocabularies and representations
#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum GqlLanguage {
    Canonical,
    Energy,
    Values,
    Society,
    Hex,
    Name,
}

impl From<GqlLanguage> for Language {
    fn from(l: GqlLanguage) -> Self {
        match l {
            GqlLanguage::Canonical => Language::Canonical,
            GqlLanguage::Energy => Language::Energy,
            GqlLanguage::Values => Language::Values,
            GqlLanguage::Society => Language::Society,
            GqlLanguage::Hex => Language::Hex,
            GqlLanguage::Name => Language::Name,
        }
    }
}

impl From<Language> for GqlLanguage {
    fn from(l: Language) -> Self {
        match l {
            Language::Canonical => GqlLanguage::Canonical,
            Language::Energy => GqlLanguage::Energy,
            Language::Values => GqlLanguage::Values,
            Language::Society => GqlLanguage::Society,
            Language::Hex => GqlLanguage::Hex,
            Language::Name => GqlLanguage::Name,
        }
    }
}

/// Link type enum
#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum GqlLinkType {
    Line,
    Connective,
}

// ============================================================================
// Graph Types
// ============================================================================

/// The full property graph
pub struct GqlGraph {
    graph: Graph,
}

impl GqlGraph {
    pub fn new(graph: Graph) -> Self {
        Self { graph }
    }
}

#[Object]
impl GqlGraph {
    /// Total number of entries
    async fn entry_count(&self) -> i32 {
        self.graph.entries.len() as i32
    }

    /// Total number of links
    async fn link_count(&self) -> i32 {
        self.graph.links.len() as i32
    }

    /// All entries in the graph
    async fn entries(&self) -> Vec<GqlEntry> {
        self.graph
            .entries
            .iter()
            .map(|e| GqlEntry::new(e.clone(), &self.graph))
            .collect()
    }

    /// All links in the graph
    async fn links(&self) -> Vec<GqlLink> {
        self.graph
            .links
            .iter()
            .map(|l| GqlLink::new(l.clone(), &self.graph))
            .collect()
    }

    /// Get entry by ID
    async fn entry(&self, id: String) -> Option<GqlEntry> {
        self.graph
            .get_entry(&id)
            .map(|e| GqlEntry::new(e.clone(), &self.graph))
    }

    /// Get link by ID
    async fn link(&self, id: String) -> Option<GqlLink> {
        self.graph
            .get_link(&id)
            .map(|l| GqlLink::new(l.clone(), &self.graph))
    }
}

// ============================================================================
// Entry Types
// ============================================================================

/// A graph entry (union type)
pub struct GqlEntry {
    entry: Entry,
    graph: Graph,
}

impl GqlEntry {
    pub fn new(entry: Entry, graph: &Graph) -> Self {
        Self {
            entry,
            graph: graph.clone(),
        }
    }
}

#[Object]
impl GqlEntry {
    /// Entry ID
    async fn id(&self) -> &str {
        self.entry.id()
    }

    /// Entry order (if applicable)
    async fn order(&self) -> Option<i32> {
        self.entry.order().map(|o| o as i32)
    }

    /// Entry position (if applicable)
    async fn position(&self) -> Option<i32> {
        self.entry.position().map(|p| p as i32)
    }

    /// Is this an order-level entry? (references Order anchor)
    async fn is_order_level_entry(&self) -> bool {
        self.entry.is_order_level()
    }

    /// Is this a location-level entry? (references Location anchor)
    async fn is_location_level_entry(&self) -> bool {
        self.entry.is_location_level()
    }

    /// Entry type name
    async fn entry_type(&self) -> &str {
        match &self.entry {
            Entry::Order(_) => "Order",
            Entry::Position(_) => "Position",
            Entry::Location(_) => "Location",
            Entry::SystemName(_) => "SystemName",
            Entry::CoherenceAttribute(_) => "CoherenceAttribute",
            Entry::TermDesignation(_) => "TermDesignation",
            Entry::ConnectiveDesignation(_) => "ConnectiveDesignation",
            Entry::Term(_) => "Term",
            Entry::Colour(_) => "Colour",
            Entry::Coordinate(_) => "Coordinate",
            Entry::Character(_) => "Character",
        }
    }

    /// Is this an anchor type?
    async fn is_anchor(&self) -> bool {
        self.entry.is_anchor()
    }

    /// Is this an order-level entry?
    async fn is_order_level(&self) -> bool {
        self.entry.is_order_level()
    }

    /// Is this a location-level entry?
    async fn is_location_level(&self) -> bool {
        self.entry.is_location_level()
    }

    /// As Order (if applicable)
    async fn as_order(&self) -> Option<GqlOrder> {
        match &self.entry {
            Entry::Order(o) => Some(GqlOrder::new(o.clone(), self.graph.clone())),
            _ => None,
        }
    }

    /// As Position (if applicable)
    async fn as_position(&self) -> Option<GqlPosition> {
        match &self.entry {
            Entry::Position(p) => Some(GqlPosition::new(p.clone(), self.graph.clone())),
            _ => None,
        }
    }

    /// As Location (if applicable)
    async fn as_location(&self) -> Option<GqlLocation> {
        match &self.entry {
            Entry::Location(l) => Some(GqlLocation::new(l.clone(), self.graph.clone())),
            _ => None,
        }
    }

    /// As SystemName (if applicable)
    async fn as_system_name(&self) -> Option<GqlSystemName> {
        match &self.entry {
            Entry::SystemName(s) => Some(GqlSystemName::new(s.clone())),
            _ => None,
        }
    }

    /// As CoherenceAttribute (if applicable)
    async fn as_coherence(&self) -> Option<GqlCoherenceAttribute> {
        match &self.entry {
            Entry::CoherenceAttribute(c) => Some(GqlCoherenceAttribute::new(c.clone())),
            _ => None,
        }
    }

    /// As TermDesignation (if applicable)
    async fn as_term_designation(&self) -> Option<GqlTermDesignation> {
        match &self.entry {
            Entry::TermDesignation(t) => Some(GqlTermDesignation::new(t.clone())),
            _ => None,
        }
    }

    /// As ConnectiveDesignation (if applicable)
    async fn as_connective_designation(&self) -> Option<GqlConnectiveDesignation> {
        match &self.entry {
            Entry::ConnectiveDesignation(c) => Some(GqlConnectiveDesignation::new(c.clone())),
            _ => None,
        }
    }

    /// As Term (if applicable)
    async fn as_term(&self) -> Option<GqlTerm> {
        match &self.entry {
            Entry::Term(t) => Some(GqlTerm::new(t.clone(), &self.graph)),
            _ => None,
        }
    }

    /// As Colour (if applicable)
    async fn as_colour(&self) -> Option<GqlColour> {
        match &self.entry {
            Entry::Colour(c) => Some(GqlColour::new(c.clone(), &self.graph)),
            _ => None,
        }
    }

    /// As Coordinate (if applicable)
    async fn as_coordinate(&self) -> Option<GqlCoordinate> {
        match &self.entry {
            Entry::Coordinate(c) => Some(GqlCoordinate::new(c.clone(), &self.graph)),
            _ => None,
        }
    }

    /// As Character (if applicable)
    async fn as_character(&self) -> Option<GqlCharacter> {
        match &self.entry {
            Entry::Character(c) => Some(GqlCharacter::new(c.clone())),
            _ => None,
        }
    }
}

// ============================================================================
// Link Type
// ============================================================================

/// A link between entries
pub struct GqlLink {
    link: Link,
    graph: Graph,
}

impl GqlLink {
    pub fn new(link: Link, graph: &Graph) -> Self {
        Self {
            link,
            graph: graph.clone(),
        }
    }
}

#[Object]
impl GqlLink {
    /// Link ID
    async fn id(&self) -> &str {
        &self.link.id
    }

    /// Base (source) entry ID
    async fn base_id(&self) -> Option<&str> {
        self.link.base_single()
    }

    /// Target entry ID
    async fn target_id(&self) -> Option<&str> {
        self.link.target_single()
    }

    /// Link type
    async fn link_type(&self) -> GqlLinkType {
        match &self.link.link_type {
            LinkType::Line => GqlLinkType::Line,
            LinkType::Connective => GqlLinkType::Connective,
        }
    }

    /// Character ID (for connective links)
    async fn character_id(&self) -> Option<&str> {
        self.link.character_id()
    }

    /// Optional tag
    async fn tag(&self) -> Option<&str> {
        self.link.tag.as_deref()
    }

    /// Base entry
    async fn base(&self) -> Option<GqlEntry> {
        self.link
            .base_single()
            .and_then(|id| self.graph.get_entry(id))
            .map(|e| GqlEntry::new(e.clone(), &self.graph))
    }

    /// Target entry
    async fn target(&self) -> Option<GqlEntry> {
        self.link
            .target_single()
            .and_then(|id| self.graph.get_entry(id))
            .map(|e| GqlEntry::new(e.clone(), &self.graph))
    }

    /// Character (for connective links)
    async fn character(&self) -> Option<GqlCharacter> {
        self.link
            .character_id()
            .and_then(|id| self.graph.get_character(id))
            .map(|c| GqlCharacter::new(c.clone()))
    }

    /// Order of this link (derived from base entry)
    async fn order(&self) -> Option<i32> {
        self.link
            .base_single()
            .and_then(|id| self.graph.get_entry(id))
            .and_then(|e| e.order())
            .map(|o| o as i32)
    }

    /// Base position (derived from base entry)
    async fn base_position(&self) -> Option<i32> {
        self.link
            .base_single()
            .and_then(|id| self.graph.get_entry(id))
            .and_then(|e| e.position())
            .map(|p| p as i32)
    }

    /// Target position (derived from target entry)
    async fn target_position(&self) -> Option<i32> {
        self.link
            .target_single()
            .and_then(|id| self.graph.get_entry(id))
            .and_then(|e| e.position())
            .map(|p| p as i32)
    }

    /// Base coordinate (for line links, returns the coordinate directly; for other links, looks up by position)
    async fn base_coordinate(&self) -> Option<GqlCoordinate> {
        let base_id = self.link.base_single()?;
        let base_entry = self.graph.get_entry(base_id)?;

        // If this is a line link, base IS the coordinate
        if let Entry::Coordinate(coord) = base_entry {
            return Some(GqlCoordinate::new(coord.clone(), &self.graph));
        }

        // Otherwise, look up coordinate by order and position
        let order = base_entry.order()?;
        let position = base_entry.position()?;
        self.graph
            .coordinate(order, position)
            .map(|c| GqlCoordinate::new(c.clone(), &self.graph))
    }

    /// Target coordinate (for line links, returns the coordinate directly; for other links, looks up by position)
    async fn target_coordinate(&self) -> Option<GqlCoordinate> {
        let target_id = self.link.target_single()?;
        let target_entry = self.graph.get_entry(target_id)?;

        // If this is a line link, target IS the coordinate
        if let Entry::Coordinate(coord) = target_entry {
            return Some(GqlCoordinate::new(coord.clone(), &self.graph));
        }

        // Otherwise, look up coordinate by order and position
        let order = target_entry.order()?;
        let position = target_entry.position()?;
        self.graph
            .coordinate(order, position)
            .map(|c| GqlCoordinate::new(c.clone(), &self.graph))
    }

    /// Base slice (term + coordinate + colour at base position)
    async fn base_slice(&self) -> Option<GqlSlice> {
        let base_id = self.link.base_single()?;
        let base_entry = self.graph.get_entry(base_id)?;
        let order = base_entry.order()?;
        let position = base_entry.position()?;
        Some(GqlSlice::new(order, position, self.graph.clone()))
    }

    /// Target slice (term + coordinate + colour at target position)
    async fn target_slice(&self) -> Option<GqlSlice> {
        let target_id = self.link.target_single()?;
        let target_entry = self.graph.get_entry(target_id)?;
        let order = target_entry.order()?;
        let position = target_entry.position()?;
        Some(GqlSlice::new(order, position, self.graph.clone()))
    }

    /// Get the corresponding line link (for connectives) or connective (for lines)
    async fn corresponding_links(&self) -> Vec<GqlLink> {
        let base_id = match self.link.base_single() {
            Some(id) => id,
            None => return vec![],
        };
        let target_id = match self.link.target_single() {
            Some(id) => id,
            None => return vec![],
        };

        let base_pos = self.graph.get_entry(base_id).and_then(|e| e.position());
        let target_pos = self.graph.get_entry(target_id).and_then(|e| e.position());
        let order = self.graph.get_entry(base_id).and_then(|e| e.order());

        match (order, base_pos, target_pos) {
            (Some(ord), Some(bp), Some(tp)) => {
                self.graph
                    .links
                    .iter()
                    .filter(|l| {
                        // Skip self
                        if l.id == self.link.id {
                            return false;
                        }
                        // Check if this link connects the same positions
                        let l_base_id = match l.base_single() {
                            Some(id) => id,
                            None => return false,
                        };
                        let l_target_id = match l.target_single() {
                            Some(id) => id,
                            None => return false,
                        };
                        let l_base = self.graph.get_entry(l_base_id);
                        let l_target = self.graph.get_entry(l_target_id);
                        match (l_base, l_target) {
                            (Some(lb), Some(lt)) => {
                                lb.order() == Some(ord)
                                    && lt.order() == Some(ord)
                                    && ((lb.position() == Some(bp) && lt.position() == Some(tp))
                                        || (lb.position() == Some(tp) && lt.position() == Some(bp)))
                            }
                            _ => false,
                        }
                    })
                    .map(|l| GqlLink::new(l.clone(), &self.graph))
                    .collect()
            }
            _ => vec![],
        }
    }
}

// ============================================================================
// Specific Entry Types
// ============================================================================

/// Character entry
pub struct GqlCharacter {
    character: Character,
}

impl GqlCharacter {
    pub fn new(character: Character) -> Self {
        Self { character }
    }
}

#[Object]
impl GqlCharacter {
    async fn id(&self) -> &str {
        &self.character.id
    }

    async fn language(&self) -> GqlLanguage {
        self.character.language.into()
    }

    async fn value(&self) -> &str {
        &self.character.value
    }
}

// ============================================================================
// Anchor Types
// ============================================================================

/// Order anchor type - the system level (1-12)
pub struct GqlOrder {
    order: Order,
    graph: Graph,
}

impl GqlOrder {
    pub fn new(order: Order, graph: Graph) -> Self {
        Self { order, graph }
    }
}

#[Object]
impl GqlOrder {
    async fn id(&self) -> &str {
        &self.order.id
    }

    async fn value(&self) -> i32 {
        self.order.value as i32
    }

    /// Standard name for this order (e.g., "Triad" for order 3)
    async fn standard_name(&self) -> Option<&str> {
        self.order.standard_name()
    }

    /// System name entry for this order
    async fn system_name(&self) -> Option<GqlSystemName> {
        self.graph
            .system_name(self.order.value)
            .map(|s| GqlSystemName::new(s.clone()))
    }

    /// Coherence attribute for this order
    async fn coherence(&self) -> Option<GqlCoherenceAttribute> {
        self.graph
            .coherence(self.order.value)
            .map(|c| GqlCoherenceAttribute::new(c.clone()))
    }

    /// Term designation for this order
    async fn term_designation(&self) -> Option<GqlTermDesignation> {
        self.graph
            .term_designation(self.order.value)
            .map(|t| GqlTermDesignation::new(t.clone()))
    }

    /// Connective designation for this order
    async fn connective_designation(&self) -> Option<GqlConnectiveDesignation> {
        self.graph
            .connective_designation(self.order.value)
            .map(|c| GqlConnectiveDesignation::new(c.clone()))
    }

    /// All locations in this order
    async fn locations(&self) -> Vec<GqlLocation> {
        self.graph
            .locations_for_order(self.order.value)
            .into_iter()
            .map(|l| GqlLocation::new(l.clone(), self.graph.clone()))
            .collect()
    }

    /// All terms in this order
    async fn terms(&self) -> Vec<GqlTerm> {
        self.graph
            .terms(self.order.value, None)
            .into_iter()
            .map(|t| GqlTerm::new(t.clone(), &self.graph))
            .collect()
    }

    /// All coordinates in this order
    async fn coordinates(&self) -> Vec<GqlCoordinate> {
        self.graph
            .coordinates(self.order.value)
            .into_iter()
            .map(|c| GqlCoordinate::new(c.clone(), &self.graph))
            .collect()
    }
}

/// Position anchor type - abstract "n-th place" (1-12)
pub struct GqlPosition {
    position: Position,
    graph: Graph,
}

impl GqlPosition {
    pub fn new(position: Position, graph: Graph) -> Self {
        Self { position, graph }
    }
}

#[Object]
impl GqlPosition {
    async fn id(&self) -> &str {
        &self.position.id
    }

    async fn value(&self) -> i32 {
        self.position.value as i32
    }

    /// All locations at this position (across all orders)
    async fn locations(&self) -> Vec<GqlLocation> {
        self.graph
            .locations_for_position(self.position.value)
            .into_iter()
            .map(|l| GqlLocation::new(l.clone(), self.graph.clone()))
            .collect()
    }
}

/// Location anchor type - the pullback of Order × Position
pub struct GqlLocation {
    location: Location,
    graph: Graph,
}

impl GqlLocation {
    pub fn new(location: Location, graph: Graph) -> Self {
        Self { location, graph }
    }
}

#[Object]
impl GqlLocation {
    async fn id(&self) -> &str {
        &self.location.id
    }

    /// Order reference ID
    async fn order_id(&self) -> &str {
        &self.location.order
    }

    /// Position reference ID
    async fn position_id(&self) -> &str {
        &self.location.position
    }

    /// Order value (extracted from reference)
    async fn order_value(&self) -> Option<i32> {
        self.location.order_value().map(|v| v as i32)
    }

    /// Position value (extracted from reference)
    async fn position_value(&self) -> Option<i32> {
        self.location.position_value().map(|v| v as i32)
    }

    /// The Order this location belongs to
    async fn order(&self) -> Option<GqlOrder> {
        self.location.order_value().and_then(|v| {
            self.graph
                .order(v)
                .map(|o| GqlOrder::new(o.clone(), self.graph.clone()))
        })
    }

    /// The abstract Position this location instantiates
    async fn position(&self) -> Option<GqlPosition> {
        self.location.position_value().and_then(|v| {
            self.graph
                .position(v)
                .map(|p| GqlPosition::new(p.clone(), self.graph.clone()))
        })
    }

    /// All terms at this location
    async fn terms(&self) -> Vec<GqlTerm> {
        self.graph
            .terms_at_location(&self.location.id)
            .into_iter()
            .map(|t| GqlTerm::new(t.clone(), &self.graph))
            .collect()
    }

    /// The coordinate at this location
    async fn coordinate(&self) -> Option<GqlCoordinate> {
        let order = self.location.order_value()?;
        let position = self.location.position_value()?;
        self.graph
            .coordinate(order, position)
            .map(|c| GqlCoordinate::new(c.clone(), &self.graph))
    }

    /// All colours at this location
    async fn colours(&self) -> Vec<GqlColour> {
        let (Some(order), Some(position)) =
            (self.location.order_value(), self.location.position_value())
        else {
            return vec![];
        };
        [Language::Hex, Language::Name]
            .iter()
            .filter_map(|lang| {
                self.graph
                    .colour(order, position, *lang)
                    .map(|c| GqlColour::new(c.clone(), &self.graph))
            })
            .collect()
    }

    /// Get colour by language
    async fn colour(&self, language: GqlLanguage) -> Option<GqlColour> {
        let order = self.location.order_value()?;
        let position = self.location.position_value()?;
        self.graph
            .colour(order, position, language.into())
            .map(|c| GqlColour::new(c.clone(), &self.graph))
    }
}

// ============================================================================
// Location-Level Entry Types
// ============================================================================

/// Term entry
pub struct GqlTerm {
    term: Term,
    graph: Graph,
}

impl GqlTerm {
    pub fn new(term: Term, graph: &Graph) -> Self {
        Self {
            term,
            graph: graph.clone(),
        }
    }
}

#[Object]
impl GqlTerm {
    async fn id(&self) -> &str {
        &self.term.id
    }

    /// Location reference ID
    async fn location_id(&self) -> &str {
        &self.term.location
    }

    /// Order value (derived from location reference)
    async fn order(&self) -> Option<i32> {
        self.term.order_value().map(|v| v as i32)
    }

    /// Position value (derived from location reference)
    async fn position(&self) -> Option<i32> {
        self.term.position_value().map(|v| v as i32)
    }

    async fn character_id(&self) -> &str {
        &self.term.character
    }

    /// The character this term references
    async fn character(&self) -> Option<GqlCharacter> {
        self.graph
            .get_character(&self.term.character)
            .map(|c| GqlCharacter::new(c.clone()))
    }

    /// The location this term belongs to
    async fn location(&self) -> Option<GqlLocation> {
        let order = self.term.order_value()?;
        let position = self.term.position_value()?;
        self.graph
            .location(order, position)
            .map(|l| GqlLocation::new(l.clone(), self.graph.clone()))
    }

    /// Connectives involving this term
    async fn connectives(&self) -> Vec<GqlLink> {
        self.graph
            .connectives_for_term(&self.term.id)
            .into_iter()
            .map(|l| GqlLink::new(l.clone(), &self.graph))
            .collect()
    }
}

/// Coordinate entry
pub struct GqlCoordinate {
    coordinate: Coordinate,
    graph: Graph,
}

impl GqlCoordinate {
    pub fn new(coordinate: Coordinate, graph: &Graph) -> Self {
        Self {
            coordinate,
            graph: graph.clone(),
        }
    }
}

#[Object]
impl GqlCoordinate {
    async fn id(&self) -> &str {
        &self.coordinate.id
    }

    /// Location reference ID
    async fn location_id(&self) -> &str {
        &self.coordinate.location
    }

    /// Order value (derived from location reference)
    async fn order(&self) -> Option<i32> {
        self.coordinate.order_value().map(|v| v as i32)
    }

    /// Position value (derived from location reference)
    async fn position(&self) -> Option<i32> {
        self.coordinate.position_value().map(|v| v as i32)
    }

    async fn x(&self) -> f64 {
        self.coordinate.value.x
    }

    async fn y(&self) -> f64 {
        self.coordinate.value.y
    }

    async fn z(&self) -> f64 {
        self.coordinate.value.z
    }

    /// The location this coordinate belongs to
    async fn location(&self) -> Option<GqlLocation> {
        let order = self.coordinate.order_value()?;
        let position = self.coordinate.position_value()?;
        self.graph
            .location(order, position)
            .map(|l| GqlLocation::new(l.clone(), self.graph.clone()))
    }
}

/// Colour entry
pub struct GqlColour {
    colour: Colour,
    graph: Graph,
}

impl GqlColour {
    pub fn new(colour: Colour, graph: &Graph) -> Self {
        Self {
            colour,
            graph: graph.clone(),
        }
    }
}

#[Object]
impl GqlColour {
    async fn id(&self) -> &str {
        &self.colour.id
    }

    /// Location reference ID
    async fn location_id(&self) -> &str {
        &self.colour.location
    }

    /// Order value (derived from location reference)
    async fn order(&self) -> Option<i32> {
        self.colour.order_value().map(|v| v as i32)
    }

    /// Position value (derived from location reference)
    async fn position(&self) -> Option<i32> {
        self.colour.position_value().map(|v| v as i32)
    }

    async fn language(&self) -> GqlLanguage {
        self.colour.language.into()
    }

    async fn value(&self) -> &str {
        &self.colour.value
    }

    /// The location this colour belongs to
    async fn location(&self) -> Option<GqlLocation> {
        let order = self.colour.order_value()?;
        let position = self.colour.position_value()?;
        self.graph
            .location(order, position)
            .map(|l| GqlLocation::new(l.clone(), self.graph.clone()))
    }
}

// ============================================================================
// Order-Level Entry Types
// ============================================================================

/// SystemName entry
pub struct GqlSystemName {
    system_name: SystemName,
}

impl GqlSystemName {
    pub fn new(system_name: SystemName) -> Self {
        Self { system_name }
    }
}

#[Object]
impl GqlSystemName {
    async fn id(&self) -> &str {
        &self.system_name.id
    }

    /// Order reference ID
    async fn order_id(&self) -> &str {
        &self.system_name.order
    }

    /// Order value (derived from order reference)
    async fn order(&self) -> Option<i32> {
        self.system_name.order_value().map(|v| v as i32)
    }

    async fn value(&self) -> &str {
        &self.system_name.value
    }
}

/// CoherenceAttribute entry
pub struct GqlCoherenceAttribute {
    coherence: CoherenceAttribute,
}

impl GqlCoherenceAttribute {
    pub fn new(coherence: CoherenceAttribute) -> Self {
        Self { coherence }
    }
}

#[Object]
impl GqlCoherenceAttribute {
    async fn id(&self) -> &str {
        &self.coherence.id
    }

    /// Order reference ID
    async fn order_id(&self) -> &str {
        &self.coherence.order
    }

    /// Order value (derived from order reference)
    async fn order(&self) -> Option<i32> {
        self.coherence.order_value().map(|v| v as i32)
    }

    async fn value(&self) -> &str {
        &self.coherence.value
    }
}

/// TermDesignation entry
pub struct GqlTermDesignation {
    term_designation: TermDesignation,
}

impl GqlTermDesignation {
    pub fn new(term_designation: TermDesignation) -> Self {
        Self { term_designation }
    }
}

#[Object]
impl GqlTermDesignation {
    async fn id(&self) -> &str {
        &self.term_designation.id
    }

    /// Order reference ID
    async fn order_id(&self) -> &str {
        &self.term_designation.order
    }

    /// Order value (derived from order reference)
    async fn order(&self) -> Option<i32> {
        self.term_designation.order_value().map(|v| v as i32)
    }

    async fn value(&self) -> &str {
        &self.term_designation.value
    }
}

/// ConnectiveDesignation entry
pub struct GqlConnectiveDesignation {
    connective_designation: ConnectiveDesignation,
}

impl GqlConnectiveDesignation {
    pub fn new(connective_designation: ConnectiveDesignation) -> Self {
        Self {
            connective_designation,
        }
    }
}

#[Object]
impl GqlConnectiveDesignation {
    async fn id(&self) -> &str {
        &self.connective_designation.id
    }

    /// Order reference ID
    async fn order_id(&self) -> &str {
        &self.connective_designation.order
    }

    /// Order value (derived from order reference)
    async fn order(&self) -> Option<i32> {
        self.connective_designation.order_value().map(|v| v as i32)
    }

    async fn value(&self) -> &str {
        &self.connective_designation.value
    }
}

// ============================================================================
// System View
// ============================================================================

/// A view of a system at a given order
pub struct GqlSystemView {
    order: u8,
    graph: Graph,
}

impl GqlSystemView {
    pub fn new(order: u8, graph: Graph) -> Self {
        Self { order, graph }
    }
}

#[Object]
impl GqlSystemView {
    async fn order(&self) -> i32 {
        self.order as i32
    }

    async fn name(&self) -> Option<String> {
        self.graph.system_name(self.order).map(|s| s.value.clone())
    }

    async fn coherence(&self) -> Option<String> {
        self.graph.coherence(self.order).map(|c| c.value.clone())
    }

    async fn term_designation(&self) -> Option<String> {
        self.graph
            .term_designation(self.order)
            .map(|t| t.value.clone())
    }

    async fn connective_designation(&self) -> Option<String> {
        self.graph
            .connective_designation(self.order)
            .map(|c| c.value.clone())
    }

    async fn terms(&self) -> Vec<GqlTerm> {
        self.graph
            .terms(self.order, None)
            .into_iter()
            .map(|t| GqlTerm::new(t.clone(), &self.graph))
            .collect()
    }

    async fn coordinates(&self) -> Vec<GqlCoordinate> {
        self.graph
            .coordinates(self.order)
            .into_iter()
            .map(|c| GqlCoordinate::new(c.clone(), &self.graph))
            .collect()
    }

    async fn colours(&self) -> Vec<GqlColour> {
        self.graph
            .colours(self.order)
            .into_iter()
            .map(|c| GqlColour::new(c.clone(), &self.graph))
            .collect()
    }

    async fn connectives(&self) -> Vec<GqlLink> {
        self.graph
            .connectives(self.order, None, None)
            .into_iter()
            .map(|l| GqlLink::new(l.clone(), &self.graph))
            .collect()
    }

    async fn lines(&self) -> Vec<GqlLink> {
        self.graph
            .lines(self.order)
            .into_iter()
            .map(|l| GqlLink::new(l.clone(), &self.graph))
            .collect()
    }

    /// All links (both connectives and lines) for this system
    async fn links(&self) -> Vec<GqlLink> {
        let mut all_links: Vec<GqlLink> = self
            .graph
            .connectives(self.order, None, None)
            .into_iter()
            .map(|l| GqlLink::new(l.clone(), &self.graph))
            .collect();
        all_links.extend(
            self.graph
                .lines(self.order)
                .into_iter()
                .map(|l| GqlLink::new(l.clone(), &self.graph)),
        );
        all_links
    }

    /// Get slice at a specific position
    async fn slice(&self, position: i32) -> GqlSlice {
        GqlSlice::new(self.order, position as u8, self.graph.clone())
    }

    /// All slices for this system
    async fn slices(&self) -> Vec<GqlSlice> {
        (1..=self.order)
            .map(|pos| GqlSlice::new(self.order, pos, self.graph.clone()))
            .collect()
    }
}

// ============================================================================
// Slice View
// ============================================================================

/// A slice - all entries at a specific order+position
pub struct GqlSlice {
    order: u8,
    position: u8,
    graph: Graph,
}

impl GqlSlice {
    pub fn new(order: u8, position: u8, graph: Graph) -> Self {
        Self {
            order,
            position,
            graph,
        }
    }
}

#[Object]
impl GqlSlice {
    async fn order(&self) -> i32 {
        self.order as i32
    }

    async fn position(&self) -> i32 {
        self.position as i32
    }

    async fn entries(&self) -> Vec<GqlEntry> {
        self.graph
            .slice(self.order, self.position)
            .into_iter()
            .map(|e| GqlEntry::new(e.clone(), &self.graph))
            .collect()
    }

    async fn term(&self) -> Option<GqlTerm> {
        self.graph
            .term(self.order, self.position)
            .map(|t| GqlTerm::new(t.clone(), &self.graph))
    }

    async fn coordinate(&self) -> Option<GqlCoordinate> {
        self.graph
            .coordinate(self.order, self.position)
            .map(|c| GqlCoordinate::new(c.clone(), &self.graph))
    }

    async fn colour(&self, language: Option<GqlLanguage>) -> Option<GqlColour> {
        let lang = language.map(|l| l.into()).unwrap_or(Language::Hex);
        self.graph
            .colour(self.order, self.position, lang)
            .map(|c| GqlColour::new(c.clone(), &self.graph))
    }

    /// All isomorphic terms at this position (across languages)
    async fn isomorphic_terms(&self) -> Vec<GqlTerm> {
        self.graph
            .isomorphic_terms(self.order, self.position)
            .into_iter()
            .map(|(t, _)| GqlTerm::new(t.clone(), &self.graph))
            .collect()
    }
}

// ============================================================================
// Functor Types
// ============================================================================

/// A single (base → target) mapping within a Functor.
#[derive(SimpleObject, Clone)]
pub struct GqlFunctorMapping {
    pub base: String,
    pub target: String,
}

impl From<&FunctorMapping> for GqlFunctorMapping {
    fn from(m: &FunctorMapping) -> Self {
        Self {
            base: m.base.clone(),
            target: m.target.clone(),
        }
    }
}

/// A stored Functor.
pub struct GqlFunctor {
    functor: Functor,
}

impl GqlFunctor {
    pub fn new(functor: Functor) -> Self {
        Self { functor }
    }
}

#[Object]
impl GqlFunctor {
    async fn id(&self) -> &str {
        &self.functor.id
    }

    async fn name(&self) -> &str {
        &self.functor.name
    }

    async fn description(&self) -> Option<&str> {
        self.functor.description.as_deref()
    }

    async fn source_language(&self) -> Option<GqlLanguage> {
        self.functor.source_language.map(Into::into)
    }

    async fn mappings(&self) -> Vec<GqlFunctorMapping> {
        self.functor.mappings.iter().map(Into::into).collect()
    }

    /// The Terms this Functor would produce if applied (a preview — does not
    /// mutate the graph). Use `applyFunctor` mutation to persist.
    async fn preview(&self, ctx: &Context<'_>) -> Vec<GqlTerm> {
        let snapshot = graph_snapshot(ctx).await;
        self.functor
            .apply()
            .into_iter()
            .map(|t| GqlTerm::new(t, &snapshot))
            .collect()
    }
}

/// Input for a single mapping in a Functor CRUD payload.
#[derive(InputObject, Clone)]
pub struct FunctorMappingInput {
    pub base: String,
    pub target: String,
}

impl From<FunctorMappingInput> for FunctorMapping {
    fn from(m: FunctorMappingInput) -> Self {
        FunctorMapping::new(m.base, m.target)
    }
}

/// Input for creating or updating a Functor.
#[derive(InputObject, Clone)]
pub struct FunctorInput {
    /// Optional ID. If omitted, one is derived from `name`.
    pub id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub source_language: Option<GqlLanguage>,
    pub mappings: Vec<FunctorMappingInput>,
}

impl FunctorInput {
    fn into_functor(self) -> Functor {
        let id = self.id.unwrap_or_else(|| {
            let slug = self
                .name
                .to_lowercase()
                .chars()
                .map(|c| if c.is_alphanumeric() { c } else { '_' })
                .collect::<String>();
            format!("functor_{}", slug)
        });
        let source_language = self.source_language.map(Into::into);
        let mappings = self.mappings.into_iter().map(Into::into).collect();
        let mut functor = Functor::new(id, self.name, source_language, mappings);
        if let Some(desc) = self.description {
            functor = functor.with_description(desc);
        }
        functor
    }
}

// ============================================================================
// Schema
// ============================================================================

pub type SystematicsSchema =
    async_graphql::Schema<QueryRoot, MutationRoot, async_graphql::EmptySubscription>;

/// Build the schema with a shared graph as context data.
pub fn create_schema(graph: SharedGraph) -> SystematicsSchema {
    async_graphql::Schema::build(
        QueryRoot,
        MutationRoot,
        async_graphql::EmptySubscription,
    )
    .data(graph)
    .finish()
}
