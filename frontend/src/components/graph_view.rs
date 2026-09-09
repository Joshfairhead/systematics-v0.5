use systematics_middleware::RenderedSystem;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::api::client::ReferenceView;

/// A single on-graph value edit — overwrite one term (by ordinality) or one
/// connective (by its endpoints). The parent re-authors the whole system with this
/// value applied (Store = write, overwrite), which also updates the list view.
#[derive(Clone, PartialEq, Debug)]
pub enum GraphEdit {
    Term { ordinality: i32, value: String },
    Connective { base: i32, target: i32, value: String },
    /// Rename the system (re-author under the new name; the parent deletes the old).
    Name { value: String },
}

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
    /// When true, labels show the canonical *class* (`system.canonical_class`).
    #[prop_or_default]
    pub show_canonical: bool,
    /// Toggle the Canonical-override switch.
    #[prop_or_default]
    pub on_toggle_canonical: Option<Callback<()>>,
    /// Update (edit) mode — on-graph editing. Click a node/edge to overwrite it.
    #[prop_or_default]
    pub editing: bool,
    /// Toggle Update mode (rendered as an in-canvas overlay, under Canonical).
    #[prop_or_default]
    pub on_toggle_editing: Option<Callback<()>>,
    /// Commit a single on-graph value edit (term or connective).
    #[prop_or_default]
    pub on_edit_value: Option<Callback<GraphEdit>>,
    /// Suggested name for a brand-new (blank) instance — pre-fills the name box when
    /// Update turns on. The parent supplies the next free "sketchNN".
    #[prop_or_default]
    pub name_hint: String,
}

pub enum ApiGraphMsg {
    NodeClicked(usize),
    EdgeClicked(usize, usize),
    /// Start renaming the system (click the title in Update mode).
    StartRename,
    /// The inline value editor's text changed.
    DraftChanged(String),
    /// Commit the current draft as an edit to the selected node/edge/name.
    CommitEdit,
    /// Abandon the current edit (clear the selection + draft).
    CancelEdit,
}

pub struct ApiGraphView {
    selected_node: Option<usize>,
    selected_edge: Option<(usize, usize)>,
    /// In-progress value for the inline editor (Update mode).
    draft: String,
    /// True while editing the system's name (rather than a node/edge value).
    renaming: bool,
}

impl Component for ApiGraphView {
    type Message = ApiGraphMsg;
    type Properties = ApiGraphViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            selected_node: None,
            selected_edge: None,
            draft: String::new(),
            renaming: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let system = &ctx.props().system;
        match msg {
            ApiGraphMsg::NodeClicked(idx) => {
                // Toggle selection
                self.renaming = false;
                let selecting = self.selected_node != Some(idx);
                if selecting {
                    self.selected_node = Some(idx);
                    self.selected_edge = None;
                    // In Update mode, seed the inline editor with the term's value —
                    // empty on a blank new instance (nothing to carry over from the seed).
                    if ctx.props().editing {
                        let blank = system.canonical_class.is_none() && !ctx.props().show_canonical;
                        self.draft = if blank {
                            String::new()
                        } else {
                            system.term_at((idx + 1) as i32).unwrap_or("").to_string()
                        };
                    }
                } else {
                    self.selected_node = None;
                }
                true
            }
            ApiGraphMsg::EdgeClicked(from, to) => {
                self.renaming = false;
                let edge = if from < to { (from, to) } else { (to, from) };
                if self.selected_edge == Some(edge) {
                    self.selected_edge = None;
                } else {
                    self.selected_edge = Some(edge);
                    self.selected_node = None;
                    // In Update mode, seed the editor with this connective's value —
                    // empty on a blank new instance.
                    if ctx.props().editing {
                        let blank = system.canonical_class.is_none() && !ctx.props().show_canonical;
                        let (b, t) = ((edge.0 + 1) as i32, (edge.1 + 1) as i32);
                        self.draft = if blank {
                            String::new()
                        } else {
                            system
                                .connectives
                                .iter()
                                .find(|c| {
                                    (c.base_ordinality.min(c.target_ordinality),
                                     c.base_ordinality.max(c.target_ordinality)) == (b, t)
                                })
                                .map(|c| c.character_value.clone())
                                .unwrap_or_default()
                        };
                    }
                }
                true
            }
            ApiGraphMsg::StartRename => {
                // Click the title in Update mode → edit the system's name. In blank mode
                // the draft starts empty (naming creates a fresh new instance).
                self.renaming = true;
                self.selected_node = None;
                self.selected_edge = None;
                let blank = system.canonical_class.is_none() && !ctx.props().show_canonical;
                self.draft = if blank {
                    String::new()
                } else if system.system_name.is_empty() {
                    system.name.clone()
                } else {
                    system.system_name.clone()
                };
                true
            }
            ApiGraphMsg::DraftChanged(v) => {
                self.draft = v;
                true
            }
            ApiGraphMsg::CommitEdit => {
                if let Some(cb) = ctx.props().on_edit_value.clone() {
                    if self.renaming {
                        if !self.draft.trim().is_empty() {
                            cb.emit(GraphEdit::Name { value: self.draft.trim().to_string() });
                        }
                    } else if let Some(idx) = self.selected_node {
                        cb.emit(GraphEdit::Term {
                            ordinality: (idx + 1) as i32,
                            value: self.draft.clone(),
                        });
                    } else if let Some((a, b)) = self.selected_edge {
                        cb.emit(GraphEdit::Connective {
                            base: (a + 1) as i32,
                            target: (b + 1) as i32,
                            value: self.draft.clone(),
                        });
                    }
                }
                self.selected_node = None;
                self.selected_edge = None;
                self.renaming = false;
                self.draft.clear();
                true
            }
            ApiGraphMsg::CancelEdit => {
                self.selected_node = None;
                self.selected_edge = None;
                self.renaming = false;
                self.draft.clear();
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old: &Self::Properties) -> bool {
        // When Update turns on, open the system-name box first (pre-filled): a fresh
        // sketch name for a blank new instance, or the current name for a custom system.
        // Clicking a node/edge then switches the panel to that element's value editor.
        if ctx.props().editing && !old.editing {
            let system = &ctx.props().system;
            let blank = system.canonical_class.is_none() && !ctx.props().show_canonical;
            self.selected_node = None;
            self.selected_edge = None;
            self.renaming = true;
            self.draft = if blank {
                ctx.props().name_hint.clone()
            } else if system.system_name.is_empty() {
                system.name.clone()
            } else {
                system.system_name.clone()
            };
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let system = &ctx.props().system;
        let show_edge_labels = ctx.props().show_edge_labels;

        // Canonical systems (no canonical_class) default to the canonical view. Turning
        // the Canonical switch OFF on one shows a **blank template** of that order for
        // fresh input. The blank view is a preview only (editing is disabled) so it can
        // never overwrite the canonical seed — author a new system via Create in the list.
        let is_canonical = system.canonical_class.is_none();
        let blank = is_canonical && !ctx.props().show_canonical;
        // Editable when it's a custom/instance system, or a blank new-instance draft.
        // A canonical seed shown canonically is READ-ONLY (no Update switch).
        let editable = system.canonical_class.is_some() || blank;
        let display_owned;
        let display: &RenderedSystem = if blank {
            let mut s = system.clone();
            for t in &mut s.terms {
                t.value.clear();
            }
            for c in &mut s.connectives {
                c.character_value.clear();
            }
            display_owned = s;
            &display_owned
        } else {
            system
        };

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

        // Canvas title = the system's own name; fall back to the order label when a
        // response didn't carry `systemName`. In blank mode it's a "New {Type}" prompt.
        let title = if blank {
            format!("New {}", system.name)
        } else if system.system_name.is_empty() {
            system.name.clone()
        } else {
            system.system_name.clone()
        };
        // The edge-labels switch reads the system's connective designation
        // (Acts / Interplays / Mutualities …). Beyond the octad (orders 9–12) the
        // designation is still being researched, so fall back to a generic "Connectives".
        let edge_label = if system.order_cardinality >= 9 || system.connective_designation.is_empty() {
            "Connectives".to_string()
        } else {
            system.connective_designation.clone()
        };

        // The shared inline input panel (draft + Save/Cancel) for a labelled edit.
        let input_panel = |label: String, draft: String| -> Html {
            let oninput = ctx.link().callback(|e: InputEvent| {
                ApiGraphMsg::DraftChanged(e.target_unchecked_into::<HtmlInputElement>().value())
            });
            let onsave = ctx.link().callback(|_| ApiGraphMsg::CommitEdit);
            let oncancel = ctx.link().callback(|_| ApiGraphMsg::CancelEdit);
            html! {
                <div class="graph-edit-panel">
                    <span class="edit-panel-label">{ label }</span>
                    <input class="edit-panel-input" value={ draft } oninput={ oninput } />
                    <button class="edit-panel-save" onclick={ onsave }>{ "Save" }</button>
                    <button class="edit-panel-cancel" onclick={ oncancel } title="Cancel">{ "✕" }</button>
                </div>
            }
        };

        // The on-graph editor (Update mode). A blank new-instance and a custom system
        // behave the same — click the name / a node / an edge to edit it. On a blank
        // instance the first such edit lazily creates the system (app-side). A canonical
        // seed shown canonically is read-only (no panel).
        let edit_panel = if ctx.props().editing && (editable || blank) {
            let target = if self.renaming {
                Some("name".to_string())
            } else {
                self.selected_node
                    .map(|idx| format!("{} {}", system.term_designation, idx + 1))
                    .or_else(|| self.selected_edge.map(|(a, b)| format!("edge {}–{}", a + 1, b + 1)))
            };
            match target {
                Some(label) => input_panel(format!("Update {label}"), self.draft.clone()),
                None => html! {
                    <div class="graph-edit-hint">{ "Update mode — click the name, a node, or an edge to edit it" }</div>
                },
            }
        } else if blank {
            html! { <div class="graph-edit-hint">{ "Blank template — turn on Update to start a new system." }</div> }
        } else {
            html! {}
        };

        // In Update mode the title is clickable to rename the system. (The .editable
        // class enables pointer events; otherwise the header ignores clicks.)
        let title_click = ctx.link().callback(|_| ApiGraphMsg::StartRename);

        html! {
            <div class="graph-view">
                // Compact single-line header. On display a system's terms /
                // connectives take their designations (Impulses/Acts for the
                // triad, Sources/Interplays for the tetrad); the generic API
                // field names stay `terms`/`connectives`. Each metadata item is
                // itself referenceable — hover shows its citation.
                <header class="graph-header">
                    <span
                        class={ classes!("graph-title", ctx.props().editing.then_some("editable")) }
                        onclick={ title_click }
                        title={ if ctx.props().editing { "Click to rename" } else { "" } }
                    >{ title }</span>
                    <span class="graph-subtitle">
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
                        { format!("{} {}", system.order_cardinality, system.term_designation) }
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
                        <span class="toggle-label">{ edge_label }</span>
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

                // Canonical switch — the top switch, present for every system. For a
                // canonical system it's ON by default (its own values); OFF shows a blank
                // template. For an instance it flips node/edge labels to the class.
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

                // Update-mode toggle — under the Edge Labels switch. Always present;
                // turning it on is a state change that drops the read-only canonical
                // view (a canonical seed becomes a blank editable new instance).
                if let Some(on_toggle) = ctx.props().on_toggle_editing.clone() {
                    <label class="update-toggle-overlay">
                        <span class="toggle-label">{ "Update" }</span>
                        <div class="toggle-switch">
                            <input
                                type="checkbox"
                                checked={ ctx.props().editing }
                                onclick={ Callback::from(move |_| on_toggle.emit(())) }
                            />
                            <span class="slider"></span>
                        </div>
                    </label>
                }

                { edit_panel }

                <svg
                    class="graph-svg"
                    viewBox="0 0 800 800"
                    preserveAspectRatio="xMidYMid meet"
                >
                    { self.render_edges(ctx, display) }
                    if show_edge_labels {
                        { self.render_edge_labels(ctx, display) }
                    }
                    { self.render_nodes(ctx, display) }
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
                let base_pos = line.base_ordinality;
                let target_pos = line.target_ordinality;

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
                // Click the edge to select it (and, in Update mode, edit its connective).
                let edge_click = ctx
                    .link()
                    .callback(move |_| ApiGraphMsg::EdgeClicked(from_idx, to_idx));

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
                            onclick={ edge_click }
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
            let line_base_pos = line.base_ordinality;
            let line_target_pos = line.target_ordinality;

            let matching_connective = system.connectives.iter().enumerate().find(|(_, conn)| {
                let conn_base = conn.base_ordinality;
                let conn_target = conn.target_ordinality;
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
                            (k.base_ordinality.min(k.target_ordinality), k.base_ordinality.max(k.target_ordinality))
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
            let ordinality = coord.ordinality;
            let idx = (ordinality - 1) as usize;  // Convert 1-based ordinality to 0-based index

            let is_selected = self.selected_node == Some(idx);

            // Get color for this node from colours array, or use default
            let fill = if is_selected {
                SELECTED_NODE_COLOR.to_string()
            } else {
                system.colour_at(ordinality)
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| DEFAULT_NODE_COLOR.to_string())
            };

            let radius = if is_selected { 18.0 } else { 12.0 };
            let onclick = ctx.link().callback(move |_| ApiGraphMsg::NodeClicked(idx));

            // Get term label for this ordinality (canonical class when overridden).
            let class_term = if ctx.props().show_canonical {
                system
                    .canonical_class
                    .as_deref()
                    .and_then(|c| c.term_at(ordinality))
                    .map(|s| s.to_string())
            } else {
                None
            };
            let term: &str = class_term
                .as_deref()
                .unwrap_or_else(|| system.term_at(ordinality).unwrap_or(""));

            // Citations for this term → native hover tooltip on the node.
            let address = format!("system:{}#term:{}", system.system_id, ordinality);
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
                        paint-order_cardinality="stroke"
                        style="font-size: 12px; font-weight: bold; pointer-events: none; user-select: none;"
                    >
                        { ordinality }
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
/// always all three lines — mirrors the Citation triad's three impulses, with a
/// dash where a field is absent, so the triad is legible at a glance.
fn format_reference(r: &ReferenceView) -> String {
    let source = r.source.as_ref().map(|s| s.name.as_str()).unwrap_or("—");
    let artefact = r.artefact.as_ref().map(|a| a.title.as_str()).unwrap_or("—");
    let locator = r.lookup.as_ref().map(|l| l.locator.as_str()).unwrap_or("—");
    let mut lines = vec![
        format!("Source: {source}"),
        format!("Artefact: {artefact}"),
        format!("Lookup: {locator}"),
    ];
    if let Some(n) = &r.note {
        lines.push(format!("Note: {n}"));
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
