# Operations & workflows

[← index](README.md) · related: [compositional-model](compositional-model.md), [validation](validation.md)

## Operations
- **Conjunction (join / addition)** — combine systems into a higher complete graph on the
  union of their distinct terms (`K_m + K_n → K_{m+n}` when disjoint; dedup shared terms).
  The v0.5 prototype exposes this as multi-select + a **Join** button — a *proof of concept*.
  Intended surface is **drag-and-drop** (drag a node onto another to make a dyad; drag a node
  onto a dyad to make a triad; …), not select-and-button.
- **Disjunction (decompose / subtraction)** — break a system into its faces (a tetrad → its
  6 dyads + 4 triads). Prototype exposes a **Decompose** button (single-select).
- These two are the **Assembly / Decomposition** dyad, which parallels **Sort / Filter**
  (assembly/decompose move *across* orders; sort/filter arrange *within* an order).
- **Compose** is *not* one of these — it is the meta **coordinator** that groups the
  primitives (create-vertex, link, add/subtract). Do not name a single join "compose".
- **Store / Load** (write / read) and **CRUD** (Create · Delete · Update · Read) remain, at
  their respective granularities.

## Known prototype flakiness (to fix by rebuilding, not patching)
- **Join round-trip mislabels nodes.** Create a triad, Decompose it, then Join the parts:
  it recomposes a triad but the node labels come back wrong. Acceptable as a POC (it proves
  the mechanic) but not the final design.
- **Navigation is inconsistent.** An order-navigable sequence (one member per order) opens in
  graph view and the nav-bar order buttons unfold it, but list view doesn't scope to its
  members (it shows all systems of the order, higher greyed out). A bucket sequence behaves
  differently again (clicking it in list view stays on the Nullad). Root cause: a split
  between "order-navigable" and "bucket" handling — and a lingering distinction between a
  *monad* and a *monad head*.

## Workflows to design properly
- **All monads are heads.** Remove the monad / monad-head distinction: every monad heads a
  sequence and opens the same way. Entering a monad should *always* scope the view to its
  components (in both list and graph), with graph additionally allowing order-stepping.
- **Creating a monad.** Two natural entry points:
  1. **Define the monad first**, then add components and assemble upward.
  2. **Jump in with "tetrad of X"** — where X is *implicitly the monad*. Phrasings, most→least
     implicate: "Tetrad of Blue Earth" → "Blue Earth tetrad" → **"Blue Earth activity field"**
     (most likely). In the last, the *subject* is Blue Earth (the monad) and the *object* is
     "activity field" — which is the tetrad's **coherence/property** (cardinality 4/6 ⇒
     coherence "Activity Field", per [validation](validation.md)). So naming a system as
     "⟨monad⟩ ⟨coherence⟩" implicitly creates the monad and the K_n in one gesture.
- **Assembly surface.** Drag-and-drop composition/decomposition over a monad's components,
  with the edge-colouring from [compositional-model](compositional-model.md) making the
  construction legible.
</content>
