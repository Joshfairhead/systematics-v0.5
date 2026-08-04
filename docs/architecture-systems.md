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

## Self-documentation → self-construction [settled goal]

Self-documentation is only the **first port of call**. The real goal is to **use
the system to construct itself** (bootstrap / self-hosting).

- **Document its own construction.** Every dyad / triad / … we build is registered
  here *and*, as far as possible, **represented in the graph itself** — so filtering
  the tool by *architecture / project* surfaces these systems and their relationships
  **without recourse to any out-of-system document**. This markdown registry is the
  interim; the app *is* its own documentation (**dynamic homoiconicity**). As the
  model changes in conversation, this updates in step.
- **Track features as fragments, then fold them in.** A **fragment** is a collection
  of isolated elements *as a unit* — a face in *some* graph (a triadic fragment = a
  triangle in an as-yet-unknown K₄/K₅). It is a real graph object (a dyadic fragment
  *is* a dyad) whose **whole is not yet known** — a dyad may belong in a pentad, a
  pentad may be a partial octad. We track the relation as *a* system until we realise
  its place. Fragments float in the **Architectural Monad** (the raw-material pool),
  tracked as **Monad members**: seeded ones resolve; not-yet-placed ones dangle.
  Current tracked fragments: the Pentad + Citation triad (seeded), and the ELT triad,
  **Sort·Tag·Filter triad**, and **by-key/by-value dyad** (dangling). Loop: build a
  feature → track it as a Monad fragment → later fold it in.
- **Folding in = category-theory-style composition [proposed, large].** Assembly is
  not built and needs real work: we must first **represent category theory in the
  system as a graph** — a triad of *identity · composition · association* (possibly
  coalesced with the SPO triad into a hexad — unsure) — before we can compose
  fragments into higher systems properly.

## Everything is a tag [proposed — working principle]

In a view, **every attribute of every element is a tag** — a `key : value` pair
(perspective, source, locator, artefact, order, coherence, designation, …). This is
why the query controls are one triad rather than separate features: **Tag (=)
reconciles Sort (+) and Filter (−)**. Sort prioritises the list by a tag; Filter
adds/removes tags from the query. Because a tag is `key : value`, each splits along
the **by-key ↔ by-value** dyad.

**A tag *is* the predicate:object of a triple.** Subject·Predicate·Object is the
core triple for codifying *any* language. The S/P/O are the abstract keys; their
values make a statement — e.g. subject=`will`, predicate=`generates`, object=`function`
→ the generic triad **will-generates-function**. A tag = the predicate:value on a
node; so nodes, edges, systems, coherence, perspectives are *all* tagged triples. A
**Perspective** is just a web of SPO links — which is why it may be retired as a
separate container and kept only as the SPO substrate (arguably *everything*).

**Three interchangeable mediums [proposed — corrected].** SPO, category theory, and
systematics are **not** a base-truth stack — none is primary. They are related and
**mutually expressive**: each can express the others, and which is "primary" is a
choice of perspective. A plausible reading: **CT = receptive, SPO = reconciler,
systematics = affirmation**. Switching between them will likely require representing
each object **three ways** — a **commuting triangle** in CT, an **SPO triple** in
English, and a **triad** in systematics — with **representational transforms**
between the three. (Linking semantics to a topological node reads as CT *associates*
— "will *associates* node-1" — not "couples".) So Perspective is the *medium*, but
the medium carries richer semantics extended by category theory.

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
| **Containers** | System · Sequence · Perspective (nouns) | node-typed | `core/{systems,sequences,perspectives}.rs` | impl (all three built; triad-ness unsettled). **Open:** *Perspective* may be retired — or kept only as the subject·predicate·object **Link** substrate, which is arguably *everything* in the system (every tag is a `key:value` predicate on a node). |
| **Query (Sort · Tag · Filter)** | **Tag (=)** reconciles **Sort (+)** and **Filter (−)**, mapping to **class / instantiation / instance**. **Sort = selecting the header tags** — which tag keys are the columns (the *class*: header/keys; `ColKey`; default Order + Citation). **Filter = scoping the data returned** in those columns (the *instances*: values) — currently by **cite-degree** (`CiteKind`: System / Node(1) / Edge(2) / Coherence / Designation), so you can view only systems (manifolds), or only nodes, etc. | reconciler-typed | UI: `components/reference_browser.rs` (Sort = column selector, Filter = degree scoper) | impl v1. Citation column in **Source · Artefact · Lookup** order |
| **Data (Data · Graph · Table)** | **Data (=)** is the content the header scopes; **Graph (+)** and **Table (−)** are its two views. The switch (right of the header menu) chooses one. | reconciler-typed (Data is the whole; Graph/Table its views) | UI: `components/system_selector.rs` (`ViewMode`) | impl — Table live; Graph = per-system K-graph (Nullad Graph = the future all-graph) |
| **Class · Instantiation · Instance** | e.g. **K₄ (class) · Tetrad (instantiation) · Canonical Tetrad (instance)**. The abstract complete graph → its systematic instantiation → a concrete instance. Also the table's own structure: header/keys (class) · data types/keys (instantiation) · data/values (instance). | node-typed (nouns) | (documented; extends the existing Class/Instance dyad) | proposed |
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

- **Nullad (0) — the unbounded registry.** *Everything* — every element (all
  "cites": systems, nodes, edges, coherence, designations) is an entry here. It is
  the **default entry point** (order-0 button leading Nullad · Monad · … · Dodecad,
  opening the **Table** so you see all on load). **Sort** selects the header tags;
  **Filter** scopes by cite-degree; the **Data · Graph · Table** switch (right of
  the menu) chooses the view; the page hosts the **ELT triad**.
- **Monad = the scoping/filter of the Nullad.** A Monad selects a subset of the
  Nullad and **invokes an organising principle** based on some relationship (those
  members may in turn have further relationships, visualised or not depending on
  scope). Produced by **Extract**. **Next:** a **Graph view** beside Table — the
  Obsidian-style force graph of everything (nodes + SPO links); the Monad is that
  graph with a selected set of fragments grouped and labelled on its own canvas.
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
