use systematics_middleware::Grammar;
use yew::prelude::*;

/// Default colors for rendering
const DEFAULT_NODE_COLOR: &str = "#4A90E2";
const DEFAULT_EDGE_COLOR: &str = "#888888";
const SELECTED_NODE_COLOR: &str = "#FF6B6B";
const SELECTED_EDGE_COLOR: &str = "#FF6B6B";

#[derive(Properties, PartialEq)]
pub struct ApiGraphViewProps {
    pub system: Grammar,
    #[prop_or_default]
    pub on_navigate: Option<Callback<String>>,
    #[prop_or_default]
    pub show_edge_labels: bool,
}

pub enum ApiGraphMsg {
    NodeClicked(usize),
    #[allow(dead_code)]
    EdgeClicked(usize, usize),
}

pub struct ApiGraphView {
    selected_node: Option<usize>,
    selected_edge: Option<(usize, usize)>,
}

impl Component for ApiGraphView {
    type Message = ApiGraphMsg;
    type Properties = ApiGraphViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            selected_node: None,
            selected_edge: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ApiGraphMsg::NodeClicked(idx) => {
                // Toggle selection
                if self.selected_node == Some(idx) {
                    self.selected_node = None;
                } else {
                    self.selected_node = Some(idx);
                    self.selected_edge = None;
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

        html! {
            <div class="graph-view">
                <svg
                    class="graph-svg"
                    viewBox="0 0 800 800"
                    preserveAspectRatio="xMidYMid meet"
                >
                    { self.render_edges(system) }
                    if show_edge_labels {
                        { self.render_edge_labels(system) }
                    }
                    { self.render_nodes(ctx, system) }
                </svg>
            </div>
        }
    }
}

impl ApiGraphView {
    /// Render edges (lines) from the system
    fn render_edges(&self, system: &Grammar) -> Html {
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
                let stroke = if is_selected {
                    SELECTED_EDGE_COLOR
                } else {
                    DEFAULT_EDGE_COLOR
                };
                let stroke_width = if is_selected { 3.0 } else { 1.5 };

                html! {
                    <line
                        x1={ from_x.to_string() }
                        y1={ from_y.to_string() }
                        x2={ to_x.to_string() }
                        y2={ to_y.to_string() }
                        stroke={ stroke }
                        stroke-width={ stroke_width.to_string() }
                        class="edge"
                    />
                }
            })
            .collect::<Html>()
    }

    /// Render edge labels for connectives
    /// Instead of iterating connectives independently, we iterate through lines
    /// and find matching connectives to ensure labels align with the correct edges
    fn render_edge_labels(&self, system: &Grammar) -> Html {
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

            let label = connective.character_value.as_str();

            if label.is_empty() {
                return html! {};
            }

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
                <>
                    // Debug: Show actual midpoint with a red circle
                    <circle
                        cx={ mid_x.to_string() }
                        cy={ mid_y.to_string() }
                        r="3"
                        fill="red"
                        style="pointer-events: none;"
                    />
                    <g class="edge-label-group" transform={ format!("translate({} {}) rotate({})", mid_x, mid_y, rotation_angle) }>
                        <rect
                            x={ (-rect_width / 2.0).to_string() }
                            y={ (-rect_height / 2.0).to_string() }
                            width={ rect_width.to_string() }
                            height={ rect_height.to_string() }
                            fill="rgba(255, 255, 255, 0.9)"
                            stroke="rgba(37, 99, 235, 0.3)"
                            stroke-width="0.5"
                            rx="4"
                            style="pointer-events: none;"
                        />
                        <text
                            x="0"
                            y="0"
                            text-anchor="middle"
                            dominant-baseline="middle"
                            class="edge-label"
                            fill="#2563eb"
                            style="font-size: 10px; font-weight: 500; pointer-events: none; user-select: none;"
                        >
                            { label }
                        </text>
                    </g>
                </>
            }
        }).collect::<Html>()
    }

    /// Render nodes from coordinates and terms
    fn render_nodes(&self, ctx: &Context<Self>, system: &Grammar) -> Html {
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

            // Get term label for this position
            let term = system.term_at(position).unwrap_or("");

            html! {
                <g class="node" onclick={ onclick }>
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
