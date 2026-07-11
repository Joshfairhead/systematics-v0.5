//! Vocabulary wire types.

use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use async_graphql::SimpleObject;

/// Ordered lists of Point and Line references for one Order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(SimpleObject))]
pub struct TopologicalVocabulary {
    pub id: String,
    pub order: i32,
    pub points: Vec<String>,
    pub lines: Vec<String>,
}

/// Ordered lists of Coordinate and Segment references for one Order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(SimpleObject))]
pub struct GeometricVocabulary {
    pub id: String,
    pub order: i32,
    pub coordinates: Vec<String>,
    pub segments: Vec<String>,
}

/// Ordered Character references for terms + connectives at one Order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(SimpleObject))]
pub struct SemanticVocabulary {
    pub id: String,
    pub name: String,
    pub order: i32,
    pub terms: Vec<String>,
    pub connectives: Vec<String>,
}
