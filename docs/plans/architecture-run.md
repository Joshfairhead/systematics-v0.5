# Architecture run — working plan (nouns · verbs · will)

> Status: **working / exploratory.** The Pentad is settled; most else is
> proposed or open. Per the precise-language rule, nothing here is asserted as
> fact unless tagged **[settled]**. This plan exists to (a) guide the near-term
> build that gets stored systems *in* and bootstraps self-tuning, and (b) become
> an **editable sequence inside the app**, where the run is refined with the tool
> itself rather than in prose.

## Core frame — nouns, verbs, will [settled shape, wording still settling]

Our system is a **graph**:
- **Nouns = nodes** (being),
- **Verbs = edges** (doing),
- **the graph as a whole — nodes + edges in relationship — = will** (the
  reconciliation; a functional representation of will).

So **being and doing are not two separate runs**; they are the two sides of one
coin, brought together in the will (the system we are building). A *Perspective*
(a graph) is therefore will-shaped: nouns + verbs in relationship.

**Validation rule [proposed — the Dyad's force]:** a **term (vertex) must be a
noun**; a **connective (edge) must be a verb**. This noun↔verb polarity, and the
*force* binding them, is the **Dyad** (Poles / Force). Mis-typed content is a
validation error — e.g. "sort · tag · filter · search" are **verbs**, so they are
**connectives**, not terms. (Enforcing this needs a part-of-speech on characters;
capture the rule now, implement later.)

Correction to an earlier framing: the "being vs doing" polarity was mis-worded.
**Doing = activity = the Tetrad** (Activity Field) — verb-like, and it *represents*
being; what was loosely called "being" is more functional / dyadic. Terminology
is still converging — treat the *nouns/verbs/will* statement above as the stable
part.

## Two aspects of the one system

- **(a) Structural / content** — what a system is *made of*.
  **Pentad = Sign · Symbol · Syntax · Semantics · Grammar** [settled]. Its nouns
  (Sign, Symbol) are nodes; its Mutualities (form, function, …) are edges;
  Grammar at the centre reconciles.
- **(b) Operational / process** — what the app *does* (ingest, discern,
  transform, sort/filter/search/tag). This is control-flow-adjacent.

These are reconciled *in the will* (the running system), not held apart.

## The process and the enneagram [proposed — mappings UNRESOLVED]

The operational process is **not a flat 1→12**; it seems to circulate on the
enneagram (hexagram flow **1 → 4 → 2 → 8 → 5 → 7 → (1)**; triangle **3 · 6 · 9**).
But *which* enneagram mapping is not settled — there are (at least) two, and they
conflict on where the monad/ingest sits. Both are recorded; neither is asserted.

**Candidate A — operations on points** (an earlier process reading):
1 = Ingest/assimilation · 4 = Sort·Tag·Search·Filter · 2 = Discrimination ·
8 = Agent/outcome · 5 = Pentad · 7 = Generation.

**Candidate B — Hodgson's expression** (systems on points): 9 = Monad · 1 = Dyad ·
2 = Triad · 4 = Tetrad · 5 = Pentad · 7 = Hexad · 8 = Heptad. ("A bit weird, but
possibly on point.") Note B puts the **Monad at 9**, which conflicts with A's
ingest-at-1 once we say *ingest = monad*.

**The 3·6·9 triangle** is open: possibly **Extract · Load · Transform**, or
possibly the triangle *represents* the Triad (3), Hexad (6) and Ennead/Nonad (9)
themselves within the structure.

Anchors that survive across readings: **8** (agent / the outcome looked toward),
**5** (the Pentad [settled]). The flow passing *through* the Pentad (point 5) is
the point: the operational will (the enneagram) contains the structural being
(the pentad) as a station.

## The bootstrap (the chicken-and-egg)

We need enough system to use it to design itself. The way through:

1. **Monad — Ingest.** An unsorted hodgepodge of stored systems/fragments —
   **unstructured data; RAG is the Monad of the system** (chuck everything into a
   vector store / latent space). *Not everything in storage is part of this system
   — it's just a collection of everything.* **Stay simple for now: treat ingest as
   the Monad, represented as a perspective graph**; the fuller vector/RAG form is
   later.
2. **Tetrad — Organise.** Distil the Monad into a **graph**: create all the
   **node/edge relations** — nodes = **term characters** (nouns), edges =
   **connective characters** (verbs). **Sort · Tag · Filter · Search are verbs →
   they are the Tetrad's *connectives*, NOT its terms** (per the validation rule
   above). The Tetrad's noun-terms (the four *sources* the operations act between)
   are **still to be defined** — this is the "edge mappings still needed" gap.
   (This is where "what is part of the system vs not" gets discriminated.)
3. **Assemble — structure into systems (sequential order).** Group graph elements
   into term/connective characters, forming a mass of **loose low-order systems —
   mostly dyadic pairs and triadic faces — as raw material**, most of which are
   **discarded**, the rest **assembled into higher-order complete graphs** (faces
   gluing into higher systems). **Relate (Triad, point 3)** sifts needed vs
   not-needed, looking toward the outcome at 8.
4. **Represent** — the architecture run as a **first-class, editable Sequence in
   the app** (the Sequence type designed but not yet built), with back-and-forth
   updates happening *there*. The **Load control should address a Sequence**, not
   just a single system: loading the *Architecture Run* sequence shows it as a
   **collection of systems** — the Pentad displays with its terms, orders not yet
   defined show blank/placeholder. "See the architectural plan represented as a
   collection of systems."
5. Then **use the system to tune its own design** (dynamic homoiconicity).

## Bennett's DU2 creation-myth pattern [generative automation, downstream]

Bennett's method, repeated in each book, is a candidate process to *automate*
the architecture's development:

> subject (monad) → its poles (dyad) → three sections (triad) → each section a
> tetrad → combined as twelve terms → examined through the Pentad → yielding an
> Octad.

Automating something like this could let the tool generate/refine its own run.
Downstream of ingest + the editable sequence; it would imply app features to run
such operations.

## Firm vs open (precise language)

- **[settled]** Pentad (5) = Sign/Symbol/Syntax/Semantics/Grammar; nouns=nodes,
  verbs=edges, graph=will.
- **[anchored]** Agent @ Octad (8).
- **[proposed]** Ingest = Monad = a perspective graph (unstructured / RAG ideal);
  Tetrad = sort/tag/filter/search; enneagram process flow 1→4→2→8→5→7; generation
  at 7.
- **[open / unresolved]** *which* enneagram mapping (operations-on-points vs
  Hodgson's systems-on-points — they conflict on the monad's point); the triangle
  3·6·9 (ELT? or triad/hexad/ennead?); the Triad; orders 9–12; how structural and
  operational aspects precisely reconcile; the exact being/doing/activity wording.
- **[discarded]** the earlier abstract 1→12 run (weak past the pentad).

## Near-term build (recommended)

**Ingest + Sort/Filter**, then **represent the run as an editable Sequence** —
the monad/dyad/tetrad of the doing-side — because it is buildable now and is the
literal prerequisite for "get the data in, then tune with the system." Details to
be specced when we pick this up.
