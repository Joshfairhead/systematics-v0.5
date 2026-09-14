# Compositional model — functional construction of complete graphs

[← index](README.md) · related: [validation](validation.md), [operations](operations-and-workflows.md)

The rebuild constructs systems **functionally**: not by hand-assembling a K_n, but by
composing small pure functions. Two primitives suffice to build every complete graph.

## The two primitives
1. **create vertex** — make a node (a K1).
2. **link vertex to all others** — given a new vertex and an existing set of vertices, add an
   edge from the new vertex to each existing one.

*(Open question the user raised: is "a set of vertices" valid in category theory — do sets
exist there? To resolve in the rebuild. The pragmatic reading is a finite ordered collection;
the categorical framing may be an object + its morphisms.)*

## Sequential generation (the generative principle)
Generate nodes sequentially, and link them sequentially — 2 links to 1 first; then 3 links to
1 and 2; etc. This makes a complete graph grow by **addition**:

```
K1 + node                         = K1
K1 + node + 1 connective          = K2
K2 + node + 2 connectives         = K3
K3 + node + 3 connectives         = K4
K4 + node + 4 connectives         = K5
K5 + node + 5 connectives         = K6
K(n-1) + node + (n-1) connectives = Kn
```

So `K3 + K1 = K4` (add a node, link it to all three). This is the **handshaking lemma**:
a K_n has `n(n-1)/2` edges. One simple generative principle completes all graphs.

## Path vs circuit (the choice at each order)
Starting from a node: add node + connective → **path** (a K2; extending the path 1→2→3 is
"expansion", the 123 reading). At each step there's a choice:
- **complete the circuit** (add the remaining connectives to close K_n), or
- **continue the path** (add another node + one connective), which then offers completing the
  *next* order.

Triad: complete K3 with a 3rd connective (a walk/circuit — edges can then be labelled
sequentially), or continue. Tetrad: complete with 3 more connectives, or continue → pentad
(6 more), → hexad (10 more), …

## Sequential edge structure + colouring
The handshaking lemma gives a striking result: a K_n's edges stratify by the order at which
they were added. In a **K5** (10 edges): **1** K2-edge, **2** K3-edges, **3** K4-edges,
**4** K5-edges.

A colouring scheme over the construction:
- **K2**: 1 red.
- **K3**: 1 red, 2 blue.
- **K4**: 1 red, 2 blue, 3 green.
- **K5**: reconfigure the greens so 1 red + 3 greens form a box, then orange connects the rest.
- **K6**: retains the K5 shape, adds a node + 6 new edges.

K2 and K3 stay **consistent** across the design space; **K4→K5 has a transition** (the three
green edges must be reconfigured); the pattern then continues to K6. This colouring is a
promising basis for the graph-construction UI and for reading a system's internal structure.

## Naming
The assembly operation is **addition / join** (not "compose"). Breaking a system into its
faces is **subtraction / decompose**. As an interface pair these are likely
**conjunction (join) / disjunction (decompose)** — to be settled in [operations](operations-and-workflows.md).
"Compose" stays reserved for the meta **coordinator** that groups the primitives.

## Worked triads (123 for both nodes AND edges)
A fully-articulated triad names its 3 nodes *and* its 3 edges, both following the 123 order:
- **Convening** — nodes: Sponsors, Participants, Stakeholders; edges: Hosting, Coordinating,
  Facilitating.
- **Ventures** — nodes: Startups, Corporates, Investors; edges: Supporting, Finding, Investing.

(In the v0.5 prototype these were seeded with the node/edge distinction blurred; the rebuild
should treat nodes and edges as separately-articulated 123 series.)
</content>
