# View model — the time tetrad (Eternity · Hyparxis · Chronos · Space)

[← index](README.md) · related: [operations-and-workflows](operations-and-workflows.md), [comparing-systems](comparing-systems.md)

The two views (list / graph) are not arbitrary UI toggles — they are two **determining
conditions** of the same content. Mapping them onto the time tetrad (Time · Hyparxis ·
Eternity · Space) gives a principled, consistent behaviour and resolves the v0.5 nav
inconsistency. This is both a v0.5 fix and a v0.6 design principle.

## The mapping
| condition | view | shows | operation |
|---|---|---|---|
| **Eternity** | **List** | the *pattern of all possibilities* — every system of a type (all triads, all tetrads …) | **filter** by type |
| **Chronos (Time)** | **Graph** | *one system at a time*, linked sequentially monad→dodecad; the chosen monad is the **head** that sets its dyad/triad/… | **sort + filter** (surface the most-relevant to the top, in graph) |
| **Hyparxis** | intermediate / **editor** | *candidates being assembled* — several systems in a monad with no single top candidate (e.g. 3 dyads); scroll them in graph, or a discriminative candidate list | **narrow** (remove options) |
| **Space** | *(proposed)* the **monad container / workspace** — the coexistent field holding the components, within which the other three operate | scope |

**The workflow is a path through the tetrad:** start in **Eternity** (all possibilities shown)
→ **narrow in Hyparxis** (remove options, discriminate candidates) → **realise a single graph
in Chronos** (one surfaced system). Space is the workspace the whole path happens inside.

> **Open question — Space.** The user mapped list/graph/editor to Eternity/Chronos/Hyparxis
> and left Space unassigned. Proposed: **Space = the monad-as-container** (the bucket field in
> which components coexist and are laid out) — so entering a monad *is* entering its Space, and
> Eternity/Hyparxis/Chronos are the modes of viewing within it. Alternative: Space = the
> realised graph's spatial *layout* (coordinates/positions on the canvas). To confirm.

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

1. **Single scope.** Replace the `active_sequence` + `scope_members` pair with one
   `scope: Option<Vec<String>>` (the entered monad's members). Both views read it.
2. **Scoped list.** In list view, when `scope` is set, filter rows to the scope (discriminative
   list); order/type buttons filter within it. Un-scoped = global.
3. **Scoped graph + per-order candidates.** In graph view, the header steps orders present in the
   scope; for an order with one member, load it; for several, add prev/next to step candidates
   (Hyparxis). Track `active_order` + `candidate_index`.
4. **View toggle preserves scope + position.** Toggling list↔graph keeps `scope` and the current
   order/candidate; list highlights the graph's current member and vice-versa.
5. **Exit affordance.** Add an explicit "up / Nullad" control (breadcrumb) visible in both views.
6. **Editor = Hyparxis.** While assembling (join/decompose), stay scoped; new members appear as
   candidates in the current order. (Ties to [operations-and-workflows](operations-and-workflows.md).)

## v0.6 note
In the rebuild this is not bolted on: the view is a **functor over the scope** — Eternity, Time,
Hyparxis, Space are four readings of the one scoped set, chosen by the determining condition. The
monad (container = Space) holds the set; list/graph/editor are projections.
