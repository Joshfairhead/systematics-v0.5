//! SixLaws — a visible readout of the **Controller** (mimic form): a system run
//! through the **six laws of three**. Self-contained — given a `system_id` it
//! fetches `runSixLaws` and renders the six readings (hexad position + colour +
//! the triad's terms reordered by each law). Direction is supplied by the law.

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::api::client::{GraphQLClient, LawReading};
use crate::app::get_graphql_endpoint;

#[derive(Properties, PartialEq)]
pub struct Props {
    /// The system to run through the six laws (its terms are read as a triad).
    pub system_id: String,
}

#[function_component(SixLaws)]
pub fn six_laws(props: &Props) -> Html {
    let readings = use_state(Vec::<LawReading>::new);
    {
        let readings = readings.clone();
        use_effect_with(props.system_id.clone(), move |sid| {
            let readings = readings.clone();
            let sid = sid.clone();
            spawn_local(async move {
                let client = GraphQLClient::new(get_graphql_endpoint());
                match client.run_six_laws(&sid).await {
                    Ok(r) => readings.set(r),
                    Err(_) => readings.set(Vec::new()),
                }
            });
            || ()
        });
    }

    // Only shown for triads (or 3-term selections); empty otherwise.
    if readings.is_empty() {
        return Html::default();
    }

    html! {
        <div class="six-laws">
            <div class="six-laws-title">{ "Controller — the six laws" }</div>
            <table class="six-laws-table">
                { for readings.iter().map(law_row) }
            </table>
        </div>
    }
}

fn law_row(r: &LawReading) -> Html {
    let alias = if r.aliases.is_empty() {
        String::new()
    } else {
        format!(" · {}", r.aliases.join(", "))
    };
    html! {
        <tr class="six-laws-row">
            <td class="six-laws-dot">
                <span class={ classes!("law-dot", format!("law-{}", r.colour)) }></span>
            </td>
            <td class="six-laws-name">{ format!("{} {}{}", r.hexad_position, r.law, alias) }</td>
            <td class="six-laws-reading">{ r.reading.join(" · ") }</td>
        </tr>
    }
}
