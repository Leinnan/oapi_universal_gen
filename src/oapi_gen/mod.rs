//! OpenAPI code generation module.
//!
//! This module provides functionality for generating Rust client code
//! from OpenAPI (formerly Swagger) specifications. It parses OpenAPI
//! JSON/YAML documents and produces type-safe Rust code including:
//!
//! - Data structures (structs and enums) for API types
//! - Request/response type definitions
//! - API service trait with method signatures
//!
//! # Example
//!
//! ```ignore
//! use oapi_universal_gen::oapi_gen::generate_from_json;
//!
//! let openapi_json = r#"{
//!   "openapi": "3.0.0",
//!   "info": { "title": "Test API", "version": "1.0.0" },
//!   "paths": {}
//! }"#;
//!
//! let generated_code = generate_from_json(openapi_json);
//! println!("{}", generated_code);
//! ```
//!
//! # Features
//!
//! - Supports OpenAPI 3.0+ specifications
//! - Generates enum types with serde derive macros
//! - Generates struct types with field serialization attributes
//! - Handles tagged (discriminated) enums
//! - Generates async API service methods

pub mod generator;
pub mod methods;
pub mod schemas;
pub mod types;
pub mod utils;

pub use generator::{generate_from_json, generate_openapi_spec};
