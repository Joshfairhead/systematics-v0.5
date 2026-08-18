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
Mapping so far (proposed) — **all edges incident to Critical Functions (CF)**:
- **sort** = CF ↔ Necessary Resources
- **filter** = CF ↔ Organisational Modes
- **citation** = CF ↔ **Intrinsic Nature** (the *top* node, sometimes "esoteric
  source") — an **edge**, not a node.
- **validation** (the typing rule / Dyad's force) = CF ↔ **Inherent Nature** (top
  left) — an **edge**. *Citation and validation are two separate edges.* (Alt
  reading: if citation is taken as the top *node*, validation = CF ↔ Intrinsic
  Values — but the settled-here reading is both are edges.)
- **edit** (edit a system in-place — name, node/edge labels — a critical function)
  = CF ↔ **Smallest Holonic Unit** (the east / "right" node). Edit is a
  *transformation*, so it also relates to the **Transform** node/operation.
- **tags** = **Smallest Holonic Unit ↔ Organisational Modes** (an edge).
- **types** = **Organisational Modes ↔ (the largest whole)** (an edge). So
  **Types · Tags** reads as a dyad — the two poles that ELT's data sits between.
- **compose** — an operation whose octad home is TBD; tracked in the Architecture
  Monad for now (`perspective:compose`), alongside **edit** (`perspective:edit`).

Rest of the seven CF-edges TBD. This gives the operations (and citation/validation)
a home in the architecture octad to be folded into, rather than free-floating "triads."

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
- **The architecture IS an MVC triad — Model (−, the base space) · View (+, the interface) ·
  Controller (=, the six laws of three).** Separate the concerns; the base-space remodel is
  the **Model**; the Controller is the *six laws*, not bare SPO (which is only the 132 /
  interaction law). See *The architecture is an MVC triad*. (user, 2026-08-18)
- The **Architecture Pentad** (Sign·Symbol·Syntax·Semantics·Grammar + 10 Mutualities) — seeded.
- **Everything is a tag** = the predicate:object of an **SPO** triple; a Perspective is a web of SPO links.
- **Nullad** (all elements) → **Extract** → **Monad** (a scoped subset, a real `Sequence`) → **core sequence** (articulates the monad).
- **Goal = self-construction:** track each feature as a **fragment** in the Monad, then **fold it in**. Self-documentation is the first port of call.
- **Determining conditions** (six laws over Time·Hyparxis·Eternity·Space) = *what the system exists to codify*.
- **Sequence-context navigation (built):** enter a monad → the header order-buttons step its **members** by order (Monad(CT)→Dyad loads Container·Operations). Sequences are a filterable element kind.
- **Monads are core-sequences OR buckets (built):** an ordered core-sequence order-steps with the unreachable orders **greyed out**; a **bucket** (several members of one order, e.g. the Architectural Monad's triads) **scopes the Table to its members for sorting**. Monad rows carry a **delete ✕** (`deleteSequence`).
- **Order·Position·Location** seeded as the topological anchor (order × position = location).
- **Sort/Filter reconciler = Data / Perspective** (not "System" — too precise): sort/filter over *any* datum in a system (term · connective · designation · reference · artefact).

**Open / unresolved (do not assert):**
- **Typing [proposal pending sign-off]:** the noun=vertex / verb=edge rule holds with **no exceptions**; "reconciler-typed" is dropped. The operation-verbs (search·sort·filter·tag + extract·load·transform) are **edges**, a fragment whose vertices are TBD — plausibly the **7 edges at the "Critical Functions" node of the Octad**. See *Proposal — typing & the operations*.
- **Sort/Filter semantics** stay as edge-fragments (latest UI reading: Sort = select header tags, Filter = scope data by degree); the **by-key/by-value** dyad is under review; pursue the **Data → key·value → ELT** sequence. The **symmetric-doubling** worked example that used by-key/value is superseded.
- **SPO ↔ category-theory ↔ systematics** are three *interchangeable* mediums (none primary); switching needs representational transforms — **not built**.
- **Folding-in = CT-style composition** — not built (needs CT represented in-graph).
- **CT axioms ↔ systematics:** axioms-as-edges is **cooled off** (user, 2026-08-10); no favoured mapping. **Traversal logic** now has a working home — see *The reference tuple*: content-addressing = **identity**; the **six laws (S₃) = the directed readings/traversals** of a referenced triple (the associative-lookup layer). Firmer than the CT-edge reading; still proposed.
- **The reference-tuple store [recommended, awaiting sign-off]:** `location · key · value · source`, every column an anchor → reciprocal key↔value lookup + provenance = the Data Object store = content-addressing done right = the Holochain/AD4M shape, built in-graph now (no runtime port). Nullad = the content-addressed heap; a Monad = a scoped "universe" over it.
- **The view's reconciler:** revised to **Systems (=) / Sort (+) / Filter (−)** (compose the view *from* systems) — supersedes "Tag (=)"; the **metadata-dodecads-as-rules** data model (4 order-12 systems → match → RAG operation) is proposed, not built.
- **The location web:** Order·Position·Location + Location·Term·Source + Lines(=Location·Connection·Location) is a **mixed fragment that may fold into a pentad** — containing whole unknown.
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
  systems **created and edited as data in-app**, retiring the Rust generator.
  **First step done:** the **`authorSystem`** mutation + the **Editor** form build a
  system from custom term/connective values at runtime and persist it (it renders
  and appears in the Nullad table). Remaining cleanup: migrate the code-defined
  seeds (fragments first, canonical last) to app/JSON-authored data. (User data —
  Monads from Extract — already takes the app→`store.json` path.)

## The architecture is an MVC triad — Model (−) · View (+) · Controller (=) [user, 2026-08-18]

The whole architecture **is** a triad, and it is now the lens that organises the base-space
remodel. Naming the impulses (canonical: affirming **+**, receptive **−**, reconciling **=**):

| MVC role | impulse | what it is | in this codebase |
|---|---|---|---|
| **Model** | **−** (receptive) | the **triadic base space** — the passive store of *what is* | backend data + the base-space triad below |
| **View** | **+** (affirming) | the **interface** — what actively presents / initiates interaction | frontend (`browser_controls`, `inspector`, `graph_view`) |
| **Controller** | **=** (reconciling) | the **algorithm / law layer** mediating View ↔ Model | middleware + the six laws of three (below) |

This supersedes the earlier loose "middleware = controller, backend = model, frontend = view"
gloss by giving each an **impulse** and naming what each *contains*.

**Separate the concerns — the earlier remodel conflated them (user, 2026-08-18).** The
base-space plan folded model, controller and view together. Pulled apart:

- **Model (−) — the triadic base space (decoupled).** A system is defined triadically:
  **Structural Topology (−)** (vertices + edge-pairs → adjacency matrix) · **Graph Template
  (=)** (order + size = the grammar; reconciles adjacency ↔ incidence as a line graph) ·
  **Semantic Projection (+)** (term + connective characters → incidence matrix). *This is the
  model only* — it holds no algorithms and no interface.
- **Controller (=) — the six laws of three, not bare SPO.** The algorithm side is what we
  have called **SPO, extended** — but **SPO is only one of the laws.** Reading
  Subject·Predicate·Object as impulses — **subject = affirming (Will, +)** · **predicate =
  reconciling (Being, =)** · **object = receptive (Function, −)** — gives the ordering
  **1-3-2 = interaction**. There are **six laws** over the triad (the S₃ permutations, table
  under *Monads, operations, and the six laws of three*); **SPO is just the *interaction*
  one** (consistent with the prior "SPO ≈ interaction (132)" note). **Graph traversal,
  storage and the semantic product want the full six**, not the single 132 reading — so the
  controller **is the six laws of three**, and SPO is retired as *the* model, kept only as
  one traversal.
- **View (+) — the interface.** The two-line **sort/filter** redesign lives here (the
  `browser_controls` module); it is the affirming face, decoupled from model and controller.

**The Graph Template holds the Controller's validation rules [insight — note, user 2026-08-18].**
The base space's reconciler is the **Graph Template (=)** (order + size = the grammar). The
Model as a whole is the **receptive (−)** pole of MVC, yet its *internal* reconciler (the
Graph Template =) **corresponds to the Controller (=) one level up** — the reconciling role
recurs **holonically**, and the two should be connected. Concretely: **order and degree are
Model data, held in the Graph Template**, and the Graph Template is where **validation rules
for the Controller** belong — the grammar prescribes which topologies/projections are legal
(how many terms, which edges), so it *is* the constraint the controller checks. So the Graph
Template feeds the Controller its validation rules. *(This gives the deferred item "order +
degree as constraint-values" a home: the Graph Template.)*

**Balanced development via the six laws [RULE — user, 2026-08-18].** The six laws (S₃
permutations of the impulse triad) are the six ways three impulses interact. **Use them as
the development lens on any triadic base, including MVC:** don't build only the one
interaction we default to (132 / SPO) — check the work across **all six** permutations so
that Model, Controller and View develop in balance. A proposed reading of the six over the
MVC base (View = **+**/1, Model = **−**/2, Controller = **=**/3), for sign-off:

| law (order) | reading over MVC (positions View=1 · Model=2 · Controller=3) |
|---|---|
| expansion (123) | View → Model → Controller |
| identity (231) | Model → Controller → View |
| order (312) | Controller → View → Model |
| interaction (132) | View → Controller → Model  ← the SPO / request reading |
| concentration (213) | Model → View → Controller |
| freedom (321) | Controller → Model → View |

*(The **rule** "check all six for balance" is settled; each row's concrete meaning is a
working guess. Note: this six-over-MVC application is **distinct** from the six laws as the
Controller's own **traversal algorithms** over the S·P·O impulse triad — same group S₃,
different base.)*

## Monads, operations, and the six laws of three [proposed — potentially major]

**A Monad is a container in category theory** — *context management*: a **container**
(dyadic: the wrapped value + an operator) plus **operations** governed by the three
monad laws — **identity · composition (bind) · associativity**.

**There are exactly six operations = the six permutations of a triad = Bennett's
"six laws of three."** A triad's three positions permute in 3! = **6** ways — the
six relational dynamics of the triad. Bennett names them **identity · freedom ·
concentration · expansion · interaction · order**. (This is why "Operations" is a
triad-derived set, not an arbitrary list.)

**ELT ≅ RAG.** Extract·Load·Transform (data-warehouse) and Retrieve·Augment·Generate
(generative-AI) are the **same isomorphic triangle** — both are **composition**. So
the **Operations** triad (renamed from ELT) links to *composition*; its operations
are permutations of the triad.

**The assignments (user, 2026-08-06)** — each ordering of the triad's positions →
one law:

| order | law | permutation type |
|---|---|---|
| 123 | **expansion** | identity perm `e` (even) |
| 231 | **identity** | 3-cycle (even) |
| 312 | **order** | 3-cycle (even) |
| 132 | **interaction** | transposition (odd) |
| 213 | **concentration** | transposition (odd) |
| 321 | **freedom** | transposition (odd) |

So the six laws **are** the symmetric group **S₃**. Even/odd splits them 3+3
(rotations {expansion, identity, order} vs reflections {interaction, concentration,
freedom}). Two clean 3-pairings fall out: by **reversal** (123↔321
expansion/freedom · 132↔231 interaction/identity · 213↔312 concentration/order) or
by **shared leading term** (start-1 {expansion, interaction} · start-2 {concentration,
identity} · start-3 {order, freedom}).

**Hypothesis [flagged as potentially big — NOT verified].** One of those 3-pairings
maps onto **category theory's three axioms — identity · composition · associativity**
— turning each CT axiom into a **dyad of JGB laws**: a homoiconic CT↔Bennett bridge
at the foundations, carrying us into the **Tetrad (Time · Space · Hyparxis ·
Eternity)** + its **six edges** (Function→Being). **Note the mismatch to resolve**:
the *group* identity is `123` yet Bennett labels that **expansion**, and calls the
3-cycle `231` **identity** — so JGB "identity" ≠ CT/group identity. Record and pursue;
do not assert.

**The S₃ multiplication table** (row ∘ col, one-line-notation composition; verified):

|        | exp | idn | ord | con | fre | int |
|--------|-----|-----|-----|-----|-----|-----|
| **exp** (123) | exp | idn | ord | con | fre | int |
| **idn** (231) | idn | ord | exp | fre | int | con |
| **ord** (312) | ord | exp | idn | int | con | fre |
| **con** (213) | con | int | fre | exp | ord | idn |
| **fre** (321) | fre | con | int | idn | exp | ord |
| **int** (132) | int | fre | con | ord | idn | exp |

The **rotations {exp, idn, ord}** form the cyclic subgroup A₃; the **reflections
{con, fre, int}** are the three transpositions. **exp = the group identity `e`.**

**CT-axioms-as-EDGES [user's reframe, more promising than pair-grouping].** Rather
than mapping the six laws to three axiom-*pairs*, map the three CT axioms to the
**three edges of the FBW triad** (generation · consent · decision):
- **composition = generation**
- **associativity = consent**
- **identity = decision**

So the axioms live on the **edges**; the six laws are the **traversals** (S₃) of the
triad — plausibly the **functorial semantics** of how composition applies. Note this
makes CT-identity = the *decision* edge — **not** JGB's "identity" law (231), matching
the confirmed mismatch. [Promising; unverified.]

**Cooled off (user, 2026-08-10):** *"I'm not sure about the CT axioms as edges."* The
edge mapping is **no longer favoured** — treat it as one candidate, not the working
model. Both the axioms-as-pairs and the axioms-as-edges readings stay parked.

**Traversal logic is the open problem [user, 2026-08-10].** Whatever CT ends up mapping
to, *how you walk a sequence/graph* is still unspecified. Entering a monad and stepping
its members by **order** (Monad→Dyad→Triad) is the first concrete traversal we built
(sequence-context navigation, `app.rs`), but the general rule — how the six laws (S₃)
act as *moves* over a triad/K_n, and how folding-in composes those moves — is **not
settled**. This is the next thing to figure out, ahead of committing to any CT bridge.

## Representation medium — AD4M, directed dyads, the reference triple [proposed]

Rethinking the representation model (user, 2026-08-06):

**AD4M codifies a DYAD, not a triad.** A perspective link is *first-node → predicate →
second-node* = **subject → predicate → object** = a **directed dyad**. SPO on its own is
a triangle with **one subject node and two object nodes** — it cannot *be* a clean triad
by itself.

**Chaining SPOs into a triad is problematic [tension — user, 2026-08-06].** Earlier we
read a triad as a chain of directed SPO dyads (object-of-one = subject-of-next). But a
**triad edge is UNDIRECTED**: Will→Function and Function→Will share the *same* edge
(*generation*). A directed SPO flow can't cleanly reconstruct an undirected triad — so
"chain through shared middle terms" is not settled. **AD4M (directed dyads) cannot do
loops**; closing a K3 needs an extra hop, and directionality fights the triad's symmetry.
Take **FBW** (Function·Being·Will) with edges **generation · consent · decision**; the
six laws over each node-edge-node give **18 readings** (3 edges × 6 permutations) — e.g.
expansion *"will generates function which consents to being"* — reading as **chains of
~5 entries** (*will–generation–function–consent–being*). [Captured; unresolved.]

**Which law is SPO? [revised — user leans *interaction*].** Prior note said SPO =
expansion (1-2-3). Latest: **SPO ≈ interaction (132)** — subject *affirms* (+),
*reconciled through* the predicate (=), *with* the object (−); *"language is always an
interaction."* (Still hedged; expansion vs interaction not final.)

**The reference system is triadic — the *Data Object*: key (+) · value (−) · reference
(=)** (seeded `system_data_object_3`; likely a Perspective once sequenced +=− / 132
interaction). A **key holds many values**, each with its own **reference** — you
reference the *value in relation to its key* (the key names the type/grouping, e.g.
`coherence`). So a Pentad takes its **coherence** ("Quintessence") from **DU1** while its
**terms** come from **DU2/DU3** — each element independently referenced; conflicts →
assemble a **compound system** citing all three. (The shape behind #25/#28.) The user is
**not sold on SPO** as the only shape — the reference may be a **variable-length chain**
(*element · key · value · reference*), not just a triple; the reference triple should be
implemented now even without composability. **CT as the substrate** is attractive but
implementation is open.

**A monad needs structure AND process.** A category-theory monad is not a real monad
without **operations** (and they must obey **identity · composition · associativity**).
Open question: are those three laws themselves **given structure** (a triad) or held
loosely? Structure and process **cannot be purely partitioned** — a process needs
structure; structure is composed through process — so **something reconciles them**, and
that reconciler looks like the identity/composition/associativity triad itself. [Open.]

**DU1 categories = a horizontal Dodecad [Q2].** *What DU1 actually says:* the seed
(`du1_perspective.py`) takes DU1 ch.2 **"The Progression of the Categories"** — its
**12 systemic attributes** (Wholeness · Polarity · Relatedness · Subsistence ·
Potentiality · Repetition · Structure · Individuality · Pattern · Creativity ·
Domination · Autocracy) — and maps each onto our **`coherence`** field. So **DU1 calls
them "categories / systemic attributes," not "coherence"** — mapping them to *coherence*
is **our** interpretation (a reasonable one: the category is what coheres the order),
not DU1's word. The **canonical** systems currently carry *different* coherences
(tetrad = "Activity Field"; DU1 = "Subsistence") that are **not definitively
referenced** — so canonical may be wrong; if DU1 proves foundational, promote DU1's
categories to canonical. **The elegant structural fact [confirmed]:** the 12 coherence
values, one per order, ARE a **Dodecad** — a *horizontal* 12-term sequence running
across the header/coherence row (order 1→12). So "the coherence attribute" is itself an
order-12 system whose terms are the categories. **[Structurally sound; adopt if the
canonical re-declaration goes DU1's way.]**

**Anchoring — the least-prescriptive substrate [proposed].** All semantics should
attach to a **topological anchor** (position in a K_n), so different semantic triangles'
`+` terms (Will, Source, …) attach to the *same* node (position 1). A point should know
only *"I am position 2 of 3"* — no categorisation schema — and semantics reference it.
(This is already the codebase shape: a **Grammar** = position-based topology; a
**Vocabulary** = the semantic word-values that reference it. So `order × position =
location` works *without* explicit category labels on the point.) The FP monad template
— **type wrapper · unit/return · bind (flatMap)** — is the target shape for composable
**code monads** (eventually all code lives in these).

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

**Revised reconciler — Systems (=), not Tag (user, 2026-08-10).** The view should be
**composed from systems themselves**, not from a free-floating notion of "tag." A tag's
keys are just the terms of the **metadata dodecads** (see next section), and those
dodecads *are* systems. So the query triad is now read as **Systems (=) reconciles
Sort (+) and Filter (−)**: **Sort** selects which system's terms become the **columns**
(which dodecad's keys you look through), **Filter** scopes the **rows** (which values /
instances survive), and **Systems** is the reconciling middle — the metadata-systems the
whole view is built out of. This supersedes "Tag (=)" as the reconciler while keeping the
+/−/= assignment (Sort affirms an ordering, Filter denies rows, Systems reconciles). It
also makes the view **self-hosting**: the thing you browse *with* is made of the same
systems you browse. [proposed — the current working reading.]

**Refined — the reconciler is Data / Perspective, not "System" (user, 2026-08-11).**
"System" is *too precise*: you sort and filter by **any datum inside a system** — a
term, a connective, a designation, a coherence, a reference, an artefact — not by whole
systems only. Two worked examples: *sort by system type, filter to only the 3rd term*;
*sort by reference, filter to a particular artefact*. What reconciles Sort (+) and
Filter (−) is therefore the **Data** (equivalently the **Perspective** — the web of
those data), the general field of addressable values, of which a system is one shape.
So: **Data / Perspective (=) reconciles Sort (+) and Filter (−)**, and both act over the
**whole datum space** (terms · connectives · designations · coherences · references),
not just the system roster. (This is why the cite-degree filter already ranges over
term/connective/designation/coherence — that range is the point, not an add-on.)

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

## Metadata dodecads as rules [proposed — the data model]

**The fundamental unit is a system.** Everything else is described *by* systems, and
that description is itself made of systems — specifically **order-12 systems** (Dodecads),
one per metadata dimension. This is the concrete form of "compose the view from systems."

**The stack of metadata dodecads (user, 2026-08-10).** A system is pinned down by
applying several 12-term systems to it:
1. **A 12-term system articulates the *set of systems*** — the roster/enumeration of the
   twelve orders (Monad … Dodecad). This is the *order* axis: every system has an order
   1→12, i.e. it is a term of this dodecad.
2. **A 12-term system supplies the `coherence` attribute** — the **DU1 horizontal
   Dodecad** (Wholeness · Polarity · … · Autocracy), one coherence value per order. (See
   *Representation medium — DU1 categories = a horizontal Dodecad*.)
3. **A 12-term system supplies `term_designation`** and **a fourth supplies
   `connective_designation`** — the impulse/act names (terms) and the force/relation
   names (connectives) drawn per order.

So each **column** in the data view is the **terms of one metadata dodecad**; a system's
row is where it lands in each. The metadata is *not* ad-hoc fields — it is **four order-12
systems** applied to the base system.

**The full specification is itself a system — a pentad/hexad/heptad (user, 2026-08-11).**
Take the four metadata dodecads as four *dimensions* of a system's specification, then
add the system's own **terms** and **connectives**:

| # | dimension | what it is |
|---|---|---|
| 1 | order | which of Monad…Dodecad (the roster dodecad) |
| 2 | coherence | the DU1 category dodecad value |
| 3 | term-designation | the impulse/act-name dodecad value |
| 4 | connective-designation | the force/relation-name dodecad value |
| 5 | terms | the vertex characters (the vocabulary's nouns) |
| 6 | connectives | the edge characters (the vocabulary's verbs) |

Order + the three designations = a **tetrad** of dodecads; **+ terms** makes a **pentad**;
**+ connectives** a **hexad** (and, if terms and connectives are counted with their
*designations* as a pair, a **heptad**). The exact order (5/6/7) is unsettled — but the
claim is firm: *a system is fully specified by a small system of dodecads plus its own
characters*, i.e. the description is homoiconic (a system describing a system). [proposed.]

**Resolved to an OCTAD — the systematics reference (user, 2026-08-13, "first port of call
before a prototype").** Add **name** and **citation** and the self-description closes at
**8** — an **octad**, and it maps onto the **Hodgson architecture octad** (the 8 systemic
attributes), each component related to the others as that octad prescribes:

| # | reference component | Hodgson octad term |
|---|---|---|
| 1 | **name** | Smallest Significant Holon (the unit's identity) |
| 2 | **order** | Integrative Totality (the largest unit / whole) |
| 3 | **coherence** | Intrinsic Nature |
| 4 | **term-designation** | Critical Functions |
| 5 | **connective-designation** | Organisational Modes |
| 6 | **terms** | Inherent Values |
| 7 | **connectives** | Supportive Platform |
| 8 | **citation** (source · artefact · lookup) | Necessary Resourcing (what grounds/sources it) |

This resolves the user's two open slots: **Organisational Modes = connective-designation**
(how the relations are designated/organised) and **Necessary Resourcing = citation** (the
provenance/resource the assertion draws on). So "codify a system" = express these 8 and
their interconnections *as* the octad — the self-describing systematics reference. A
single **term**'s relations are the same octad from the term's point of view (position =
Smallest Holon, order = Integrative Totality, coherence = Intrinsic Nature, term-designation
= Critical Functions, other terms = Inherent Values, connectives = Supportive Platform,
citation/coords = Necessary Resourcing, connective-designation/perspective = Organisational
Modes). [Proposed; the two resolved slots are the working guess.] **The hexad is the
abstraction→instance transition** (per the user): above the hexad the reference stops being
a generic template and becomes a concrete *instance* (name + citation pin it to a real
artefact) — which is why the reference lands at order 8, past the hexad.

**The metadata acts as a set of rules.** Once every system carries `{order, coherence,
term-designation, connective-designation}` as dodecad-terms, you can **match over them**:

> `match: system.name is Triad AND coherence is Dynamism → …`

A rule is a **pattern over the metadata columns** (the keys) selecting a **value** in
each — i.e. the same **key : value** tag mechanism, now read as a query predicate. This
connects directly to **Sort/Filter** (Filter = the match clause; Sort = which columns you
match on) and to the **Systems (=) reconciler** above.

**The rule's action is an Operations/Composition (RAG) triad [user, 2026-08-10].** The
"→" of a rule is plausibly the **RAG/Composition** triad applied as an operation:
**retrieve data · augment with graph · generate vocabulary** (`system_composition_3`,
Retrieve·Augment·Generate). So: *match the metadata* (the pattern) → *retrieve* the
matching systems → *augment* with their graph neighbourhood → *generate* new vocabulary
(a derived system/term). Rules are therefore **systems acting on systems** — the
self-construction loop expressed as query + composition. [proposed; the operation triad
binding is a working guess, not built.]

## The location web — Order · Position · Location and its triples [proposed]

**Order · Position · Location is a seeded architectural triad** (`system_order_position_
location_3`). It is the **topological anchor** made explicit: **order × position =
location**. An element does not need a category schema — it needs only to know *"I am
position p of an order-n system"*, and that `(order, position)` pair **is** its location
(a node in a K_n). Semantics then *reference* the location rather than living on it (the
Grammar = topology, Vocabulary = referencing words split — see *Anchoring*).

**Triples attach to a location [user, 2026-08-10].** With a location as the stable anchor,
we codify further triads that hang off it:
- **Location · Term · Source** — *this location* carries *this term character*, per *this
  source* (the provenance triple, the shape behind the reference/citation model). A
  location can hold many such triples (one per perspective/source), which is exactly the
  cross-perspective comparison the Reference Browser exists for.
- **Lines are triples too: Location · Connection · Location** — an **edge** is a triple of
  two locations joined by a connection. (Nodes are anchored by `(order, position)`; edges
  are anchored by the *pair* of locations they join.) This gives edges the same
  first-class, referenceable footing as nodes (the #28 direction).

**A mixed system that may fold into a pentad [user, 2026-08-10, flagged].** Order,
Position, Location, Term, Source, Connection are not one clean triad — they interlock
(a triad of anchoring + a triad of attachment + a triad of connection). The user
suspects this **"interesting mixed system … MAY filter into a pentad."** Tracked as a
fragment; the containing whole (pentad?) is **not yet known** — do not assert its order.

## The reference tuple — anchoring, content-addressing, reciprocal lookup [proposed — the store]

This is the concrete data model behind "the Data Object store," articulated with the
user (2026-08-11). It resolves *how* a system is assembled from components with provenance.

**RESOLVED — the primitive is an SPO triple, not `key·value·source` (user, 2026-08-11).**
The user correctly rejected `location · key · value · source` as the unit: in it,
*location* and *source* are **both keys with their own values**, and "order 3" already
splits into key `order` + value `3` — so singling out one "key" and one "value" is
arbitrary, and *"keys and values don't make a whole lot of sense"* at this grain. The fix:
the atomic unit is a **Subject · Predicate · Object** triple — a directed **AD4M/RDF link**.
Everything decomposes into these:
- `⟨node⟩ · order · 3` · `⟨node⟩ · position · 1` · `⟨node⟩ · term · Will`
- `⟨system⟩ · coherence · Dynamism`

So there is **no privileged key/value**: `order`, `position`, `coherence`, the designations,
`source` are all **predicates**; `3`, `1`, `Dynamism`, `DU3` are all **objects**; the entity
is the **subject**. This answers the open questions:
- **"All keys are anchors."** ✅ Yes — every **predicate** is an anchor (a column you can
  enter the graph from). System Order, Coherence, Term-Designation, Connective-Designation
  are the predicates every system carries; Term/Connective **characters** are predicates too,
  more instance-dependent.
- **"Location is made up; use order + position, compute location."** ✅ Adopted. Don't store
  a `location` node — store `⟨node⟩·order·3` and `⟨node⟩·position·1`; **location = the pair,
  computed.** Terms attach to those (Will at order 3, position 1), no `location` anchor needed.
- **Provenance = reify the link.** *"There must be a link from reference to 3 and coherence
  to know the value is Dynamism"* — exactly RDF reification / an AD4M link-as-entity: the
  assertion becomes a subject, `⟨assertion⟩·source·DU3`, and the Citation triad
  (Source·Artefact·Lookup) hangs off it. Two assertions coexist on the same
  `(system, coherence)`: `·source·DU3 → Dynamism` and `·source·DU1 → Relatedness` — a
  compound, no contradiction.
- **Sort/Filter = index by triple position.** *"Sort by the value of DU1 and filter by the
  key of triads"* → **Sort** indexes the **object** (value = `DU1`); **Filter** indexes the
  **predicate/object** (key = `order`=`3`). Each of Sort/Filter can act on any of S/P/O — so
  they are not fixed to key or value; that flexibility *is* the by-key/by-value freedom. A
  triple-store keeps the **S₃ permutation indexes** (SPO, POS, OSP, …) precisely for this —
  the concrete, non-hand-wavy version of "the six laws are the traversals."
- **We already have most of it.** The codebase `Reference` is nearly this SPO-with-provenance
  link: `target = "system:<id>#<fragment>"` is **subject + predicate**, and it carries
  `source/artefact/lookup` (**provenance**). The missing piece is the **object** — the
  *asserted value* (e.g. `Dynamism`) as a first-class, content-addressed node rather than a
  string on the target system. Completing that = the #25 remodel = the first cut.
- **Tension to respect [known].** SPO is **directed**; a systematics triad is **undirected**
  (AD4M codifies a *directed dyad*). The store layer is directed SPO (fine for facts/queries);
  the **six laws (S₃) are the six directed readings** of the undirected triad — so directedness
  enters by *choosing a reading*. Don't conflate the storage primitive (directed SPO) with the
  semantic object (the undirected K_n).
- **RESOLUTION of the tension [user, 2026-08-12] — you store directed, and declare undirected
  as closure under the six laws.** You cannot write a fact without a direction (bytes have an
  order). So: store directed SPO atoms; **an undirected triad = the equivalence class (orbit) of
  any one directed reading under S₃** — i.e. "undirected" *means* "closed under the six laws."
  You don't choose *store-undirected* vs *think-algorithmically*: **undirected storage = directed
  storage + the six-law symmetry group.** The Grammar (topology) already encodes the undirected
  K_n; the six laws are its symmetry group; an SPO write is one serialization; reading it back
  through S₃ recovers the whole. The **six laws ARE the traversal algorithm** — concretely, a
  triple store keeps the **S₃ permutation indexes** (SPO, SOP, PSO, POS, OSP, OPS; cf. Hexastore)
  so a query fixing *any* subset of positions range-scans the right ordering. "Sort by value,
  filter by key" = pick the permutation to scan. So the traversal-logic gap and the
  directed/undirected tension are the *same* thing, resolved together.

*(The `location · key · value · source` framing below is superseded by SPO above, but its
content-addressing / reciprocal-lookup / six-laws-as-traversal reasoning still holds — read
"key" as "predicate", "value" as "object", "location" as "the order+position predicates".)*

**Two strata: canonical topology (unreferenced) vs referenced semantics.**
- **Topology is canonical and needs no reference.** Order *n* ⇒ *n* positions and
  `C(n,2)` edges (a K_n). "Position 1 of a triad," "the edge (1,2)" are pure coordinates —
  no citation. This is the **Grammar** (`grammar_<n>`), already in the codebase.
- **Semantics are referenced.** The term/connective *values* affixed to those positions
  (`Will`, `Function`, …) are claims by a **source** and must carry provenance.

**The anchor is the LOCATION; the value is content-addressed; provenance is a link.**
- A **location** = `(order, position)` — the topological coordinate (the OPL triad:
  `order × position = location`). This is the **anchor / key**. It is canonical, so it is
  itself content-addressable (hash of the coordinate).
- A **value** (a term character like `Will`, a coherence like `Dynamism`) is
  **content-addressed** — one shared entry, reused by every location/system that cites it
  (the codebase already dedups `char_word_<slug>`; content-addressing = *id is the hash of
  the word*). **Content addressing = the identity function** (a value is itself, addressed
  by itself) — it gives dedup/identity but *not* lookup.
- A **reference** links a location-anchor to a value **with a source** (the Citation
  triad: Source · Artefact · Lookup). *This is the answer to the user's question:*
  **yes — topological positions become anchor keys, and term characters become
  content-addressed values linked to them, each link carrying its provenance.**

**So the unit of storage is a REFERENCE TUPLE, and every column is an anchor:**

> `(location = ⟨order, position⟩) · key · value · source`

e.g. `⟨3,1⟩ · term · Will · DU3`  and  `⟨3,·⟩ · coherence · Dynamism · DU3`.
Coherence anchors at the **order** level (whole system), not a single position; terms
anchor at a **position**. This is why *"3 · coherence · Dynamism"* and
*"3 · position 1 · Will"* are different arities of the same tuple shape — coherence is
`⟨order⟩·key·value·src`, a term is `⟨order,position⟩·key·value·src`.

**Reciprocal lookup = functional dependencies over the tuple [the user's XXX/YYY].**
Because *every column is independently anchored* (content-addressed + linked from each
side), you can enter the relation from any dimension and project the rest — exactly the
user's `3·coherence·XXX·source·Bennett·YYY`:
- `(order 3, key coherence, perspective **DU3**) → value **Dynamism**`
- `(order 3, key coherence, perspective **DU1**) → value **Relatedness**`
- reciprocally `(order 3, key coherence, value **Dynamism**) → perspective **DU3**`;
  `(… value **Relatedness**) → perspective **DU1**`.

So `coherence` genuinely **holds two values** at order 3, disambiguated by **source** —
no contradiction, a *compound* citing both. Filling any blank that functionally
determines the rest retrieves the others; choosing DU1 vs DU3 *transforms* the projection
("reciprocal transformations that update state chains"). **This is precisely
sort/filter by key OR by value:** grouping by the `key` column is sort-by-key; grouping by
the `value` column is sort-by-value; filtering by `source` is filter-by-reference. The
store and the query are the *same* structure seen from different columns.

**BUILT — the Filter is now an SPO query (2026-08-12).** The primitive cite-degree filter
was replaced by a **predicate → object** drill-down (`reference_browser.rs`): pick a
**predicate (key)** — Type · Order · Coherence · Source — and the filter surfaces that
predicate's distinct **objects (values)** to pick from. Crucially the value is **not
attached to the subject** (no coherence *column*); selecting the `coherence` predicate
*discovers* its options (Wholeness … Relatedness … Autocracy, and any conflicting
Dynamism) and picking one keeps the subjects asserting it (Relatedness → the canonical
Triad). `Type` keeps the old row-kind chips as the base predicate. This is the user's
"display the two options when the coherence key is selected." **Only `coherence` is a real
SPO object so far** (from DU1's references); `order`/`source` read system-fields/perspective,
`term`/`connective` predicates come as the data is **transformed into SPO** — which the user
noted is *itself a meta-ELT/Operations act* (extract the field → load as a reference →
transform the primitive): the tool ELT-ing its own schema. Next: a Term predicate over
`#term:N` reference objects, then systems.

**Content-addressing (identity) + the six laws (associativity of reading) [proposed —
answers "are the six laws associativity laws?"].** Content addressing supplies **identity**
(the `e`/`123` law: a value equals itself). The *lookup* is the other half, and it is
**directional traversal**: reading the tuple `location → key → value` vs
`value → key → location` (the reciprocal) are two different **orderings** of the same
triadic relation. The **six laws of three = S₃ = the six directed readings** of a triad
(`123 … 321`). So the six laws are best read as the **traversal orders** of a referenced
triple — *not* associativity in the strict monoid sense, but the thing associativity
*guarantees*: that chained traversals compose unambiguously (you can hop
location→key→value→source in any grouping and land in the same place). Precisely:
- **identity** (content address) = a value/anchor is itself;
- **the six S₃ permutations** = the six ways to *read/traverse* the triad (key→value,
  value→key, …) — the associative-lookup layer;
- **associativity** = the property that lets those traversals chain into a coherent index.

Mapped onto **topology · geometry · semantics**: the **topology** (K_n positions) is the
*set*; **S₃ / the six laws** are the *permutations/traversals* acting on it (at order 3,
S₃ is also the triangle's symmetry group D₃ — geometry and permutation coincide);
**semantics** are the referenced values the traversals read. This is the same **traversal
logic** flagged open earlier — now with a concrete home: *how you read a reference tuple*.
[Proposed; the S₃-as-traversal reading is firmer than the earlier CT-axioms-as-edges one,
which stays cooled off.]

**Is this content addressing / Holochain? Is it a transition moment? [recommendation].**
The *model* — content-addressed value entries + typed links from multiple anchors + source
provenance — **is** the Holochain/AD4M shape (DHT entries = content-addressed values;
links = the key↔value index; source chain / citation = provenance). **Recommendation: do
not port to a Holochain runtime now** (a hApp is a real rewrite); instead **adopt the
discipline in the current graph** — content-address the value characters, make
`location→key→value→source` explicit links, keep the store DHT-portable — so a
Holochain/AD4M backend later is a *swap, not a rewrite*. The **Data Object store and
content addressing are the same work**, not alternatives; that is the "existing
infrastructure" the user pointed at, expressed in-graph. The **Nullad = the
content-addressed heap** (the DHT / bulk); a **Monad = a scoped perspective that draws a
boundary — a "universe"** — over that heap (so fragments need no home *other than* the
heap; the Monad is the home). [Recommended direction — awaiting sign-off before build.]

**Many triads need "worlds" to fit into [user, 2026-08-11].** Scraping *The Dramatic
Universe* Vol. 2 for triads yields a *lot* of them — free-floating triads that need
**containing systems** to make sense of. This is the **fragment → fold-in** loop and the
**Monad-as-universe** boundary together: an isolated triad is a *face* of some larger K_n,
and its **"world"** is the higher-order system (or the scoped Monad) it inhabits — Bennett's
own doctrine of **Worlds** (DU Vol. 2–3) is exactly this containment. So the store must
make a triad **first-class and addressable even before its world is known** (a dangling
fragment in the heap), and let a Monad later *claim* it into a world. Practically: the SPO
store + the bucket/Monad = a triad can exist in the Nullad heap and be pulled into as many
worlds (Monads) as cite it. [Direction; the "world" system-type is not yet modelled.]

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
| **Operations (ELT ≅ RAG)** | Extract · Load · Transform — **renamed from ELT to "Operations"** (not "compose"). **ELT ≅ RAG** (data-warehouse ↔ generative-AI): the same isomorphic triangle, both **composition**. The six permutations of the triad = the **six laws of three** (see below). Wired: `createSequence` · file-picker · `applyFunctor` + author (`authorSystem`), surfaced by the in-app **Editor**. **SPO = the *Perspective* triad** (Subject·Predicate·Object as edges). **[open] ELT may BE the sort/filter triad** — *Load = selecting the keys to display values* (i.e. Sort = header tags). If so there is a sequence **Data (monad) → key·value (dyad) → ELT (triad)** — "data" being what we call *tags* (everything is a tag ⇒ it is just data). Recorded, not resolved. | **edge-typed** (all verbs) | `graphql/types.rs`; UI: `components/reference_browser.rs` `elt_triad` (Nullad page) | impl (not seeded). **Extract** is wired (Nullad → Monad): materializes the current data-view selection (distinct `system:<id>` of the filtered references) into a persisted Monad via `createSequence` + `create_sequence` (client). **Load** is wired (`loadPerspective` → `on_load`). **Transform** (apply a Functor) is surfaced but **not yet wired**. Monad auto-naming is provisional (the members' *integral* is a later refinement) |
| **Containers** | System · Sequence · Perspective (nouns) | node-typed | `core/{systems,sequences,perspectives}.rs` | impl (all three built; triad-ness unsettled). **Open:** *Perspective* may be retired — or kept only as the subject·predicate·object **Link** substrate, which is arguably *everything* in the system (every tag is a `key:value` predicate on a node). |
| **Query (Sort · Tag · Filter)** | **Tag (=)** reconciles **Sort (+)** and **Filter (−)**, mapping to **class / instantiation / instance**. **Sort = selecting the header tags** — which tag keys are the columns (the *class*: header/keys; `ColKey`; default Order + Citation). **Filter = scoping the data returned** in those columns (the *instances*: values), by **cite-degree** — the data categorised by number per the schema **1 term-designation · 2 connective-designation · 3 coherence · 4 term · 5 connective · 6 system** (their coalescence). So you can view only systems (*manifolds* = systems not yet placed), only terms, only connectives, etc. | reconciler-typed | UI: `components/reference_browser.rs` (`CiteKind`; Sort = column selector, Filter = degree scoper) | impl v1. Citation column in **Source · Artefact · Lookup** order |
| **Data (Data · Graph · Table)** | **Data (=)** is the content the header scopes; **Graph (+)** and **Table (−)** are its two views. The switch (right of the header menu) chooses one. | reconciler-typed (Data is the whole; Graph/Table its views) | UI: `components/system_selector.rs` (`ViewMode`) | impl — Table live; Graph = per-system K-graph (Nullad Graph = the future all-graph) |
| **Class · Instantiation · Instance** | e.g. **K₄ (class) · Tetrad (instantiation) · Canonical Tetrad (instance)**. The abstract complete graph → its systematic instantiation → a concrete instance. Also the table's own structure: header/keys (class) · data types/keys (instantiation) · data/values (instance). | node-typed (nouns) | (documented; extends the existing Class/Instance dyad) | proposed |
| **Link / triple** | subject · predicate · object (`source · predicate · target`) | the edge itself | `core/perspectives.rs` `Link` | impl (AD4M) |
| **Architecture MVC** ⭐ | **Model (−)** · **View (+)** · **Controller (=)** (nouns) — base-space store · interface · six-laws algo layer. The **organising frame for the base-space remodel**: separate the concerns; Controller = the six laws of three (SPO = only the 132/interaction law). The Graph Template (Model's inner =) holds the Controller's validation rules. | node-typed (nouns) | model = backend data + base-space triad · view = `frontend/components/{browser_controls,inspector,graph_view}` · controller = middleware + six laws | proposed (user 2026-08-18) — see *The architecture is an MVC triad* |

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

**A monad is either an ordered core-sequence or a bucket [built, user 2026-08-11].**
A `Sequence`'s members can play two different roles, and the UI now tells them apart by
whether the members' **system orders are distinct** (`is_order_navigable`):
- **Ordered core-sequence** — at most one member per order (Monad(CT):
  Monad·Container·Operations·Identity·Assoc·Composition; Data: Data·Data·Operations).
  Entering it opens the **graph** and the header **steps it by order** (Monad→Dyad→Triad);
  the orders it lacks are **greyed out** (unreachable in that context). Nullad exits.
- **Bucket** — a *group* with several members of the **same order** (the Architectural
  Monad has three triads). It is not a path, so it can't be order-stepped; entering it
  **scopes the Table to its members** (a group *for sorting*) and greys every order
  button. This is exactly "group several systems into a monad as members for sorting."
A bucket is the natural home for the **`sequence` filter kind** and for the "associate
X with architecture" gesture — associating = **adding X to the architecture bucket**.

**Systems are lenses on the monad.** Each order is the *same* monad **reformulated
at a resolution**: the Pentad expresses its significance in 5 Limits + 10
Mutualities; a triad reformulates its core dynamics in 3 terms + 3 connectives;
and so on. Moreover a **node inside one reformulation can itself unfold into a
system** — e.g. `Sign` (a Pentad node) may unfold into a *dyad of number and
strings*. Everything is **fractal / holonic**: five tetrads in the monad may be a
single node in the core Pentad.

The live **Monad** is `sequence_architectural_monad` (seeded in
`backend/data/perspectives/architecture_monad.json`), now a **bucket** of the
architecture systems for sorting — members are **explicit** (a resolvable `system:`
address) or **implicit** (a dangling address = material still to be assembled).
**6 members:** the **Data (key·value) dyad**, **Order·Position·Location**, **Citation**,
**Identity·Associativity·Composition** and the **Architecture Pentad** (explicit, seeded)
+ the **Architecture Octad** (dangling — documented in the octad mapping below, not yet
seeded as a system). The feature-fragments it used to track (ELT, Sort·Tag·Filter,
Data·Graph·Table, Class·Instantiation·Instance, by-key/by-value) are superseded by real
seeded systems and are documented here rather than dangling in the bucket. Other
author + systematics-core fragments (`docs/fragments.md`) are seeded as real systems but
not yet placed.

## Practice

Whenever we implement a systematic grouping (dyad / triad / tetrad / …), **add it
here** with: its terms (nouns → nodes), its connectives (verbs → edges), the code
location, and its status. Keep the code's names aligned to the schema. Companion
docs: `docs/architecture-pentad.md` (the pentad in depth) and
`docs/plans/architecture-run.md` (the 1→12 run, exploratory).
