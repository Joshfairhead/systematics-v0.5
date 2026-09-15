# Validation — archetype ↔ instance

[← index](README.md) · related: [compositional-model](compositional-model.md)

A constructed **topology** must be validated against a **system**. The rebuild makes the
correspondence explicit as an **archetype** (the rules/vocabulary) and an **instance** (a
concrete K_n filled with characters). Both can be saved as hexads.

## Archetype validation — the equivalences
These pairs are asserted equivalent (topology facet ↔ system facet):

| topology (archetype) | system (archetype) | example |
|---|---|---|
| **Cardinality** (n, size) | **System** | (3,3) = Triad; (4,6) = Tetrad |
| **Eigenvalue** *(proposed)* | **Coherence** attribute | Dynamism; Activity Field |
| **Order** (n) | **Term designation** | 3 = Impulses; 4 = Sources |
| **Size** (edges) | **Connective designation** | 3 = Acts; 4 = Interplays |
| **Vertex ordinality** | **Term ordinality** | 1 = term1, 2 = term2 |
| **Edge ordinality** | **Connective ordinality** | 1 = connective1, 2 = connective2 |

Reading: a K_n **is** its **cardinality** — "K4" is only notation for `(4,6)` — so cardinality
book-matches the **system** name. The graph's spectral *quality* (**eigenvalue**, the Laplacian
spectrum: `0` once = the null space / monad, and `n` with multiplicity `n−1` = the algebraic
connectivity) book-matches the **coherence** attribute. **Order/size ↔ term/connective
designations** (the component counts name the components); **vertex/edge ordinality ↔
term/connective ordinality** (both vertices *and* edges carry a placement). The category-theory
triad for the operations themselves — **Composition(+) · Associativity(−) · Identity(=)** — is
correct, but its terms are **probably edges** (connectives), not nodes; see
[node-edge-inversion](node-edge-inversion.md).

> **Correction (supersedes the initial phrasing in [source-intent](source-intent.md)).** The
> first draft paired `Graph ↔ System` and `Cardinality ↔ Coherence`, and called the sixth facet
> "edge **seriality**". Refined: **cardinality** *is* the graph (so it pairs with **system**);
> **eigenvalue** is the spectral quality that pairs with **coherence**; and edges carry
> **ordinality** like vertices do. **Seriality** is *not* a hexad facet — it is the six-laws
> *arrangement* of the ordinalities (123, 132, 321, …); it lives at the Controller level.

> **Implemented (v0.5 backend).** The two faces are first-class **peer hexads**, both derived
> from the one cardinality: a **TopologyHexad** (cardinality · eigenvalue · order · size · vertex
> ordinality · edge ordinality) and the **SystematicsHexad** (system · coherence · term
> designation · connective designation · term/connective cardinality). An **EquivalenceHexad**
> holds both and **book-matches** them dimension-for-dimension (`core/equivalence.rs`), with a
> numeric bridge (`order == term_cardinality`, `size == connective_cardinality`) guaranteeing
> the ordinality serialisations line up. Surfaced over GraphQL as `topologyHexad(cardinality)`,
> `systematicsHexad(cardinality)`, `equivalenceHexad(cardinality)` (topology + system + pairs +
> mismatches), and `validateSystemEquivalence(id)` for a stored instance.

## Topology instance validation (worked: K4)
```
Cardinality       = (4,6)          (order, size) — identifies K4 = Tetrad
Eigenvalue        = 0 (×1), 4 (×3) (Laplacian spectrum) — proposed ↔ coherence
Order(n)          = 4
Size(n)           = 6
Ordinality(order) = Vertex(1,2,3,4)
Ordinality(size)  = Edge(1,2,3,4,5,6)
```

## System instance validation (worked: a triad)
```
Coherence(Dynamism)
TermDesignation(Impulses)
ConnectiveDesignation(Acts)
TermCharacters      = <the three term values>
ConnectiveCharacters= <the three connective values>
```

## How construction uses this
Compose the **topology** first (assign vertex + edge **ordinality**, per
[compositional-model](compositional-model.md)); then return to the archetypal rules and
**ensure the topology instance matches the system instance** — anchoring each term to a vertex
and each connective to an edge (t1→v1 … tn→vn, c1→e1 … cm→em), and asserting the coherence +
designations for that order. The **six laws** are the **serialities** — the arrangements of
those ordinalities over a triad (v1·e1·v2·e2·v3·e3 = 123 *order*; v1·e3·v3·e2·v2·e1 = 132
*interaction*; etc.). So *ordinality* is the placement of a single element; *seriality* is the
order in which the placed elements are read — the Controller's six laws, not a hexad facet.
