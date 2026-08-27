//! RenderedSystem — a System resolved into a complete bound K-graph. The backend
//! joins a System's Grammar (structure) with its Vocabulary (terms) and the
//! canonical colour vocabulary to produce this. Kept as a wire type so the
//! frontend renderer can consume a resolved system without walking the System.

use serde::{Deserialize, Serialize};

use super::Coordinate;

#[cfg(feature = "server")]
use async_graphql::SimpleObject;

/// A rendered term at a specific position within a system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(SimpleObject))]
pub struct GrammarTerm {
    pub position: i32,
    pub value: String,
}

/// A colour at a specific position within a system (hex form).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(SimpleObject))]
pub struct GrammarColour {
    pub position: i32,
    pub value: String,
}

/// A rendering edge between two coordinates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(SimpleObject))]
pub struct GrammarLine {
    pub id: String,
    #[serde(rename = "basePosition")]
    pub base_position: i32,
    #[serde(rename = "targetPosition")]
    pub target_position: i32,
}

/// A connective within a system, with its character label and endpoints.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(SimpleObject))]
pub struct GrammarConnective {
    pub id: String,
    #[serde(rename = "basePosition")]
    pub base_position: i32,
    #[serde(rename = "targetPosition")]
    pub target_position: i32,
    #[serde(rename = "characterValue")]
    pub character_value: String,
}

/// A complete rendered view of a system at a given order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(SimpleObject))]
pub struct RenderedSystem {
    pub order: i32,
    #[serde(rename = "systemId")]
    pub system_id: String,
    pub name: String,
    pub coherence: String,
    #[serde(rename = "termDesignation")]
    pub term_designation: String,
    #[serde(rename = "connectiveDesignation")]
    pub connective_designation: String,
    pub terms: Vec<GrammarTerm>,
    pub coordinates: Vec<Coordinate>,
    pub colours: Vec<GrammarColour>,
    pub lines: Vec<GrammarLine>,
    pub connectives: Vec<GrammarConnective>,
    /// The canonical *class* this system instantiates (same order); `None` when
    /// this system is itself canonical. Drives the "Canonical override" toggle —
    /// terms/connectives pair with the instance's by position.
    #[serde(default, rename = "canonicalClass")]
    pub canonical_class: Option<Box<RenderedSystem>>,
}

impl RenderedSystem {
    pub fn display_name(&self) -> String {
        self.name.clone()
    }

    pub fn k_notation(&self) -> String {
        format!("K{}", self.order)
    }

    pub fn description(&self) -> String {
        self.coherence.clone()
    }

    pub fn node_count(&self) -> usize {
        self.order as usize
    }

    pub fn term_at(&self, position: i32) -> Option<&str> {
        self.terms
            .iter()
            .find(|t| t.position == position)
            .map(|t| t.value.as_str())
    }

    pub fn colour_at(&self, position: i32) -> Option<&str> {
        self.colours
            .iter()
            .find(|c| c.position == position)
            .map(|c| c.value.as_str())
    }

    pub fn coordinate_at(&self, position: i32) -> Option<&Coordinate> {
        self.coordinates.iter().find(|c| c.position == position)
    }
}
