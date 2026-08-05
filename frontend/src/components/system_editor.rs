//! In-graph system editor — edit a loaded system's name + node (term) and edge
//! (connective) labels, then Save. v1 authors the edited values as a system
//! (a fork via `authorSystem`); in-place update is a later refinement.
//!
//! Mount with `key = system_id` so switching systems resets the fields.

use systematics_middleware::RenderedSystem;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::reference_browser::AuthorRequest;

#[derive(Properties, PartialEq)]
pub struct SystemEditorProps {
    pub system: RenderedSystem,
    pub on_author: Callback<AuthorRequest>,
}

#[function_component(SystemEditor)]
pub fn system_editor(props: &SystemEditorProps) -> Html {
    let sys = &props.system;
    let name = use_state(|| sys.name.clone());
    let terms = use_state(|| sys.terms.iter().map(|t| t.value.clone()).collect::<Vec<String>>());
    let conns = use_state(|| {
        sys.connectives
            .iter()
            .map(|c| c.character_value.clone())
            .collect::<Vec<String>>()
    });

    let on_name = {
        let name = name.clone();
        Callback::from(move |e: InputEvent| name.set(e.target_unchecked_into::<HtmlInputElement>().value()))
    };
    // An input bound to index `i` of a Vec<String> state.
    let vec_input = |state: &UseStateHandle<Vec<String>>, i: usize| -> Html {
        let val = state.get(i).cloned().unwrap_or_default();
        let state = state.clone();
        let oninput = Callback::from(move |e: InputEvent| {
            let v = e.target_unchecked_into::<HtmlInputElement>().value();
            let mut next = (*state).clone();
            if i < next.len() {
                next[i] = v;
            }
            state.set(next);
        });
        html! { <input class="ed-input" value={ val } oninput={ oninput } /> }
    };

    let order = sys.order;
    let on_save = {
        let (on_author, name, terms, conns) = (props.on_author.clone(), name.clone(), terms.clone(), conns.clone());
        Callback::from(move |_: MouseEvent| {
            on_author.emit(AuthorRequest {
                name: (*name).clone(),
                order,
                terms: (*terms).clone(),
                connectives: (*conns).clone(),
            });
        })
    };
    let can_save = !name.trim().is_empty()
        && terms.iter().all(|t| !t.trim().is_empty())
        && conns.iter().all(|c| !c.trim().is_empty());

    html! {
        <div class="editor-form system-editor">
            <div class="editor-row">
                <input class="ed-input ed-name" placeholder="System name" value={ (*name).clone() } oninput={ on_name } />
                <span class="ed-label">{ format!("order {order}") }</span>
                <button class="elt-btn" disabled={ !can_save } onclick={ on_save }>{ "Save as system" }</button>
            </div>
            <div class="editor-fields">
                <span class="facet-label">{ "Nodes (terms)" }</span>
                { for (0..terms.len()).map(|i| vec_input(&terms, i)) }
            </div>
            <div class="editor-fields">
                <span class="facet-label">{ "Edges (connectives)" }</span>
                { for (0..conns.len()).map(|j| vec_input(&conns, j)) }
            </div>
            <p class="ed-hint">{ "Save authors these values as a system (rename to fork; same name errors if it exists)." }</p>
        </div>
    }
}
