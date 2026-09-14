# Validation — archetype ↔ instance

[← index](README.md) · related: [compositional-model](compositional-model.md)

A constructed **topology** must be validated against a **system**. The rebuild makes the
correspondence explicit as an **archetype** (the rules/vocabulary) and an **instance** (a
concrete K_n filled with characters). Both can be saved as hexads.

## Archetype validation — the equivalences
These pairs are asserted equivalent (topology term ↔ system term):

| topology (archetype) | system (archetype) | example |
|---|---|---|
| **Graph** | **System** | K3 = triad |
| **Cardinality** | **Coherence** attribute | (3,3) = Dynamism; (4,6) = Activity Field |
| **Order** (n) | **Term designation** | 3 = Impulses; 4 = Sources |
| **Size** (edges) | **Connective designation** | 3 = Acts; 4 = Interplays |
| **Vertex ordinality** | **Term position** | 1 = term1, 2 = term2 |
| **Edge seriality** | **Connective position/sequence** | 1 = connective1, 2 = connective2 |

Reading: **coherence ↔ cardinality**; **term/connective designations ↔ order/size**; term and
connective *designations* relate to the coherence attribute, while *order and size* relate to
cardinality. The category-theory triad for the operations themselves —
**Composition(+) · Associativity(−) · Identity(=)** — is correct, but its terms are **probably
edges** (connectives), not nodes; see [node-edge-inversion](node-edge-inversion.md) for the
node↔edge duality that places them there.

## Topology instance validation (worked: K4)
```
Type            = K4
Cardinality(n,n)= 4, 6            (order, size)
Order(n)        = 4
Size(n)         = 6
Ordinality(order) = Vertex(1,2,3,4)
Seriality(size)   = Edge(1,2,3,4,5,6)
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
Compose the **topology** first (assign vertex ordinality + edge seriality, per
[compositional-model](compositional-model.md)); then return to the archetypal rules and
**ensure the topology instance matches the system instance** — anchoring each term to a vertex
and each connective to an edge (t1→v1 … tn→vn, c1→e1 … cm→em), and asserting the coherence +
designations for that order. The **six laws** are the associative orderings of the
edge-serialisation over a triad (v1·e1·v2·e2·v3·e3 = 123 *order*; v1·e3·v3·e2·v2·e1 = 132
*interaction*; etc.).
</content>
