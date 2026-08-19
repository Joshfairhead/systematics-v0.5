//! The six laws of three — the Controller's morphisms.
//!
//! A triad's three impulses (affirming **+**, receptive **−**, reconciling **=**)
//! can be read in `3! = 6` orders. Those six orderings are Bennett's **six laws of
//! three**, and together they are the symmetric group `S₃`. Each law is a
//! **morphism** — a permutation of the three positions — and the Controller applies
//! them to turn the *undirected* base triad into a *directed* reading: **direction
//! is broken by the law, not stored in the base**. `SPO` is one law (interaction,
//! `132`). The six are laid out as a **Hexad** (order 6) with fixed colours.
//!
//! This generalises `functors.rs` (which implements a single `Sₙ` morphism) to the
//! full six for a triad: `Law::as_functor` hands each law to that same machinery.

use super::functors::Functor;

/// One of the six laws of three (an element of `S₃`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Law {
    Expansion,
    Identity,
    Order,
    Interaction,
    Concentration,
    Freedom,
}

impl Law {
    /// The six laws in **Hexad-position order** (positions 1..=6, with colours):
    /// 1 red identity · 2 blue expansion · 3 yellow order · 4 green freedom ·
    /// 5 purple interaction · 6 orange concentration. *(Layout per the user,
    /// 2026-08-18; the hexad itself may still flip — separate inquiry.)*
    pub const HEXAD: [Law; 6] = [
        Law::Identity,      // 1 red
        Law::Expansion,     // 2 blue
        Law::Order,         // 3 yellow
        Law::Freedom,       // 4 green
        Law::Interaction,   // 5 purple  (alias SPO)
        Law::Concentration, // 6 orange
    ];

    /// The law's permutation in one-line notation over positions `1..=3`:
    /// `permutation()[i]` is the position read into slot `i`. (Verified against
    /// the S₃ table in `docs/design-intent.md`.)
    pub fn permutation(&self) -> [u8; 3] {
        match self {
            Law::Expansion => [1, 2, 3],     // e (identity permutation)
            Law::Identity => [2, 3, 1],      // 3-cycle
            Law::Order => [3, 1, 2],         // 3-cycle
            Law::Interaction => [1, 3, 2],   // transposition — SPO
            Law::Concentration => [2, 1, 3], // transposition
            Law::Freedom => [3, 2, 1],       // transposition
        }
    }

    /// The permutation as a `Vec<u8>` for the `Functor`/`Sₙ` machinery.
    pub fn permutation_vec(&self) -> Vec<u8> {
        self.permutation().to_vec()
    }

    /// Bennett's name for the law.
    pub fn name(&self) -> &'static str {
        match self {
            Law::Expansion => "expansion",
            Law::Identity => "identity",
            Law::Order => "order",
            Law::Interaction => "interaction",
            Law::Concentration => "concentration",
            Law::Freedom => "freedom",
        }
    }

    /// Hexad position `1..=6` (see `HEXAD`).
    pub fn hexad_position(&self) -> u8 {
        (Self::HEXAD.iter().position(|l| l == self).unwrap() + 1) as u8
    }

    /// The law's Hexad colour.
    pub fn colour(&self) -> &'static str {
        match self.hexad_position() {
            1 => "red",
            2 => "blue",
            3 => "yellow",
            4 => "green",
            5 => "purple",
            _ => "orange",
        }
    }

    /// Alternative names for the law (e.g. interaction ← `SPO`).
    pub fn aliases(&self) -> &'static [&'static str] {
        match self {
            Law::Interaction => &["SPO"],
            _ => &[],
        }
    }

    /// Even (a rotation — the cyclic subgroup `A₃`) vs odd (a reflection — a
    /// transposition). `expansion` is the group identity `e`.
    pub fn is_even(&self) -> bool {
        matches!(self, Law::Expansion | Law::Identity | Law::Order)
    }

    /// The **directed reading** of an (undirected) triad under this law: reorder
    /// its three terms by the permutation. `read[i] = triad[permutation[i] − 1]`.
    /// e.g. `interaction` on `[Will, Function, Being]` → `[Will, Being, Function]`
    /// = subject · predicate · object (affirming · reconciling · receptive) — SPO.
    pub fn read<T: Copy>(&self, triad: &[T; 3]) -> [T; 3] {
        let p = self.permutation();
        [
            triad[(p[0] - 1) as usize],
            triad[(p[1] - 1) as usize],
            triad[(p[2] - 1) as usize],
        ]
    }

    /// Read an **undirected** triad as a **directed closed walk** under this law —
    /// the substrate move: the base holds no direction; the *law supplies it at read
    /// time*. Threads the three nodes (in law order) through the undirected edges
    /// between consecutive nodes, closing the loop:
    /// `[node, edge, node, edge, node, edge]` (the last edge returns to the first).
    /// `edges` are in canonical order `[(1,2), (1,3), (2,3)]`. e.g. `interaction`
    /// over nodes `[Will, Function, Being]`, edges `[generation, decision, consent]`
    /// → `[Will, decision, Being, consent, Function, generation]`.
    pub fn read_walk<T: Copy>(&self, nodes: &[T; 3], edges: &[T; 3]) -> [T; 6] {
        let [a, b, c] = self.permutation();
        [
            nodes[(a - 1) as usize],
            edges[triad_edge_index(a, b)],
            nodes[(b - 1) as usize],
            edges[triad_edge_index(b, c)],
            nodes[(c - 1) as usize],
            edges[triad_edge_index(c, a)],
        ]
    }

    /// Recover a law from its one-line permutation.
    pub fn from_permutation(p: [u8; 3]) -> Option<Law> {
        Law::HEXAD.into_iter().find(|l| l.permutation() == p)
    }

    /// Group composition in `S₃`: `self ∘ other` (apply `other` first, then
    /// `self`) — matching the `row ∘ col` multiplication table in the design doc.
    pub fn compose(&self, other: &Law) -> Law {
        let (s, o) = (self.permutation(), other.permutation());
        let r = [
            s[(o[0] - 1) as usize],
            s[(o[1] - 1) as usize],
            s[(o[2] - 1) as usize],
        ];
        Law::from_permutation(r).expect("S₃ is closed under composition")
    }

    /// This law as a same-grammar `Functor` (an `S₃` morphism) on an order-3
    /// system — the bridge to the existing morphism machinery. Each of the six
    /// laws is one such morphism; the Controller is the set of all six.
    pub fn as_functor(
        &self,
        id: impl Into<String>,
        source_ref: impl Into<String>,
        target_ref: impl Into<String>,
    ) -> Functor {
        Functor::new(id, self.name(), 3, source_ref, target_ref, self.permutation_vec())
    }
}

/// The canonical index of the undirected edge `{a, b}` (positions `1..=3`) in a
/// triad's edge list `[(1,2), (1,3), (2,3)]`. Order-independent (the base is
/// undirected): `{a,b}` and `{b,a}` return the same index.
fn triad_edge_index(a: u8, b: u8) -> usize {
    let (lo, hi) = if a < b { (a, b) } else { (b, a) };
    match (lo, hi) {
        (1, 2) => 0,
        (1, 3) => 1,
        (2, 3) => 2,
        _ => unreachable!("triad positions are 1..=3, a != b"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hexad_layout() {
        // 1 red identity · 2 blue expansion · 3 yellow order · 4 green freedom ·
        // 5 purple interaction · 6 orange concentration.
        let expect = [
            (1, "red", Law::Identity),
            (2, "blue", Law::Expansion),
            (3, "yellow", Law::Order),
            (4, "green", Law::Freedom),
            (5, "purple", Law::Interaction),
            (6, "orange", Law::Concentration),
        ];
        for (pos, colour, law) in expect {
            assert_eq!(law.hexad_position(), pos);
            assert_eq!(law.colour(), colour);
        }
    }

    #[test]
    fn test_even_odd_split() {
        // rotations {expansion, identity, order} even; reflections odd.
        assert!(Law::Expansion.is_even() && Law::Identity.is_even() && Law::Order.is_even());
        assert!(!Law::Interaction.is_even() && !Law::Concentration.is_even() && !Law::Freedom.is_even());
    }

    #[test]
    fn test_spo_is_interaction_reading() {
        // interaction = SPO: subject (affirming) · predicate (reconciling) · object
        // (receptive). Canonical triad nodes 1=Will(+) 2=Function(−) 3=Being(=).
        assert_eq!(Law::Interaction.aliases(), &["SPO"]);
        let triad = ["Will", "Function", "Being"];
        assert_eq!(Law::Interaction.read(&triad), ["Will", "Being", "Function"]);
        // expansion is the trivial reading (identity permutation).
        assert_eq!(Law::Expansion.read(&triad), ["Will", "Function", "Being"]);
    }

    #[test]
    fn test_composition_matches_s3_table() {
        // Verified entries from the design-doc multiplication table (row ∘ col):
        assert_eq!(Law::Interaction.compose(&Law::Identity), Law::Freedom); // int∘idn = fre
        assert_eq!(Law::Interaction.compose(&Law::Interaction), Law::Expansion); // int∘int = e
        assert_eq!(Law::Identity.compose(&Law::Order), Law::Expansion); // idn∘ord = exp
        // expansion is the identity element: e ∘ x = x for all x.
        for law in Law::HEXAD {
            assert_eq!(Law::Expansion.compose(&law), law);
            assert_eq!(law.compose(&Law::Expansion), law);
        }
    }

    #[test]
    fn test_as_functor_bridges_to_morphism() {
        let f = Law::Interaction.as_functor("law_interaction", "sys_a", "sys_b");
        assert_eq!(f.order, 3);
        assert_eq!(f.permutation, vec![1, 3, 2]);
        assert!(f.validate().is_ok()); // a genuine S₃ bijection
    }

    #[test]
    fn test_read_walk_supplies_direction() {
        // Undirected canonical triad: nodes Will·Function·Being, edges (in canonical
        // order (1,2)(1,3)(2,3)) generation·decision·consent.
        let nodes = ["Will", "Function", "Being"];
        let edges = ["generation", "decision", "consent"];
        // expansion (1-2-3): the natural loop 1→2→3→1.
        assert_eq!(
            Law::Expansion.read_walk(&nodes, &edges),
            ["Will", "generation", "Function", "consent", "Being", "decision"]
        );
        // interaction (1-3-2 = SPO): Will→Being→Function→Will.
        assert_eq!(
            Law::Interaction.read_walk(&nodes, &edges),
            ["Will", "decision", "Being", "consent", "Function", "generation"]
        );
        // the walk is a closed loop: last edge joins the 3rd node back to the 1st.
        for law in Law::HEXAD {
            let w = law.read_walk(&nodes, &edges);
            let (n0, n1, n2) = (w[0], w[2], w[4]);
            // all three distinct nodes are present (a genuine triad traversal).
            assert!(n0 != n1 && n1 != n2 && n0 != n2);
        }
    }

    #[test]
    fn test_run_mvc_triad_through_the_six_laws() {
        // Run the architecture's own triad through the six laws (the "analysis" —
        // undirected base, direction supplied per law). Node-level ordering only
        // (MVC edges are unnamed): Model(−,pos2) · View(+,pos1) · Controller(=,pos3).
        // Positions: 1 = View (+), 2 = Model (−), 3 = Controller (=).
        let mvc = ["View", "Model", "Controller"];
        assert_eq!(Law::Expansion.read(&mvc), ["View", "Model", "Controller"]); // 123
        assert_eq!(Law::Interaction.read(&mvc), ["View", "Controller", "Model"]); // 132 (SPO)
        assert_eq!(Law::Freedom.read(&mvc), ["Controller", "Model", "View"]); // 321
        // all six give distinct orderings (S₃ acts freely on the 3 positions).
        let readings: Vec<[&str; 3]> = Law::HEXAD.iter().map(|l| l.read(&mvc)).collect();
        for i in 0..6 {
            for j in (i + 1)..6 {
                assert_ne!(readings[i], readings[j]);
            }
        }
    }
}
