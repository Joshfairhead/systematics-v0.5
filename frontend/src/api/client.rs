use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use systematics_middleware::{ApiError, Coordinate, RenderedSystem};

#[derive(Serialize)]
struct GraphQLRequest {
    query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    variables: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
struct GraphQLResponse<T> {
    data: Option<T>,
    errors: Option<Vec<GraphQLError>>,
}

#[derive(Deserialize, Debug)]
struct GraphQLError {
    message: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct SystemQueryResponse {
    system: Option<RenderedSystem>,
}

#[derive(Deserialize, Debug)]
struct SystemByNameQueryResponse {
    #[serde(rename = "systemByName")]
    system_by_name: Option<RenderedSystem>,
}

#[derive(Deserialize, Debug)]
struct AllSystemsQueryResponse {
    #[serde(rename = "allSystems")]
    all_systems: Vec<RenderedSystem>,
}

// -------- referencing layer (read-only) --------

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct RefSource {
    pub name: String,
    pub kind: Option<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct RefArtefact {
    pub title: String,
    pub url: Option<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct RefLookup {
    pub locator: String,
}

/// The System behind a reference's target address (resolved server-side): the
/// grouping key (`order`) and the value each perspective gives it (`coherence`).
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct RefSystem {
    pub id: String,
    pub order: i32,
    pub name: String,
    pub coherence: String,
    #[serde(rename = "termDesignation")]
    pub term_designation: String,
    #[serde(rename = "connectiveDesignation")]
    pub connective_designation: String,
}

/// A resolved citation for display: source → artefact → lookup, plus the owning
/// perspective and the resolved target system (for the reference browser).
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ReferenceView {
    pub id: String,
    pub target: String,
    pub note: Option<String>,
    pub source: Option<RefSource>,
    pub artefact: Option<RefArtefact>,
    pub lookup: Option<RefLookup>,
    /// Owning perspective name (the reference web). `None` on the older
    /// per-system tooltip query, which doesn't select it.
    #[serde(rename = "perspectiveName", default)]
    pub perspective_name: Option<String>,
    /// Fragment of the target after `#` (`coherence`, `term:2`, …; empty = whole
    /// system). `None` unless selected.
    #[serde(rename = "targetFragment", default)]
    pub target_fragment: Option<String>,
    /// Resolved target System (order/name/coherence/designations). `None` unless
    /// selected or when the target system isn't loaded.
    #[serde(rename = "targetSystem", default)]
    pub target_system: Option<RefSystem>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct ReferencesForResponse {
    #[serde(rename = "referencesFor")]
    references_for: Vec<ReferenceView>,
}

#[derive(Deserialize, Debug)]
struct ReferencesForSystemResponse {
    #[serde(rename = "referencesForSystem")]
    references_for_system: Vec<ReferenceView>,
}

#[derive(Deserialize, Debug)]
struct AllReferencesResponse {
    #[serde(rename = "allReferences")]
    all_references: Vec<ReferenceView>,
}

#[derive(Clone)]
pub struct GraphQLClient {
    endpoint: String,
}

impl GraphQLClient {
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }

    const SYSTEM_FIELDS: &'static str = r#"
        order
        systemId
        name
        coherence
        termDesignation
        connectiveDesignation
        terms {
            position
            characterId
            value
        }
        coordinates {
            id
            pointRef
            order
            position
            x
            y
            z
        }
        colours {
            position
            value
        }
        lines {
            id
            basePosition
            targetPosition
        }
        connectives {
            id
            basePosition
            targetPosition
            characterValue
        }
    "#;

    #[allow(dead_code)]
    pub async fn fetch_system_by_order(&self, order: i32) -> Result<RenderedSystem, ApiError> {
        let query = format!(
            r#"
            query GetSystem($order: Int!) {{
                system(order: $order) {{
                    {}
                }}
            }}
        "#,
            Self::SYSTEM_FIELDS
        );

        let variables = serde_json::json!({ "order": order });

        let response: GraphQLResponse<SystemQueryResponse> =
            self.execute_query(&query, Some(variables)).await?;

        if let Some(errors) = response.errors {
            return Err(ApiError::ParseError(
                errors
                    .iter()
                    .map(|e| e.message.clone())
                    .collect::<Vec<_>>()
                    .join(", "),
            ));
        }

        let system = response
            .data
            .and_then(|d| d.system)
            .ok_or_else(|| ApiError::NotFound(format!("System with order {} not found", order)))?;

        Ok(self.transform_coordinates(system))
    }

    pub async fn fetch_system(&self, system_name: &str) -> Result<RenderedSystem, ApiError> {
        let query = format!(
            r#"
            query GetSystemByName($name: String!) {{
                systemByName(name: $name) {{
                    {}
                }}
            }}
        "#,
            Self::SYSTEM_FIELDS
        );

        let variables = serde_json::json!({ "name": system_name });

        let response: GraphQLResponse<SystemByNameQueryResponse> =
            self.execute_query(&query, Some(variables)).await?;

        if let Some(errors) = response.errors {
            return Err(ApiError::ParseError(
                errors
                    .iter()
                    .map(|e| e.message.clone())
                    .collect::<Vec<_>>()
                    .join(", "),
            ));
        }

        let system = response
            .data
            .and_then(|d| d.system_by_name)
            .ok_or_else(|| ApiError::NotFound(format!("System '{}' not found", system_name)))?;

        Ok(self.transform_coordinates(system))
    }

    pub async fn fetch_all_systems(&self) -> Result<Vec<RenderedSystem>, ApiError> {
        let query = format!(
            r#"
            query GetAllSystems {{
                allSystems {{
                    {}
                }}
            }}
        "#,
            Self::SYSTEM_FIELDS
        );

        let response: GraphQLResponse<AllSystemsQueryResponse> =
            self.execute_query(&query, None).await?;

        if let Some(errors) = response.errors {
            return Err(ApiError::ParseError(
                errors
                    .iter()
                    .map(|e| e.message.clone())
                    .collect::<Vec<_>>()
                    .join(", "),
            ));
        }

        let data = response
            .data
            .ok_or_else(|| ApiError::NotFound("No systems found".to_string()))?;

        web_sys::console::log_1(
            &format!(
                "Fetched {} systems from allSystems query",
                data.all_systems.len()
            )
            .into(),
        );

        let systems: Vec<RenderedSystem> = data
            .all_systems
            .into_iter()
            .map(|sys| self.transform_coordinates(sys))
            .collect();

        Ok(systems)
    }

    /// Fetch all citations for a single Expression address (retained for
    /// direct lookups; the graph prefetches per-system for hover tooltips).
    #[allow(dead_code)]
    pub async fn fetch_references_for(
        &self,
        address: &str,
    ) -> Result<Vec<ReferenceView>, ApiError> {
        let query = r#"
            query Refs($address: String!) {
                referencesFor(address: $address) {
                    id
                    target
                    note
                    source { name kind }
                    artefact { title url }
                    lookup { locator }
                }
            }
        "#;
        let variables = serde_json::json!({ "address": address });
        let response: GraphQLResponse<ReferencesForResponse> =
            self.execute_query(query, Some(variables)).await?;

        if let Some(errors) = response.errors {
            return Err(ApiError::ParseError(
                errors
                    .iter()
                    .map(|e| e.message.clone())
                    .collect::<Vec<_>>()
                    .join(", "),
            ));
        }

        Ok(response.data.map(|d| d.references_for).unwrap_or_default())
    }

    /// Fetch every citation within a System at once (for hover tooltips).
    /// Each `ReferenceView.target` encodes the cited term/connective.
    pub async fn fetch_references_for_system(
        &self,
        system_id: &str,
    ) -> Result<Vec<ReferenceView>, ApiError> {
        let query = r#"
            query SysRefs($systemId: String!) {
                referencesForSystem(systemId: $systemId) {
                    id
                    target
                    note
                    source { name kind }
                    artefact { title url }
                    lookup { locator }
                }
            }
        "#;
        let variables = serde_json::json!({ "systemId": system_id });
        let response: GraphQLResponse<ReferencesForSystemResponse> =
            self.execute_query(query, Some(variables)).await?;

        if let Some(errors) = response.errors {
            return Err(ApiError::ParseError(
                errors.iter().map(|e| e.message.clone()).collect::<Vec<_>>().join(", "),
            ));
        }

        Ok(response.data.map(|d| d.references_for_system).unwrap_or_default())
    }

    /// Every citation in the graph, enriched with owning perspective + resolved
    /// target system — the single query that powers the reference browser
    /// (both the table and the compare-by-order matrix).
    pub async fn fetch_all_references(&self) -> Result<Vec<ReferenceView>, ApiError> {
        let query = r#"
            query AllRefs {
                allReferences {
                    id
                    target
                    note
                    perspectiveName
                    targetFragment
                    source { name kind }
                    artefact { title url }
                    lookup { locator }
                    targetSystem {
                        id order name coherence termDesignation connectiveDesignation
                    }
                }
            }
        "#;
        let response: GraphQLResponse<AllReferencesResponse> =
            self.execute_query(query, None).await?;

        if let Some(errors) = response.errors {
            return Err(ApiError::ParseError(
                errors.iter().map(|e| e.message.clone()).collect::<Vec<_>>().join(", "),
            ));
        }

        Ok(response.data.map(|d| d.all_references).unwrap_or_default())
    }

    async fn execute_query<T: for<'de> Deserialize<'de>>(
        &self,
        query: &str,
        variables: Option<serde_json::Value>,
    ) -> Result<GraphQLResponse<T>, ApiError> {
        let request_body = GraphQLRequest {
            query: query.to_string(),
            variables,
        };

        let response = Request::post(&self.endpoint)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .map_err(|e| ApiError::ParseError(e.to_string()))?
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;

        if !response.ok() {
            return Err(ApiError::NetworkError(format!(
                "Request failed with status: {}",
                response.status()
            )));
        }

        response
            .json::<GraphQLResponse<T>>()
            .await
            .map_err(|e| ApiError::ParseError(e.to_string()))
    }

    fn transform_coordinates(&self, mut system: RenderedSystem) -> RenderedSystem {
        let viewport_width = 800.0;
        let viewport_height = 800.0;
        let margin = 100.0;

        system.coordinates = transform_coordinates_to_viewport(
            system.coordinates,
            viewport_width,
            viewport_height,
            margin,
        );

        system
    }
}

fn transform_coordinates_to_viewport(
    coords: Vec<Coordinate>,
    viewport_width: f64,
    viewport_height: f64,
    margin: f64,
) -> Vec<Coordinate> {
    if coords.is_empty() {
        return coords;
    }

    if coords.len() == 1 {
        let mut coord = coords.into_iter().next().unwrap();
        coord.x = viewport_width / 2.0;
        coord.y = viewport_height / 2.0;
        return vec![coord];
    }

    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for coord in &coords {
        min_x = min_x.min(coord.x);
        max_x = max_x.max(coord.x);
        min_y = min_y.min(coord.y);
        max_y = max_y.max(coord.y);
    }

    let center_x = (min_x + max_x) / 2.0;
    let center_y = (min_y + max_y) / 2.0;
    let extent_x = (max_x - min_x).max(0.0001);
    let extent_y = (max_y - min_y).max(0.0001);
    let max_extent = extent_x.max(extent_y);
    let available_width = viewport_width - 2.0 * margin;
    let available_height = viewport_height - 2.0 * margin;
    let available_size = available_width.min(available_height);
    let scale = available_size / max_extent;
    let viewport_center_x = viewport_width / 2.0;
    let viewport_center_y = viewport_height / 2.0;

    coords
        .into_iter()
        .map(|mut coord| {
            coord.x = (coord.x - center_x) * scale + viewport_center_x;
            coord.y = -(coord.y - center_y) * scale + viewport_center_y;
            coord
        })
        .collect()
}
