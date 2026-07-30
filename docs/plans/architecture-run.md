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

## The process circulates on the enneagram [proposed]

The operational process is **not a flat 1→12**; it follows the enneagram's
hexagram circulation **1 → 4 → 2 → 8 → 5 → 7 → (1)**, with the triangle **3 · 6 · 9**
as the reconciling shock-points (to be worked out):

| point | operation (proposed) | note |
|---|---|---|
| **1** | **Ingest** = *assimilation* — bring the unsorted collection in | monad = the collection itself; ingest realises it |
| **4** | **Sort · Tag · Search · Filter** — the activity/tetrad (Activity Field) | 1→4 inner line |
| **2** | **Discrimination** — what we are *doing*; weigh related & valuable options toward the end result | 4→2 inner line; dyad = in/out complementarity |
| **8** | **Agent / emitter** (thermostat, LLM — a will) | 2→8; agent @ octad [user-anchored] |
| **5** | **the Pentad** (Sign/Symbol/Syntax/Semantics/Grammar) | 8→5; the structural core sits on the flow [settled] |
| **7** | **Generation** — emitting new systems/symbols/code | 5→7 |
| 3 · 6 · 9 | reconciling shocks (triangle) | open |

The flow passing *through* the Pentad (point 5) is the point: the operational
will (the enneagram) contains the structural being (the pentad) as a station.

## The bootstrap (the chicken-and-egg)

We need enough system to use it to design itself. The way through:

1. **Ingest** — load an unsorted hodgepodge of stored systems/fragments (extend
   `loadPerspective` to bulk / arbitrary input). *Not everything in storage is
   part of this system — it's just a collection of everything.*
2. **Discern + organise** — sort / filter / search / tag; discriminate what is
   part of the system from what is not (two overlapping monads).
3. **Represent** — the architecture run as a **first-class, editable Sequence in
   the app** (the Sequence type designed but not yet built), with back-and-forth
   updates happening *there*.
4. Then **use the system to tune its own design** (dynamic homoiconicity).

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
- **[proposed]** enneagram process flow 1→4→2→8→5→7; 1=ingest/assimilation;
  4=sort/filter/search/tag; 2=discrimination; 7=generation.
- **[open]** the Triad; the triangle 3·6·9; orders 9–12; how structural and
  operational aspects precisely reconcile; the exact terminology of the
  being/doing/activity mapping.
- **[discarded]** the earlier abstract 1→12 run (weak past the pentad).

## Near-term build (recommended)

**Ingest + Sort/Filter**, then **represent the run as an editable Sequence** —
the monad/dyad/tetrad of the doing-side — because it is buildable now and is the
literal prerequisite for "get the data in, then tune with the system." Details to
be specced when we pick this up.
