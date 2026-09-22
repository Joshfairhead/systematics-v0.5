# View model — the time tetrad (Eternity · Hyparxis · Chronos · Space)

[← index](README.md) · related: [operations-and-workflows](operations-and-workflows.md), [comparing-systems](comparing-systems.md)

The two views (list / graph) are not arbitrary UI toggles — they are two **determining
conditions** of the same content. Mapping them onto the time tetrad (Time · Hyparxis ·
Eternity · Space) gives a principled, consistent behaviour and resolves the v0.5 nav
inconsistency. This is both a v0.5 fix and a v0.6 design principle.

## The mapping
The tetrad **nests** — it is not four flat views. **Space** is the whole; **Hyparxis** is the
scope you enter; **Eternity/Chronos** are the two lenses used within any scope:

```
Space  = the whole interface (the outer frame; always present)
 └─ scope:  Nullad (global)   |   Monad = HYPARXIS (entered a monad — the working container)
      └─ lens:  ETERNITY = list        |   CHRONOS = graph
```

| condition | role | is | shows |
|---|---|---|---|
| **Space** | the whole | the entire interface — the coexistent field everything sits in | (frame) |
| **Eternity** | lens | **list** view | the *pattern of all possibilities* — all-of-a-type (all triads, all tetrads …). **Filter.** |
| **Chronos (Time)** | lens | **graph** view | *one system at a time*, linked sequentially monad→dodecad; the chosen monad is the **head**. **Sort + filter** (surface the most-relevant). |
| **Hyparxis** | scope | the **monad as container** — the *working-between-graph-and-list* mode you are in once a monad is entered | a **list of candidate top-graphs**, each also viewable *as a graph* (scroll candidates vertically), narrowing toward a **settled** final candidate. **The narrowing.** |

**The workflow is a path:** start at the Nullad in **Eternity** (all possibilities, list) → enter
a monad (**Hyparxis** — narrow its candidates, flipping each between list and graph) → **settle a
single graph** (**Chronos**). All of it inside **Space** (the interface).

## Current v0.5 behaviour (the inconsistency, diagnosed)
`ApiApp::ViewSequence` (`frontend/src/app.rs`) branches on `is_order_navigable(members)` (a
sequence with at most one system member per order):

- **Order-navigable** (Blue Earth Ventures — monad→dyad→triad→tetrad, one each): opens **Graph**,
  sets `active_sequence`, the header steps by order (greying the rest). *But* toggling to list
  view **drops the scope** and shows a global filter of all systems by order type.
- **Bucket** (Blue Earth — several members at some order): opens **Table** scoped via
  `scope_members`, order buttons greyed (`enabled_order_keys` returns empty), no graph, and the
  only exit is the Nullad key (unobvious → "no way back bar refresh").

Root cause: **scope lives in two half-wired places** — `active_sequence` (order-stepping, graph
only) and `scope_members` (table-scoping, list only) — and **neither survives a view toggle**.
Plus the lingering **monad vs monad-head / order-navigable vs bucket** split (see
[operations-and-workflows](operations-and-workflows.md)).

## Root cause: an unaddressed state model
The edit dimension already has an explicit state model (`CanvasMode { Viewing, Editing }`); the
**navigation** dimension does not. Scope is smeared across `active_sequence` (graph order-step),
`scope_members` (table scope), `mode`, and `selected_key`, with no single owner — so the two
views disagree and a toggle loses the scope. The fix is to give navigation the same treatment: a
first-class state model.

```
struct Nav {
    scope: Scope,      // Space is the whole interface; scope is where you are within it
    lens:  Lens,       // the determining-condition reading of the current scope
    at:    Option<Position>,   // active order + candidate index within scope
}
enum Scope { Nullad, Monad { members: Vec<String> } }   // Monad = Hyparxis (the working container)
enum Lens  { Eternity /* list */, Chronos /* graph */ }
```

`Nav` is the single source of truth both views render from; `CanvasMode` stays orthogonal
(Viewing/Editing *within* Chronos). Transitions are centralised (like `enter_viewing`):
`enter_monad`, `exit_to_nullad`, `set_lens`, `step_order`, `step_candidate`.

## Target behaviour (unified)
1. **Entering a monad scopes *both* views to its members.** One scope (the monad's member set),
   honoured by list *and* graph. A view toggle never loses it.
2. **List = Eternity, scoped:** show the monad's members grouped/filtered by type
   (a discriminative list — only this monad's offerings). The order/type buttons filter *within*
   the scope. (Un-scoped Nullad list = the global Eternity: all systems by type.)
3. **Graph = Chronos, scoped:** surface one member at a time; the header steps monad→dodecad.
   Where an order has **one** member, step straight to it (today's order-navigable path). Where an
   order has **several** candidates (Hyparxis), step *within* that order through the candidates
   (prev/next), rather than falling back to a bucket.
4. **Consistent exit:** a breadcrumb / "up to Nullad" affordance in both views (not only the
   Nullad key), so there is always a way back without refresh.
5. **Retire the monad vs monad-head distinction:** every monad heads a sequence and opens the
   same way (per [operations-and-workflows](operations-and-workflows.md)). "Order-navigable" vs
   "bucket" becomes a *per-order* property (one vs many candidates), not two kinds of monad.

## v0.5 implementation plan (frontend only; no backend change)
Incremental, each step shippable:

1. **Introduce `Nav`.** Replace `active_sequence` + `scope_members` with the single `Nav`
   (`scope`, `lens`, `at`); centralise transitions. Both views render from it.
2. **Scoped list (Eternity).** When `scope = Monad`, filter rows to its members (discriminative
   list); order/type buttons filter *within* scope. `Nullad` = global (all systems by type).
3. **Scoped graph + per-order candidates (Chronos + Hyparxis).** Header steps orders present in
   scope; one member ⇒ show it; several ⇒ prev/next scroll the candidates (the Hyparxis narrowing).
4. **Toggle preserves scope + position.** `set_lens` keeps `scope` + `at`; list highlights the
   graph's current member and vice-versa.
5. **Exit affordance.** An explicit "up / Nullad" control (breadcrumb) in both views — always a
   way back without refresh.
6. **Editor stays in scope.** Assembling (join/decompose) keeps `scope`; new members appear as
   candidates in the current order. (Ties to [operations-and-workflows](operations-and-workflows.md).)

## v0.6 note
In the rebuild this is not bolted on: the lens is a **functor over the scope** — Eternity and
Chronos are two readings of the one scoped set; Hyparxis is the monad container being narrowed;
Space is the interface. `list`/`graph` are projections of `Nav`, not independent screens.
