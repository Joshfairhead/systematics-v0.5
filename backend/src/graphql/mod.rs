//! GraphQL module for the Systematics property graph API.

pub mod types;

pub use types::{create_schema, MutationRoot, QueryRoot, SharedGraph, SystematicsSchema};
