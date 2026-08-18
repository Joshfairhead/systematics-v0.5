# Composable Perspectives — Fable-reviewed plan (supersedes the Opus draft)

## Context

The Opus draft proposed: referential modules, content-address-ready ids, an E/L/T
triad seeded as a K3, functor-as-data. Fable review (5 independent read-only code
lenses + adversarial refutation pass, 15 agents) confirmed the **core model** but
found the draft's *Current state* stale, its headline example wrong, and two of
its five build steps premature. This plan replaces it.

**Core model (upheld):** the addressed graph is the composable unit; files are
projections; composition = links/references targeting addresses (reference, not
copy). This is *already real* in the repo: DU3's module carries 0 systems and
composes purely by address; the Qualsystems course composes purely by links.

## Corrected current state (verified against code, commit 3ee15cc)

- The scraped corpus is **already secured**: 14 git-tracked module files under
  `backend/data/perspectives/` (68 references), loaded at startup
  (`main.rs:87` → `load_perspective_modules`, `data/mod.rs:37`), then re-marked
  canonical (`main.rs:90`). `store.json` does not exist. "Secure the data" is done.
- `exportPerspective(id)` already exists as a GraphQL **query** (`graphql/types.rs:366`).
- `export_perspective` (`graph.rs:403`) is **not** a deep-copy closure: it excludes
  canonical systems/characters. The canonical tetrad is *not* duplicated anywhere.

## The real defects (all verified, all currently live)

1. **`canonical_ids` conflates "bundled" with "canonical"** (critical). Loaded
   modules are re-marked canonical, so: re-exporting a loaded module **drops its
   own 16 systems/vocabs/characters** (round-trip data loss), and GraphQL edits to
   module content are **silently non-durable** (`user_content()` excludes them).
2. **Export is link-blind** (major). `graph.rs:413` harvests from References only,
   never `Perspective.links` — a perspective-of-perspectives (the course) exports
   as a bare shell; no dependency manifest exists.
3. **Ownership filter inconsistent** (major). `is_user` is applied to
   systems/characters but **not** vocabularies/sources/artefacts/lookups —
   `source_j_g__bennett` is duplicated across 5 module files *today* (the real
   duplication; not the one the draft claimed).
4. **Zero tests** over `load_perspective_modules`/the 14 modules/export round-trip;
   malformed modules are silently skipped (eprintln, no failure).
5. **Deploy gap** (critical for prod). Dockerfile never copies root `data/` (the
   `include_str!` seed can't even build in the image as-is); fly.toml has no
   volume; runtime-fs modules never reach production; all paths are cwd-relative
   (`./data/perspectives` only works when cwd = `backend/`).
6. **`#term:N` addresses are positional** — reordering a vocabulary's terms
   silently retargets every existing reference (document as invariant: term lists
   are append/replace-in-place only).

## Rulings on the 8 open questions

1. **Ownership vs content-hash:** ownership now; **no `id = hash(content)`** —
   addresses embed ids, so hash-ids would orphan every reference/link on every
   edit. Add an optional advisory `content_digest` field instead. (Precedent:
   `Link.id`/`Reference.id` are already content-derived — safe *because* they are
   immutable derived edges.)
2. **Module shape:** **keep typed `GraphContent` arrays**; homogeneous RDF-style
   format is churn without payoff. Add an additive `#[serde(default)] manifest`
   field (external addresses, built by scanning **both** links and references).
3. **Dependency resolution:** scan-on-load merge (upsert is deterministic under
   sorted load); manifest is a **validation/warn** pass, not the mechanism;
   dangling address = tolerated/lazy (an unloaded citation simply doesn't
   resolve — matches `references_for` semantics); cycles are fine (links are data).
4. **ELT:** Extract/Load are **code** (and mostly exist: export query + loader);
   Transform is the one genuinely **data-interpreted** operation. **Do not seed
   the E/L/T K3 now** — as specified it's decorative (nothing consumes it; its
   three connectives have no semantics; Transform-as-reconciling doesn't actually
   mediate E and L). Revisit when something reads it.
5. **Functor scope:** **same-grammar only**, stored as the **term/position
   permutation alone**; the connective map is **derived** from endpoints
   (`conn:min-max` canonicalized) — functorial by construction (automorphisms of
   K_n = S_n). Name the general `{from,to}` table **Mapping**; reserve **Functor**
   for the validated case (totality over source, sort preservation, derived-edge
   law) with an advisory `validateFunctor` query in `core/functors.rs`, mirroring
   the existing advisory `validate_system` pattern. Cross-grammar (dyad→triad) is
   explicitly deferred research, not v1.
6. **Canonical re-declaration:** split the fused meanings — `bundled_ids`
   (persistence filter) vs **canonical = a small per-order pointer table stored
   as data** (`order → system address`). Re-declaration = repoint. Only the 12
   archetypes + citation triad are canonical/immutable; module content is
   durable-but-editable.
7. **Persistence:** **keep `include_str!`** for canonical/citation seed (compile-time
   guarantee); modules stay runtime-fs locally; for prod either `include_dir!` the
   modules or fix Docker COPY + add a fly volume. Fix cwd-relative paths
   (resolve from a `SYSTEMATICS_DATA` root or the manifest dir).
8. **Holochain:** **emulate**; don't commit. Keep ids opaque except in the address
   parser; optional digest field; `Grammar::validate` is the zome analogue. Known
   losses (DHT, signatures, intrinsic dedup) accepted for now.

## Revised build path (ordering was backwards; tests first)

1. **Pin current behaviour (read-only, no code changes):** a test that mirrors
   `main.rs` exactly — `build_graph()` → `load_perspective_modules()` → second
   `mark_canonical()` — asserting 14 perspectives load, all 68 references resolve,
   and the 16 module-owned systems are present. Nothing else moves until green.
2. **The one structural fix:** introduce `bundled_ids` distinct from
   `canonical_ids` (graph.rs; `mark_canonical` → `mark_bundled` where appropriate)
   so loaded modules re-export losslessly and module edits persist. Canonical
   pointer table per ruling #6.
3. **Referential export done right:** apply ownership filtering to **all** entity
   kinds; harvest `Perspective.links` targets; emit the `manifest` field.
   Round-trip tests: export DU1 → fresh graph → reload → identical content;
   export the course → links + manifest intact (no longer a bare shell).
4. **Deploy fix:** Docker COPY of `data/`; fly volume or `include_dir!`; path-root
   env var. (Without this, everything above works only on a laptop.)
5. **Mapping/Functor v1 (same-grammar):** `core/functors.rs`, permutation-only
   storage, derived edges, advisory validation, `applyFunctor` → materialised
   perspective. *Only after 1–4.*

**Cut/deferred:** E/L/T seeded K3; cross-grammar functors; content-hash ids;
homogeneous serialization rewrite; Holochain commitment.

## Verification

- Step-1 pin test green before any change; re-run after each step.
- Round-trip: `exportPerspective("perspective_dramatic_universe_vol_1")` after a
  full startup equals the committed module file (systems included).
- Course export contains 8 `hasModule` links + manifest listing the 8 child
  perspective addresses; loading course-without-children warns but works;
  loading children later resolves.
- Edit a DU1 system via GraphQL → restart → edit survives (bundled ≠ frozen).
- `validateFunctor` rejects a partial or sort-violating Mapping; `applyFunctor`
  on a seeded S_3 permutation maps the DU3 triad onto a relabelled perspective;
  composing two functors equals the composed permutation.
- `cargo test --workspace` + UI smoke (12 systems render; tooltips still resolve).
