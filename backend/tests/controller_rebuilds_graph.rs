//! Validation: the **Controller can rebuild the graph**. The existing seeded graph
//! is the ground truth (the working models — visualisation + correct assignments);
//! the substrate's `generate_topology` (the controller's topology generator) must
//! reproduce each order's topology — its vertices and its undirected edges — from
//! the graph rules alone. As anchoring (terms / geometry / colour) is added, this
//! rebuild check extends to more of the graph.

use systematics_backend::core::substrate::{compose_system, generate_topology};
use systematics_backend::data;

/// The trailing integer of an id like `point_3_1` → 1 (the position).
fn tail(id: &str) -> u8 {
    id.rsplit('_').next().unwrap().parse().unwrap()
}

#[test]
fn controller_rebuilds_the_topology_of_every_seeded_order() {
    let graph = data::build_graph();

    for order in 1..=12u8 {
        let topo = graph
            .topology_for_order(order)
            .unwrap_or_else(|| panic!("order {order} has a seeded topology"));

        // Ground truth from the seeded topological vocabulary: points → vertex
        // positions, lines (`line_{order}_{lo}_{hi}`) → undirected edges.
        let mut gt_positions: Vec<u8> = topo.points.iter().map(|p| tail(p)).collect();
        gt_positions.sort_unstable();
        let mut gt_edges: Vec<(u8, u8)> = topo
            .lines
            .iter()
            .map(|l| {
                let mut it = l.rsplit('_');
                let b: u8 = it.next().unwrap().parse().unwrap();
                let a: u8 = it.next().unwrap().parse().unwrap();
                (a, b)
            })
            .collect();
        gt_edges.sort_unstable();

        // The controller's rebuild, generated from the graph rules (the Template).
        let rebuilt = generate_topology(order);
        let mut rb_positions: Vec<u8> = rebuilt.elements.iter().map(|e| e.position).collect();
        rb_positions.sort_unstable();
        let mut rb_edges: Vec<(u8, u8)> = rebuilt.links.iter().map(|l| l.endpoints).collect();
        rb_edges.sort_unstable();

        assert_eq!(rb_positions, gt_positions, "order {order}: vertices must match");
        assert_eq!(rb_edges, gt_edges, "order {order}: edges must match");
    }
}

/// The Controller must also rebuild the graph's **semantics** — the term/connective
/// assignments at their canonical positions. Pull the seeded canonical triad's terms
/// (in position order) as ground truth, `compose_system` from them, and check the
/// anchoring lands each term on its canonical vertex.
#[test]
fn controller_rebuilds_canonical_triad_term_assignments() {
    let graph = data::build_graph();
    let sys = graph
        .system("system_canonical_triad_3")
        .expect("seeded canonical triad");
    let vocab = graph.vocabulary(&sys.vocabulary_ref).expect("its vocabulary");

    let char_value = |cid: &String| {
        graph
            .character(cid)
            .map(|c| c.value.clone())
            .unwrap_or_default()
    };
    let terms: Vec<String> = vocab.terms.iter().map(&char_value).collect();
    let connectives: Vec<String> = vocab.connectives.iter().map(&char_value).collect();

    let hg = compose_system(3, &terms, &connectives);

    for (i, term) in terms.iter().enumerate() {
        let pos = i + 1;
        let anchored = hg
            .data
            .iter()
            .find(|d| d.id == format!("term_3_{pos}"))
            .unwrap_or_else(|| panic!("term at position {pos}"));
        assert_eq!(&anchored.character, term, "canonical term at vertex {pos}");
        // and it is orthogonally anchored to that exact vertex element.
        assert!(
            hg.links.iter().any(|l| {
                l.base == format!("el_3_{pos}") && l.target == format!("term_3_{pos}")
            }),
            "term_3_{pos} must anchor to vertex el_3_{pos}"
        );
    }
}
