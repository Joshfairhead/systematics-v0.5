# Interpretation → Representation Transform (Functor CRUD)

> **Provenance**: The architectural moves in this document were reasoned by an
> AI assistant (Claude Opus 4.7) in dialogue with the human, who set direction
> and constraints. Treat the recommendations as AI-generated proposals, not
> human-authored decisions.

## Context

The high-level goal is a transform that takes unstructured input, **interprets** it, and produces a **structured representation** as a set of systems (the systematics data model). This standardises methodologies/credits so semantics, syntax, and semiotics live in comparable places — enabling cross-system comparison.

To reach that goal, the concrete groundwork is a **CRUD interface for functors** — pieces of code that map from a source (a vocabulary term, a dotted path, an interpretation) to a target (any anchor in the graph: a location, an order, a whole system). A single functor may hold anywhere from one mapping to a whole-sequence mapping; the taxonomy (monomorphism, polymorphism, endomorphism, homomorphism, holomorphism, etc.) is *emergent* from the shape of its mappings, not baked into the type.

For a functor's target to be *precisely* addressable, the anchor model must grow. Today `Location = (Order, Position)` and can only address an **entry**. To also address a **connective**, `Location` needs an optional second position. When a `Location` binds one position it is an *entry-shaped anchor*; when it binds two, it is a *link-shaped anchor*. This preserves the entry/link distinction that Holochain uses in its core data model (relevant for a future port), while unifying entries and links under a single `Location` type.

Fixed order metadata (`SystemName`, `CoherenceAttribute`, `TermDesignation`, `ConnectiveDesignation`) already exists at `backend/src/core/entries.rs` and is populated for all 12 orders at `backend/src/data/mod.rs:111-198`. That work is done — the plan only affirms its immutability.

## Current architecture (verified)

- **Rust workspace**: `backend/` (Axum + async-graphql, read-only), `middleware/` (shared wire types), `frontend/` (Yew/WASM).
- **Anchors**: `Order`, `Position`, `Location` at `backend/src/core/entries.rs:41-131`. `Location` = `{ id, order, position }` — no secondary position.
- **Connectives today**: modelled as `Link` entries (edges, not `Entry`s) at `backend/src/core/links.rs:18-31`, connecting two `Location` IDs with a `Character` ID in `tag`. Populated in `data/mod.rs:406-455`.
- **Terms**: `Term { id, location, character }` at `entries.rs:350-397` — reference `Location` only.
- **Graph**: in-memory `Graph { entries, links }` at `graph.rs:20-24`, constructed once by `build_graph()` at `data/mod.rs:19-48`.
- **GraphQL**: `QueryRoot` only — no mutations exist yet.
- **No Functor / Interpretation / vocabulary-mapping type exists.**

## Recommended approach (four-part rollout)

### Part 1 — Extend `Location` so it can be entry-shaped or link-shaped

`Location` remains the single anchor type. Its *shape* (entry vs link) is determined by whether it binds one or two positions.

**File**: `backend/src/core/entries.rs:100-131`

```rust
pub struct Location {
    pub id: String,
    pub order: String,                       // "order_3"
    pub position: String,                    // "position_1"
    pub position_secondary: Option<String>,  // Some("position_2") ⇒ this Location is link-shaped
}
```

- ID convention:
  - entry-shaped: `loc_3_1` (order 3, position 1) — unchanged
  - link-shaped: `loc_3_1_2` (order 3, connecting position 1 and position 2)
- Constructors:
  - `Location::new(order, position)` — kept; produces entry-shaped Locations.
  - `Location::link(order, p1, p2)` — new; produces link-shaped Locations. Enforces `p1 < p2` for a canonical undirected form.
- Helpers: `is_entry()`, `is_link()`, `position_secondary_value()`.
- Seeding (`data/mod.rs:69-75` `add_locations`) also emits every link-shaped Location: one per undirected pair `(p1 < p2)` for each order 1–12. Counts: `entries = Σ(1..=12) = 78`, `links = Σ C(n,2) for n∈1..=12 = 220`.
- **Wire type** (`middleware/src/types/entries.rs`): if a `Location` GraphQL type exists there, add the optional `position_secondary` field; otherwise no wire change needed.

### Part 2 — Point content types at the extended `Location`

`Term { id, location, character }` at `entries.rs:350-397` already references `Location` by ID string. No struct change — the same `Term` can now target either an entry-shaped location (`loc_3_1`) or a link-shaped location (`loc_3_1_2`). Existing terms continue to work unchanged.

- Add convenience constructor `Term::for_link(order, p1, p2, character_id)` producing `id = "term_{order}_{p1}_{p2}"`, `location = "loc_{order}_{p1}_{p2}"`.
- Add graph queries at `graph.rs`:
  - `term_at_link_location(order, p1, p2) -> Option<&Term>`
  - `terms_for_order_links(order) -> Vec<&Term>`
- **Existing `Link` (edge) type stays**: it's still the transport mechanism the frontend renders. Data migration in `data/mod.rs:406-455` continues to emit `Link::connective(...)` edges *and* — in parallel — a `Term::for_link(...)` at the link-shaped location, giving one unified "vocabulary → location" mapping mechanism regardless of shape.

### Part 3 — Introduce `Functor` as a first-class data type + apply/CRUD

A `Functor` is data (a stored mapping table) plus a small piece of code (`apply`) that materialises `Term`s from it.

**Core type** (new file `backend/src/core/functors.rs`; register in `backend/src/core/mod.rs`):

```rust
pub struct Functor {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub source_language: Option<Language>,  // vocabulary tag if source is Character-shaped
    pub mappings: Vec<FunctorMapping>,
}

pub struct FunctorMapping {
    pub base: String,    // any entry ID: character ID ("char_canonical_will")
                         // OR a dotted path ("aspectsofexperiencetriad.triad")
    pub target: String,  // any anchor ID: "loc_3_1", "loc_3_1_2", "order_3", ...
}
```

- `base` and `target` are opaque strings that reference existing entries — Location, Order, Character, System, etc. Kind of functor (mono/poly/endo/homo/holo) is derived from the *shape of the mapping set*, not encoded in the type. Future classifier methods can live alongside (`Functor::kind()`).
- `Functor::apply(&self, graph: &mut Graph) -> Vec<Term>`: for each mapping whose target is a Location ID, produce a `Term { id: derived, location: target, character: base }` and add it to the graph. Mappings whose target is a higher-level anchor (e.g. `order_3`, a whole system) are recorded but not directly materialised as `Term`s — they represent structural intent for later expansion.

**Wire type** (new file `middleware/src/types/functors.rs`): mirror `Functor` and `FunctorMapping` with `#[cfg_attr(feature = "server", derive(SimpleObject))]`.

**Graph storage** (`backend/src/core/graph.rs:20-24`): add `pub functors: Vec<Functor>` alongside `entries` and `links`. Add queries `functors()`, `functor(id)`, `functor_by_name(name)`.

**GraphQL CRUD** (`backend/src/graphql/types.rs`):
- Add a `MutationRoot`; expose via `Schema::build(QueryRoot, MutationRoot, EmptySubscription)` in `backend/src/main.rs`.
- Queries: `functor(id)`, `functors()`.
- Mutations:
  - `createFunctor(input) -> Functor`
  - `updateFunctor(id, input) -> Functor`
  - `deleteFunctor(id) -> Boolean`
  - `applyFunctor(id) -> [Term]` — the interpretation → representation transform.
- Wrap the shared graph as `Arc<RwLock<Graph>>` in `main.rs` so mutations can write. **In-memory only in this pass** — persistence is deliberately a separate concern (input format ≠ storage format, as clarified).

### Part 4 — Affirm fixed order metadata

The four per-order attributes are already fixed and populated:
- `SystemName` (`entries.rs:180`), `CoherenceAttribute` (`entries.rs:236`), `TermDesignation` (`entries.rs:273`), `ConnectiveDesignation` (`entries.rs:310`).
- All 12 orders populated at `data/mod.rs:113-197`.

Change required: **none in code**. To lock the invariant, the new `MutationRoot` deliberately exposes no create/update/delete for these four types. Add a short doc-comment at the top of `entries.rs` calling out that these are immutable schema-level metadata.

## Critical files to modify

| Purpose | Path |
|---|---|
| Extend `Location` | `backend/src/core/entries.rs:100-131` |
| Add `Term::for_link` helper | `backend/src/core/entries.rs:359-397` |
| Wire type update (if present) | `middleware/src/types/entries.rs` |
| Seed link-shaped locations | `backend/src/data/mod.rs:69-75` |
| Emit `Term::for_link` alongside `Link::connective` | `backend/src/data/mod.rs:402-497` |
| New `Functor` core type | `backend/src/core/functors.rs` (new) |
| Register module | `backend/src/core/mod.rs` |
| New Functor wire type | `middleware/src/types/functors.rs` (new) |
| Add `functors` field + queries | `backend/src/core/graph.rs:20-24` |
| Add `MutationRoot` + Functor GraphQL types | `backend/src/graphql/types.rs` |
| Wire mutations into schema, wrap graph | `backend/src/main.rs` |

## Reuse & non-changes

- `Character`, `Language`, `Term`, `Link`, all order-metadata types unchanged.
- `Graph::add_entry`, `Graph::get_character`, `Graph::term_at_location` all continue to work as-is.
- Existing `Link`-based rendering keeps working; Part 2 adds a `Term` alongside each existing `Link::connective` so vocabulary lookup is unified without breaking the edge model.

## Out of scope for this pass

- **Persistence** of functors (in-memory only; hot-reload loses state). Storage backend is deliberately separated per user's guidance ("how data has been mapped is to be stored is another question").
- **Input formats** for functors (JSON, YAML, dotted-path DSLs) — the CRUD accepts the direct `Functor` shape; higher-level ingestion (LLM, parsers) layers on top later.
- **Functor taxonomy classification** (`Functor::kind() -> Monomorphism | Homomorphism | ...`) — leave classification to a follow-up once real mappings exist to characterise.
- **New vocabularies** for `Language::Energy | Values | Society` — functors *are* the mechanism for creating them; that work follows this feature.

## Verification

1. **Unit tests** (`cargo test -p backend`):
   - `Location::link(3, 1, 2)` → `id == "loc_3_1_2"`, `is_link() == true`, `position_secondary_value() == Some(2)`.
   - `Location::new(3, 1)` → `is_entry() == true`, `position_secondary_value() == None`.
   - `Term::for_link(3, 1, 2, "char_canonical_generation")` produces `id == "term_3_1_2"`, `location == "loc_3_1_2"`.
   - `Functor::apply` on a 3-mapping Triad functor produces 3 `Term`s at the correct locations.
   - `build_graph()` produces 78 entry-shaped and 220 link-shaped Locations across orders 1–12.
2. **GraphQL smoke test** (`cargo run -p backend`, then GraphiQL/curl):
   - `mutation { createFunctor(input: { name: "canonical-triad", mappings: [...] }) { id } }` returns a Functor.
   - `mutation { applyFunctor(id: "...") { id location character } }` returns the materialised Terms.
   - `query { functors { id name } }` lists it.
   - `mutation { deleteFunctor(id: "...") }` returns `true`; subsequent `functors` list omits it.
3. **Frontend regression**: `cargo run` the Yew app; Triad, Tetrad, Pentad renderings should be pixel-identical to today (link-shaped locations don't affect rendering unless explicitly queried).
4. **No mutations for fixed metadata**: GraphQL schema introspection shows only `functor*` under `MutationRoot` — no mutations for `SystemName`, `CoherenceAttribute`, `TermDesignation`, `ConnectiveDesignation`.
