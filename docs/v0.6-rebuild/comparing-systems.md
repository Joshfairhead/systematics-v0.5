# Comparing / relating systems (breadcrumb)

[← index](README.md) · related: [compositional-model](compositional-model.md), [validation](validation.md)

> **Breadcrumb, not spec.** Captures a distinction reached in conversation; the products /
> functor work it points to is **parked**.

Relating two systems splits into two axes. The key fact: **a spectrum sees *structure*, not
*labels*.**

## 1. Structural — spectral (blind to content)
The eigenvalue spectrum of a complete graph depends only on its order `n`, not on what its
terms *say*. So:

- It **distinguishes orders**: triad `K3 = {2, −1, −1}` vs tetrad `K4 = {3, −1, −1, −1}`
  (adjacency); or Laplacian `{0, 3, 3}` vs `{0, 4, 4, 4}`.
- It is **cospectral-blind within an order**: **Landry's triad and Bennett's triad are both
  `K3`**, so they have the *same* spectrum. The spectrum **cannot** tell them apart — comparing
  two triads by spectrum is degenerate.

Within the spectral axis, the two spectra do different jobs (the within/between dyad):
- **Laplacian = within** — the coherence/connectivity of *one* system (the coherence facet; see
  [validation](validation.md)).
- **Adjacency = between** — the currency of graph **products** that *combine* two systems:
  tensor `A × B` → eigenvalues **multiply** (`λᵢ·μⱼ`); Cartesian `A □ B` → **add** (`λᵢ + μⱼ`).

## 2. Semantic — functorial (the labels)
Comparing **Landry's triad vs Bennett's triad** — same `K3`, different content — is *not*
spectral. It is a **same-grammar functor / mapping**: a term-to-term (and connective-to-connective)
correspondence between two instances of the same archetype. This is the Functor/Mapping work,
and it is exactly what the spectrum is blind to.

## 3. Cross-order is containment, not distance
"Compare a triad to a tetrad" is not a flat comparison: a **tetrad contains four triads**
(`K4 ⊃ 4·K3`, since `C(4,3) = 4`), just as [decomposition](operations-and-workflows.md) breaks a
K_n into its faces. So relating across orders means asking *"is this triad one of the tetrad's
faces?"* — a subgraph/face relation. **Beyond scope for now.**

## Summary
| relate… | same order, diff content | diff order |
|---|---|---|
| **how** | functor / mapping (labels) | face / containment (K4 ⊃ 4 K3) |
| **spectrum?** | blind (cospectral) | distinguishes (different structure) |
