//! Validation: the **Controller can rebuild the graph**. The existing seeded graph
//! is the ground truth (the working models — visualisation + correct assignments);
//! the substrate's `generate_topology` (the controller's topology generator) must
//! reproduce each order's topology — its vertices and its undirected edges — from
//! the graph rules alone. As anchoring (terms / geometry / colour) is added, this
//! rebuild check extends to more of the graph.

use systematics_backend::core::substrate::generate_topology;
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
