# Node ↔ edge inversion (the triad duality)

[← index](README.md) · related: [compositional-model](compositional-model.md), [validation](validation.md)

It is useful to be able to **express any triad as nodes**, and then **invert the nodes to
edges** — a node↔edge duality. You author the three terms as vertices, then flip them so they
become the three connectives.

## The mapping (user)
- Node **1** → the edge between nodes **1 and 3**.
- Node **2** → the edge between nodes **2 and 3**.
- Node **3** → the edge between nodes **1 and 2**.

## It "walks the path backwards" — reversal of the serialisation
Take the **walk/circuit** serialisation of a triad's edges from
[compositional-model](compositional-model.md) (completing the circuit 1→2→3→1):
`e1 = {1,2}`, `e2 = {2,3}`, `e3 = {3,1} = {1,3}`. Then the mapping above is exactly

```
node_i  ↔  e_(n+1 − i)      (n = 3)
node 1  ↔  e3 = {1,3}       ✓
node 2  ↔  e2 = {2,3}       ✓
node 3  ↔  e1 = {1,2}       ✓
```

i.e. the node at serial position *i* maps to the edge at the **reversed** serial position.
This is why, "on first appearances," the conversion **walks the path backwards** — it is the
reversal (reflection) of the 123 serialisation, the same reflection that turns a 123 *order*
reading into a 132 *interaction* reading in the six laws.

## Consequence: Composition · Associativity · Identity are edges
The category-theory triad **Composition(+) · Associativity(−) · Identity(=)** is correct, but
its terms are **probably edges** (connectives / acts), not nodes. So a triad may be authored
as three nodes and then inverted so that these live on the edges — the inversion tells you
where a given articulation "really" belongs.

## Scope / open question
For a **triad**, `#nodes = #edges = 3`, so the inversion is a clean **bijection**. For other
orders the counts diverge (a tetrad has 4 nodes but 6 edges = `n(n-1)/2`), so this exact
node↔edge bijection is **special to the triad**; the general-order form of the duality (and
whether it becomes a line-graph / dual-graph relation, or the `node_i ↔ e_(n+1−i)` reversal
only holds on the triad) is to be pinned down in the rebuild.
