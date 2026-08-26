//! Template: the complete graph `K_n` — structural skeleton + validation rules.
//!
//! A `Template` is the *syntax* of a system: the invariant complete-graph
//! structure for one Order (it references the topological + geometric
//! substrate vocabularies) plus the arity rules a Vocabulary must satisfy to
//! fill it — a triad (`K_3`) has `order` terms and `C(order,2)` connectives
//! ("three impulses, three acts"). Grammars are deterministic per Order and are
//! seeded in code (like the substrate vocabularies), not persisted as content.

use serde::{Deserialize, Serialize};

use super::vocabularies::{Geometry, Vocabulary, Topology};

/// The complete graph `K_n` for one Order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Template {
    pub id: String,
    pub order: u8,
    pub topological_vocab_ref: String,
    pub geometric_vocab_ref: String,
}

impl Template {
    pub fn new(
        id: impl Into<String>,
        order: u8,
        topological_vocab_ref: impl Into<String>,
        geometric_vocab_ref: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            order,
            topological_vocab_ref: topological_vocab_ref.into(),
            geometric_vocab_ref: geometric_vocab_ref.into(),
        }
    }

    /// The canonical Template for an Order: `grammar_{order}`, wired to the
    /// canonical `topvocab_{order}` / `geovocab_{order}` substrate.
    pub fn for_order(order: u8) -> Self {
        Self::new(
            format!("grammar_{}", order),
            order,
            format!("topvocab_{}", order),
            format!("geovocab_{}", order),
        )
    }

    /// Number of terms (nodes) a Vocabulary must supply: one per vertex.
    pub fn expected_terms(&self) -> usize {
        self.order as usize
    }

    /// Number of connectives (edges) a Vocabulary must supply: `C(order, 2)`.
    pub fn expected_connectives(&self) -> usize {
        let n = self.order as usize;
        n * n.saturating_sub(1) / 2
    }

    // ---- Constraint-values the template prescribes (Stage A) ----
    //
    // `order` and `degree` are the two constraint-values held here (the Graph
    // Template is the Model's reconciler, and supplies the Controller its rules).
    // (In due course the template will also carry the other systematics variables
    // — coherence, designations — as a fuller template.)

    /// **Order** — the number of vertices (`n`). The first constraint-value.
    pub fn order(&self) -> u8 {
        self.order
    }

    /// **Degree** — the connectivity of every vertex in `K_n`: `n − 1` (each vertex
    /// joins all others). The second constraint-value.
    pub fn degree(&self) -> u8 {
        self.order.saturating_sub(1)
    }

    /// **Size** — the number of edges, `C(order, 2)` (an alias of
    /// `expected_connectives`, named for the graph-theory term).
    pub fn size(&self) -> usize {
        self.expected_connectives()
    }

    /// The `C(order,2)` edges as `(lo, hi)` vertex pairs, in the canonical
    /// lexicographic order `(1,2),(1,3),…,(n−1,n)` — the same order the render path
    /// and the frontend `nth_edge` use, so matrix columns line up with edge indices.
    pub fn edges(&self) -> Vec<(u8, u8)> {
        let mut es = Vec::with_capacity(self.expected_connectives());
        for p1 in 1..=self.order {
            for p2 in (p1 + 1)..=self.order {
                es.push((p1, p2));
            }
        }
        es
    }

    /// **Adjacency matrix** `A` (`n × n`, the Structural Topology as a matrix):
    /// `A[i][j] = 1` iff vertices `i+1` and `j+1` are joined by an edge. For the
    /// complete graph `K_n` this is `1` off the diagonal and `0` on it.
    pub fn adjacency_matrix(&self) -> Vec<Vec<u8>> {
        let n = self.order as usize;
        let mut a = vec![vec![0u8; n]; n];
        for (p1, p2) in self.edges() {
            let (i, j) = ((p1 - 1) as usize, (p2 - 1) as usize);
            a[i][j] = 1;
            a[j][i] = 1;
        }
        a
    }

    /// **Incidence matrix** `B` (`n × size`, vertices × edges — the **RECONCILER**):
    /// `B[v][e] = 1` iff vertex `v+1` is an endpoint of edge `e` (edges in `edges()`
    /// order). It bridges vertex-space and edge-space — both the **adjacency**
    /// (`A = BBᵀ − D`, topology) and the **line graph** (`L = BᵀB − 2I`, semantics)
    /// derive from it — so incidence is the `=` term that brings topology and
    /// semantics together (and where tensor products live).
    pub fn incidence_matrix(&self) -> Vec<Vec<u8>> {
        let n = self.order as usize;
        let edges = self.edges();
        let mut b = vec![vec![0u8; edges.len()]; n];
        for (e, (p1, p2)) in edges.iter().enumerate() {
            b[(*p1 - 1) as usize][e] = 1;
            b[(*p2 - 1) as usize][e] = 1;
        }
        b
    }

    /// **Line graph** `L` (`size × size`) — the **Semantic Projection**. Each edge
    /// (connective) becomes a vertex, and `L[e1][e2] = 1` iff edges `e1`, `e2` share
    /// an endpoint. Elevating **connectives (operators) to nodes** is what lets their
    /// **tensor products** compute as adjacencies (VSA / quantum-linguistics / an
    /// Edge-GNN "dual graph"): if `A —c1→ B —c2→ C`, the operator interaction is at
    /// `B`, and here `c1, c2` are *adjacent nodes*. The line graph is the **inverse**
    /// of the adjacency (topology); **incidence reconciles** the two, so `L` is
    /// composed from the incidence matrix (`L = BᵀB`, diagonal zeroed).
    pub fn line_graph(&self) -> Vec<Vec<u8>> {
        // COMPOSED from the incidence matrix `B` (which stands on its own): the
        // line graph is `L = Bᵀ·B` with the diagonal zeroed. `(Bᵀ·B)[i][j]` counts
        // the shared endpoints of edges `i, j` — 0 or 1 in a simple graph — so off
        // the diagonal it *is* the line-graph adjacency (two edges are adjacent iff
        // they share a vertex). Adjacency and incidence are the primitives; the line
        // graph is derived, not a parallel definition.
        let b = self.incidence_matrix(); // n × size
        let n = b.len();
        let m = b.first().map_or(0, |row| row.len());
        let mut l = vec![vec![0u8; m]; m];
        for i in 0..m {
            for j in 0..m {
                if i == j {
                    continue; // zero the diagonal (Bᵀ·B has 2 there — each edge's 2 ends)
                }
                let shared: u8 = (0..n).map(|v| b[v][i] & b[v][j]).sum();
                l[i][j] = shared.min(1);
            }
        }
        l
    }

    // ---- The morphism grammar: the matrices gate which morphisms are legal ----
    //
    // A grammar says which combinations are *well-formed*. The **morphism grammar**
    // says which morphisms are well-formed — and that is **read off the three
    // matrices**, one per morphism class (the corrected matrix→domain mapping):
    //   • adjacency `A` (−, topology)   → edge morphisms   (legal iff `A[a][b] = 1`)
    //   • incidence `B` (=, reconciler) → anchor morphisms (legal iff vertex ∈ edge)
    //   • line graph `L` (+, semantics) → connective composition (legal iff `L[e1][e2]=1`)
    // Nothing here is hard-coded: change the topology and the legal morphisms change
    // with it. This is what makes the matrices *load-bearing* rather than decorative.

    /// Grammar: is `index` a legal **vertex** site? Legal iff `1..=order` — the
    /// adjacency matrix has a row/column for it.
    pub fn admits_vertex(&self, index: u8) -> bool {
        index >= 1 && index <= self.order
    }

    /// Grammar: is `{a, b}` a legal **edge** site? Reads the **adjacency matrix**
    /// (`−`, topology): legal iff `A[a−1][b−1] = 1` — the pair is an edge of `K_n`
    /// (distinct, in range). A self-loop `(v,v)` or an out-of-range pair is
    /// ungrammatical — the topology itself forbids it.
    pub fn admits_edge(&self, a: u8, b: u8) -> bool {
        let n = self.order;
        if a < 1 || b < 1 || a > n || b > n || a == b {
            return false;
        }
        self.adjacency_matrix()[(a - 1) as usize][(b - 1) as usize] == 1
    }

    /// Grammar: may a term at vertex `v` **anchor** the edge `{a, b}`? Reads the
    /// **incidence matrix** (`=`, the reconciler): legal iff vertex `v` is an
    /// endpoint of that edge (`B[v−1][e] = 1`). Incidence is what relates
    /// vertex-space (terms) to edge-space (connectives), so it gates the anchors.
    pub fn admits_anchor(&self, v: u8, a: u8, b: u8) -> bool {
        let (lo, hi) = if a < b { (a, b) } else { (b, a) };
        let Some(e) = self.edges().iter().position(|&edge| edge == (lo, hi)) else {
            return false;
        };
        let b_mat = self.incidence_matrix();
        v >= 1 && (v as usize) <= b_mat.len() && b_mat[(v - 1) as usize][e] == 1
    }

    /// Grammar: may the **connectives** on edges `e1`, `e2` (indices into `edges()`)
    /// **compose**? Reads the **line graph** (`+`, the semantic projection): legal
    /// iff `L[e1][e2] = 1` — the two connective-operators share a vertex, so their
    /// tensor product is defined. NB the *operation* (the tensor product itself) is
    /// still to be designed; this is its **grammar** — *which* pairs may compose —
    /// which is live now and reads straight off `L`.
    pub fn admits_composition(&self, e1: usize, e2: usize) -> bool {
        let l = self.line_graph();
        e1 < l.len() && e2 < l.len() && l[e1][e2] == 1
    }

    /// Validate that a Vocabulary satisfies this Template's rules (order + arity).
    pub fn validate(&self, vocab: &Vocabulary) -> Result<(), Vec<String>> {
        let mut errs = Vec::new();
        if vocab.order != self.order {
            errs.push(format!(
                "Template {}: vocabulary '{}' order {} doesn't match grammar order {}",
                self.id, vocab.id, vocab.order, self.order
            ));
        }
        if vocab.terms.len() != self.expected_terms() {
            errs.push(format!(
                "Template {}: vocabulary '{}' has {} terms, expected {}",
                self.id,
                vocab.id,
                vocab.terms.len(),
                self.expected_terms()
            ));
        }
        if vocab.connectives.len() != self.expected_connectives() {
            errs.push(format!(
                "Template {}: vocabulary '{}' has {} connectives, expected {}",
                self.id,
                vocab.id,
                vocab.connectives.len(),
                self.expected_connectives()
            ));
        }
        if errs.is_empty() {
            Ok(())
        } else {
            Err(errs)
        }
    }

    /// Full structural validation against the resolved substrate + a Vocabulary:
    /// checks the referenced substrate matches by id/order and delegates arity
    /// to the substrate vocabularies and this Template's rules.
    pub fn validate_with(
        &self,
        topology: &Topology,
        geometry: &Geometry,
        vocab: &Vocabulary,
    ) -> Result<(), Vec<String>> {
        let mut errs = Vec::new();

        if topology.id != self.topological_vocab_ref {
            errs.push(format!(
                "Template {}: topological_vocab_ref '{}' doesn't match passed topology '{}'",
                self.id, self.topological_vocab_ref, topology.id
            ));
        }
        if geometry.id != self.geometric_vocab_ref {
            errs.push(format!(
                "Template {}: geometric_vocab_ref '{}' doesn't match passed geometry '{}'",
                self.id, self.geometric_vocab_ref, geometry.id
            ));
        }
        if topology.order != self.order {
            errs.push(format!(
                "Template {}: order {} doesn't match topology order {}",
                self.id, self.order, topology.order
            ));
        }
        if geometry.order != self.order {
            errs.push(format!(
                "Template {}: order {} doesn't match geometry order {}",
                self.id, self.order, geometry.order
            ));
        }

        if let Err(e) = topology.validate() {
            errs.extend(e);
        }
        if let Err(e) = geometry.validate() {
            errs.extend(e);
        }
        if let Err(e) = vocab.validate_against(topology) {
            errs.extend(e);
        }
        if let Err(e) = self.validate(vocab) {
            errs.extend(e);
        }

        if errs.is_empty() {
            Ok(())
        } else {
            Err(errs)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_order_arity() {
        let g = Template::for_order(3);
        assert_eq!(g.id, "grammar_3");
        assert_eq!(g.expected_terms(), 3);
        assert_eq!(g.expected_connectives(), 3);
        let tetrad = Template::for_order(4);
        assert_eq!(tetrad.expected_connectives(), 6);
    }

    #[test]
    fn test_validate_arity() {
        let g = Template::for_order(3);
        let ok = Vocabulary::new(
            "vocab_x_3",
            "X",
            3,
            vec!["a".into(), "b".into(), "c".into()],
            vec!["d".into(), "e".into(), "f".into()],
        );
        assert!(g.validate(&ok).is_ok());
        let bad = Vocabulary::new("vocab_y_3", "Y", 3, vec!["a".into()], vec![]);
        assert!(g.validate(&bad).is_err());
    }

    #[test]
    fn test_constraint_values() {
        let t = Template::for_order(4); // a tetrad, K_4
        assert_eq!(t.order(), 4); // vertices
        assert_eq!(t.degree(), 3); // each vertex joins the other 3
        assert_eq!(t.size(), 6); // C(4,2) edges
        // the monad edge-cases: no edges, degree 0
        let m = Template::for_order(1);
        assert_eq!(m.degree(), 0);
        assert_eq!(m.size(), 0);
    }

    #[test]
    fn test_edges_canonical_order() {
        // K_4 edges in lexicographic order, matching the render path / nth_edge.
        let t = Template::for_order(4);
        assert_eq!(
            t.edges(),
            vec![(1, 2), (1, 3), (1, 4), (2, 3), (2, 4), (3, 4)]
        );
    }

    #[test]
    fn test_adjacency_matrix_is_complete() {
        // K_3: all-1s off the diagonal, 0 on it.
        let a = Template::for_order(3).adjacency_matrix();
        assert_eq!(
            a,
            vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]]
        );
    }

    #[test]
    fn test_incidence_matrix() {
        // K_3, edges (1,2),(1,3),(2,3): each column has exactly two 1s (its endpoints).
        let b = Template::for_order(3).incidence_matrix();
        assert_eq!(
            b,
            vec![
                vec![1, 1, 0], // vertex 1 ∈ edges (1,2),(1,3)
                vec![1, 0, 1], // vertex 2 ∈ edges (1,2),(2,3)
                vec![0, 1, 1], // vertex 3 ∈ edges (1,3),(2,3)
            ]
        );
        // every edge column sums to 2 (two endpoints), for any order.
        let b4 = Template::for_order(4).incidence_matrix();
        for e in 0..6 {
            let col_sum: u8 = b4.iter().map(|row| row[e]).sum();
            assert_eq!(col_sum, 2);
        }
    }

    #[test]
    fn test_line_graph_reconciles_incidence() {
        // In K_3 every pair of edges shares a vertex, so L(K_3) = K_3 (all adjacent).
        let l = Template::for_order(3).line_graph();
        assert_eq!(l, vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]]);
        // K_4: edge (1,2) and edge (3,4) are disjoint → not adjacent in the line graph.
        let t = Template::for_order(4);
        let edges = t.edges();
        let l4 = t.line_graph();
        let i12 = edges.iter().position(|&e| e == (1, 2)).unwrap();
        let i34 = edges.iter().position(|&e| e == (3, 4)).unwrap();
        assert_eq!(l4[i12][i34], 0);
        let i13 = edges.iter().position(|&e| e == (1, 3)).unwrap();
        assert_eq!(l4[i12][i13], 1); // (1,2) & (1,3) share vertex 1
    }

    #[test]
    fn admits_edge_reads_the_adjacency_matrix() {
        let k3 = Template::for_order(3);
        // legal edges of K_3 (all off-diagonal pairs).
        assert!(k3.admits_edge(1, 2) && k3.admits_edge(1, 3) && k3.admits_edge(2, 3));
        assert!(k3.admits_edge(2, 1)); // order-independent (undirected)
        // ungrammatical: self-loop (on the diagonal → A = 0) and out-of-range.
        assert!(!k3.admits_edge(1, 1));
        assert!(!k3.admits_edge(1, 4));
        assert!(!k3.admits_edge(0, 2));
    }

    #[test]
    fn admits_vertex_reads_the_order() {
        let k3 = Template::for_order(3);
        assert!(k3.admits_vertex(1) && k3.admits_vertex(3));
        assert!(!k3.admits_vertex(0) && !k3.admits_vertex(4));
    }

    #[test]
    fn admits_anchor_reads_the_incidence_matrix() {
        let k3 = Template::for_order(3);
        // vertex 1 is an endpoint of edge (1,2); vertex 3 is not.
        assert!(k3.admits_anchor(1, 1, 2) && k3.admits_anchor(2, 1, 2));
        assert!(!k3.admits_anchor(3, 1, 2));
        // a non-edge has no incidence column → no vertex anchors it.
        assert!(!k3.admits_anchor(1, 1, 1));
    }

    #[test]
    fn admits_composition_reads_the_line_graph() {
        // K_3: every pair of connectives shares a vertex → all compose.
        let k3 = Template::for_order(3);
        for e1 in 0..3 {
            for e2 in 0..3 {
                if e1 != e2 {
                    assert!(k3.admits_composition(e1, e2));
                }
            }
        }
        // K_4: connectives on (1,2) and (3,4) are disjoint → they do NOT compose.
        let k4 = Template::for_order(4);
        let edges = k4.edges();
        let i12 = edges.iter().position(|&e| e == (1, 2)).unwrap();
        let i34 = edges.iter().position(|&e| e == (3, 4)).unwrap();
        let i13 = edges.iter().position(|&e| e == (1, 3)).unwrap();
        assert!(!k4.admits_composition(i12, i34)); // disjoint operators
        assert!(k4.admits_composition(i12, i13)); // share vertex 1
    }
}
