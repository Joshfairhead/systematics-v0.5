# Perspective / System / Sequence browser — design

## Context

The big-table reference browser was the wrong shape (too complex; now hidden
behind `SHOW_REFERENCES_VIEW=false`, kept as a detailed perspective browser).
The intended model is **another triad, and it's the browser's own selector**
(homoiconic — the UI mirrors the ontology):

> **Perspective** (`=`, reconciling) reconciles **System** (`−`) with
> **Sequence** (`+`).

- **System (`−`)** = one `K_n` (a triad, tetrad, …). The particular/bounded pole.
  Already first-class (`core/systems.rs`), already has a lens (the graph canvas).
- **Sequence (`+`)** = an *ordered series* — the extensive/reaching-out pole. Its
  members are **any addresses**: systems, *or* modules (perspectives) that
  themselves contain sequences, *or* other sequences → **sequences nest**. The
  bare **ordered skeleton**, no annotation. Not first-class yet: today it is
  implicit (the Qualsystems course = a Perspective with 8 `hasModule` links, no
  order field on `Link`).
- **Perspective (`=`)** = the skeleton **made rich**. It reconciles the two
  structural poles by *decorating* them: attaching **excerpts** (text blobs) and
  **references** (provenance) to the systems/sequences/components. Already
  first-class (`core/perspectives.rs`).

The browser lets you **load/view by perspective · system · sequence**, selector
in the **menu bar** (not floating over the canvas — the old switch collided with
the edge-labels toggle).

## Decisions (confirmed with the user 2026-07-27)

1. **Poles**: System `−`, Sequence `+`, Perspective `=`. ✓
2. **Sequence members = any address** → nesting of modules and sequences. ✓
3. **Provenance lives on the Perspective**, not the Sequence. The Perspective
   carries all referencing + excerpt data; a Sequence is pure ordered structure.
4. **Sequence lens = a breadcrumb-style strip** of its ordered members.
5. The **reference schema is itself a pluggable systematic module** — triadic
   (Source/Artefact/Lookup) today, upgradeable to tetradic/octadic/mixed later.
   (Design direction; v1 keeps the triad.)

## Sequence model (backend) — `core/sequences.rs`

```rust
pub struct Sequence {
    pub id: String,
    pub name: String,
    /// Ordered member addresses — `system:<id>`, `perspective:<id>` (a module),
    /// or `sequence:<id>` (nesting). Order = Vec position. Pure structure: no
    /// references or excerpts of its own (those live on the Perspective).
    pub members: Vec<String>,
}
```

- Address `sequence:<id>` (add to the `address` module + `Graph::resolves`).
- `GraphContent.sequences: Vec<Sequence>` (additive `#[serde(default)]`); wire
  through `apply_content` / `content_snapshot` / `user_content` / `ids` + the
  `bundled_ids` split — mirrors the `Functor` change exactly.
- `export_perspective` owns the sequences a perspective references and records
  external ones (and their unresolved members) in the manifest.

## Perspective as the reconciling container

A Perspective holds structural members **by address** in its link web
(`system:<id>`, `sequence:<id>`) and decorates them:

- **References** (existing): citation edges keyed to a component address
  (`system:…#coherence`, `system:…#term:2`, `sequence:<id>`, …). Already built.
- **Excerpts (new, v2)**: `Excerpt { id, target: <address>, text }` — a text blob
  attached to any component address. Added to `GraphContent.excerpts`, surfaced
  in the Perspective lens next to the thing it annotates.

This is the concrete System/Sequence vs Perspective distinction: the poles are
bare structure; the Perspective adds the interpretive layer.

## Reference system as a pluggable module (direction; v1 keeps the triad)

Today a `Reference` hardcodes `source_ref / artefact_ref / lookup_ref` — i.e. it
is an instance of **the Citation triad** (K3, dogfooded in `data/citation.json`).
The generalization: a reference is *an instance of whatever reference-system is
active* — triad (3 terms), tetrad (4), octad (8), or a mix — its fields being
that system's terms. v1 **keeps the triad as-is** (no churn); the design note is
that `Reference` should later resolve its shape from a chosen reference-system
module rather than three fixed fields. Flagged so v1 choices don't foreclose it
(e.g. keep the citation-triad addressable and don't over-couple the browser to
exactly three fields).

## The triadic browser (frontend)

- **Menu-bar selector**: a segmented control **Perspective | System | Sequence**
  in the sidebar (`.top-nav`, above the system list), not over the canvas. A
  second-level picker chooses which one.
- **System lens (`−`)**: the existing `ApiGraphView` graph canvas — unchanged.
- **Sequence lens (`+`)**: a **breadcrumb strip** of the sequence's ordered
  members (`Monad › Dyad › … › Dodecad`, or `QSM1 › QSM2 › …`). A member that is
  itself a sequence/module expands (nesting). Click a member → its System lens.
- **Perspective lens (`=`)**: the web made rich — its held systems + sequences,
  each with its references and (v2) excerpts. **Reuse the hidden reference
  browser** here, scoped to one perspective, as the "perspective detail" view.

## Course migration (backward-compatible)

Add `Sequence seq_qualsystems_course` with `members = [perspective:qsm1 …
perspective:qsm_workbook]` and a `hasSequence` link from the course perspective
to it (keep the existing `hasModule` links; additive). Seed a demonstration
`seq_du_progression` (`system:system_canonical_monad_1 … dodecad_12`). No module
file breaks (new fields are `#[serde(default)]`).

## Build path (phased; tests-first where it counts)

**v1 — Sequence + the triadic browser**
1. Backend `Sequence` core + `GraphContent.sequences` + Graph plumbing +
   `address::sequence` + `resolves`; unit tests (mirror `functors.rs`).
2. GraphQL Sequence type + CRUD + `sequences`/`sequenceById`; resolver test;
   introspection allowlist gains `Sequence`.
3. Seed the course + progression sequences; export to module files;
   export/load round-trip carries sequences (manifest).
4. Frontend menu-bar triadic selector + the breadcrumb Sequence lens; repurpose
   the hidden reference browser as the Perspective lens.

**v2 — Excerpts**: `Excerpt` component on perspectives, shown in the Perspective
lens.

**v3 — Pluggable reference module**: generalize `Reference` to an instance of a
chosen reference-system (triad → tetrad/octad/mix).

## Verification

- `cargo test` green (Sequence unit + GraphQL resolver + export/load round-trip).
- App: menu-bar selector switches lenses; Sequence lens shows the course as an
  ordered strip of its 8 modules and the DU progression as monad→dodecad; a
  member click lands on its System graph; the Perspective lens shows a
  perspective's references.
