# Language guide

The canonical terminology for this codebase. The words here are not arbitrary labels —
they are **systematic**, and much of this language *already lives in the graph* as
seeded systems (the library is the living source; this guide mirrors it and records the
pending renames). When code and this guide disagree, the guide is the target: the code
is being refactored toward it (`position → ordinality`, etc.).

Convention below: **+** affirming · **−** receptive · **=** reconciling.

---

## 1. Counting & placement (the core dyads → a tetrad)

- **Cardinality = count.** **Ordinality = placement.** (The reconciling concept is
  *number* — one value read two ways.)
- These cross with the **Order / Degree** dyad (vertex-side / edge-side) to give a
  **tetrad** (`Cardinality ++ · Ordinality −− · Order −+ · Degree +−`):

| term | meaning | current code name |
|---|---|---|
| **order-cardinality** | number of vertices `n` | `order` |
| **order-ordinality** | a vertex's placement `1..n` | `position` |
| **degree-cardinality** | number of connectives `C(n,2)` (graph "size") | `expected_connectives` / `size` |
| **degree-ordinality** | a connective's placement | read off adjacency/incidence/line |

**Rename map (pending):** `position → ordinality`; `order` (the count field) → keep as
the count but understand it as **order-cardinality**; reserve "**order**" for
node-counting only.

---

## 2. The systems, by order (the vocabulary)

Order names: **Monad · Dyad · Triad · Tetrad · Pentad · Hexad · Heptad · Octad · Ennead
· Decad · Undecad · Dodecad** (orders 1–12).

Each order has a **hexadic-systematics** row (its "shape", `core::hexadicsystems`) — six
mutually-determining facets keyed by cardinality:

`{ name · coherence · term-designation · connective-designation · term-cardinality (|V|=n)
· connective-cardinality (|E|=C(n,2)) }`

e.g. `hexad(3) = {Triad, Dynamism, Impulses, Acts, 3, 3}`. These facets are themselves
seeded as **dodecad** systems (Coherence Attributes, Term Designations, Connective
Designations), composed from the single source.

- **Term** = a node's character (a word). **Connective** = an edge's character.
- **Coherence** = the quality that makes an order-n system one whole (e.g. Triad =
  Dynamism, Hexad = Coalescence).

---

## 3. The architecture (a tetrad)

The app *is* a systematic tetrad:

| face | polarity | role |
|---|---|---|
| **Controller** | ++ | the assembly mechanism / grammar-gate — **composes** |
| **Substrate** | −− | the data source (ground): elements + links, vocabulary anchored to topology |
| **Model** | +− or −+ | left projection |
| **View** | −+ or +− | right projection |

(Which of Model/View is `+−` vs `−+` is open.) Maps onto the base-space tetrad:
**Template(++) ↔ Controller · Topology(−−) ↔ Substrate · Geometry(+−)/Vocabulary(−+) ↔
Model/View**.

**MVC triad connectives** (the edges, LOCKED): **compose** (View–Model) · **resolve**
(Model–Controller) · **render** (Controller–View). Directed 3-cycle
`View →compose→ Model →resolve→ Controller →render→ View`. The serving path is the
`321`/Freedom reading — "Controller resolves the Model, which composes the View."

---

## 4. The graph triad (the matrices)

The three matrices, as **nodes**, grounded in the incidence matrix `B`:

| matrix | polarity | domain | relation |
|---|---|---|---|
| **Adjacency** `A` | − | Topology (vertex↔vertex) | `A = B·Bᵀ` |
| **Line** `L` | + | Semantics (edge↔edge) | `L = Bᵀ·B` |
| **Incidence** `B` | = | Reconciler (vertex↔edge) | the generator/bridge |

**Edges = category-theory axioms:** **identity** (Adjacency–Line, duals/orbital) ·
**composition** (Adjacency–Incidence, `A=BBᵀ`) · **associativity** (Line–Incidence,
`A·B=B·L`). Seeded as the **Graph Construction** system.

---

## 5. The six laws of three (`S₃`)

The 3 impulses of a triad read in `3! = 6` orders. Each law = a permutation; the six
are the Controller's traversals.

| law | perm | parity | hexad pos · colour |
|---|---|---|---|
| Expansion | 123 | rotation | 2 · blue |
| Identity | 231 | rotation | 1 · red |
| Order | 312 | rotation | 3 · yellow |
| Interaction (SPO) | 132 | reflection | 5 · purple |
| Concentration | 213 | reflection | 6 · orange |
| Freedom | 321 | reflection | 4 · green |

Rotations walk a triad's directed 3-cycle **clockwise** (natural readings); reflections
**counter-clockwise** (the inverse morphisms). Tooling: the `triad-six-laws` skill.

---

## 6. Morphisms

The morphism-type ladder (edges should aim for **isomorphism**, so a triad is fully
bidirectional): **injective / surjective / bijective → monomorphism / epimorphism /
isomorphism → functor**. In the substrate, a system is **composed** by bundling
morphisms and applying them through the **grammar gate** (the matrices decide which
morphisms are legal).

---

## 7. Substrate terms (Holochain-flavoured)

- **Element** — a raw, content-addressed data node (the value *is* the identity; no
  separate reference ids — content-addressing supersedes them).
- **Link** — a relationship between elements (`base–type–target`). Relationships live in
  links, **never** as fields on an element.
- **Anchor** — an orthogonal link mapping a topology element to its data (vertex → term,
  orbit → connective). **Lateral** — a within-system link (`term –connective– term`).
- **Modular seam** — a connective/link that is a clean swap boundary (every architectural
  edge is one).

---

## Notes
- JSON (`data/*.json`) is a **legacy loading device** — ad hoc temp storage. The target
  is vocabulary anchored to topology in the substrate (Holochain / vector DB), with JSON
  as export only.
- This guide is a companion to the **in-graph** representation (the Architecture Pentad /
  the seeded systems). Prefer noting new language *into the library* (as systems +
  connectives) — this doc mirrors that.
