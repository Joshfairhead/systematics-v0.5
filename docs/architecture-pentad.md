# The Architecture Pentad — glossary & conventions (self-describing)

## Thesis

The tool's own system architecture is an **instance of the canonical Pentad
"Significance and Potential"** (order 5). Its five *Limits* (nodes) are the kinds
of thing in the system; its ten *Mutualities* (edges) are the operations between
them. This is a **specific pentad applied to our architecture** — and, applied
*homoiconographically*, the tool can hold this very description as one of its own
Symbols: a self-describing Pentad system. Proof of the pudding is in the eating —
so the endpoint is to **seed it as a real Pentad in the tool**, not just prose.

Everything in the system is a **Sign**. Symbol, Syntax, Semantics and Grammar are
Signs playing roles; the Pentad is one differentiation of the monadic sign-ground.

## The five Limits (nodes)

The five node terms (term designation: *Limits*), their pentad positions, and the
current code they name:

| Pentad position | Term | Definition | Current code |
|---|---|---|---|
| **Source** | **Sign** | anything in the system — a key, a value, any field | address (`system:X#term:2`) |
| **Purpose** | **Symbol** | the assembled whole; a Sign realized as an integrated graph | `RenderedSystem` |
| **Lower Potential** | **Syntax** | the formal expression compiling Signs into Symbols | (the compile step — `resolve_system`) |
| **Higher Potential** | **Semantics** | the functional meaning of each Sign | (vocabulary meaning) |
| **Quintessence** | **Grammar** | the generative centre reconciling the four; radiates the operations | `Grammar` (arity), generalized |

Two axes meet at the centre: **Source→Purpose = Sign→Symbol** (actualization /
existence) and **Lower↔Higher Potential = Syntax↔Semantics** (the range of
potential / essence), reconciled by **Grammar** at the Quintessence.

## The ten Mutualities (edges)

The canonical pentad's connectives (designation: *Mutualities*), in lexicographic
line order (`data/mod.rs:378`). Each edge is a concrete **embodiment** — a real
thing in the system, whether it exists in the codebase today or is yet to be
built (an input, say, may be a *module* or data stream; an output, a generated
codebase). Naming what's there is the start, not the ceiling. `[U]` = anchored by
the user; `[P]` = proposed (to be tested with the pentad itself). The remaining
links are still being worked out — likely via category theory (connective
characters expressed as functors/morphisms with the right name).

| Edge (Limit–Limit) | Mutuality | Instance in the tool | |
|---|---|---|---|
| Grammar – Sign | **quantitative-match** | **Number** — Order (1–12) + Position (expresses order *and* position); a CT axiom — **associativity** or **composition** [direction UNRESOLVED, see below] | [U] |
| Grammar – Symbol | **qualitative-match** | **Colour** — the colour vocabulary (qualitative rendering); a CT axiom — **composition** or **associativity** [direction UNRESOLVED] | [U] |
| Grammar – Syntax | **operation** | **Functor / morphism** — category theory: the substrate *language* the system is written in (core notions: identity, composition, associativity) | [U] |
| Grammar – Semantics | **aspiration** | the **set of coherence attributes** (monad→dodecad) as *demands* on each system — coherence-as-potential (a digital embodiment, not a label-field), actualized at the hexad | [P?] |
| Sign – Semantics | **function** ✓ | the functional meaning of each sign | [U] |
| Sign – Syntax | **input** | **assembly pieces** — points, coordinates, terms; and, in scope, **modules / data streams** | [U] |
| Sign – Symbol | **range-of-significance** | **3D upcast geometry** — dodecahedron etc. (external potential) | [U] |
| Semantics – Syntax | **range-of-potential** | **the rendered systems 1–12 as a whole** (internal potential); **CT identity ≙ content-addressing** (a value is itself) — the line between Semantics and Syntax [user 2026-08-11] | [U] |
| Semantics – Symbol | **output** | **self-describing protocols** — the symbol as a system that describes itself; generative (labelled graphs → small language models / codebase automation) | [P] |
| Syntax – Symbol | **form** ✓ | **`resolve_system`** — compiles signs into the symbol | [U] |

The payoff: the pentad's edges are already-present layers of the codebase
(number, colour, geometry/3D, functors, rendered systems, the compiler) — and
where they aren't yet, they name what to build (module inputs, generative
outputs). The architecture organizes itself into its own order-5 system.

**The CT axioms live on the pentad's edges [user, 2026-08-11 — supersedes CT-axioms-as-triad-edges].**
The three category-theory axioms find a home not on a *triad's* edges (the cooled-off
FBW reading) but as **mutualities** of this pentad:
- **identity ≙ content-addressing** = the **Semantics–Syntax** edge (*range-of-potential*) —
  *"content addressing is the line between semantics and syntax"* (user's correction; **not**
  the Functor/operation edge as first written). A value is itself.
- **composition** and **associativity** = the **Number** (Grammar–Sign, quantitative-match)
  and **Colour** (Grammar–Symbol, qualitative-match) edges — **but which-is-which is
  UNRESOLVED**: the user gave two opposite mappings in one message —
  *"number should be composition, colour associativity"* vs *"composition replaces colour and
  associativity replaces number as qualitative/quantitative match."* **Not baked into the graph
  data** until confirmed. The user also notes these may sit better as **new nodes on the
  aspiration (Grammar–Semantics) / operation (Grammar–Syntax) edges** rather than relabelling
  Number/Colour — a later refinement.

This dovetails with *The reference tuple* in `architecture-systems.md`: content-addressing
supplies **identity**, and the S₃ traversals supply the **associative** lookup — so the
store's identity/associativity halves are pentad edges. [Proposed; firmer than the
CT-axioms-as-triad-edges reading, which stays cooled off. Number/Colour direction pending.]

## AD4M resolution (subject · predicate · object)

An AD4M Link `{source, predicate, target}` reads as **subject · predicate ·
object**: the **Grammar node is the subject**, the **predicate is an operation**
(a Mutuality — form / function / input / output / …), and the **object is the
term** it connects with. So `Grammar --form--> term` etc.

Edges in general are **connective characters** — the labels on predicates. In
*this* pentad the connective characters are form/function/input/output/operation/
…; in a triad they are generation/decision/consent; any labelled edge is one.
Crucially, each connective character is **itself described by this pentad**: the
character *generation* has a form (its word), a function (its meaning), possible
inputs/outputs, and its own semantics/syntax/signs/symbols within the system. The
Pentad is therefore the **grammar for any element** — applied homoiconographically,
it describes even its own edge-labels.

## The architecture as a progression of systems (monad → hexad)

The architecture isn't only the pentad — it's a **Sequence of Systems, one per
order**, each describing the tool at a level (itself homoiconic: the tool's
architecture told in the tool's own systems). Working sketch (tentative above the
pentad; `?` = unsettled):

- **Monad** — the sign-ground: unity-in-diversity; *everything is a sign*.
- **Dyad** — ? personas & profiles.
- **Triad** — **agents · languages · perspectives** (to be checked against the
  triad's affirming/receptive/reconciling connectives; stays abstract).
- **Tetrad** — ? never settled; user's rough guess *interfaces · representations ·
  interpretations · expressions*; the canonical tetrad designations
  *Ideal/Ground/Directive/Instrumental* may anchor it better.
- **Pentad** — **Sign · Symbol · Syntax · Semantics · Grammar** (this doc).
- **Hexad** — **coalescence + cyclicity**: "the beginning of homoiconicity." The
  cyclic/coalescent structure is where the description **closes on itself** —
  isomorphic, the map equal to the territory. Likely a coalescence of two triads
  (Bennett: nearer *facts / values* than operational/experiential); the
  **range-of-significance** edge (Sign–Symbol) is what focuses it. **CT and
  Systematics** appear as two instances of signs↔symbols — CT on the input side
  (sign→syntax), Systematics on the output side (semantics→symbol). The
  export↔load round-trip (`export∘load = id`) is one seed of it, but the system is
  not yet fully holographic (instrument ≠ object); the hexad is where it becomes so.

## Where Perspective sits

A **Perspective is composed of grammars** — it deploys the pentad rather than
being one of its five Limits. It is the generic container/web (AD4M) that holds
many sign→symbol assemblies and decorates them (references, excerpts). Its exact
placement relative to the pentad (the monadic container that *is also a sign*; the
"will" that reconciles) is the one piece still being set — see open questions.

**System vs Perspective:** a **System is a *scoped* perspective** — a lens
constrained by its grammar (bounded, one K_n). The current `Perspective` is the
**open graph** — a useful catch-all with no such bound. AD4M's Agent/Language/
Perspective triad is kept in mind to blend: take the good parts, reinterpret them
through systematics, leave the mess.

## Rename map (code → convention)

Conventions to migrate the code toward (a real, staged refactor — not this pass):

| Current | Convention | Note |
|---|---|---|
| address | **Sign** | key, value, any field |
| `RenderedSystem` | **Symbol** | the assembled graph |
| `resolve_system` | **form** (a grammar operation) | compiles Syntax over Signs → Symbol |
| `Grammar` (K_n arity + structure) | **Grammar** node (Quintessence) | the generative centre; its arity shows up on the quantitative-match edge (= Number) |
| `Functor` / morphism | an **operation** grammar | a Mutuality; symbol→symbol |
| `Vocabulary` | Signs (node characters) + **connective characters** (edge labels) | — |
| `Perspective` | **Perspective** | unchanged — the container of grammars |

## Homoiconic next step (proof of the pudding)

Seed the Architecture Pentad as a real **Pentad System** in the tool: term
characters `{Sign, Symbol, Syntax, Semantics, Grammar}` filling the five Limits;
connective characters = the ten Mutualities (which already exist as the canonical
pentad's connectives); coherence "Significance and Potential". Then the glossary
is no longer prose — it is a Symbol you can browse, the system describing itself
in its own terms.

## Open questions

1. **Perspective's placement** — the monadic container that is also a sign, the
   "will" element, or a level above the pentad entirely?
2. **`resolve_system` = form specifically**, or the whole Grammar node? It compiles
   (→ form) but also validates (→ quantitative/qualitative match).
3. **Rename staging** — the code refactor is broad (core + GraphQL + frontend);
   do it once the model is frozen and the self-describing pentad is seeded. The
   direction (per the user) is **dynamic homoiconicity**: as the description is
   corrected by progressive approximation, the codebase is refactored to match it,
   until the code reads as if articulated from the systems.
4. **Does a pentad edge contain a triad?** (e.g. the `operation` edge and CT's
   identity/composition/associativity, or Extract/Load/Transform.) A *possibility,
   not settled* — do not assert it.
5. **Vertices and links as systems.** A vertex may represent another system
   (plausible); a link may represent a collection of links or systems (uncertain).
   Open — not decided.
