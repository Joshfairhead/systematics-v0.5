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
line order (`data/mod.rs:378`), map edge-for-edge onto the architecture. ✓ marks
the two you anchored directly; both matched the lexicographic order, confirming
the rest.

| Edge (Limit–Limit) | Mutuality | Reading |
|---|---|---|
| Grammar – Sign | **quantitative-match** | grammar matches the *count* of signs — **this is the existing arity `Grammar`** |
| Grammar – Semantics | **aspiration** | grammar reaches toward meaning |
| Grammar – Syntax | **operation** | grammar operates through form |
| Grammar – Symbol | **qualitative-match** | grammar matches the assembled *quality* of the symbol |
| Sign – Semantics | **function** ✓ | a sign's function *is* its meaning |
| Sign – Syntax | **input** | signs are input to the syntactic compile |
| Sign – Symbol | **range-of-significance** | the span from atomic sign to realized symbol |
| Semantics – Syntax | **range-of-potential** | form↔meaning — Hodgson's "internal and external potentiality" |
| Semantics – Symbol | **output** | the symbol outputs its meaning |
| Syntax – Symbol | **form** ✓ | form compiles signs into the symbol — **this is `resolve_system`** |

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

## Where Perspective sits

A **Perspective is composed of grammars** — it deploys the pentad rather than
being one of its five Limits. It is the generic container/web (AD4M) that holds
many sign→symbol assemblies and decorates them (references, excerpts). Its exact
placement relative to the pentad (the monadic container that *is also a sign*; the
"will" that reconciles) is the one piece still being set — see open questions.

## Rename map (code → convention)

Conventions to migrate the code toward (a real, staged refactor — not this pass):

| Current | Convention | Note |
|---|---|---|
| address | **Sign** | key, value, any field |
| `RenderedSystem` | **Symbol** | the assembled graph |
| `resolve_system` | **form** (a grammar operation) | compiles Syntax over Signs → Symbol |
| `Grammar` (K_n arity) | **quantitative-match** Mutuality | an aspect of the Grammar node (Grammar–Sign) |
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
   do it once the model is frozen and the self-describing pentad is seeded.
