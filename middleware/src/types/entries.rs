//! Substrate wire types (Point, Line, Coordinate, Segment, Character).

use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use async_graphql::SimpleObject;

/// A topological anchor at a single vertex.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(SimpleObject))]
pub struct Point {
    pub id: String,
    pub order: i32,
    pub ordinality: i32,
}

/// A topological anchor at an edge (two vertices).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(SimpleObject))]
pub struct Line {
    pub id: String,
    pub order: i32,
    pub ordinality: i32,
    #[serde(rename = "positionSecondary")]
    pub position_secondary: i32,
}

/// A geometric coordinate bound to a Point.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(SimpleObject))]
pub struct Coordinate {
    pub id: String,
    #[serde(rename = "pointRef")]
    pub point_ref: String,
    pub order: i32,
    pub ordinality: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// A geometric segment bound to a Line.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(SimpleObject))]
pub struct Segment {
    pub id: String,
    #[serde(rename = "lineRef")]
    pub line_ref: String,
    #[serde(rename = "startCoordRef")]
    pub start_coord_ref: String,
    #[serde(rename = "endCoordRef")]
    pub end_coord_ref: String,
}

/// A semantic value (content only — anchor comes from vocabulary lookup).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(SimpleObject))]
pub struct Character {
    pub id: String,
    pub kind: String,
    pub value: String,
}
