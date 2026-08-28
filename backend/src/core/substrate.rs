//! The substrate — Holochain-style **elements + links** (placeholders until a real
//! Holochain backend). Each element and each link is a **function-monad**: a
//! *container with a boundary + a set of actions* (functional composition). **NB:
//! this is the FP monad, NOT our systematics `Monad` (the order-1 system) — do not
//! conflate them.**
//!
//! **Generate, don't read (user, 2026-08-20).** Systems are *composed then stored*,
//! so the substrate is built by **generating** structure, not by materialising an
//! already-composed system. Step 1 (here): from the **graph rules** — cardinality + size
//! (the `Template`) and its adjacency/incidence — **generate a topology**: the K_n
//! shape as bare elements (vertices) + undirected links (edges). We work
//! **UNDIRECTED**: a triad's six directed links simplify to **3 bidirectional
//! 'orbits'**, so a `Link` holds an *unordered* endpoint pair.
//!
//! Later: **store** the topology in the DHT, then **anchor** semantics onto it —
//! term/connective characters, geometry, colour — each as a *separate* mapping
//! (a functor of morphisms over the semantic pentad; the anchoring links are the
//! orthogonal `vertex-element ──▶ term-character-element`). See `docs/design-intent.md`
//! → *CORRECTED — materialisation is a Functor…*. (Retires the AD4M 'perspective'
//! language.)
//!
//! **The morphism grammar (2026-08-26).** Composition is **gated by the grammar**: every
//! `Morphism` declares its topological `site` (`Vertex` | `Edge`), and `compose_checked`
//! admits it only where the `Template`'s matrices allow — the **adjacency matrix** gates
//! edges and **order** gates vertices in this build path. (The **incidence** and **line
//! graph** rules — `admits_anchor` / `admits_composition` in `grammar.rs` — are the
//! reconciler and semantic-composition primitives for the anchor & tensor-product work to
//! come; they exist and are tested, but no morphism `site` triggers them yet.) So the
//! adjacency matrix is **load-bearing**, not decorative: an ungrammatical edge/vertex
//! morphism is rejected, not applied.

use serde::{Deserialize, Serialize};

use super::grammar::Template;

/// A vertex of the topology — a Holochain-style **element** (a function-monad:
/// container + actions). Bare: it holds only its topological anchor `(cardinality,
/// index)`. Data (term character, geometry, colour) is **linked on** later, not
/// embedded.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Element {
    pub id: String,
    pub cardinality: u8,
    /// The topological anchor: index `1..=cardinality`.
    pub index: u8,
}

/// An edge of the topology — a Holochain-style **link** (also a function-monad).
/// One **undirected 'orbit'** (the two opposite directed links treated as one):
/// an unordered vertex-index pair `{a, b}` with `a < b`. The connective character
/// is **linked on** later, not embedded.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Link {
    pub id: String,
    pub endpoints: (u8, u8),
}

/// A generated topology (a K_n) as a hypergraph of bare elements + undirected links.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TopologyGraph {
    pub cardinality: u8,
    pub elements: Vec<Element>,
    pub links: Vec<Link>,
}

impl TopologyGraph {
    /// **Validate against the graph rules** (the `Template`): cardinality (vertex count),
    /// size (orbit count), and shape (every orbit is a legal, undirected template
    /// edge). This is the Controller's guarantee — the Graph Template holds the
    /// validation rules, so a correct topology is *enforced*, not hoped for.
    pub fn validate_against(&self, template: &Template) -> Result<(), Vec<String>> {
        let mut errs = Vec::new();
        let n = template.order();
        if self.elements.len() != n as usize {
            errs.push(format!(
                "cardinality: {} vertices, expected {n}",
                self.elements.len()
            ));
        }
        if self.links.len() != template.size() {
            errs.push(format!(
                "size: {} orbits, expected {}",
                self.links.len(),
                template.size()
            ));
        }
        let legal: std::collections::HashSet<(u8, u8)> =
            template.edges().into_iter().collect();
        for l in &self.links {
            if l.endpoints.0 >= l.endpoints.1 {
                errs.push(format!("orbit {} is not undirected (a < b)", l.id));
            } else if !legal.contains(&l.endpoints) {
                errs.push(format!("orbit {:?} is not a legal template edge", l.endpoints));
            }
        }
        if errs.is_empty() {
            Ok(())
        } else {
            Err(errs)
        }
    }
}

/// A **data element** — a Holochain entry holding a character (a **term** or
/// **connective**). It is *linked onto* the topology, **not embedded** (the
/// fully-linked form: `vertex-element ──▶ term-character-element`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataElement {
    pub id: String,
    pub kind: String,      // "term" | "connective"
    pub character: String, // the character value / ref
}

/// A **hyperlink** — the general Holochain link `base ──type──▶ target` between two
/// element ids. Used for the **orthogonal anchors** (topology element ▶ data element)
/// and the **lateral** system links (`term ──connective── term`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HyperLink {
    pub id: String,
    pub base: String,
    pub target: String,
    pub link_type: String, // "anchor" | a connective element id (lateral)
}

/// The **hypergraph** the Controller composes: the bare `topology` (the *lateral
/// topology* domain, vertex–edge–vertex), plus the anchored **semantics** — `data`
/// (term/connective character elements = the *lateral system* domain) and `links`
/// (the **orthogonal anchors** + the **lateral** `term–connective–term`). The
/// semantics stay **decoupled**: drop the anchors and a standalone semantic triad
/// remains; re-adding them is a **dyadic remap** (term→vertex, connective→edge).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Hypergraph {
    pub topology: TopologyGraph,
    pub data: Vec<DataElement>,
    pub links: Vec<HyperLink>,
}

// ---- The Controller's morphisms that compose a system ----

/// The **kind** of a morphism — the taxonomy. Topology + semantic anchoring use
/// `Monomorphism`; a decoupled-triad remap is a **`Bimorphism`** (two orthogonal
/// maps); `Homomorphism` is a placeholder for the composition to come.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MorphismKind {
    Monomorphism,
    Bimorphism,
    Homomorphism,
}

/// The **topological site** a morphism acts on — the coordinate the *grammar* checks
/// before the morphism may be applied. A vertex-index morphism declares `Vertex`; an
/// edge morphism declares `Edge`. The `Template` admits or rejects the morphism by
/// reading its matrices at this site (adjacency for edges, order for vertices), so
/// legality is *read off the topology*, not hard-coded into each morphism.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MorphismSite {
    Vertex(u8),
    Edge(u8, u8),
}

/// A **morphism** — a container with a *single function point* (`apply`). Concrete morphisms (structs) implement this and declare their `kind`.
/// A system is composed by **bundling** morphisms and applying them to build up the
/// hypergraph — "the model is a bundled set of monomorphisms".
pub trait Morphism {
    /// Which kind of morphism this is (the enum taxonomy).
    fn kind(&self) -> MorphismKind;
    /// The topological **site** this morphism acts on — what the grammar gates it
    /// by (a vertex index, or an edge pair).
    fn site(&self) -> MorphismSite;
    /// The single function point: apply this morphism to the hypergraph under
    /// construction (its one action).
    fn apply(&self, hg: &mut Hypergraph);
}

/// `index ↦ vertex-element` — a monomorphism (one vertex per index).
pub struct IndexToVertex {
    pub cardinality: u8,
    pub index: u8,
}
impl Morphism for IndexToVertex {
    fn kind(&self) -> MorphismKind {
        MorphismKind::Monomorphism
    }
    fn site(&self) -> MorphismSite {
        MorphismSite::Vertex(self.index)
    }
    fn apply(&self, hg: &mut Hypergraph) {
        hg.topology.elements.push(Element {
            id: format!("el_{}_{}", self.cardinality, self.index),
            cardinality: self.cardinality,
            index: self.index,
        });
    }
}

/// `edge ↦ orbit-link` — a monomorphism (one undirected orbit per edge).
pub struct EdgeToOrbit {
    pub cardinality: u8,
    pub a: u8,
    pub b: u8,
}
impl Morphism for EdgeToOrbit {
    fn kind(&self) -> MorphismKind {
        MorphismKind::Monomorphism
    }
    fn site(&self) -> MorphismSite {
        MorphismSite::Edge(self.a, self.b)
    }
    fn apply(&self, hg: &mut Hypergraph) {
        let (a, b) = if self.a < self.b { (self.a, self.b) } else { (self.b, self.a) };
        hg.topology.links.push(Link {
            id: format!("lk_{}_{}_{}", self.cardinality, a, b),
            endpoints: (a, b),
        });
    }
}

/// `term character ↦ vertex` — a monomorphism: adds the term as a data element and an
/// **orthogonal anchor** from its canonical vertex to it (the index must not be
/// missed — the topology is the anchor).
pub struct TermToVertex {
    pub cardinality: u8,
    pub index: u8,
    pub character: String,
}
impl Morphism for TermToVertex {
    fn kind(&self) -> MorphismKind {
        MorphismKind::Monomorphism
    }
    fn site(&self) -> MorphismSite {
        MorphismSite::Vertex(self.index)
    }
    fn apply(&self, hg: &mut Hypergraph) {
        let term_id = format!("term_{}_{}", self.cardinality, self.index);
        hg.data.push(DataElement {
            id: term_id.clone(),
            kind: "term".to_string(),
            character: self.character.clone(),
        });
        hg.links.push(HyperLink {
            id: format!("anchor_v_{}_{}", self.cardinality, self.index),
            base: format!("el_{}_{}", self.cardinality, self.index),
            target: term_id,
            link_type: "anchor".to_string(),
        });
    }
}

/// `connective character ↦ orbit` — a monomorphism: adds the connective as a data
/// element, an **orthogonal anchor** from its orbit to it, and the **lateral**
/// `term ──connective── term` link between the orbit's two endpoint terms.
pub struct ConnectiveToOrbit {
    pub cardinality: u8,
    pub a: u8,
    pub b: u8,
    pub character: String,
}
impl Morphism for ConnectiveToOrbit {
    fn kind(&self) -> MorphismKind {
        MorphismKind::Monomorphism
    }
    fn site(&self) -> MorphismSite {
        MorphismSite::Edge(self.a, self.b)
    }
    fn apply(&self, hg: &mut Hypergraph) {
        let (a, b) = if self.a < self.b { (self.a, self.b) } else { (self.b, self.a) };
        let conn_id = format!("conn_{}_{}_{}", self.cardinality, a, b);
        hg.data.push(DataElement {
            id: conn_id.clone(),
            kind: "connective".to_string(),
            character: self.character.clone(),
        });
        // orthogonal anchor: orbit ▶ connective character.
        hg.links.push(HyperLink {
            id: format!("anchor_e_{}_{}_{}", self.cardinality, a, b),
            base: format!("lk_{}_{}_{}", self.cardinality, a, b),
            target: conn_id.clone(),
            link_type: "anchor".to_string(),
        });
        // lateral system link: term_a ──connective── term_b (the link type is the
        // connective element, so the triad stands alone even without the topology).
        hg.links.push(HyperLink {
            id: format!("lat_{}_{}_{}", self.cardinality, a, b),
            base: format!("term_{}_{}", self.cardinality, a),
            target: format!("term_{}_{}", self.cardinality, b),
            link_type: conn_id,
        });
    }
}

/// `coordinate ↦ vertex` — a monomorphism: the geometry mapping (orthogonal anchor
/// vertex ▶ coordinate). One of the *separate* mappings (parallel to terms).
pub struct CoordinateToVertex {
    pub cardinality: u8,
    pub index: u8,
    pub coordinate: [f64; 3],
}
impl Morphism for CoordinateToVertex {
    fn kind(&self) -> MorphismKind {
        MorphismKind::Monomorphism
    }
    fn site(&self) -> MorphismSite {
        MorphismSite::Vertex(self.index)
    }
    fn apply(&self, hg: &mut Hypergraph) {
        let coord_id = format!("coord_{}_{}", self.cardinality, self.index);
        hg.data.push(DataElement {
            id: coord_id.clone(),
            kind: "coordinate".to_string(),
            character: format!("{},{},{}", self.coordinate[0], self.coordinate[1], self.coordinate[2]),
        });
        hg.links.push(HyperLink {
            id: format!("anchor_g_{}_{}", self.cardinality, self.index),
            base: format!("el_{}_{}", self.cardinality, self.index),
            target: coord_id,
            link_type: "anchor".to_string(),
        });
    }
}

/// `colour ↦ vertex` — a monomorphism: the colour mapping (orthogonal anchor
/// vertex ▶ colour).
pub struct ColourToVertex {
    pub cardinality: u8,
    pub index: u8,
    pub colour: String,
}
impl Morphism for ColourToVertex {
    fn kind(&self) -> MorphismKind {
        MorphismKind::Monomorphism
    }
    fn site(&self) -> MorphismSite {
        MorphismSite::Vertex(self.index)
    }
    fn apply(&self, hg: &mut Hypergraph) {
        let colour_id = format!("colour_{}_{}", self.cardinality, self.index);
        hg.data.push(DataElement {
            id: colour_id.clone(),
            kind: "colour".to_string(),
            character: self.colour.clone(),
        });
        hg.links.push(HyperLink {
            id: format!("anchor_c_{}_{}", self.cardinality, self.index),
            base: format!("el_{}_{}", self.cardinality, self.index),
            target: colour_id,
            link_type: "anchor".to_string(),
        });
    }
}

/// `coordinate ↦ coordinate` — the geometry **lateral**: a **line** between the two
/// endpoint coordinates of an edge (the transitivity partner of coordinate→vertex).
pub struct CoordinateLine {
    pub cardinality: u8,
    pub a: u8,
    pub b: u8,
}
impl Morphism for CoordinateLine {
    fn kind(&self) -> MorphismKind {
        MorphismKind::Monomorphism
    }
    fn site(&self) -> MorphismSite {
        MorphismSite::Edge(self.a, self.b)
    }
    fn apply(&self, hg: &mut Hypergraph) {
        let (a, b) = if self.a < self.b { (self.a, self.b) } else { (self.b, self.a) };
        let line_id = format!("line_{}_{}_{}", self.cardinality, a, b);
        // lateral: coordinate → coordinate.
        hg.links.push(HyperLink {
            id: line_id.clone(),
            base: format!("coord_{}_{}", self.cardinality, a),
            target: format!("coord_{}_{}", self.cardinality, b),
            link_type: "line".to_string(),
        });
        // orthogonal: the line anchors to its topological orbit (edge ↔ line).
        hg.links.push(HyperLink {
            id: format!("anchor_line_{}_{}_{}", self.cardinality, a, b),
            base: format!("lk_{}_{}_{}", self.cardinality, a, b),
            target: line_id,
            link_type: "anchor".to_string(),
        });
    }
}

/// `colour ↦ colour` — the colour **lateral** (the transitivity partner of
/// colour→vertex), relating the two endpoint colours of an edge.
pub struct ColourLine {
    pub cardinality: u8,
    pub a: u8,
    pub b: u8,
}
impl Morphism for ColourLine {
    fn kind(&self) -> MorphismKind {
        MorphismKind::Monomorphism
    }
    fn site(&self) -> MorphismSite {
        MorphismSite::Edge(self.a, self.b)
    }
    fn apply(&self, hg: &mut Hypergraph) {
        let (a, b) = if self.a < self.b { (self.a, self.b) } else { (self.b, self.a) };
        let colline_id = format!("colline_{}_{}_{}", self.cardinality, a, b);
        // lateral: colour → colour.
        hg.links.push(HyperLink {
            id: colline_id.clone(),
            base: format!("colour_{}_{}", self.cardinality, a),
            target: format!("colour_{}_{}", self.cardinality, b),
            link_type: "colour".to_string(),
        });
        // orthogonal: the colour-line anchors to its topological orbit (edge ↔ colour-line).
        hg.links.push(HyperLink {
            id: format!("anchor_colline_{}_{}_{}", self.cardinality, a, b),
            base: format!("lk_{}_{}_{}", self.cardinality, a, b),
            target: colline_id,
            link_type: "anchor".to_string(),
        });
    }
}

// ---- The grammar gate: the Template's matrices decide which morphisms are legal ----

/// Does the **grammar** (the `Template`'s matrices) admit a morphism at this site?
/// A vertex site is checked for range; an **edge** site is checked against the
/// **adjacency matrix** (topology) — so an edge morphism on a non-edge (self-loop,
/// out-of-range pair) is ungrammatical. This is the single dispatch point that turns
/// the matrices into the legality rule for composition.
pub fn admits(template: &Template, site: MorphismSite) -> bool {
    match site {
        MorphismSite::Vertex(i) => template.admits_vertex(i),
        MorphismSite::Edge(a, b) => template.admits_edge(a, b),
    }
}

/// **Grammar-checked composition** — the Controller applies a morphism bundle only
/// where the grammar admits it. Each morphism's `site` is checked against the
/// Template's matrices *before* it is applied; an ungrammatical morphism is
/// **rejected**, not applied, and its site reported. This is what makes the matrices
/// *load-bearing*: composition is **gated by the grammar**, not merely validated
/// after the fact. The canonical bundles are grammatical by construction, so the
/// build path (below) routes through here and always succeeds; a hand-built or
/// composed bundle that strays off the topology is caught.
pub fn compose_checked(
    cardinality: u8,
    bundle: Vec<Box<dyn Morphism>>,
) -> Result<Hypergraph, Vec<String>> {
    let template = Template::for_order(cardinality);
    let mut hg = Hypergraph::default();
    hg.topology.cardinality = cardinality;
    let mut errs = Vec::new();
    for m in &bundle {
        let site = m.site();
        if admits(&template, site) {
            m.apply(&mut hg);
        } else {
            errs.push(format!(
                "ungrammatical morphism at {site:?}: not a legal site for K_{cardinality} (rejected by the grammar)"
            ));
        }
    }
    if errs.is_empty() {
        Ok(hg)
    } else {
        Err(errs)
    }
}

/// The **bundle** of monomorphisms that composes a K_n topology from its graph rules
/// (the `Template`): a `Position→Vertex` per vertex, an `Edge→Orbit` per edge.
pub fn topology_morphisms(cardinality: u8) -> Vec<Box<dyn Morphism>> {
    let mut bundle: Vec<Box<dyn Morphism>> = Vec::new();
    for index in 1..=cardinality {
        bundle.push(Box::new(IndexToVertex { cardinality, index }));
    }
    for (a, b) in Template::for_order(cardinality).edges() {
        bundle.push(Box::new(EdgeToOrbit { cardinality, a, b }));
    }
    bundle
}

/// **Generate the topology** for an cardinality by **composing the monomorphism bundle**
/// (applying each morphism to build up the graph), then **validating** the result
/// against the graph rules (the `Template`). Correct topology is guaranteed by the
/// constraints. No semantics — those are anchored on by `compose_system`.
pub fn generate_topology(cardinality: u8) -> TopologyGraph {
    let hg = compose_checked(cardinality, topology_morphisms(cardinality))
        .expect("canonical topology bundle is grammatical");
    debug_assert!(
        hg.topology
            .validate_against(&Template::for_order(cardinality))
            .is_ok(),
        "generated topology must satisfy the template"
    );
    hg.topology
}

/// **Compose a full system hypergraph** for an cardinality: the topology bundle, then the
/// semantic bundle — a `TermToVertex` per index, a `ConnectiveToOrbit` per edge.
/// `terms` are in index cardinality (`1..=cardinality`); `connectives` in canonical edge cardinality
/// (`(1,2),(1,3),…`). Topology is composed first so the semantic anchors reference
/// existing vertex/orbit elements.
/// The **vocabulary morphisms** — the semantic bundle: a `TermToVertex` per index +
/// a `ConnectiveToOrbit` per edge. `terms` in index order (`1..=cardinality`);
/// `connectives` in canonical edge order.
pub fn vocabulary_morphisms(
    cardinality: u8,
    terms: &[String],
    connectives: &[String],
) -> Vec<Box<dyn Morphism>> {
    let mut bundle: Vec<Box<dyn Morphism>> = Vec::new();
    for index in 1..=cardinality {
        if let Some(character) = terms.get((index - 1) as usize) {
            bundle.push(Box::new(TermToVertex {
                cardinality,
                index,
                character: character.clone(),
            }));
        }
    }
    for ((a, b), character) in Template::for_order(cardinality)
        .edges()
        .into_iter()
        .zip(connectives.iter())
    {
        bundle.push(Box::new(ConnectiveToOrbit {
            cardinality,
            a,
            b,
            character: character.clone(),
        }));
    }
    bundle
}

pub fn compose_system(cardinality: u8, terms: &[String], connectives: &[String]) -> Hypergraph {
    let mut bundle = topology_morphisms(cardinality);
    bundle.extend(vocabulary_morphisms(cardinality, terms, connectives));
    compose_checked(cardinality, bundle).expect("canonical system bundle is grammatical")
}

/// **Compose the render** for an cardinality — topology + **geometry + colour**, on the
/// system's *own terms* (no metadata). `coordinates`/`colours` are per-index
/// (`1..=cardinality`). Adds each vertex's coordinate + colour (orthogonal `→vertex`), then
/// the **lateral** lines (`coordinate→coordinate`) and colour-lines (`colour→colour`)
/// per edge — the transitivity partners.
/// The **geometry morphisms** — the render bundle (view): a `CoordinateToVertex` +
/// `ColourToVertex` per index, and a `CoordinateLine` + `ColourLine` per edge.
pub fn geometry_morphisms(
    cardinality: u8,
    coordinates: &[[f64; 3]],
    colours: &[String],
) -> Vec<Box<dyn Morphism>> {
    let mut bundle: Vec<Box<dyn Morphism>> = Vec::new();
    for index in 1..=cardinality {
        let idx = (index - 1) as usize;
        if let Some(coordinate) = coordinates.get(idx) {
            bundle.push(Box::new(CoordinateToVertex {
                cardinality,
                index,
                coordinate: *coordinate,
            }));
        }
        if let Some(colour) = colours.get(idx) {
            bundle.push(Box::new(ColourToVertex {
                cardinality,
                index,
                colour: colour.clone(),
            }));
        }
    }
    for (a, b) in Template::for_order(cardinality).edges() {
        bundle.push(Box::new(CoordinateLine { cardinality, a, b }));
        bundle.push(Box::new(ColourLine { cardinality, a, b }));
    }
    bundle
}

pub fn compose_render(cardinality: u8, coordinates: &[[f64; 3]], colours: &[String]) -> Hypergraph {
    let mut bundle = topology_morphisms(cardinality);
    bundle.extend(geometry_morphisms(cardinality, coordinates, colours));
    compose_checked(cardinality, bundle).expect("canonical render bundle is grammatical")
}

/// **Compose the singular Model** — the *unified* builder: topology + vocabulary +
/// geometry + colour anchored on **one** hypergraph. This expresses the base-space
/// **tetrad** as a single model — Template (the grammar gate) · Topology (generated) ·
/// Vocabulary (terms/connectives) · Geometry (coordinates/colours) — rather than the
/// two partial builders (`compose_system` = topology+vocabulary, `compose_render` =
/// topology+geometry). It is the **convergence target**: the one `compose` the serving
/// path (`resolve_system`) will read from, so the Controller draws the view instead of
/// the old stored-vocabulary stitching. Every bundle is applied through the grammar
/// gate. `terms`/`connectives` are per-index/per-edge; `coordinates`/`colours` per-index.
pub fn compose_model(
    cardinality: u8,
    terms: &[String],
    connectives: &[String],
    coordinates: &[[f64; 3]],
    colours: &[String],
) -> Hypergraph {
    let mut bundle = topology_morphisms(cardinality);
    bundle.extend(vocabulary_morphisms(cardinality, terms, connectives));
    bundle.extend(geometry_morphisms(cardinality, coordinates, colours));
    compose_checked(cardinality, bundle).expect("canonical model bundle is grammatical")
}

// ---- The substrate as a DATA SOURCE (not just an assembly mechanism) ----

/// The substrate **store** — the persistent data source: raw, **content-addressed**
/// elements + links. Data is *ingested* here as bare elements (e.g. from the legacy
/// JSON at first), then systems are **composed from the store** — no JSON at compose
/// time. This is the `−−` ground of the architecture tetrad. Elements are raw (just the
/// datum); relationships live in links (Holochain-style) — never as fields on an element.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SubstrateStore {
    pub elements: Vec<DataElement>,
    pub links: Vec<HyperLink>,
}

impl SubstrateStore {
    /// The **content address** of a datum: the id *is* derived from the content, so the
    /// same value is one element (natural dedup — content-addressing supersedes ref ids).
    pub fn content_id(kind: &str, value: &str) -> String {
        format!("{kind}:{value}")
    }

    /// **Ingest** a raw datum as a content-addressed element (deduped by content).
    /// Returns its content id — the handle you compose with.
    pub fn ingest(&mut self, kind: &str, value: &str) -> String {
        let id = Self::content_id(kind, value);
        if !self.elements.iter().any(|e| e.id == id) {
            self.elements.push(DataElement {
                id: id.clone(),
                kind: kind.to_string(),
                character: value.to_string(),
            });
        }
        id
    }

    /// Look up a stored element by its content id.
    pub fn get(&self, id: &str) -> Option<&DataElement> {
        self.elements.iter().find(|e| e.id == id)
    }
}

/// **Compose a system from the store** — the substrate-as-data-source path. Given the
/// ordinality (order-cardinality) and the *content ids* of the term/connective elements
/// (looked up from the store, e.g. by searching the coherence dodecad by cardinality),
/// generate the topology and anchor the stored elements to it. No raw strings pass in —
/// the data comes from the substrate.
pub fn compose_from_store(
    store: &SubstrateStore,
    cardinality: u8,
    term_ids: &[String],
    connective_ids: &[String],
) -> Hypergraph {
    let resolve = |ids: &[String]| -> Vec<String> {
        ids.iter()
            .filter_map(|id| store.get(id).map(|e| e.character.clone()))
            .collect()
    };
    compose_system(cardinality, &resolve(term_ids), &resolve(connective_ids))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triad_topology_is_3_vertices_3_orbits() {
        let t = generate_topology(3);
        assert_eq!(t.elements.len(), 3);
        assert_eq!(t.links.len(), 3); // C(3,2) undirected orbits (not 6)
        assert_eq!(t.elements[0].id, "el_3_1");
        assert_eq!(t.elements[0].index, 1);
        // edges in canonical cardinality, undirected (a < b).
        assert_eq!(t.links[0].endpoints, (1, 2));
        assert_eq!(t.links[1].endpoints, (1, 3));
        assert_eq!(t.links[2].endpoints, (2, 3));
        assert_eq!(t.links[2].id, "lk_3_2_3");
    }

    #[test]
    fn edge_count_matches_the_template_size_for_any_order() {
        for (cardinality, size) in [(1u8, 0usize), (2, 1), (4, 6), (6, 15), (8, 28)] {
            let t = generate_topology(cardinality);
            assert_eq!(t.elements.len(), cardinality as usize);
            assert_eq!(t.links.len(), size);
            assert_eq!(t.links.len(), Template::for_order(cardinality).size());
        }
    }

    #[test]
    fn links_are_undirected_low_high() {
        for l in &generate_topology(5).links {
            assert!(l.endpoints.0 < l.endpoints.1, "undirected: a < b");
        }
    }

    #[test]
    fn topology_morphisms_are_monomorphisms_one_per_vertex_and_edge() {
        let bundle = topology_morphisms(4);
        // 4 vertices + C(4,2)=6 edges = 10 morphisms, all monomorphisms.
        assert_eq!(bundle.len(), 10);
        for m in &bundle {
            assert_eq!(m.kind(), MorphismKind::Monomorphism);
        }
    }

    #[test]
    fn generated_topology_validates_and_broken_one_is_rejected() {
        let template = Template::for_order(3);
        let good = generate_topology(3);
        assert!(good.validate_against(&template).is_ok());

        // Break the shape: an orbit (1,1) is not a legal undirected template edge.
        let mut bad = good.clone();
        bad.links[0].endpoints = (1, 1);
        assert!(bad.validate_against(&template).is_err());

        // Break the size: drop a vertex → wrong cardinality.
        let mut short = good;
        short.elements.pop();
        assert!(short.validate_against(&template).is_err());
    }

    #[test]
    fn compose_system_anchors_terms_and_connectives_at_canonical_positions() {
        let hg = compose_system(
            3,
            &["Will".into(), "Function".into(), "Being".into()],
            &["Generation".into(), "Decision".into(), "Consent".into()], // (1,2)(1,3)(2,3)
        );
        // topology intact.
        assert_eq!(hg.topology.elements.len(), 3);
        assert_eq!(hg.topology.links.len(), 3);
        // 3 term + 3 connective data elements, LINKED on (not embedded).
        assert_eq!(hg.data.iter().filter(|d| d.kind == "term").count(), 3);
        assert_eq!(hg.data.iter().filter(|d| d.kind == "connective").count(), 3);
        // canonical index: term_3_1 = Will, anchored orthogonally to vertex el_3_1.
        assert_eq!(hg.data.iter().find(|d| d.id == "term_3_1").unwrap().character, "Will");
        let anchor = hg.links.iter().find(|l| l.id == "anchor_v_3_1").unwrap();
        assert_eq!(anchor.base, "el_3_1");
        assert_eq!(anchor.target, "term_3_1");
        // connective on the (1,2) orbit = Generation.
        assert_eq!(hg.data.iter().find(|d| d.id == "conn_3_1_2").unwrap().character, "Generation");
        // lateral term–connective–term: term_3_1 ──conn_3_1_2── term_3_2.
        let lat = hg.links.iter().find(|l| l.id == "lat_3_1_2").unwrap();
        assert_eq!(lat.base, "term_3_1");
        assert_eq!(lat.target, "term_3_2");
        assert_eq!(lat.link_type, "conn_3_1_2");
    }

    #[test]
    fn semantic_triad_is_decoupled_from_topology() {
        // Dropping the orthogonal anchors leaves a standalone semantic triad
        // (term–connective–term) — the decoupled projection.
        let hg = compose_system(
            3,
            &["a".into(), "b".into(), "c".into()],
            &["x".into(), "y".into(), "z".into()],
        );
        let lateral: Vec<_> = hg.links.iter().filter(|l| l.id.starts_with("lat_")).collect();
        assert_eq!(lateral.len(), 3); // the triad's internal term–connective–term structure
        for l in lateral {
            assert!(l.base.starts_with("term_") && l.target.starts_with("term_"));
            assert!(l.link_type.starts_with("conn_")); // linked BY a connective character
        }
    }

    #[test]
    fn compose_render_places_geometry_and_colour_with_laterals() {
        let hg = compose_render(
            3,
            &[[0.0, 1.0, 0.0], [-1.0, -1.0, 0.0], [1.0, -1.0, 0.0]],
            &["red".into(), "green".into(), "blue".into()],
        );
        // topology intact + 3 coordinate + 3 colour data elements (own terms, no metadata).
        assert_eq!(hg.topology.elements.len(), 3);
        assert_eq!(hg.data.iter().filter(|d| d.kind == "coordinate").count(), 3);
        assert_eq!(hg.data.iter().filter(|d| d.kind == "colour").count(), 3);
        // orthogonal anchors: vertex el_3_1 ▶ coord_3_1 and ▶ colour_3_1.
        assert!(hg.links.iter().any(|l| l.base == "el_3_1" && l.target == "coord_3_1"));
        assert!(hg.links.iter().any(|l| l.base == "el_3_1" && l.target == "colour_3_1"));
        assert_eq!(hg.data.iter().find(|d| d.id == "colour_3_3").unwrap().character, "blue");
        assert_eq!(hg.data.iter().find(|d| d.id == "coord_3_1").unwrap().character, "0,1,0");
        // laterals: 3 coordinate→coordinate lines + 3 colour→colour lines per edge.
        assert_eq!(hg.links.iter().filter(|l| l.link_type == "line").count(), 3);
        assert_eq!(hg.links.iter().filter(|l| l.link_type == "colour").count(), 3);
        let line = hg.links.iter().find(|l| l.id == "line_3_1_2").unwrap();
        assert_eq!(line.base, "coord_3_1");
        assert_eq!(line.target, "coord_3_2");
        // and each line / colour-line ANCHORS to its topological orbit (edge ↔ line).
        let anchor_line = hg.links.iter().find(|l| l.id == "anchor_line_3_1_2").unwrap();
        assert_eq!(anchor_line.base, "lk_3_1_2"); // the orbit
        assert_eq!(anchor_line.target, "line_3_1_2");
        assert!(hg
            .links
            .iter()
            .any(|l| l.id == "anchor_colline_3_1_2" && l.base == "lk_3_1_2"));
    }

    #[test]
    fn every_canonical_morphism_declares_its_topological_site() {
        // vertex morphisms → Vertex(index); edge morphisms → Edge(a,b).
        assert_eq!(IndexToVertex { cardinality: 3, index: 2 }.site(), MorphismSite::Vertex(2));
        assert_eq!(EdgeToOrbit { cardinality: 3, a: 1, b: 2 }.site(), MorphismSite::Edge(1, 2));
        assert_eq!(
            TermToVertex { cardinality: 3, index: 3, character: "x".into() }.site(),
            MorphismSite::Vertex(3)
        );
        assert_eq!(
            ConnectiveToOrbit { cardinality: 3, a: 2, b: 3, character: "y".into() }.site(),
            MorphismSite::Edge(2, 3)
        );
    }

    #[test]
    fn grammar_checked_build_equals_the_direct_build() {
        // Routing the canonical bundle through the grammar gate yields exactly the
        // same topology as generate_topology (the gate admits every legal morphism).
        let checked = compose_checked(3, topology_morphisms(3)).expect("canonical bundle is legal");
        assert_eq!(checked.topology, generate_topology(3));
    }

    #[test]
    fn grammar_rejects_an_ungrammatical_edge_morphism() {
        // A self-loop (1,1) is not an edge of K_3 → the adjacency matrix has no such
        // entry → the grammar rejects the bundle (composition is gated, not blind).
        let mut bundle = topology_morphisms(3);
        bundle.push(Box::new(EdgeToOrbit { cardinality: 3, a: 1, b: 1 }));
        let result = compose_checked(3, bundle);
        assert!(result.is_err(), "self-loop must be rejected");
        assert!(result.unwrap_err()[0].contains("Edge(1, 1)"));

        // An out-of-range edge (2,5) on a triad is likewise ungrammatical.
        let mut bundle = topology_morphisms(3);
        bundle.push(Box::new(ConnectiveToOrbit { cardinality: 3, a: 2, b: 5, character: "z".into() }));
        assert!(compose_checked(3, bundle).is_err(), "out-of-range edge must be rejected");
    }

    #[test]
    fn grammar_rejects_an_out_of_range_vertex_morphism() {
        // Vertex 4 does not exist in K_3 → rejected.
        let mut bundle = topology_morphisms(3);
        bundle.push(Box::new(IndexToVertex { cardinality: 3, index: 4 }));
        let err = compose_checked(3, bundle).unwrap_err();
        assert!(err[0].contains("Vertex(4)"));
    }

    #[test]
    fn compose_model_unifies_topology_vocabulary_geometry_colour() {
        // The singular Model: one topology, all four layers anchored on it.
        let hg = compose_model(
            3,
            &["Will".into(), "Function".into(), "Being".into()],
            &["Generation".into(), "Decision".into(), "Consent".into()],
            &[[0.0, 1.0, 0.0], [0.0, -1.0, 0.0], [1.0, 0.0, 0.0]],
            &["#FF0000".into(), "#0000FF".into(), "#FFFF00".into()],
        );
        assert_eq!(hg.topology.elements.len(), 3);
        assert_eq!(hg.topology.links.len(), 3);
        for (kind, n) in [("term", 3), ("connective", 3), ("coordinate", 3), ("colour", 3)] {
            assert_eq!(hg.data.iter().filter(|d| d.kind == kind).count(), n, "{kind} count");
        }
        // the same anchors the two partial builders produce, now in ONE hypergraph.
        assert_eq!(hg.data.iter().find(|d| d.id == "term_3_1").unwrap().character, "Will");
        assert_eq!(hg.data.iter().find(|d| d.id == "colour_3_3").unwrap().character, "#FFFF00");
        assert_eq!(hg.data.iter().find(|d| d.id == "coord_3_1").unwrap().character, "0,1,0");
        // vertex el_3_1 anchors term, coordinate AND colour — the tetrad meeting at one vertex.
        for target in ["term_3_1", "coord_3_1", "colour_3_1"] {
            assert!(hg.links.iter().any(|l| l.base == "el_3_1" && l.target == target), "anchor {target}");
        }
    }

    #[test]
    fn store_ingests_content_addressed_then_composes_from_it() {
        let mut store = SubstrateStore::default();
        // content-addressed: the same value ingested twice is ONE element (dedup).
        let a = store.ingest("word", "Will");
        let b = store.ingest("word", "Will");
        assert_eq!(a, b);
        assert_eq!(a, "word:Will");
        assert_eq!(store.elements.len(), 1);

        // ingest a triad's vocabulary as raw elements, keep their content ids.
        let terms: Vec<String> = ["Will", "Function", "Being"]
            .iter()
            .map(|v| store.ingest("word", v))
            .collect();
        let conns: Vec<String> = ["Generation", "Decision", "Consent"]
            .iter()
            .map(|v| store.ingest("word", v))
            .collect();
        assert_eq!(store.elements.len(), 6); // Will already there; +2 terms +3 conns

        // compose the system FROM the store (no raw strings passed in).
        let hg = compose_from_store(&store, 3, &terms, &conns);
        assert_eq!(hg.topology.elements.len(), 3);
        assert_eq!(hg.data.iter().find(|d| d.id == "term_3_1").unwrap().character, "Will");
        assert_eq!(hg.data.iter().find(|d| d.id == "conn_3_1_2").unwrap().character, "Generation");
    }
}
