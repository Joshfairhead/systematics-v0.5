# Plan — correct-by-construction assembly, then products

[← index](README.md) · related: [compositional-model](compositional-model.md), [comparing-systems](comparing-systems.md)

**Goal (user, 2026-09-17):** implement *correct-by-construction* graph design — articulate the
**nodes**, then the **edges**, via the out-degree cascade — so the base topology is deterministic
and ordinality falls out. Then check that the **lexicographical product** (and other products)
remain possible on that foundation. Prototype now; formalise for v6.

## Where we already are
- `grammar.rs::edges()` **is** the cascade (1-based): `for p1 in 1..=n { for p2 in p1+1..=n }`
  → `(1,2),(1,3),…,(2,3),…`. Nodes are `1..n`. So the base topology is *already* built this way;
  what's missing is making it **explicit and first-class** (an instruction stream + validation),
  and pinning the **ConnectiveOrdinality** to the cascade index.

## Does the cascade rule out the lex product? No — it enables it.
The cascade is the **construction primitive** (build one graph). A product **combines two graphs**.
They live at different layers:
- **Cascade** = build a graph from an instruction stream (`InitNodes`, `AddEdge{u<v}`).
- **Lex product `G[H]`** = a *new* graph on the pair set `V(G)×V(H)`, with adjacency
  `(g₁,h₁)~(g₂,h₂)  iff  g₁~g₂  OR  (g₁=g₂ ∧ h₁~h₂)`.
To build `G[H]` you (1) have `G` and `H` (built by the cascade), (2) enumerate the pair-vertices in
lex order (cascade over pairs), (3) emit edges by the rule above — which just reads `G`/`H`
adjacency. So the cascade is a **prerequisite + enumerator**, not a constraint. Products are a
higher operation layered on it (the parked **Graph Products** tetrad).

## Implementation (prototype now → substrate in v6)
**Phase 1 — make the cascade first-class (correct-by-construction).**
- A `topology::cascade(n) -> Vec<AssemblyInstruction>` (`InitNodes{count}`, `AddEdge{batch,source,target}`),
  with `validate(instruction)` enforcing `source < target`, `batch == source+1`, bounds. (Mirror
  Gemma's Rust directly.)
- Assert `edges()` equals the cascade's `AddEdge`s (a *correct-by-construction* pin test).
- Define **ConnectiveOrdinality(edge) = its index in the cascade** (`(1,2)→1, (1,3)→2, …`), and
  surface it on connectives (replaces the "Force 1 Needs Research" filler with the real serial).
- Nodes-then-edges: `InitNodes` first, then the edge batches — the articulation order the user wants.

**Phase 2 — products as operations over two graphs.**
- `lex_product(g, h)`: vertex set = `V(g)×V(h)` (enumerated lex), edges by the rule above, emitted
  as a fresh instruction stream (so a product is itself cascade-constructed and content-addressable).
- Generalise to the Graph-Products tetrad (Cartesian `□`, Tensor `×`, Strong `⊠`, Lexicographical `[]`)
  — same shape, different adjacency predicate. (See [comparing-systems](comparing-systems.md): these
  use the **adjacency** spectrum; the cascade gives the deterministic enumeration.)

**Phase 3 (v6) — substrate.** The `AssemblyInstruction` stream is the Holochain build path
(MessagePack, agent-safe by the `u<v` rule). Construction, ordinality, products all serialise
through it.

## Why this is the foundation (honest scope)
The load-bearing result is the **cascade as a canonical, deterministic construction serialisation**
— it settles ordinality and is replayable/distributed-safe. Lexicographical *order* is the general
linearisation principle it uses; the lexicographical *product* is a separate operation that shares
that principle (enumerate pairs in lex order) but is **defined by its adjacency rule**, not founded
on the cascade. So: build bases correct-by-construction now; products are unblocked, not implied.

## Lexicographic product — concrete blueprint (Gemma, 2026-09-30; not gospel)
Gemma's summary gives a buildable `G·H` on the same cascade, via **1D flattening**:
- **Flatten** the pair `(g, h)` to a single id: `id = g * n_h + h`. So the product's vertex set
  is `0 .. n_g*n_h`, enumerated in lexicographical (g-major) order — the cascade's native order.
- **Two edge passes** (both obeying `source < target`, so no dup/omission, state-free validation):
  1. **Inter-clique / cross edges** — for `u_g < v_g`, wire *every* H-node under `u_g` to *every*
     H-node under `v_g` (the G-adjacency dominates — a full bipartite join between the two fibres).
  2. **Intra-clique / local edges** — for each `g`, wire its internal H-copy by the standard
     cascade (`u_h < v_h`).
- Same `AssemblyInstruction` stream (`InitNodes`/`AddEdge{batch,source,target}`), **MessagePack**
  (`rmp-serde`) on the wire for Holochain (not JSON).

This is Phase 2 of the plan above, made concrete: the product is a second instruction generator
over the flattened pair-space, reusing the cascade + the `source < target` validator. For K_m·K_n
it yields `K_{mn}` (complete-graph closure). **Treat as a strong draft, not settled** — verify the
cross/intra decomposition against our topology before adopting.

## Two notes (user, 2026-09-30)
- **+ / = / − are *impulses*, not polarities** — the three impulses of a triad (active / reconciling
  / passive). Encoded today by **term order** (position 1/2/3); no separate impulse field yet.
- **List sorting is a Sort-operation concern.** The alphabetical name sort is a stopgap; ordering
  (by name, type, …) belongs to the **Sort** edge of the sort/filter module (the Browser triad),
  configurable — not a hardcoded `sort_by`.
