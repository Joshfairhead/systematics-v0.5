//! Shared enums for Systematics wire format.

use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use async_graphql::Enum;

/// Link type — currently only `Line` (coordinate-to-coordinate rendering edge).
/// Kept as an enum so future edge kinds can slot in without a schema change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(Enum))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LinkType {
    Line,
}
