# Systematic architecture registry

The codebase's language follows a **systematic schema**: every dyad, triad,
tetrad, … we implement *is* a system, and is documented here as one. This is the
map for auditing and refactoring the code back toward its systematic description
(dynamic homoiconicity) — when the codebase becomes a mess, come here.

**The typing rule** (the Dyad's force): a **term (vertex) is a noun**; a
**connective (edge) is a verb**. So a triad of *nouns* is node-typed (containers,
things); a triad of *verbs* is edge-typed (operations). Every entry below is
tagged accordingly.

**Status legend:** `impl` = in code · `seeded` = also a real System instance in
the graph · `proposed` = named but not built/settled.

## Self-documentation rule [settled]

**The system must document its own construction.** Every dyad / triad / … we build
is registered here *and*, as far as possible, **represented in the graph itself** —
so that filtering the tool by *architecture / project* surfaces these systems and
their relationships **without recourse to any out-of-system document**. This
markdown registry is the interim; the north star is that the app *is* its own
documentation (**dynamic homoiconicity**). As the model changes in conversation,
this updates in step.

## Everything is a tag [proposed — working principle]

In a view, **every attribute of every element is a tag** — a `key : value` pair
(perspective, source, locator, artefact, order, coherence, designation, …). This is
why the query controls are one triad rather than separate features: **Tag (=)
reconciles Sort (+) and Filter (−)**. Sort prioritises the list by a tag; Filter
adds/removes tags from the query. Because a tag is `key : value`, each of Sort and
Filter splits along the **by-key ↔ by-value** dyad. The control is therefore a small
tree — **Tag → {Sort, Filter} → {key, value}** — and it is **fractal**: the same
shape recurs at every level.

## The generative process — symmetric doubling [proposed]

The move that *builds* the architecture (distinct from the canonical 1→12 run) is a
**symmetric doubling**: take a whole and split it, keeping symmetry, so **1 → 2 → 4
→ 8**:

- **Monad → Dyad** — the whole splits into two poles (Tag → Sort/Filter; or by-key
  ↔ by-value).
- **Dyad → Tetrad** — each pole splits again (Sort → {by-key, by-value}; Filter →
  {by-key, by-value}) — four leaves, still symmetric.
- **Tetrad → Octad** — each leaf splits once more, retaining symmetry.

By the time the unfolding reaches a **System**, its own description is such a tree:
**order · coherence · designation · characters**, where **designation** splits into
*term-designation · connective-designation* and **characters** split into *terms ·
connectives* — the same left/right symmetry all the way down. This is the in-app
counterpart of the canonical run: the run *enumerates* orders 1→12; the doubling
*constructs* a system by successive symmetric division.

## Dyads (order 2 — Poles / Force)

| system | poles (nouns) | force (verb) | code | status |
|---|---|---|---|---|
| Language | **Grammar ↔ Vocabulary** | *reconciled by* System | `core/{grammar,vocabularies,systems}.rs` | impl |
| Class/Instance | **canonical ↔ instance** | *instantiates / overrides* | `renderSystem.canonicalClass` (`graphql/types.rs`) | impl |
| Query axis | **by key ↔ by value** | *sorts / selects* | `components/reference_browser.rs` (within the Query triad) | impl v1 — a tag has a *key* (its type) and a *value*; Sort and Filter each act on one axis |

## Triads (order 3 — Impulses / Acts)

| system | terms / role | node- or edge-typed | code | status |
|---|---|---|---|---|
| **Citation** | Source · Artefact · Lookup (nouns); edges *recordedIn · atLocation · cites* (verbs) | nouns=nodes, verbs=edges | `core/citations.rs`; seeded `system_citation_3` | impl + seeded |
| **Operations (ELT)** | Extract · Load · Transform (verbs) → `createSequence` · `loadPerspective` · `applyFunctor` | **edge-typed** (all verbs) | `graphql/types.rs`; UI: `components/reference_browser.rs` `elt_triad` (Nullad page) | impl (not seeded). **Extract** is wired (Nullad → Monad): materializes the current data-view selection (distinct `system:<id>` of the filtered references) into a persisted Monad via `createSequence` + `create_sequence` (client). **Load** is wired (`loadPerspective` → `on_load`). **Transform** (apply a Functor) is surfaced but **not yet wired**. Monad auto-naming is provisional (the members' *integral* is a later refinement) |
| **Containers** | System · Sequence · Perspective (nouns) | node-typed | `core/{systems,sequences,perspectives}.rs` | impl (all three types built; the *triad-ness* itself still unsettled) |
| **Query (Sort · Tag · Filter)** | **Tag (=)** reconciles **Sort (+)** and **Filter (−)**. *Everything in a view is a tag*; Sort prioritises the list by a tag, Filter adds/removes tags from the query. Each of Sort and Filter splits by the **by-key ↔ by-value** dyad (act on the tag's *key* = its type, or its *value*). | reconciler-typed (Tag is the whole; Sort +, Filter − its poles) | UI: `components/reference_browser.rs` (the Tag reconciler tree) | impl v1 — Sort-by-key + Filter-by-value live; Sort-by-value / Filter-by-key forthcoming |
| **Link / triple** | subject · predicate · object (`source · predicate · target`) | the edge itself | `core/perspectives.rs` `Link` | impl (AD4M) |

## Tetrad (order 4 — Sources / Interplays)

| system | terms / edges | typing | status |
|---|---|---|---|
| **Organise** | four *sources* (nouns) — **TBD**; operations *sort · tag · filter · search* | **superseded** — the operations are now read as the **Sort · Tag · Filter triad** (above), with *search* a separate free-text mechanism, not a fourth term | proposed → recast as a triad |

## Pentad (order 5 — Limits / Mutualities) — settled

| system | terms (nouns) | connectives (verbs) | code | status |
|---|---|---|---|---|
| **Architecture Pentad** | Sign · Symbol · Syntax · Semantics · Grammar | 10 Mutualities — instances: Number · Coherence · Functor · Colour · Meaning · Category-Theory · Geometry · Progression · Systematics · Render; classes: quantitative-match · aspiration · operation · … | seeded `system_architecture_pentad_5`; `docs/architecture-pentad.md` | impl + seeded |

## Scaffold / substrate

| system | note | code | status |
|---|---|---|---|
| Canonical run | the 12 canonical systems (monad→dodecad) — the systematic backbone | `data/canonical.json` | seeded |
| Functor | same-grammar morphism (an `S_n` permutation); the CT operations identity · composition · associativity sit behind it | `core/functors.rs` | impl |

## Nullad, Monad, and the core sequence

Three distinct things — do not conflate them:

- **Nullad (0) — the unbounded registry.** *Everything* in the tool. No scope.
  Best expressed as a **data view over an `all` query** — the raw registry.
  **UI: the (repurposed) reference browser** — a "Nullad" page (the view switch
  sits top-left of the header, before Monad). Search (free text) + **sort /
  filter as button facets** (perspective · source · artefact · order); each
  reference carries its **citation triad as tags** (source · locator · artefact),
  and the source/artefact tags double as filter buttons. The page also hosts the
  **ELT triad** (Load wired; Extract/Transform surfaced but unwired). Not a
  curated list. *(Still fed by `allReferences`, not yet a true `all` query.)*
- **Monad (1) — a scoped registry.** A bounded universe of inquiry with a
  **central point naming its unity** (e.g. "system architecture"), linking every
  graph of relevance: the class/instance dyad, the ALP and ELT triads, the pegged
  Pentad, other potential pentads/tetrads, and *implicit* knowledge-graph material
  not yet assembled. Raw material for sorting/assembly, **not** the core sequence.
  **Produced by Extract** (Nullad → Monad): a selection over the Nullad,
  materialized as a persisted `Sequence` — so the scope is a **real graph object**
  (id + members), not a transient client-side key. **UI: one entry on the current
  graph view** (that node not yet built); for now Extract confirms the created
  Monad by id.
- **Core sequence (1→12).** The systems of interest, produced **by performing
  operations on the monad** (sort → assemble → …). It *articulates* the monad; it
  is not the monad.

**Systems are lenses on the monad.** Each order is the *same* monad **reformulated
at a resolution**: the Pentad expresses its significance in 5 Limits + 10
Mutualities; a triad reformulates its core dynamics in 3 terms + 3 connectives;
and so on. Moreover a **node inside one reformulation can itself unfold into a
system** — e.g. `Sign` (a Pentad node) may unfold into a *dyad of number and
strings*. Everything is **fractal / holonic**: five tetrads in the monad may be a
single node in the core Pentad.

The live **Monad** is `sequence_architectural_monad` (seeded in
`backend/data/perspectives/architecture_monad.json`) — members may be **explicit**
(a resolvable `system:` address) or **implicit** (a dangling address = material
still to be assembled). Members now: the **Pentad** and the **Citation triad**
(explicit) and the **ELT triad** (`perspective:elt_triad`, implicit — documented
but not yet seeded as a graph object).

## Practice

Whenever we implement a systematic grouping (dyad / triad / tetrad / …), **add it
here** with: its terms (nouns → nodes), its connectives (verbs → edges), the code
location, and its status. Keep the code's names aligned to the schema. Companion
docs: `docs/architecture-pentad.md` (the pentad in depth) and
`docs/plans/architecture-run.md` (the 1→12 run, exploratory).
