//! Link wire type (edge rendering shim).

use serde::{Deserialize, Serialize};

use super::LinkType;

#[cfg(feature = "server")]
use async_graphql::SimpleObject;

/// A rendering-line edge between two coordinates (retained until the frontend
/// consumes `Segment` entries directly).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(SimpleObject))]
pub struct Link {
    pub id: String,
    #[serde(rename = "baseId")]
    pub base_id: String,
    #[serde(rename = "targetId")]
    pub target_id: String,
    #[serde(rename = "linkType")]
    pub link_type: LinkType,
    #[serde(rename = "orderCardinality")]
    pub order_cardinality: Option<i32>,
}
