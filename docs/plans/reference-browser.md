# Reference Browser — design

## Context

The referencing corpus now spans multiple perspectives (DU1/DU2/DU3, Elementary
Systematics, with Hodgson + DU4 coming). The intent is to **keep every
non-canonical perspective and compare them** — "canonical" is just a bootstrap;
what matters is that each characterization *has a reference*, and that you can
see, side by side, how each source names the same system (e.g. the heptad's
coherence: DU1 "Structure" · DU3 "transformation" · Hodgson "Emergent structure"
· ES "integration"). Today references are only visible as hover tooltips inside
one system's graph — there is no way to browse all citations, sort/filter them,
or compare across perspectives. This builds that view.

## Current state (verified)

- **UI** (`frontend/src/app.rs`): single flow — `<aside>` `SystemSelector` +
  `<main>` `ApiGraphView` (the K-graph canvas). No tabs/routing. References
  reach the UI only via `fetch_references_for_system` → hover tooltips
  (`graph_view.rs`).
- **Data surface** (`backend/src/graphql/types.rs`):
  - `allReferences` → `GqlReference` already exposes `id`, `perspectiveRef`,
    `target`, `note`, nested `source{name,kind}`, `artefact{title,url}`,
    `lookup{locator}`.
  - `perspectives` → `{id,name}`; `systemsForOrder(order)` → **all** systems of
    an order (canonical *and* perspective-owned) with `coherence`, `name`,
    designations; `systemById(id)` → any `GqlSystem`.
- **Grouping key = ORDER, not a shared address.** Confirmed from the module
  files: DU1 cites its *own* `system:system_dramatic_universe_i_heptad_7`; DU3
  cites `system:system_canonical_*`. So cross-perspective comparison must group
  by the target system's **order**, then read each perspective's own system
  value — not by exact target string.
- Client models: `frontend/src/api/client.rs` `ReferenceView` (missing
  `perspective` + resolved target fields); `RefSource/RefArtefact/RefLookup`.

## Backend changes (small, additive) — `backend/src/graphql/types.rs`

Enrich `GqlReference` so `allReferences` is self-describing and both views need
**one** query. Add resolvers (mirroring the existing `source`/`artefact`/`lookup`
ones that use `graph_snapshot(ctx)`):

- `perspective_name: Option<String>` — `g.perspective(perspective_ref).name`.
- `target_fragment: String` — the part after `#` (`coherence`, `term:2`,
  `conn:1-2`, `term-designation`, …; empty = whole system). Pure string parse.
- `target_system: Option<GqlSystem>` — resolve `system:<id>` from `target` and
  return the `GqlSystem` (carries `order`, `name`, `coherence`, designations).
  Reuses `g.system(id)`; powers the compare matrix's per-perspective value.

No new list queries are needed — filter dropdown values are derived client-side
from the fetched references; the compare matrix joins on `target_system.order`.

## Frontend changes

1. **App-level view switch** (`frontend/src/app.rs`): add `view: AppView`
   (`Graph` | `References`) to `ApiApp` + `ApiAppMsg::SetView`. Render a
   persistent right-aligned **segmented control** at the top of `<main>` (above
   the breadcrumbs), switching between `<ApiGraphView>` and a new
   `<ReferenceBrowser>`. Lives outside the canvas so it works in both modes.

2. **New component** `frontend/src/components/reference_browser.rs`
   (register in `frontend/src/components/mod.rs`). Fetches enriched
   `allReferences` + `perspectives` once on mount. Two inner tabs:
   - **Table** (provenance/audit): rows = references; columns = *Perspective ·
     Source · Artefact · Locator · Target (order · system · fragment) · Note*.
     Click a header to sort; a filter bar with dropdowns (perspective, source,
     artefact, order, fragment-kind) built from the data's distinct values, plus
     a free-text search. All sorting/filtering is client-side over the fetched
     vec.
   - **Compare (by order)** (the comparative lens): derive the distinct
     `(perspective, target_system)` pairs from the references, group by
     `target_system.order` (rows = orders 1–12), columns = perspectives; each
     cell shows that perspective's `target_system.coherence` (and, for `#term:N`
     citations, the term) with its citation provenance (source/locator) as a
     tooltip. Empty cell = that perspective doesn't cover that order yet.
   - A row's target may be navigable back into the graph via the existing
     `on_navigate` callback (optional).

3. **Client** (`frontend/src/api/client.rs`): extend `ReferenceView` with
   `perspective: Option<RefPerspective{name}>`, `target_fragment`, and
   `target_system: Option<RefSystem{ id order name coherence ... }>`; add
   `fetch_all_references()` selecting the enriched fields. (Compare view reads
   everything from this one call — no per-order fetch needed.)

4. **Styling** (`frontend/styles/style.css`): a `.reference-browser` panel that
   scrolls within `<main>` under the viewport-fit constraint (body/app are
   `100vh; overflow:hidden`); a `.ref-table` (sticky header, zebra rows), a
   `.ref-filter-bar`, a `.view-switch` segmented control, and a
   `.compare-matrix` grid. Reuse existing overlay/toggle conventions (the
   edge-labels toggle, `.graph-header`).

## Verification

- Run backend (`cd backend && cargo run`) + frontend (`cd frontend && trunk
  serve`); open the app.
- Switch to **References** → Table shows all 68 references; sorting by Source /
  Artefact / Order works; filtering perspective=DU3 shows exactly its 14; text
  search narrows rows.
- **Compare** tab: the order-7 row shows DU1's heptad coherence in its column
  (and, once Hodgson/DU3 heptad citations land, theirs alongside) — each cell
  carrying its citation.
- Backend: `cargo test` green; a new resolver test asserts a `GqlReference` for
  a DU1 heptad citation resolves `perspectiveName="…Vol 1"`,
  `targetFragment="coherence"`, `targetSystem.order=7`.
- The duplicate DU2 artefact (short vs full title) is merged (data cleanup) so
  the artefact filter lists it once.

## Sequencing (related roadmap — not part of this design)

The **Hodgson** perspective build, **DU4** light applications perspective,
**canonical re-declaration** (heptad→transformation per DU3; Legacy perspective
for unsourced values; regen `canonical.json`), and **Bennett dedup** follow
established patterns (`.context/du1_perspective.py` + `exportPerspective` + seed
regen) and don't need this design pass. The browser gains value as more
perspectives land, but is independent and can be built now.
