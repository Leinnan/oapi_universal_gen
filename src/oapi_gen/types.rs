//! Data structures used by the OpenAPI code generator.
//!
//! This module contains type definitions for representing API metadata,
//! parameters, methods, and inline type definitions that are collected
//! during the OpenAPI specification parsing phase.

use crate::RequestType;
use openapiv3_1::path::ParameterIn;

#[derive(Debug, Clone, PartialEq)]
/// Represents the location of a parameter in an HTTP request.
///
/// OpenAPI parameters can be located in different parts of the request:
/// - `Path`: Parameters embedded in the URL path
/// - `Query`: Parameters in the URL query string
/// - `Header`: Parameters in HTTP headers
/// - `Cookie`: Parameters in Cookie headers
pub enum ParameterLocation {
    /// Parameter is part of the URL path
    Path,
    /// Parameter is in the URL query string
    Query,
    /// Parameter is in an HTTP header
    Header,
    /// Parameter is in a Cookie header
    Cookie,
}

impl From<ParameterIn> for ParameterLocation {
    fn from(inp: ParameterIn) -> Self {
        match inp {
            ParameterIn::Query => ParameterLocation::Query,
            ParameterIn::Path => ParameterLocation::Path,
            ParameterIn::Header => ParameterLocation::Header,
            ParameterIn::Cookie => ParameterLocation::Cookie,
        }
    }
}

#[derive(Debug, Clone)]
/// Information about a single API parameter.
///
/// This struct captures all metadata about a parameter from the
/// OpenAPI specification, including its name, type, location, and
/// whether it is required.
pub struct ParameterInfo {
    /// The parameter name as defined in the OpenAPI spec
    pub name: String,
    /// The Rust type string for this parameter
    pub ty: String,
    /// Whether this parameter is required
    pub required: bool,
    /// Optional description of the parameter
    pub description: Option<String>,
    /// Where the parameter is located in the request
    pub param_location: ParameterLocation,
}

#[derive(Debug, Clone)]
/// Information about an API method/endpoint.
///
/// This struct captures all metadata about an API endpoint including
/// its HTTP method, parameters, request body type, and response type.
pub struct MethodInfo {
    /// The name of the method (generated from path and HTTP method)
    pub name: String,
    /// The HTTP method type
    pub http_method: RequestType,
    /// List of parameters for this endpoint
    pub parameters: Vec<ParameterInfo>,
    /// The type name of the request body, if any
    pub request_body_type: Option<String>,
    /// Whether the request body is required
    pub request_body_required: bool,
    /// The type name of the response, if any
    pub response_type: Option<String>,
    /// Optional description of the endpoint
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
/// Information about an inline enum type discovered during parsing.
///
/// When the OpenAPI spec contains inline enum definitions that aren't
/// explicitly named, this struct captures their metadata for code generation.
pub struct InlineEnumInfo {
    /// The generated name for this enum
    pub name: String,
    /// The raw schema object
    pub schema: openapiv3_1::Object,
    /// Optional documentation for this enum
    pub description: Option<String>,
    /// Whether this is a tagged (discriminated) enum
    pub is_tagged: bool,
    /// The discriminator field name for tagged enums
    pub tag_field: Option<String>,
}

#[derive(Debug, Clone)]
/// Information about an inline struct type discovered during parsing.
///
/// When the OpenAPI spec contains inline object definitions, this struct
/// captures their field definitions for code generation.
pub struct InlineStructInfo {
    /// The generated name for this struct
    pub name: String,
    /// The generated field definitions as token streams
    pub fields: Vec<proc_macro2::TokenStream>,
    /// The schema object for structural comparison and deduplication
    pub schema: openapiv3_1::Object,
}
