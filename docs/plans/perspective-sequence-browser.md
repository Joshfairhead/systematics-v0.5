# Perspective / System / Sequence browser — design (proposal)

## Context

The big-table reference browser was the wrong shape (too complex; now hidden
behind `SHOW_REFERENCES_VIEW=false`, kept as a detailed perspective browser).
The intended model is **another triad, and it's the browser's own selector**
(homoiconic — the UI mirrors the ontology):

> **Perspective** (`=`, reconciling) reconciles **System** (`−`) with
> **Sequence** (`+`).

- **System** = one `K_n` (a triad, tetrad, …). Already first-class
  (`core/systems.rs`), already has a lens (the graph canvas `ApiGraphView`).
- **Sequence** = an *ordered series* of systems (the Qualsystems course's
  modules; DU's monad→dodecad progression; the enneagram). **Not first-class
  yet** — today it's implicit: `perspective_qualsystems_course` is a Perspective
  with 8 `hasModule` links and **no order field** on `Link`; DU3 cites an ordered
  run of canonical systems via `cites`. Order is only Vec insertion order.
- **Perspective** = the all-inclusive web that holds systems *and* sequences
  (and references/excerpts). Already first-class (`core/perspectives.rs`).

The browser should let you **load/view by perspective · system · sequence**, with
the selector in the **menu bar** (not floating over the canvas — the old switch
collided with the edge-labels toggle).

## Approaches considered

- **A — first-class `Sequence` type** (recommended). A small new core type, a
  peer of `System`, addressable as `sequence:<id>`; a Perspective holds it by
  address.
- **B — ordered links, no new type.** Add `index` to `Link`; a sequence = a
  Perspective's ordered member-links. Minimal, but a sequence isn't its own
  addressable/citable entity, and a perspective holding several sequences needs
  disambiguation.
- **C — Sequence as a tagged Perspective.** Maximal unification, but it collapses
  the very distinction the triad draws (System `−` and Sequence `+` are *distinct
  poles*, not both perspectives; a System is a `K_n`, not a web).

**Recommendation: A.** The user names System and Sequence as *distinct poles*
reconciled by Perspective, so Sequence should be a first-class peer of System —
not a Perspective (C) nor a mere link pattern (B). Sequences are also objects of
study in their own right (the enneagram, the progression). A grafts B's insight
(members are an *ordered list of addresses*) and C's (a Sequence is *addressable
and composable* into a Perspective's web). It follows the exact pattern already
used cleanly for `Functor` and `System`, so the plumbing is routine.

## Sequence model (backend) — `core/sequences.rs`

```rust
pub struct Sequence {
    pub id: String,
    pub name: String,
    /// Ordered member addresses — usually `system:<id>`, but any address
    /// (`perspective:<id>`, even `sequence:<id>`) so a course can sequence
    /// modules and sequences can nest. Order = Vec position.
    pub members: Vec<String>,
}
```

- Address form `sequence:<id>` (add to the `address` module + `Graph::resolves`).
- `GraphContent.sequences: Vec<Sequence>` (additive, `#[serde(default)]`); wire
  through `apply_content` / `content_snapshot` / `user_content` / `ids` and the
  `bundled_ids` split — mirrors the `Functor` change exactly.
- A **Perspective holds a Sequence** by a link targeting `sequence:<id>` (e.g.
  predicate `hasSequence`), so `Perspective(=)` genuinely reconciles
  `System(−)` + `Sequence(+)` as addressable members of its web.
- `export_perspective` already harvests link/reference addresses into the
  manifest; extend it to *own* the sequences a perspective references (like
  systems) and record external ones in the manifest.

## GraphQL surface — `graphql/types.rs`

- Query: `sequences`, `sequenceById(id)`, and a resolved `SequenceView` whose
  members resolve to `{address, kind, order?, name?}` (reuse the target-resolution
  pattern from the enriched `GqlReference`).
- Mutations: `createSequence(input)`, `updateSequence(id,input)`,
  `deleteSequence(id)` (mirror `Functor` CRUD).
- The introspection allowlist test gains `"Sequence"`.

## The triadic browser (frontend)

- **Menu-bar selector**: a segmented control **Perspective | System | Sequence**
  in the sidebar (`.top-nav`, above the system list), not floating over the
  canvas. Second-level picker chooses which one.
- **System lens** (`−`): the existing `ApiGraphView` graph canvas — unchanged.
- **Sequence lens** (`+`): render a sequence's ordered members as a *series* — a
  horizontal strip of small system cards/mini-graphs in order (reuse the graph
  renderer at small scale, or a compact card per member with order index). Click
  a member → jump to its System lens.
- **Perspective lens** (`=`): the perspective's web — its held systems +
  sequences + its references/excerpts. **Reuse the hidden reference browser**
  here (scoped to one perspective) as the "perspective detail" view.

## Course migration (backward-compatible)

Add a `Sequence` `seq_qualsystems_course` with `members = [perspective:qsm1 …
perspective:qsm_workbook]`, and a `hasSequence` link from
`perspective_qualsystems_course` to it. Keep the existing `hasModule` links
(additive; can be retired later). Seed a demonstration `seq_du_progression`
(`system:system_canonical_monad_1 … dodecad_12`) to show the canonical
progression. No existing module file breaks (new field is `#[serde(default)]`).

## Build path (tests-first where it counts)

1. **Backend `Sequence` core** + `GraphContent.sequences` + Graph plumbing +
   `address::sequence` + `resolves`; unit tests (mirror `functors.rs`).
2. **GraphQL** Sequence type + CRUD + `sequences`/`sequenceById`; a resolver
   test; update the introspection allowlist.
3. **Seed** the course + progression sequences (data), export to module files.
4. **Frontend** menu-bar triadic selector + Sequence lens; repurpose the hidden
   reference browser as the Perspective lens.
5. **Export/load** carry sequences (manifest); round-trip test.

## Open questions (confirm before building)

1. **Pole reading**: System `−` (receptive/particular — a bounded `K_n`),
   Sequence `+` (affirming/extensive — a reaching-out series), Perspective `=`.
   Right, or flipped?
2. **Sequence members**: systems only, or any address (systems + perspectives +
   sequences)? (Recommend *any address* — needed for the course, enables nesting.)
3. **Does a Sequence carry its own references/excerpts**, or only Perspectives?
   (Recommend Sequence stays lean = ordered members; provenance lives on the
   Perspective.)
4. **Sequence lens rendering**: mini-graph strip vs. compact ordered cards for v1?
