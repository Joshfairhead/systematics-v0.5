//! Functor: a same-grammar morphism between two Systems of one Order.
//!
//! For a fixed Order `n` the complete graph `K_n` has exactly the symmetric
//! group `S_n` as its automorphisms — in `K_n` every pair of vertices is
//! adjacent, so *any* permutation of the vertices preserves adjacency. A
//! same-grammar Functor is therefore stored as a single *position permutation*
//! (the object map on terms); the morphism map on connectives is **derived**: a
//! connective between source positions `p,q` maps to the connective between
//! `f(p),f(q)`. Structure preservation (the functor square
//! `f(conn:p-q) = conn:f(p)-f(q)`) then holds by construction — the connective
//! map is never stored, only computed, and cannot disagree with the object map.
//!
//! This is deliberately the honest, minimal case. Cross-grammar functors
//! (order-changing, e.g. dyad → triad — Bennett's "qualitative Fourier
//! transform") are non-order-preserving and are deferred; they are *not*
//! expressible as an `S_n` permutation and must not be forced into this type.

use serde::{Deserialize, Serialize};

/// A same-grammar morphism between two Systems of equal Order, stored as an
/// `S_order` permutation of term positions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Functor {
    pub id: String,
    pub name: String,
    /// The shared Order of the source and target systems.
    pub order: u8,
    /// Source System id (the object in the domain).
    pub source_ref: String,
    /// Target System id (the object in the codomain).
    pub target_ref: String,
    /// The object map as a permutation in `S_order`: `permutation[i]` is the
    /// 1-based target position that source position `i + 1` maps to. To be a
    /// functor it must have length `order` and be a bijection of `1..=order`.
    pub permutation: Vec<u8>,
}

impl Functor {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        order: u8,
        source_ref: impl Into<String>,
        target_ref: impl Into<String>,
        permutation: Vec<u8>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            order,
            source_ref: source_ref.into(),
            target_ref: target_ref.into(),
            permutation,
        }
    }

    /// The identity endomorphism on a System of `order` (position `i` ↦ `i`).
    pub fn identity(
        id: impl Into<String>,
        name: impl Into<String>,
        order: u8,
        system_ref: impl Into<String>,
    ) -> Self {
        let system_ref = system_ref.into();
        Self::new(id, name, order, system_ref.clone(), system_ref, (1..=order).collect())
    }

    /// Map a source position (1-based) to its target position, if in range.
    pub fn map_position(&self, position: u8) -> Option<u8> {
        if position == 0 {
            return None;
        }
        self.permutation.get((position - 1) as usize).copied()
    }

    /// Validate the functor laws (advisory — mirrors `System`/`Template`
    /// validation; nothing is rejected at write time):
    ///  * totality — one target per source position (`permutation.len() == order`),
    ///  * sort/type preservation and invertibility — the permutation is a
    ///    bijection of `1..=order` (each target position hit exactly once).
    ///
    /// A construct that fails these is a mere relabelling *table*, not a functor.
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errs = Vec::new();
        let n = self.order as usize;

        if self.order == 0 {
            errs.push(format!("Functor {}: order must be >= 1", self.id));
        }
        if self.permutation.len() != n {
            errs.push(format!(
                "Functor {}: permutation has {} entries, expected {} (one per source position — not total)",
                self.id,
                self.permutation.len(),
                n
            ));
        }

        let mut seen = vec![false; n];
        for &p in &self.permutation {
            if p < 1 || p as usize > n {
                errs.push(format!(
                    "Functor {}: target position {} out of range 1..={}",
                    self.id, p, self.order
                ));
            } else if seen[(p - 1) as usize] {
                errs.push(format!(
                    "Functor {}: target position {} mapped more than once (not a bijection)",
                    self.id, p
                ));
            } else {
                seen[(p - 1) as usize] = true;
            }
        }

        if errs.is_empty() {
            Ok(())
        } else {
            Err(errs)
        }
    }

    /// Remap one address from the source System to the target System under this
    /// functor. Addresses over any *other* system (or non-system addresses) pass
    /// through unchanged, so the functor acts on a perspective that spans many
    /// systems by touching only its own source.
    ///
    ///  * `system:src`                 → `system:tgt`
    ///  * `system:src#term:i`          → `system:tgt#term:f(i)`
    ///  * `system:src#conn:a-b`        → `system:tgt#conn:min-max(f(a),f(b))`
    ///  * `system:src#coherence` (etc.) → `system:tgt#coherence` (system-level
    ///    fragment retargeted, unchanged)
    ///
    /// An address whose position is out of range or unparseable is passed
    /// through unchanged rather than corrupted.
    pub fn map_address(&self, address: &str) -> String {
        let Some(rest) = address.strip_prefix("system:") else {
            return address.to_string();
        };
        let (sys, frag) = match rest.split_once('#') {
            Some((s, f)) => (s, Some(f)),
            None => (rest, None),
        };
        if sys != self.source_ref {
            return address.to_string();
        }

        match frag {
            None => format!("system:{}", self.target_ref),
            Some(f) => {
                if let Some(pos) = f.strip_prefix("term:") {
                    if let Some(fp) = pos.parse::<u8>().ok().and_then(|p| self.map_position(p)) {
                        return format!("system:{}#term:{}", self.target_ref, fp);
                    }
                    address.to_string()
                } else if let Some(pair) = f.strip_prefix("conn:") {
                    if let Some((a, b)) = pair.split_once('-') {
                        if let (Some(fa), Some(fb)) = (
                            a.parse::<u8>().ok().and_then(|p| self.map_position(p)),
                            b.parse::<u8>().ok().and_then(|p| self.map_position(p)),
                        ) {
                            let (lo, hi) = if fa <= fb { (fa, fb) } else { (fb, fa) };
                            return format!("system:{}#conn:{}-{}", self.target_ref, lo, hi);
                        }
                    }
                    address.to_string()
                } else {
                    // #coherence / #term-designation / #connective-designation:
                    // a system-level fragment — retarget the system, keep the tag.
                    format!("system:{}#{}", self.target_ref, f)
                }
            }
        }
    }

    /// Compose two functors: `self.then(g)` applies `self` first, then `g`
    /// (`g ∘ self` on positions). Requires equal order and `self.target_ref ==
    /// g.source_ref`; the result maps `self.source_ref → g.target_ref`. Returns
    /// `None` when the categories don't line up (non-composable).
    pub fn then(
        &self,
        g: &Functor,
        id: impl Into<String>,
        name: impl Into<String>,
    ) -> Option<Functor> {
        if self.order != g.order || self.target_ref != g.source_ref {
            return None;
        }
        let permutation = self
            .permutation
            .iter()
            .map(|&p| g.map_position(p))
            .collect::<Option<Vec<u8>>>()?;
        Some(Functor::new(
            id,
            name,
            self.order,
            self.source_ref.clone(),
            g.target_ref.clone(),
            permutation,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rotate_triad() -> Functor {
        // 1→2, 2→3, 3→1 over a triad, system A → system B.
        Functor::new("functor_rot", "rotate", 3, "system_a_3", "system_b_3", vec![2, 3, 1])
    }

    #[test]
    fn identity_validates_and_is_fixed() {
        let id = Functor::identity("functor_id", "identity", 4, "system_x_4");
        assert!(id.validate().is_ok());
        assert_eq!(id.map_position(3), Some(3));
        assert_eq!(id.map_address("system:system_x_4#term:2"), "system:system_x_4#term:2");
    }

    #[test]
    fn permutation_validates() {
        assert!(rotate_triad().validate().is_ok());
    }

    #[test]
    fn non_total_permutation_is_rejected() {
        let f = Functor::new("f", "partial", 3, "a", "b", vec![2, 3]); // missing one
        assert!(f.validate().is_err());
    }

    #[test]
    fn non_bijection_is_rejected() {
        let f = Functor::new("f", "collapse", 3, "a", "b", vec![1, 1, 2]); // 1 twice, 3 never
        assert!(f.validate().is_err());
    }

    #[test]
    fn out_of_range_target_is_rejected() {
        let f = Functor::new("f", "bad", 3, "a", "b", vec![1, 2, 9]);
        assert!(f.validate().is_err());
    }

    #[test]
    fn maps_term_addresses() {
        let f = rotate_triad();
        assert_eq!(f.map_address("system:system_a_3#term:1"), "system:system_b_3#term:2");
        assert_eq!(f.map_address("system:system_a_3#term:3"), "system:system_b_3#term:1");
    }

    #[test]
    fn derived_connective_map_is_canonicalized() {
        // conn between positions 2,3 -> f(2)=3, f(3)=1 -> canonical min-max = 1-3.
        let f = rotate_triad();
        assert_eq!(f.map_address("system:system_a_3#conn:2-3"), "system:system_b_3#conn:1-3");
        // and it agrees regardless of input endpoint order (structure preservation).
        assert_eq!(f.map_address("system:system_a_3#conn:3-2"), "system:system_b_3#conn:1-3");
    }

    #[test]
    fn system_level_fragments_and_bare_system_retarget() {
        let f = rotate_triad();
        assert_eq!(f.map_address("system:system_a_3"), "system:system_b_3");
        assert_eq!(f.map_address("system:system_a_3#coherence"), "system:system_b_3#coherence");
    }

    #[test]
    fn foreign_and_nonsystem_addresses_pass_through() {
        let f = rotate_triad();
        assert_eq!(f.map_address("system:system_other_3#term:1"), "system:system_other_3#term:1");
        assert_eq!(f.map_address("perspective:p"), "perspective:p");
    }

    #[test]
    fn composition_equals_composed_permutation() {
        // σ: A→B = (1→2,2→3,3→1); τ: B→C = swap (1↔2, 3 fixed).
        let sigma = rotate_triad();
        let tau = Functor::new("tau", "swap", 3, "system_b_3", "system_c_3", vec![2, 1, 3]);
        let comp = sigma.then(&tau, "comp", "sigma;tau").expect("composable");
        assert_eq!(comp.source_ref, "system_a_3");
        assert_eq!(comp.target_ref, "system_c_3");
        // position 1: σ(1)=2, τ(2)=1  => 1
        // position 2: σ(2)=3, τ(3)=3  => 3
        // position 3: σ(3)=1, τ(1)=2  => 2
        assert_eq!(comp.permutation, vec![1, 3, 2]);
        assert!(comp.validate().is_ok());
        // and applying the composite equals applying σ then τ.
        let via_comp = comp.map_address("system:system_a_3#term:2");
        let via_steps = tau.map_address(&sigma.map_address("system:system_a_3#term:2"));
        assert_eq!(via_comp, via_steps);
        assert_eq!(via_comp, "system:system_c_3#term:3");
    }

    #[test]
    fn non_composable_returns_none() {
        let sigma = rotate_triad(); // target system_b_3
        let mismatched = Functor::new("g", "g", 3, "system_z_3", "system_c_3", vec![1, 2, 3]);
        assert!(sigma.then(&mismatched, "x", "x").is_none());
    }
}
