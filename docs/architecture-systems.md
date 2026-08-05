# Systematic architecture registry

The codebase's language follows a **systematic schema**: every dyad, triad,
tetrad, … we implement *is* a system, and is documented here as one. This is the
map for auditing and refactoring the code back toward its systematic description
(dynamic homoiconicity) — when the codebase becomes a mess, come here.

**The typing rule** (the Dyad's force): a **term (vertex) is a noun**; a
**connective (edge) is a verb**. So a triad of *nouns* is node-typed (containers,
things); a triad of *verbs* is edge-typed (operations). Every entry below is
tagged accordingly.

### Proposal — typing & the operations [for sign-off]

**Apply the typing rule with no exceptions; drop "reconciler-typed."** When we
wrote "Tag (=) reconciles Sort (+) / Filter (−)" as a triad of *terms*, that was
mis-typed: **Sort, Filter, Tag, Search are verbs → they are edges (connectives),
not vertices.** **Impulse labels (+ / − / =) apply only to terms (vertices), NOT to
edges.** So a set of operation-verbs is a bundle of **edges** — a *fragment* of a
system whose **vertices (nouns) are not yet identified**. We store such fragments
and let the incomplete system show us what is and isn't figured out.

**The Hodgson architecture octad [proposed].** It is *not* an "operations octad" —
it is just the **architecture octad**, with **Critical Functions** as one node.
Hodgson's octad, going round from the east: **smallest unit · critical functions ·
supportive platform · necessary resources · integrative totality · inherent nature ·
intrinsic values · organisational modes**. A K₈ vertex has degree **7**, so the
seven operation-verbs **pair as edges incident to the Critical Functions node**.
Mapping so far (proposed):
- **sort** = Critical Functions ↔ Necessary Resources
- **filter** = Critical Functions ↔ Organisational Modes
- **validation** (the typing rule / Dyad's force) = another Critical-Functions edge,
  likely to **Inherent Nature** or **Intrinsic Values**
- **Citation** = the **Intrinsic Values** node itself (a *node*, not an edge) — so
  *validation* may be exactly the edge Critical Functions ↔ Citation.

This gives the operations (and citation) a home in the architecture octad to be
folded into, rather than free-floating "triads." Rest of the seven edges TBD.

**Sort/Filter, concretely.** Don't over-fix their semantics (they've churned): keep
them as **edge-fragments** with the current UI realisation (Sort selects header
tags; Filter scopes data by degree) as *one* working reading, and figure out the
settled operation as we assemble the octad. **Also pursue the sequence Data (monad)
→ key·value (dyad) → ELT (triad)** — data = tags; there is "something with sort ·
filter · data · key · value" to resolve alongside it.

**Status legend:** `impl` = in code · `seeded` = also a real System instance in
the graph · `proposed` = named but not built/settled.

## At a glance — settled vs open (review map)

**Settled (load-bearing):**
- The **Architecture Pentad** (Sign·Symbol·Syntax·Semantics·Grammar + 10 Mutualities) — seeded.
- **Everything is a tag** = the predicate:object of an **SPO** triple; a Perspective is a web of SPO links.
- **Nullad** (all elements) → **Extract** → **Monad** (a scoped subset, a real `Sequence`) → **core sequence** (articulates the monad).
- **Goal = self-construction:** track each feature as a **fragment** in the Monad, then **fold it in**. Self-documentation is the first port of call.
- **Determining conditions** (six laws over Time·Hyparxis·Eternity·Space) = *what the system exists to codify*.

**Open / unresolved (do not assert):**
- **Typing [proposal pending sign-off]:** the noun=vertex / verb=edge rule holds with **no exceptions**; "reconciler-typed" is dropped. The operation-verbs (search·sort·filter·tag + extract·load·transform) are **edges**, a fragment whose vertices are TBD — plausibly the **7 edges at the "Critical Functions" node of the Octad**. See *Proposal — typing & the operations*.
- **Sort/Filter semantics** stay as edge-fragments (latest UI reading: Sort = select header tags, Filter = scope data by degree); the **by-key/by-value** dyad is under review; pursue the **Data → key·value → ELT** sequence. The **symmetric-doubling** worked example that used by-key/value is superseded.
- **SPO ↔ category-theory ↔ systematics** are three *interchangeable* mediums (none primary); switching needs representational transforms — **not built**.
- **Folding-in = CT-style composition** — not built (needs CT represented in-graph).
- **Scoping shape:** order × degree, 3-D+ (DU1 geometry); order/degree terminology unsettled.
- **Determining conditions:** hexad-of-laws vs tetrad-with-law-edges vs both.
- **Perspective** may be retired into the SPO substrate.
- **ELT may *be* the Query triad** (Load = select keys); Data(monad)→key·value(dyad)→ELT(triad) unresolved.
- **Seeding is code-defined, not app-created [cleanup question].** "Seed" here means
  **both**: a system is *defined in Rust tables* (`data/mod.rs`
  `build_*_from_tables` + `push_triadic_system`) → serialized to a *JSON data file*
  (`data/{canonical,citation,fragments}.json`, via ignored regen tests) → *loaded
  into the graph* at startup (`build_graph` `apply_content`). So systems **are placed
  in the system** (resolvable, visible) but their **source of truth is code**, not
  data authored *through* the app. The homoiconic / self-construction goal wants
  systems **created and edited as data in-app** (an `createSystem`-style flow /
  compose-via-Extract), retiring the Rust generator. That is the "big cleanup" —
  scoped, not yet done. (User data — Monads from Extract — already takes the
  app→`store.json` path.)

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
  The architecture Monad tracks the Pentad + Citation triad (seeded) and the ELT,
  Sort·Tag·Filter, Data·Graph·Table, Class·Instantiation·Instance triads +
  by-key/by-value dyad (dangling). Loop: build a feature → track it as a Monad
  fragment → later fold it in. **Author-contributed and systematics-core fragments
  live in [`fragments.md`](fragments.md)** (the Aesthetics·Harmony·Maths triad; the
  determining-conditions tetrad) — now **seeded as real systems**, still to be
  *placed* (folded into a whole).
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

**Scoping is multi-dimensional [proposed — more than a matrix].** Locating a
specific element is a relation of at least **order** (the system name, 1→12) and
**degree** (term-designation / connective-designation / coherence / term /
connective / …); within that combined scope a specific term or connective
character is pinned. This is **not** a flat 2-D matrix — it is **3-D or higher**
(the DU1 six-dimensional geometry). If "order = number of vertices" and "degree =
connectivity" (terminology **unsettled**), the combined scope may be their
**product**. Build the data-view (sort/filter) scoping first; the graph view comes
after.

**A tag *is* the predicate:object of a triple.** Subject·Predicate·Object is the
core triple for codifying *any* language. The S/P/O are the abstract keys; their
values make a statement — e.g. subject=`will`, predicate=`generates`, object=`function`
→ the generic triad **will-generates-function**. A tag = the predicate:value on a
node; so nodes, edges, systems, coherence, perspectives are *all* tagged triples. A
**Perspective** is just a web of SPO links — which is why it may be retired as a
separate container and kept only as the SPO substrate (arguably *everything*).

**Definition — a Perspective is a bundle of Subject·Predicate·Object labels
[proposed].** It is **self-referential** (a Perspective can be referenced within
itself). This is likely a **tetrad: Perspective · Subject · Predicate · Object** —
constructible in **AD4M** and **recursively modelled**: the *subject→predicate* link
is itself a triple `subject(subject) · predicate · object(predicate)`, and the
*predicate→object* link likewise `subject(predicate) · predicate · object(object)`,
and so on (the Prolog engine unfolds it). So **AD4M's core architecture can be
represented as a tetrad within AD4M itself** — a concrete instance of the
self-documentation → self-construction goal.

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

- **Monad → Dyad** — the whole splits into two poles.
- **Dyad → Tetrad** — each pole splits again — four leaves, still symmetric.
- **Tetrad → Octad** — each leaf splits once more, retaining symmetry.

*(The earlier worked example — Tag → {Sort, Filter} → {by-key, by-value} — is
**superseded**: Sort/Filter were since redefined as header-tags / data-scoping, and
the by-key/by-value dyad is under review. The system-level example below is the
stable one.)*

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
| **Operations (ELT)** | Extract · Load · Transform (verbs) → `createSequence` · `loadPerspective` · `applyFunctor`. **[open] ELT may BE the sort/filter triad** — *Load = selecting the keys to display values* (i.e. Sort = header tags). If so there is a sequence **Data (monad) → key·value (dyad) → ELT (triad)** — "data" being what we call *tags* (everything is a tag ⇒ it is just data). Recorded, not resolved. | **edge-typed** (all verbs) | `graphql/types.rs`; UI: `components/reference_browser.rs` `elt_triad` (Nullad page) | impl (not seeded). **Extract** is wired (Nullad → Monad): materializes the current data-view selection (distinct `system:<id>` of the filtered references) into a persisted Monad via `createSequence` + `create_sequence` (client). **Load** is wired (`loadPerspective` → `on_load`). **Transform** (apply a Functor) is surfaced but **not yet wired**. Monad auto-naming is provisional (the members' *integral* is a later refinement) |
| **Containers** | System · Sequence · Perspective (nouns) | node-typed | `core/{systems,sequences,perspectives}.rs` | impl (all three built; triad-ness unsettled). **Open:** *Perspective* may be retired — or kept only as the subject·predicate·object **Link** substrate, which is arguably *everything* in the system (every tag is a `key:value` predicate on a node). |
| **Query (Sort · Tag · Filter)** | **Tag (=)** reconciles **Sort (+)** and **Filter (−)**, mapping to **class / instantiation / instance**. **Sort = selecting the header tags** — which tag keys are the columns (the *class*: header/keys; `ColKey`; default Order + Citation). **Filter = scoping the data returned** in those columns (the *instances*: values), by **cite-degree** — the data categorised by number per the schema **1 term-designation · 2 connective-designation · 3 coherence · 4 term · 5 connective · 6 system** (their coalescence). So you can view only systems (*manifolds* = systems not yet placed), only terms, only connectives, etc. | reconciler-typed | UI: `components/reference_browser.rs` (`CiteKind`; Sort = column selector, Filter = degree scoper) | impl v1. Citation column in **Source · Artefact · Lookup** order |
| **Data (Data · Graph · Table)** | **Data (=)** is the content the header scopes; **Graph (+)** and **Table (−)** are its two views. The switch (right of the header menu) chooses one. | reconciler-typed (Data is the whole; Graph/Table its views) | UI: `components/system_selector.rs` (`ViewMode`) | impl — Table live; Graph = per-system K-graph (Nullad Graph = the future all-graph) |
| **Class · Instantiation · Instance** | e.g. **K₄ (class) · Tetrad (instantiation) · Canonical Tetrad (instance)**. The abstract complete graph → its systematic instantiation → a concrete instance. Also the table's own structure: header/keys (class) · data types/keys (instantiation) · data/values (instance). | node-typed (nouns) | (documented; extends the existing Class/Instance dyad) | proposed |
| **Link / triple** | subject · predicate · object (`source · predicate · target`) | the edge itself | `core/perspectives.rs` `Link` | impl (AD4M) |

## Tetrad (order 4 — Sources / Interplays)

| system | terms / edges | typing | status |
|---|---|---|---|
| **Determining Conditions** ⭐ | nodes **Time (Chronos) · Hyparxis · Eternity (Aionios) · Space** (the four "dimensions"); the **six laws as its six K₄ edges** — *statistical · correspondence · classification · conservation · irreversibility · coexistence*. **This is what the whole system is trying to codify** as a representational medium. | nodes=nouns, edges (laws)=the determining conditions | seeded `system_determining_conditions_4`; `docs/fragments.md`; [[determining-conditions]] | seeded (edge→law assignment TBD; may *also* be a hexad of the laws as nodes) |
| ~~Organise~~ | four *sources* (nouns) — TBD; operations *sort · tag · filter · search* | **superseded** — the operations are now the **Sort · Tag · Filter triad** (below); *search* is a separate free-text mechanism, not a fourth term | discarded |

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
  **[done]** The Nullad Table now shows **all elements as rows** — every **System**
  (via `instanceSystems`, incl. the seeded fragments) *and* every **Reference** —
  unified as `Row::Sys | Row::Ref`, with a **Name** column. **Load** is now a
  **file picker** (opens the OS browser for a JSON system file; import format still
  TBD) — no longer a list of internal systems. *(Not yet a single backend `all`
  query — the frontend unions systems + references.)*
- **Monad (1) — the scoping/filter of the Nullad.** A bounded sub-universe: a Monad
  selects a subset of the Nullad and **invokes an organising principle** with a
  **central point naming its unity** (e.g. "system architecture"). It is **raw
  material** for sorting/assembly (fragments + implicit material), **not** the core
  sequence. **Produced by Extract** (Nullad → Monad): the selection is materialised
  as a persisted `Sequence` — a **real graph object** (id + members), not a transient
  client key. **UI:** one node on the graph view (not yet built; for now Extract
  confirms the created Monad by id). **Graph view is deferred** until the data-view
  sort/filter params settle; a *Nullad* graph condenses a Hilbert-space-like density
  (SPO-triads-of-SPO-triads) — likely needs 3+ dimensions. Table↔Graph should
  **share scope/selection**.
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
still to be assembled). **7 members:** the **Pentad** + **Citation triad** (explicit,
seeded) and — dangling — the **ELT**, **Sort·Tag·Filter**, **Data·Graph·Table**,
**Class·Instantiation·Instance** triads and the **by-key/by-value** dyad. The
author + systematics-core fragments (`docs/fragments.md`) are now **seeded as real
systems** (`system_aesthetics_harmony_maths_3`, …, `system_determining_conditions_4`)
but not yet added as Monad members / placed.

## Practice

Whenever we implement a systematic grouping (dyad / triad / tetrad / …), **add it
here** with: its terms (nouns → nodes), its connectives (verbs → edges), the code
location, and its status. Keep the code's names aligned to the schema. Companion
docs: `docs/architecture-pentad.md` (the pentad in depth) and
`docs/plans/architecture-run.md` (the 1→12 run, exploratory).
