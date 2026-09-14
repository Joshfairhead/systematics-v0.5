# systematics-v0.6 — rebuild intent

**Status: intent capture, not implementation.** systematics-v0.5 (this repo) is a
**prototype**. It works well enough to *demonstrate the intent*, but the architecture was
grown reactively and is not a foundation to build on. v0.6 will **rebuild most of the
functionality from scratch on sturdier foundations** (the Holochain substrate; a principled,
functional construction of systems). The v0.5 code is kept as an *indicator of intent*, not
a base to extend.

This folder captures that intent **as precisely as possible**, so the rebuild can proceed
from a clear specification rather than from reverse-engineering the prototype.

## Why chunks, not a monolith
A single long document is nearly impossible to review — it is "basically a monad that needs
to be sorted into dyads and higher-order systems." So this spec is deliberately **assembled
from small chunks (nodes) with explicit relations (links)** — the same compression-of-intent
at different resolutions that the tool itself is about. Read the node you need; follow the
links for context.

## The chunks (nodes)
- **[compositional-model.md](compositional-model.md)** — the principled, *functional*
  construction of complete graphs: create-vertex + link-to-all, the handshaking-lemma
  generative sequence, path-vs-circuit choices, sequential edge structure + colouring.
- **[validation.md](validation.md)** — the archetype ↔ instance validation: the equivalences
  (Graph=System, Cardinality=Coherence, Order=TermDesignation, Size=ConnectiveDesignation,
  VertexOrdinality=TermPosition, EdgeSeriality=ConnectivePosition), saved as hexads, and how
  a topology instance is checked against a system instance.
- **[operations-and-workflows.md](operations-and-workflows.md)** — the operations
  (conjunction/disjunction = the assembly join / decompose, as a *drag-and-drop* surface not
  a select-and-button POC; store/load; CRUD) and the creation/navigation workflows (define a
  monad first, or "tetrad of X"; all monads are heads; unify list/graph navigation).
- **[source-intent.md](source-intent.md)** — the user's own writing, preserved verbatim, as
  the source of truth these synthesised chunks derive from.

## Relations (links between the nodes)
- The **compositional-model** produces topologies; **validation** binds each topology to a
  system (archetype ↔ instance); **operations-and-workflows** are how a person drives the
  model + validation through the interface.
- Everything is **functional**: systems are *constructed* by composing small pure functions
  (create node, link node), never hand-assembled. "Compose" is the meta **coordinator** that
  groups those primitives — reserved, not spent on a single join.
- Grounds in the seeded **Identity · Associativity · Composition** triad and the
  **Assembly / Decomposition** dyad (which parallels Sort / Filter).

## What carries over from the prototype (as intent, to be rebuilt cleanly)
Registry of systems; two views (graph/list); store/load; CRUD with on-graph editing; the
interface state model (Viewing/Editing → CRUD); join & decompose (proof-of-concept, flaky —
see operations doc); the seeded library of systems as example content.
</content>
