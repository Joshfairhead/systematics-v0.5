# v0.6 refactor plan (roadmap)

[← index](README.md)

> **Draft roadmap, open to revision.** Sequences the rebuild. The *what* lives in the chunk
> docs; this is the *order* and the load-bearing decisions. v0.5 is a prototype kept as an
> indicator of intent (see [README](README.md)).

## The shape of v0.6
Systems are **constructed functionally** on a **Holochain substrate**, and every system is an
**archetype** (rules/vocabulary) with **instances** (filled K_n). The three pillars:

1. **Substrate** — content-addressed elements + links (Holochain integrity/coordinator zomes).
   The graph is data that describes itself (homoiconic); the hexads live *in* the graph as K6s
   whose terms are field keys (see [validation](validation.md)), not as Rust structs.
2. **Functional construction** — two primitives (`create-vertex`, `link-to-all`) + add/subtract;
   the handshaking-lemma generative sequence builds K_n; "compose" is the meta coordinator that
   groups primitives (see [compositional-model](compositional-model.md)).
3. **Archetype validation, split three ways** — see next section.

## Archetype validation — split into three (the current headline change)
The v0.5 equivalence work validates a **system archetype** (both faces together). v0.6 factors
this into three peer rules:

| rule | validates | against |
|---|---|---|
| **Topology archetype** | a topology instance | `TopologyHexad` (cardinality · eigenvalue · order · size · vertex ordinality · edge ordinality) |
| **Vocabulary archetype** | a vocabulary instance | `SystematicsHexad` (system · coherence · term designation · connective designation · term ordinality · connective ordinality) |
| **System archetype** | the two faces **book-matched** | the `EquivalenceHexad` (the equivalence that binds them) |

Topology and vocabulary become **first-class peers**, each authored/validated independently; the
system archetype is the equivalence between them. (v0.5 already has the hexads + the combined
rule in `core/equivalence.rs` + `core/hexadicsystems.rs`; v0.6 separates the two standalone
validations out.)

## The other pillars, as chunks
- **[compositional-model](compositional-model.md)** — functional construction, edge colouring.
- **[validation](validation.md)** — archetype ↔ instance, the two book-matched hexads.
- **[node-edge-inversion](node-edge-inversion.md)** — node↔edge duality (triad).
- **[comparing-systems](comparing-systems.md)** — relating systems (spectral vs functorial).
- **[view-model](view-model.md)** — the time-tetrad view model (Eternity/Chronos/Hyparxis/Space).
- **[operations-and-workflows](operations-and-workflows.md)** — conjunction/disjunction as
  drag-and-drop; all monads are heads; creation flows.

## Sequencing (proposed)
1. **Substrate first** — content-addressed store + link model; hexads-in-graph (the schema
   represented in the graph it describes). Everything else builds on this.
2. **Archetypes on the substrate** — the three validation rules (topology / vocabulary / system),
   reading the in-graph hexads.
3. **Functional construction** — `create-vertex` + `link-to-all` + add/subtract; generate K_n;
   the coordinator ("compose") groups them.
4. **View model** — Eternity/Chronos/Hyparxis/Space as functors over a scoped set; unify list +
   graph; retire the monad vs monad-head split.
5. **Operations** — conjunction/disjunction as drag-and-drop over a monad's Space; creation via
   "tetrad of X".
6. **Comparing / products** — parked until the above land (adjacency spectrum, graph products,
   functors between systems).

## Relationship to v0.5 work happening now
- The **equivalence/hexad** work (done in v0.5) is the seed of pillar 3's archetypes — carried
  forward, split into the three rules.
- The **view-model fix** is being done in v0.5 now (frontend only, see
  [view-model](view-model.md)) as an intent-proving prototype; v0.6 rebuilds it as functors.
