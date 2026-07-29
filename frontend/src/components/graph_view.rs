use systematics_middleware::RenderedSystem;
use yew::prelude::*;

use crate::api::client::{InstanceSystem, ReferenceView};

/// Default colors for rendering
const DEFAULT_NODE_COLOR: &str = "#4A90E2";
const DEFAULT_EDGE_COLOR: &str = "#888888";
const SELECTED_NODE_COLOR: &str = "#FF6B6B";
const SELECTED_EDGE_COLOR: &str = "#FF6B6B";

#[derive(Properties, PartialEq)]
pub struct ApiGraphViewProps {
    pub system: RenderedSystem,
    #[prop_or_default]
    pub on_navigate: Option<Callback<String>>,
    #[prop_or_default]
    pub show_edge_labels: bool,
    /// Toggles the edge-label switch (rendered as an in-canvas overlay).
    #[prop_or_default]
    pub on_toggle_edge_labels: Option<Callback<()>>,
    /// All citations within this system; shown as hover tooltips on nodes.
    #[prop_or_default]
    pub references: Vec<ReferenceView>,
    /// Non-canonical instance systems, listed by the in-canvas Load control.
    #[prop_or_default]
    pub instance_systems: Vec<InstanceSystem>,
    /// Load an instance system (by id) into the canvas.
    #[prop_or_default]
    pub on_load: Option<Callback<String>>,
    /// When true, labels show the canonical *class* (`system.canonical_class`).
    #[prop_or_default]
    pub show_canonical: bool,
    /// Toggle the Canonical-override switch.
    #[prop_or_default]
    pub on_toggle_canonical: Option<Callback<()>>,
}

pub enum ApiGraphMsg {
    NodeClicked(usize),
    #[allow(dead_code)]
    EdgeClicked(usize, usize),
    ToggleLoadMenu,
}

pub struct ApiGraphView {
    selected_node: Option<usize>,
    selected_edge: Option<(usize, usize)>,
    load_menu_open: bool,
}

impl Component for ApiGraphView {
    type Message = ApiGraphMsg;
    type Properties = ApiGraphViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            selected_node: None,
            selected_edge: None,
            load_menu_open: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ApiGraphMsg::ToggleLoadMenu => {
                self.load_menu_open = !self.load_menu_open;
                true
            }
            ApiGraphMsg::NodeClicked(idx) => {
                // Toggle selection
                let selecting = self.selected_node != Some(idx);
                if selecting {
                    self.selected_node = Some(idx);
                    self.selected_edge = None;
                } else {
                    self.selected_node = None;
                }
                true
            }
            ApiGraphMsg::EdgeClicked(from, to) => {
                let edge = if from < to { (from, to) } else { (to, from) };
                if self.selected_edge == Some(edge) {
                    self.selected_edge = None;
                } else {
                    self.selected_edge = Some(edge);
                    self.selected_node = None;
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let system = &ctx.props().system;
        let show_edge_labels = ctx.props().show_edge_labels;

        // The metadata elements (coherence + designations) are themselves
        // citable: hovering a header item shows its own reference, if any.
        let refs = &ctx.props().references;
        let sid = &system.system_id;
        let coherence_tip =
            tooltip_for(refs, &format!("system:{}#coherence", sid)).map(AttrValue::from);
        let term_tip =
            tooltip_for(refs, &format!("system:{}#term-designation", sid)).map(AttrValue::from);
        let conn_tip = tooltip_for(refs, &format!("system:{}#connective-designation", sid))
            .map(AttrValue::from);

        html! {
            <div class="graph-view">
                // Compact single-line header. On display a system's terms /
                // connectives take their designations (Impulses/Acts for the
                // triad, Sources/Interplays for the tetrad); the generic API
                // field names stay `terms`/`connectives`. Each metadata item is
                // itself referenceable — hover shows its citation.
                <header class="graph-header">
                    <span class="graph-title">
                        { format!("{} · {}", system.name, system.k_notation()) }
                    </span>
                    <span
                        class={ classes!("designation", "designation-coherence", coherence_tip.is_some().then_some("cited")) }
                        title={ coherence_tip.clone() }
                    >
                        { &system.coherence }
                    </span>
                    <span
                        class={ classes!("designation", "designation-term", term_tip.is_some().then_some("cited")) }
                        title={ term_tip.clone() }
                    >
                        { format!("{} {}", system.order, system.term_designation) }
                    </span>
                    <span
                        class={ classes!("designation", "designation-connective", conn_tip.is_some().then_some("cited")) }
                        title={ conn_tip.clone() }
                    >
                        { format!("{} {}", system.connectives.len(), system.connective_designation) }
                    </span>
                </header>
                if let Some(on_toggle) = ctx.props().on_toggle_edge_labels.clone() {
                    <label class="edge-toggle-overlay">
                        <span class="toggle-label">{ "Edge Labels" }</span>
                        <div class="toggle-switch">
                            <input
                                type="checkbox"
                                checked={ show_edge_labels }
                                onclick={ Callback::from(move |_| on_toggle.emit(())) }
                            />
                            <span class="slider"></span>
                        </div>
                    </label>
                }

                // Canonical-override toggle — only when the system has a class
                // (i.e. it's an instance). Flips node/edge labels to the class.
                if system.canonical_class.is_some() {
                    if let Some(on_toggle) = ctx.props().on_toggle_canonical.clone() {
                        <label class="canonical-toggle-overlay">
                            <span class="toggle-label">{ "Canonical" }</span>
                            <div class="toggle-switch">
                                <input
                                    type="checkbox"
                                    checked={ ctx.props().show_canonical }
                                    onclick={ Callback::from(move |_| on_toggle.emit(())) }
                                />
                                <span class="slider"></span>
                            </div>
                        </label>
                    }
                }

                // Load control (top-right): browse + load instance systems.
                if let Some(on_load) = ctx.props().on_load.clone() {
                    <div class="load-overlay">
                        <button
                            class="load-button"
                            onclick={ ctx.link().callback(|_| ApiGraphMsg::ToggleLoadMenu) }
                        >{ if self.load_menu_open { "Load ▴" } else { "Load ▾" } }</button>
                        if self.load_menu_open {
                            <div class="load-menu">
                                { for ctx.props().instance_systems.iter().map(|inst| {
                                    let id = inst.id.clone();
                                    let on_load = on_load.clone();
                                    let toggle = ctx.link().callback(|_| ApiGraphMsg::ToggleLoadMenu);
                                    let onclick = Callback::from(move |_| {
                                        on_load.emit(id.clone());
                                        toggle.emit(());
                                    });
                                    html! {
                                        <button class="load-item" onclick={ onclick }>
                                            { format!("{} · {}", inst.order, inst.name) }
                                        </button>
                                    }
                                }) }
                            </div>
                        }
                    </div>
                }
                <svg
                    class="graph-svg"
                    viewBox="0 0 800 800"
                    preserveAspectRatio="xMidYMid meet"
                >
                    { self.render_edges(ctx, system) }
                    if show_edge_labels {
                        { self.render_edge_labels(ctx, system) }
                    }
                    { self.render_nodes(ctx, system) }
                </svg>
            </div>
        }
    }
}

impl ApiGraphView {
    /// Render edges (lines) from the system
    fn render_edges(&self, ctx: &Context<Self>, system: &RenderedSystem) -> Html {
        web_sys::console::log_1(
            &format!("render_edges: {} lines to render", system.lines.len()).into(),
        );

        system
            .lines
            .iter()
            .map(|line| {
                // Positions come directly (1-based).
                let base_pos = line.base_position;
                let target_pos = line.target_position;

                web_sys::console::log_1(
                    &format!(
                        "Line: {} (base_pos={}, target_pos={})",
                        line.id, base_pos, target_pos
                    )
                    .into(),
                );

                if base_pos <= 0 || target_pos <= 0 {
                    web_sys::console::log_1(&"Skipping line: invalid positions".into());
                    return html! {};
                }

                // Look up coordinates from the system's transformed coordinates array
                // (Don't use embedded link coordinates - they aren't transformed correctly)
                let (from_x, from_y) = if let Some(coord) = system.coordinate_at(base_pos) {
                    (coord.x, coord.y)
                } else {
                    web_sys::console::log_1(
                        &format!("Could not find from coordinate for pos {}", base_pos).into(),
                    );
                    return html! {};
                };

                let (to_x, to_y) = if let Some(coord) = system.coordinate_at(target_pos) {
                    (coord.x, coord.y)
                } else {
                    web_sys::console::log_1(
                        &format!("Could not find to coordinate for pos {}", target_pos).into(),
                    );
                    return html! {};
                };

                // Convert to 0-based for selection comparison
                let from_idx = (base_pos - 1) as usize;
                let to_idx = (target_pos - 1) as usize;

                let edge_tuple = if from_idx < to_idx {
                    (from_idx, to_idx)
                } else {
                    (to_idx, from_idx)
                };

                let is_selected = self.selected_edge == Some(edge_tuple);

                // Citations for this connective → native hover tooltip on the edge
                // (the edge line itself stays neutral; the *label* carries the cue).
                let (p1, p2) = (base_pos.min(target_pos), base_pos.max(target_pos));
                let address = format!("system:{}#conn:{}-{}", system.system_id, p1, p2);
                let edge_refs: Vec<&ReferenceView> = ctx
                    .props()
                    .references
                    .iter()
                    .filter(|r| r.target == address)
                    .collect();
                let tooltip = build_tooltip(&edge_refs);

                let stroke = if is_selected {
                    SELECTED_EDGE_COLOR
                } else {
                    DEFAULT_EDGE_COLOR
                };
                let stroke_width = if is_selected { 3.0 } else { 1.5 };

                html! {
                    <g class="edge-group">
                        if let Some(text) = tooltip {
                            <title>{ text }</title>
                        }
                        <line
                            x1={ from_x.to_string() }
                            y1={ from_y.to_string() }
                            x2={ to_x.to_string() }
                            y2={ to_y.to_string() }
                            stroke={ stroke }
                            stroke-width={ stroke_width.to_string() }
                            class="edge"
                        />
                        // Transparent, wider hit-area so the thin edge is easy to hover.
                        <line
                            x1={ from_x.to_string() }
                            y1={ from_y.to_string() }
                            x2={ to_x.to_string() }
                            y2={ to_y.to_string() }
                            stroke="transparent"
                            stroke-width="12"
                            style="cursor: pointer;"
                        />
                    </g>
                }
            })
            .collect::<Html>()
    }

    /// Render edge labels for connectives
    /// Instead of iterating connectives independently, we iterate through lines
    /// and find matching connectives to ensure labels align with the correct edges
    fn render_edge_labels(&self, ctx: &Context<Self>, system: &RenderedSystem) -> Html {
        web_sys::console::log_1(
            &format!(
                "render_edge_labels: {} lines, {} connectives",
                system.lines.len(),
                system.connectives.len()
            )
            .into(),
        );

        system.lines.iter().enumerate().map(|(line_idx, line)| {
            let line_base_pos = line.base_position;
            let line_target_pos = line.target_position;

            let matching_connective = system.connectives.iter().enumerate().find(|(_, conn)| {
                let conn_base = conn.base_position;
                let conn_target = conn.target_position;
                (conn_base == line_base_pos && conn_target == line_target_pos) ||
                (conn_base == line_target_pos && conn_target == line_base_pos)
            });

            let Some((conn_idx, connective)) = matching_connective else {
                web_sys::console::log_1(&format!("No connective found for line {}: {}→{}",
                    line_idx, line_base_pos, line_target_pos).into());
                return html! {};
            };

            // Canonical override: show the class connective at this edge instead.
            let (ep1, ep2) = (line_base_pos.min(line_target_pos), line_base_pos.max(line_target_pos));
            let class_label = if ctx.props().show_canonical {
                system.canonical_class.as_deref().and_then(|c| {
                    c.connectives
                        .iter()
                        .find(|k| {
                            (k.base_position.min(k.target_position), k.base_position.max(k.target_position))
                                == (ep1, ep2)
                        })
                        .map(|k| k.character_value.clone())
                })
            } else {
                None
            };
            let label: &str = class_label
                .as_deref()
                .unwrap_or(connective.character_value.as_str());

            if label.is_empty() {
                return html! {};
            }

            // Is this connective cited? Highlight the label (not the edge line).
            let (cp1, cp2) = (line_base_pos.min(line_target_pos), line_base_pos.max(line_target_pos));
            let conn_address = format!("system:{}#conn:{}-{}", system.system_id, cp1, cp2);
            let conn_refs: Vec<&ReferenceView> = ctx
                .props()
                .references
                .iter()
                .filter(|r| r.target == conn_address)
                .collect();
            let cited = !conn_refs.is_empty();
            let label_tooltip = build_tooltip(&conn_refs);
            let label_fill = if cited { "#B4741A" } else { "#2563eb" };
            let label_weight = if cited { "700" } else { "500" };
            let rect_stroke = if cited { "rgba(226, 162, 74, 0.9)" } else { "rgba(37, 99, 235, 0.3)" };

            web_sys::console::log_1(&format!("Line {} ({}→{}) matched with connective {} (label='{}')",
                line_idx, line_base_pos, line_target_pos, conn_idx, label).into());

            // Use the SAME coordinate lookup as render_edges to ensure alignment
            let (from_x, from_y) = if let Some(coord) = system.coordinate_at(line_base_pos) {
                (coord.x, coord.y)
            } else {
                web_sys::console::log_1(&format!("No coordinate for base_pos {}", line_base_pos).into());
                return html! {};
            };

            let (to_x, to_y) = if let Some(coord) = system.coordinate_at(line_target_pos) {
                (coord.x, coord.y)
            } else {
                web_sys::console::log_1(&format!("No coordinate for target_pos {}", line_target_pos).into());
                return html! {};
            };

            // Calculate midpoint for label placement
            let mid_x = (from_x + to_x) / 2.0;
            let mid_y = (from_y + to_y) / 2.0;

            // Calculate angle for label rotation
            let dx = to_x - from_x;
            let dy = to_y - from_y;
            let angle = dy.atan2(dx) * 180.0 / std::f64::consts::PI;

            // Keep text readable (not upside down)
            let rotation_angle = if !(-90.0..=90.0).contains(&angle) {
                angle + 180.0
            } else {
                angle
            };

            let rect_width = label.len() as f64 * 7.0;
            let rect_height = 16.0;

            html! {
                <g class="edge-label-group" transform={ format!("translate({} {}) rotate({})", mid_x, mid_y, rotation_angle) }>
                    if let Some(text) = label_tooltip {
                        <title>{ text }</title>
                    }
                    <rect
                        x={ (-rect_width / 2.0).to_string() }
                        y={ (-rect_height / 2.0).to_string() }
                        width={ rect_width.to_string() }
                        height={ rect_height.to_string() }
                        fill="rgba(255, 255, 255, 0.9)"
                        stroke={ rect_stroke }
                        stroke-width={ if cited { "1.2" } else { "0.5" } }
                        rx="4"
                        style={ if cited { "cursor: pointer;" } else { "pointer-events: none;" } }
                    />
                    <text
                        x="0"
                        y="0"
                        text-anchor="middle"
                        dominant-baseline="middle"
                        class="edge-label"
                        fill={ label_fill }
                        style={ format!("font-size: 10px; font-weight: {}; user-select: none; {}", label_weight, if cited { "cursor: pointer;" } else { "pointer-events: none;" }) }
                    >
                        { label }
                    </text>
                </g>
            }
        }).collect::<Html>()
    }

    /// Render nodes from coordinates and terms
    fn render_nodes(&self, ctx: &Context<Self>, system: &RenderedSystem) -> Html {
        system.coordinates.iter().map(|coord| {
            let position = coord.position;
            let idx = (position - 1) as usize;  // Convert 1-based position to 0-based index

            let is_selected = self.selected_node == Some(idx);

            // Get color for this node from colours array, or use default
            let fill = if is_selected {
                SELECTED_NODE_COLOR.to_string()
            } else {
                system.colour_at(position)
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| DEFAULT_NODE_COLOR.to_string())
            };

            let radius = if is_selected { 18.0 } else { 12.0 };
            let onclick = ctx.link().callback(move |_| ApiGraphMsg::NodeClicked(idx));

            // Get term label for this position (canonical class when overridden).
            let class_term = if ctx.props().show_canonical {
                system
                    .canonical_class
                    .as_deref()
                    .and_then(|c| c.term_at(position))
                    .map(|s| s.to_string())
            } else {
                None
            };
            let term: &str = class_term
                .as_deref()
                .unwrap_or_else(|| system.term_at(position).unwrap_or(""));

            // Citations for this term → native hover tooltip on the node.
            let address = format!("system:{}#term:{}", system.system_id, position);
            let node_refs: Vec<&ReferenceView> = ctx
                .props()
                .references
                .iter()
                .filter(|r| r.target == address)
                .collect();
            let tooltip = build_tooltip(&node_refs);
            let cited = tooltip.is_some();
            let node_class = if cited { "node cited" } else { "node" };

            html! {
                <g class={ node_class } onclick={ onclick }>
                    if let Some(text) = tooltip {
                        <title>{ text }</title>
                    }
                    if cited {
                        <circle
                            cx={ coord.x.to_string() }
                            cy={ coord.y.to_string() }
                            r={ (radius + 4.0).to_string() }
                            fill="none"
                            stroke="#E2A24A"
                            stroke-width="2"
                            stroke-dasharray="3 3"
                            class="cited-ring"
                        />
                    }
                    <circle
                        cx={ coord.x.to_string() }
                        cy={ coord.y.to_string() }
                        r={ radius.to_string() }
                        fill={ fill }
                        stroke="white"
                        stroke-width="2"
                        style="cursor: pointer;"
                    />
                    <text
                        x={ coord.x.to_string() }
                        y={ coord.y.to_string() }
                        text-anchor="middle"
                        dominant-baseline="middle"
                        fill="white"
                        stroke="black"
                        stroke-width="1"
                        paint-order="stroke"
                        style="font-size: 12px; font-weight: bold; pointer-events: none; user-select: none;"
                    >
                        { position }
                    </text>
                    // Render vocabulary label if available
                    if !term.is_empty() {
                        <text
                            x={ coord.x.to_string() }
                            y={ (coord.y + radius + 16.0).to_string() }
                            text-anchor="middle"
                            dominant-baseline="middle"
                            fill="#333"
                            style="font-size: 14px; font-weight: 500; pointer-events: none; user-select: none;"
                        >
                            { term }
                        </text>
                    }
                </g>
            }
        }).collect::<Html>()
    }
}

/// A citation rendered as its own labelled triad (Source / Artefact / Lookup),
/// one field per line — mirrors the Citation triad's three impulses.
fn format_reference(r: &ReferenceView) -> String {
    let mut lines = Vec::new();
    lines.push(format!(
        "Source: {}",
        r.source.as_ref().map(|s| s.name.as_str()).unwrap_or("—")
    ));
    if let Some(a) = &r.artefact {
        lines.push(format!("Artefact: {}", a.title));
    }
    if let Some(l) = &r.lookup {
        lines.push(format!("Lookup: {}", l.locator));
    }
    if let Some(n) = &r.note {
        lines.push(format!("Note: {}", n));
    }
    lines.join("\n")
}

/// Build a tooltip from the citations for an element: each as a labelled triad.
/// Returns `None` when there are no citations.
fn build_tooltip(refs: &[&ReferenceView]) -> Option<String> {
    if refs.is_empty() {
        return None;
    }
    Some(
        refs.iter()
            .map(|r| format_reference(r))
            .collect::<Vec<_>>()
            .join("\n\n"),
    )
}

/// Citations targeting a specific Expression address (e.g. a metadata element
/// like `system:<id>#coherence`), rendered as a tooltip. `None` if uncited.
fn tooltip_for(references: &[ReferenceView], address: &str) -> Option<String> {
    let matched: Vec<&ReferenceView> = references.iter().filter(|r| r.target == address).collect();
    build_tooltip(&matched)
}
