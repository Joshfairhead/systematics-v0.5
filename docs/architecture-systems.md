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

## Dyads (order 2 — Poles / Force)

| system | poles (nouns) | force (verb) | code | status |
|---|---|---|---|---|
| Language | **Grammar ↔ Vocabulary** | *reconciled by* System | `core/{grammar,vocabularies,systems}.rs` | impl |
| Class/Instance | **canonical ↔ instance** | *instantiates / overrides* | `renderSystem.canonicalClass` (`graphql/types.rs`) | impl |

## Triads (order 3 — Impulses / Acts)

| system | terms / role | node- or edge-typed | code | status |
|---|---|---|---|---|
| **Citation** | Source · Artefact · Lookup (nouns); edges *recordedIn · atLocation · cites* (verbs) | nouns=nodes, verbs=edges | `core/citations.rs`; seeded `system_citation_3` | impl + seeded |
| **Operations (ELT)** | Extract · Load · Transform (verbs) → `exportPerspective` · `loadPerspective` · `applyFunctor` | **edge-typed** (all verbs) | `graphql/types.rs` | impl (not seeded) |
| **Containers** | System · Sequence · Perspective (nouns) | node-typed | `core/{systems,sequences,perspectives}.rs` | impl (all three types built; the *triad-ness* itself still unsettled) |
| **Link / triple** | subject · predicate · object (`source · predicate · target`) | the edge itself | `core/perspectives.rs` `Link` | impl (AD4M) |

## Tetrad (order 4 — Sources / Interplays)

| system | terms / edges | typing | status |
|---|---|---|---|
| **Organise** | four *sources* (nouns) — **TBD**; operations *sort · tag · filter · search* (verbs) | verbs are edges; noun-terms are the gap | proposed (see `docs/plans/architecture-run.md`) |

## Pentad (order 5 — Limits / Mutualities) — settled

| system | terms (nouns) | connectives (verbs) | code | status |
|---|---|---|---|---|
| **Architecture Pentad** | Sign · Symbol · Syntax · Semantics · Grammar | 10 Mutualities — instances: Number · Coherence · Functor · Colour · Meaning · Category-Theory · Geometry · Progression · Systematics · Render; classes: quantitative-match · aspiration · operation · … | seeded `system_architecture_pentad_5`; `docs/architecture-pentad.md` | impl + seeded |

## Scaffold / substrate

| system | note | code | status |
|---|---|---|---|
| Canonical run | the 12 canonical systems (monad→dodecad) — the systematic backbone | `data/canonical.json` | seeded |
| Functor | same-grammar morphism (an `S_n` permutation); the CT operations identity · composition · associativity sit behind it | `core/functors.rs` | impl |

## The live registry — the Architectural Monad

The in-app, live form of *this document* is the **Architectural Monad**
(`sequence_architectural_monad`, seeded in
`backend/data/perspectives/architecture_monad.json`) — a `Sequence` holding the
**declared architecture systems**. It is the bounded universe of inquiry: it
contains everything of relevance (structured and unstructured, floating dyads and
triads that later resolve into higher systems), as a list of members. Currently:
the Architecture Pentad. Undeclared orders are simply absent (blank in the view).

## Practice

Whenever we implement a systematic grouping (dyad / triad / tetrad / …), **add it
here** with: its terms (nouns → nodes), its connectives (verbs → edges), the code
location, and its status. Keep the code's names aligned to the schema. Companion
docs: `docs/architecture-pentad.md` (the pentad in depth) and
`docs/plans/architecture-run.md` (the 1→12 run, exploratory).
