//! The **ELT bar** — the operation edge as a self-contained, swappable module.
//!
//! Data-organisation reads as a pipeline across three stages (the ELT triad), with
//! impulses +/−/= = active/receptive/reconciling:
//!  * **Extract (+)** — a *live stage indicator*, not a button: reach out and select
//!    across sources. Ticking a system's checkbox selects it; the pill lights up and
//!    shows the count. Selecting *is* extracting.
//!  * **Load (−)** — the one actionable control: name the monad, click Load, and the
//!    ticked systems are dumped *raw* into a new named Monad (a single graph node + its
//!    bucket). Disabled until ≥1 system is ticked.
//!  * **Transform (=)** — clean up / join / sequence. Scope-aware: dim in Nullad, live
//!    only once you're *inside* a loaded monad (there's nothing to transform until then).
//!
//! The bar is presentational: it owns only the name field; the selection (Extract) and
//! the load effect (Load) live with the parent, emitted via `on_load`.

use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EltBarProps {
    /// How many systems are currently ticked — the Extract (+) stage's live count.
    pub count: usize,
    /// True when scoped inside a loaded monad — enables the Transform (=) stage.
    #[prop_or_default]
    pub in_monad: bool,
    /// Feedback from the last operation (shown beside the bar).
    #[prop_or_default]
    pub note: Option<String>,
    /// Load (−): fire with the entered monad name to land the ticked systems in a new
    /// Monad. An empty name is allowed — the parent falls back to a provisional name.
    pub on_load: Callback<String>,
}

#[function_component(EltBar)]
pub fn elt_bar(props: &EltBarProps) -> Html {
    let name = use_state(String::new);
    let on_name = {
        let name = name.clone();
        Callback::from(move |e: InputEvent| {
            name.set(e.target_unchecked_into::<HtmlInputElement>().value())
        })
    };
    let can_load = props.count > 0;
    let on_load_click = {
        let name = name.clone();
        let on_load = props.on_load.clone();
        Callback::from(move |_: MouseEvent| {
            on_load.emit((*name).clone());
            name.set(String::new());
        })
    };

    let extract_cls = if props.count > 0 {
        "elt-pill elt-pill-active"
    } else {
        "elt-pill"
    };
    let transform_cls = if props.in_monad {
        "elt-pill elt-pill-active"
    } else {
        "elt-pill elt-pill-dim"
    };
    let extract_label = if props.count > 0 {
        format!("Extract ({})", props.count)
    } else {
        "Extract".to_string()
    };

    html! {
        <div class="elt-module">
            <span class="elt-module-label">{ "Operation" }</span>
            <div class="elt-stages">
                <span class={ extract_cls }
                    title="Extract (+) — reach out and select across sources. Tick a system to select it.">
                    { extract_label }
                </span>
                <span class="elt-arrow">{ "→" }</span>
                <span class="elt-stage-load">
                    <input
                        class="elt-name"
                        placeholder="name sequence…"
                        value={ (*name).clone() }
                        oninput={ on_name }
                        disabled={ !can_load }
                    />
                    <button class="elt-pill elt-pill-load" disabled={ !can_load } onclick={ on_load_click }
                        title="Load (−) — dump the selected systems raw into a named sequence (a monad-head node + its members).">
                        { "Load" }
                    </button>
                </span>
                <span class="elt-arrow">{ "→" }</span>
                <span class={ transform_cls }
                    title="Transform (=) — clean up, join, sequence. Available inside a loaded sequence.">
                    { "Transform" }
                </span>
                if let Some(note) = props.note.as_deref() {
                    <span class="elt-note">{ note }</span>
                }
            </div>
        </div>
    }
}
