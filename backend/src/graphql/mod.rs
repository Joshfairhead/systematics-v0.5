//! GraphQL module for the Systematics property graph API.

pub mod types;

pub use types::{
    create_schema, create_schema_with_store, MutationRoot, QueryRoot, SharedGraph, StorePath,
    SystematicsSchema,
};
