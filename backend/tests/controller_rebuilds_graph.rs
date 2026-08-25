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

/// The Controller must also rebuild the graph's **semantics** — the term assignments
/// at their canonical positions — for **every full seeded system, beyond the triad**
/// (tetrad … dodecad). For each system whose vocabulary is complete (`terms.len() ==
/// order`), `compose_system` from its real terms and check each lands on its vertex.
#[test]
fn controller_rebuilds_term_assignments_beyond_the_triad() {
    let graph = data::build_graph();
    let mut checked_orders = std::collections::BTreeSet::new();

    for sys in graph.systems.iter() {
        let order = sys.order;
        let Some(vocab) = graph.vocabulary(&sys.vocabulary_ref) else {
            continue;
        };
        // only full systems (a complete vocabulary for the order).
        if vocab.terms.len() != order as usize {
            continue;
        }
        let value = |cid: &String| graph.character(cid).map(|c| c.value.clone()).unwrap_or_default();
        let terms: Vec<String> = vocab.terms.iter().map(value).collect();
        let connectives: Vec<String> = vocab.connectives.iter().map(value).collect();

        let hg = compose_system(order, &terms, &connectives);

        for (i, term) in terms.iter().enumerate() {
            let pos = (i + 1) as u8;
            let anchored = hg
                .data
                .iter()
                .find(|d| d.id == format!("term_{order}_{pos}"))
                .unwrap_or_else(|| panic!("{} order {order}: term at {pos}", sys.id));
            assert_eq!(&anchored.character, term, "{} term at vertex {pos}", sys.id);
            assert!(
                hg.links.iter().any(|l| {
                    l.base == format!("el_{order}_{pos}")
                        && l.target == format!("term_{order}_{pos}")
                }),
                "{} term_{order}_{pos} must anchor to vertex el_{order}_{pos}",
                sys.id
            );
        }
        checked_orders.insert(order);
    }

    // proves it rebuilds well beyond the triad, across many orders.
    assert!(
        checked_orders.iter().filter(|&&o| o >= 4).count() >= 3,
        "should rebuild systems of order ≥ 4 (beyond the triad); orders checked: {checked_orders:?}"
    );
}

/// And the Controller must rebuild the graph's **geometry** — the actual coordinates.
/// Pull the seeded canonical triad's coordinates from the graph, `compose_render` from
/// them, and check each coordinate value lands anchored on its canonical vertex.
#[test]
fn controller_rebuilds_canonical_triad_geometry() {
    use systematics_backend::core::substrate::compose_render;
    let graph = data::build_graph();

    let coords: Vec<[f64; 3]> = (1..=3u8)
        .map(|p| {
            let c = graph.coordinate(3, p).expect("seeded coordinate");
            [c.x, c.y, c.z]
        })
        .collect();

    let hg = compose_render(3, &coords, &[]);

    for p in 1..=3u8 {
        let c = coords[(p - 1) as usize];
        let expected = format!("{},{},{}", c[0], c[1], c[2]);
        let de = hg
            .data
            .iter()
            .find(|d| d.id == format!("coord_3_{p}"))
            .expect("coordinate data element");
        assert_eq!(de.character, expected, "coordinate value at vertex {p}");
        assert!(
            hg.links
                .iter()
                .any(|l| l.base == format!("el_3_{p}") && l.target == format!("coord_3_{p}")),
            "coord_3_{p} must anchor to vertex el_3_{p}"
        );
    }
}
