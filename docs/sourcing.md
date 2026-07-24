# Systematics Sourcing Plan & Source Registry

How we attach authoritative references to the systems, organise them as
**perspectives**, and decide what is canonical. The scraped work is durable in
git‑tracked module files (`backend/data/perspectives/*.json`), embedded into the
binary at build; the writable user store (`backend/data/store.json`, gitignored)
holds only genuinely new, unsaved user content. This doc is the map. See
`docs/plans/composable-perspectives.md` for the full infrastructure design.

## Modular file infrastructure (built)

Each perspective is a **module file** under `backend/data/perspectives/*.json`
(git‑tracked; one source = one file) — the durable home for the scraped work,
not the ephemeral user store.

- **Export** — `exportPerspective(id)` returns a self‑contained `GraphContent`
  bundle built by *reference, not copy* (`graph.export_perspective`):
  - the perspective (with its links) + its references;
  - only the entities it **owns** — a canonical/shared vocabulary, system,
    source, artefact or lookup is never copied in (the ownership filter applies
    to *every* kind);
  - a `manifest` of the **external addresses** it depends on but doesn't own
    (canonical systems, sibling perspectives, shared citation entities), so
    composition resolves by address once those homes load. An empty manifest is
    omitted, so files stay diff‑stable.
- **Load** — `data::load_perspective_modules` applies every module at startup,
  then `mark_bundled` (not a second `mark_canonical`). Resolution order:
  `$SYSTEMATICS_DATA/perspectives` → the source tree (fresh files in dev/CI,
  cwd‑independent via `CARGO_MANIFEST_DIR`) → the copy embedded into the binary
  via `include_dir!` (the deployed path — no filesystem, volume or
  working‑directory assumption).
- **Bundled vs. canonical** — two separate id sets:
  - `canonical_ids` = the immutable seed archetypes (the 12 per‑order systems +
    the citation triad + substrate). Export never copies these; they are the
    "one home" for shared structure.
  - `bundled_ids` = everything sourced from a file (seed **and** modules); a
    superset. Kept out of the writable user store, but module content is
    *editable and re‑exportable* — durable ≠ frozen.
- **Durability** — a module's home is its file. In‑app edits to a loaded module
  are session‑scoped until re‑exported back to the file; the user store is only
  for genuinely new content.

Composition is by address: DU3 owns 0 systems yet cites the canonical tetrad by
`system:…` address (recorded in its manifest); the Qualsystems course owns 0
systems and composes its 8 modules purely via `hasModule` links (recorded as
`perspective:…` manifest deps).

Verified: launched from any working directory, the backend loads all 14 modules
(68 references) — from the source tree in dev, or the embedded copy when
deployed with no filesystem present.

## The model — schema vs. reference

Every **source = a Perspective** (a reference web grouping). Whether a source
also gets its own *schema* depends on one rule:

> **Same words → reference the canonical system. Different words → new schema.**

- A source whose vocabulary **differs** from canonical gets its own `System`s +
  `Vocabulary`s (non‑canonical), each element cited to its page.
  → **DU1, DU2, Elementary Systematics, Hodgson/Qualsystems.**
- A source that **is** the canonical vocabulary attaches references **directly to
  the canonical systems** (no new schema).
  → **DU3** (Ch. 37 = the canonical source).

Addresses used for reference targets:
`system:<id>` · `#coherence` · `#term-designation` · `#connective-designation` ·
`#term:<n>` · `#conn:<p1>-<p2>`.

## Canonical re‑declaration (decided)

**DU3 Ch. 37 "The Structure of the World" is the authoritative canonical
reference.** Consequences:

1. Canonical values **DU3 confirms** → keep canonical, sourced to DU3:
   monad–hexad `coherence` (universality, complementarity, dynamism, activity,
   significance‑potentiality, coalescence), designations **Poles/Impulses/Sources**,
   tetrad terms **Ideal/Ground/Directive/Instrumental**, dodecad *perfection*.
2. Canonical values DU3 **contradicts / does not source** are currently
   **unsourced**: heptad *Generation*, octad *Self‑Sufficiency*, ennead
   *Transformation*, decad *Intrinsic Harmony*, undecad *Articulate Symmetry*,
   and the pentad **terms** (*Quintessence / Higher & Lower Potential / …*, 0 hits in DU3).
   → Move these into a **"Legacy (unsourced)"** non‑canonical perspective, and
   adopt DU3's names for canonical where DU3 gives them (DU3 says heptad→*transformation*,
   octad→*completedness*; confirm §§14.37.10–14.37.13). **Re‑declare** canonical
   vs non‑canonical once the full corpus is scraped.
3. Mechanism (deferred until scraping complete): edit the seed
   (`data/mod.rs` `canonical_coherence` / term tables) + regenerate
   `canonical.json`; create the Legacy perspective capturing the old values.

## Source registry

| Source (artefact) | Provides | Modelled as | Status |
|---|---|---|---|
| **DU1** – *Foundations of Natural Philosophy* | 12 systemic attributes (own names: Wholeness…Autocracy, Ch.2); Triad of Experience terms Function/Being/Will (Ch.3) | own schema — 12 `DU1` systems | ✅ done |
| **DU2** – *Foundations of Moral Philosophy* | Dyad (Fact/Value; Essence/Existence p.23); Triad = Will → Affirmation/Receptivity/Reconciliation (pp.85–89); Tetrad = Being (p.215) | own schema — 3 `DU2` systems + canonical dyad terms sourced | ✅ done |
| **DU3** – *Man and His Nature* | **Canonical source.** coherence + designations + tetrad terms (Ch.37 §§14.37.2, .7) | references on **canonical** systems | ◑ partial (monad–hexad, dodecad, tetrad terms, Poles/Impulses/Sources) |
| **DU4** – *History* | systems applied to history/society (triad, tetrad of activity, **heptad of completion**) — may supply heptad/octad terms | TBD (own schema if vocab differs) | ☐ to scrape |
| **Elementary Systematics** (ed. Seamon) | Triad: relatedness → affirming/receptive/reconciling (Table 3.1 p.38, Table A p.111) | own schema — `ES` triad | ✅ done (triad; monad/dyad/tetrad/pentad pending) |
| **Hodgson – Qualsystems** (*Qualitative Systems Thinking*, v12) | modern vocabulary, monad→octad+ (89 pp.) | own schema — `Hodgson` systems | ☐ to scrape |
| **Hodgson – Qualsystems Course** (QSM Workbook + Modules 1–7) | course articulations & worked examples per system | **nested perspectives** (Course → Module → System) | ◑ scaffold built |

### Qualsystems Course structure (nested perspectives)

Modelled with AD4M links (`hasModule`, `hasSystem`), since a Perspective node
can target another Perspective (webs of webs):

```
Qualsystems Course (perspective_qualsystems_course)
 └─ hasModule → QSM1 Number, Qualities, Perception   (intro)
 └─ hasModule → QSM2 The Basic Qualsystems           (monad–tetrad)
 └─ hasModule → QSM3 The Intermediate Qualsystems    (pentad, hexad, octad)
 └─ hasModule → QSM4 Developing / Qualtum / Unfolding of Structure
 └─ hasModule → QSM5 The Cosmic Laws of Unfolding
 └─ hasModule → QSM6 The Complex Qualsystems
 └─ hasModule → QSM7 Praxis
 └─ hasModule → QSM Workbook
        └─ hasSystem → each worked example (a System instance, cited to the module/page)
```

Scaffold (course + 8 module perspectives + `hasModule` links) is built; each
module is populated as it is scraped.

## Remaining work (roadmap)

1. **Scrape DU4** → its historical system applications; check for heptad/octad terms.
2. **Scrape Hodgson/Qualsystems** → a full non‑canonical perspective (its own term characters per system).
3. **Finish DU3**: confirm heptad/octad/ennead attribute names (§§14.37.10–.13); source what matches.
4. **Canonical re‑declaration**: Legacy perspective for the unsourced values; seed edit + regen; re‑declare.
5. **Reference browser** (final): a "References" tab over `allReferences`, sort/filter by **source · artefact · system · lookup**; also use it to **merge the duplicate DU2 artefact** (short vs full title).

## Corpus snapshot

68 references across the 14 module files: *Dramatic Universe* (early, 9), DU1
(27), DU2 (13), DU3 (14), Elementary Systematics (5); the 8 Qualsystems Course
modules carry links only (references added as each is scraped). 14 perspectives:
*Dramatic Universe* (early canonical citations), *Dramatic Universe Vol 1/2/3*,
*Elementary Systematics*, the *Qualsystems Course* + its 8 module perspectives.
16 module‑owned systems (DU1 ×12, DU2 ×3, ES ×1); the 12 canonical + citation
systems are seeded, not module‑owned.

## Known discrepancies / cleanups

- Heptad/octad attribute naming diverges by source: DU1 *individuality* (octad), DU3 *transformation*(heptad)/*completedness*(octad), ES *integration*(heptad)/*individuality*(octad), canonical *Generation/Self‑Sufficiency*.
- Pentad **terms** unsourced (not in DU1/DU3).
- Duplicate DU2 artefact (`The Dramatic Universe, Vol. 2` vs full title) → merge in the browser.
- Perspective naming: consolidate "Dramatic Universe" (early) into the Vol‑numbered perspectives.
- Shared `source_j_g__bennett` is currently copied into all 5 Bennett module
  files (and one artefact + 3 lookups sit in both the early *Dramatic Universe*
  and DU1 files). Export now dedups shared entities going forward (ownership
  filter + manifest), but the committed files were not regenerated — a one‑time
  data cleanup, best done together with the early‑perspective consolidation
  above.
