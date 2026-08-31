//! SPO — the flat triple store and its query API, isolated as its own module so the
//! **mechanism is swappable** (a spike; the base-space remodel may replace it with a
//! six-laws / path-distance navigation). Everything an attribute of an element is a
//! `(subject, predicate, object)` with a topological `ordinality` and a `citation` — so
//! the browser/filter has **zero per-predicate code** and consumes this API rather than
//! embedding SPO internals.

use std::collections::{BTreeSet, HashMap, HashSet};

use crate::api::client::{InstanceSystem, ReferenceView, SequenceView};

/// Standard order_cardinality labels (the invariant systematics names).
const ORDER_NAMES: [&str; 12] = [
    "Monad", "Dyad", "Triad", "Tetrad", "Pentad", "Hexad", "Heptad", "Octad",
    "Ennead", "Decad", "Undecad", "Dodecad",
];

pub fn order_name(order_cardinality: i32) -> String {
    ORDER_NAMES
        .get((order_cardinality - 1) as usize)
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("order_cardinality {order_cardinality}"))
}

// -- small field accessors (references carry Option-wrapped nested data) --
pub fn persp(r: &ReferenceView) -> String {
    r.perspective_name.clone().unwrap_or_default()
}
pub fn src(r: &ReferenceView) -> String {
    r.source.as_ref().map(|s| s.name.clone()).unwrap_or_default()
}
pub fn art(r: &ReferenceView) -> String {
    r.artefact.as_ref().map(|a| a.title.clone()).unwrap_or_default()
}
pub fn loc(r: &ReferenceView) -> String {
    r.lookup.as_ref().map(|l| l.locator.clone()).unwrap_or_default()
}
pub fn order_of(r: &ReferenceView) -> Option<i32> {
    r.target_system.as_ref().map(|s| s.order_cardinality)
}
pub fn frag(r: &ReferenceView) -> String {
    r.target_fragment.clone().unwrap_or_default()
}

/// Map a reference `#fragment` to its predicate key (`term:2` -> `term`, ...).
pub fn frag_pred(f: &str) -> &'static str {
    if f == "coherence" {
        "coherence"
    } else if f.starts_with("term:") {
        "term"
    } else if f.starts_with("conn:") {
        "connective"
    } else if f == "term-designation" {
        "term-designation"
    } else if f == "connective-designation" {
        "connective-designation"
    } else {
        ""
    }
}

/// A flat **SPO triple** — the uniform unit the Filter operates over. Every attribute
/// of every element is `(subject, predicate, object)` + a topological `ordinality` (node
/// index `"1"` / edge `"1-2"` / `""`) + a `citation` (source·artefact·lookup; "" for
/// base-space facts). Base-space topology + fiber assertions flatten into one list.
#[derive(Clone)]
pub struct Triple {
    pub subject: String,
    pub predicate: String,
    pub object: String,
    pub ordinality: String,
    pub citation: String,
}

/// Author·artefact·lookup formatted as one Citation string (the quad's provenance).
pub fn ref_citation(r: &ReferenceView) -> String {
    [src(r), art(r), loc(r)]
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" · ")
}

/// The topological ordinality encoded in a reference `#fragment`: `term:1` → `1`,
/// `conn:1-2` → `1-2`; whole-system fragments carry no ordinality.
pub fn frag_position(f: &str) -> String {
    if let Some(n) = f.strip_prefix("term:") {
        n.to_string()
    } else if let Some(e) = f.strip_prefix("conn:") {
        e.to_string()
    } else {
        String::new()
    }
}

/// Flatten all data into SPO triples. Base-space facts come from each system's own
/// topology/characters (graph-derived positions, no citation); fiber assertions come
/// from references' objects, each carrying its Citation. `source` = the originator.
pub fn build_triples(
    systems: &[InstanceSystem],
    refs: &[ReferenceView],
    seqs: &[SequenceView],
) -> Vec<Triple> {
    let mut t: Vec<Triple> = Vec::new();
    let mut base = |subject: &str, predicate: &str, object: String, ordinality: String| {
        t.push(Triple {
            subject: subject.to_string(),
            predicate: predicate.to_string(),
            object,
            ordinality,
            citation: String::new(),
        });
    };
    for s in systems {
        base(&s.id, "name", s.name.clone(), String::new()); // scope to a *specific* system
        base(&s.id, "order_cardinality", order_name(s.order_cardinality), String::new());
        // Terms/connectives carry their GRAPH ordinality (node index / base–target edge),
        // supplied by the backend — terms anchor to nodes, connectives to edges.
        for term in &s.terms {
            base(&s.id, "term", term.value.clone(), term.ordinality.clone());
        }
        for c in &s.connectives {
            base(&s.id, "connective", c.value.clone(), c.ordinality.clone());
        }
    }
    let _ = seqs; // monads carry no base-space triples yet (they show unfiltered)
    for r in refs {
        let Some(ts) = &r.target_system else { continue };
        let cite = ref_citation(r);
        let fragment = frag(r);
        if let Some(obj) = &r.object {
            let pred = frag_pred(&fragment);
            if !pred.is_empty() {
                t.push(Triple {
                    subject: ts.id.clone(),
                    predicate: pred.to_string(),
                    object: obj.clone(),
                    ordinality: frag_position(&fragment),
                    citation: cite.clone(),
                });
            }
        }
        // Provenance as a filterable predicate: `source` = the originator (author),
        // falling back to the perspective when there is no named author.
        let origin = {
            let a = src(r);
            if a.is_empty() { persp(r) } else { a }
        };
        if !origin.is_empty() {
            t.push(Triple {
                subject: ts.id.clone(),
                predicate: "source".to_string(),
                object: origin,
                ordinality: String::new(),
                citation: cite,
            });
        }
    }
    t
}

/// The distinct **predicates** present in the data (auto-discovered): a friendly lead
/// order_cardinality (name · order_cardinality · source), then the rest alphabetically.
pub fn all_predicates(triples: &[Triple]) -> Vec<String> {
    let mut set: BTreeSet<String> = BTreeSet::new();
    for t in triples {
        set.insert(t.predicate.clone());
    }
    let mut out: Vec<String> = Vec::new();
    for lead in ["name", "order_cardinality", "source"] {
        if set.remove(lead) {
            out.push(lead.to_string());
        }
    }
    out.extend(set);
    out
}

/// Subjects that satisfy every constraint **except** `skip` — the current filter
/// context for a drill-down (so picking `order_cardinality=Triad` narrows `term` to triadic terms).
pub fn subjects_passing_except(
    triples: &[Triple],
    constraints: &HashMap<String, HashSet<String>>,
    skip: &str,
) -> HashSet<String> {
    let subjects: HashSet<String> = triples.iter().map(|t| t.subject.clone()).collect();
    subjects
        .into_iter()
        .filter(|s| {
            constraints.iter().all(|(p, vals)| {
                p == skip
                    || vals.is_empty()
                    || triples
                        .iter()
                        .any(|t| &t.subject == s && &t.predicate == p && vals.contains(&t.object))
            })
        })
        .collect()
}

/// Distinct **objects** for a predicate, **scoped** to the subjects that pass the other
/// active constraints — the SPO drill-down that narrows as you go.
pub fn pred_objects_scoped(
    triples: &[Triple],
    pred: &str,
    constraints: &HashMap<String, HashSet<String>>,
) -> Vec<String> {
    let subs = subjects_passing_except(triples, constraints, pred);
    let mut set: BTreeSet<String> = BTreeSet::new();
    for t in triples {
        if t.predicate == pred && subs.contains(&t.subject) {
            set.insert(t.object.clone());
        }
    }
    set.into_iter().collect()
}

/// Title-case a predicate key for display (`term-designation` -> `Term designation`).
pub fn pred_label(pred: &str) -> String {
    let mut out = String::new();
    for (i, w) in pred.split(['-', '_']).enumerate() {
        if i > 0 {
            out.push(' ');
        }
        let mut c = w.chars();
        if let Some(f) = c.next() {
            if i == 0 {
                out.extend(f.to_uppercase());
            } else {
                out.push(f);
            }
            out.push_str(c.as_str());
        }
    }
    out
}
