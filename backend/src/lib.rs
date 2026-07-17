//! Systematics API - A property graph implementation for Bennett's Systematics.
//!
//! This crate provides a GraphQL API for exploring systematic structures
//! from orders 1-12 (Monad through Dodecad).

pub mod core;
pub mod data;
pub mod graphql;
pub mod persistence;

pub use graphql::{create_schema, create_schema_with_store, SharedGraph, SystematicsSchema};
