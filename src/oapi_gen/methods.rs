//! Method generation from OpenAPI path items.
//!
//! This module provides functions for generating Rust method signatures
//! and implementations from OpenAPI path definitions. It handles HTTP
//! method mapping, parameter processing, request body handling, and
//! response type extraction.

use openapiv3_1::{Components, OpenApi};
use quote::{format_ident, quote};

use super::types::{
    InlineEnumInfo, InlineStructInfo, MethodInfo, ParameterInfo, ParameterLocation,
};
use super::utils::type_to_tokens;

use super::schemas::{add_parameter, extract_request_body_type, extract_response_type};

/// Builds method information from an OpenAPI path item.
///
/// Collects all metadata about an API endpoint including its name,
/// HTTP method, parameters, request body type, and response type.
///
/// # Arguments
///
/// * `path` - The API path
/// * `http_method` - The HTTP method type
/// * `operation` - The operation definition
/// * `path_params` - Parameters inherited from the path item
/// * `components` - The OpenAPI components
/// * `inline_structs` - Collector for inline struct definitions
/// * `inline_enums` - Collector for inline enum definitions
/// * `main_inline_structs` - Main collector for inline structs
///
/// # Returns
///
/// A `MethodInfo` structure with all endpoint metadata
pub fn build_method_info(
    path: &str,
    http_method: crate::RequestType,
    operation: Option<&openapiv3_1::path::Operation>,
    path_params: &[openapiv3_1::path::Parameter],
    components: &Components,
    inline_structs: &mut Vec<proc_macro2::TokenStream>,
    inline_enums: &mut Vec<InlineEnumInfo>,
    main_inline_structs: &mut Vec<InlineStructInfo>,
) -> MethodInfo {
    let method_name = format!(
        "{}_{}",
        super::utils::clean_path_for_method_name(path),
        super::utils::method_suffix(&http_method)
    );

    let mut parameters: Vec<ParameterInfo> = Vec::new();

    for param in path_params {
        add_parameter(&mut parameters, param);
    }
    if let Some(oper) = operation {
        if let Some(params) = &oper.parameters {
            for param in params {
                add_parameter(&mut parameters, param);
            }
        }
        let (request_body_type, request_body_required) = oper
            .request_body
            .as_ref()
            .map(|rb| {
                extract_request_body_type(
                    &openapiv3_1::RefOr::T(rb.clone()),
                    components,
                    inline_structs,
                    inline_enums,
                    main_inline_structs,
                    &method_name,
                )
            })
            .unwrap_or((None, false));

        let response_type = extract_response_type(
            &oper.responses,
            components,
            inline_structs,
            inline_enums,
            &method_name,
            main_inline_structs,
        );

        let description = oper.description.clone().or(oper.summary.clone());

        MethodInfo {
            name: method_name,
            http_method,
            parameters,
            request_body_type,
            request_body_required,
            response_type,
            description,
        }
    } else {
        MethodInfo {
            name: method_name,
            http_method,
            parameters,
            request_body_type: None,
            request_body_required: false,
            response_type: None,
            description: None,
        }
    }
}

/// Generates all methods for an OpenAPI specification.
///
/// Iterates through all paths and their operations, generating method
/// signatures and implementations for each endpoint.
///
/// # Arguments
///
/// * `openapi` - The parsed OpenAPI specification
/// * `components` - The OpenAPI components
/// * `inline_enums` - Collector for inline enum definitions
/// * `main_inline_structs` - Main collector for inline structs
///
/// # Returns
///
/// A tuple of (inline input struct definitions, method implementations)
pub fn generate_methods(
    openapi: &OpenApi,
    components: &Components,
    inline_enums: &mut Vec<InlineEnumInfo>,
    main_inline_structs: &mut Vec<InlineStructInfo>,
) -> (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) {
    let mut inline_input_structs = Vec::new();
    let mut methods = Vec::new();

    for (path, path_item) in openapi.paths.paths.iter() {
        let path_params: Vec<openapiv3_1::path::Parameter> = path_item
            .parameters
            .as_ref()
            .map(|p| p.clone())
            .unwrap_or_default();

        let operations = [
            ("get", crate::RequestType::Get, path_item.get.as_ref()),
            ("post", crate::RequestType::Post, path_item.post.as_ref()),
            ("put", crate::RequestType::Put, path_item.put.as_ref()),
            ("patch", crate::RequestType::Patch, path_item.patch.as_ref()),
            (
                "delete",
                crate::RequestType::Delete,
                path_item.delete.as_ref(),
            ),
        ];

        for (_method_name, http_method, operation) in operations {
            if let Some(op) = operation {
                let method_info = build_method_info(
                    path,
                    http_method,
                    Some(op),
                    &path_params,
                    components,
                    &mut inline_input_structs,
                    inline_enums,
                    main_inline_structs,
                );

                let method_body = generate_method_body(path, &method_info);
                methods.push(method_body);
            }
        }
    }

    (inline_input_structs, methods)
}

/// Formats a path with its path parameters.
///
/// Converts OpenAPI path parameter syntax (e.g., `{id}`) to Rust format
/// strings and generates the parameter encoding expressions.
///
/// # Arguments
///
/// * `path` - The OpenAPI path
/// * `params` - The list of parameters
///
/// # Returns
///
/// A tuple of (format token stream, replacement expressions)
pub fn format_path_with_params(
    path: &str,
    params: &[ParameterInfo],
) -> (proc_macro2::TokenStream, Vec<proc_macro2::TokenStream>) {
    let mut format_string = path.to_string();
    let mut placeholders = Vec::new();
    let mut replacements = Vec::new();

    for param in params
        .iter()
        .filter(|p| p.param_location == ParameterLocation::Path)
    {
        let placeholder = format!("{{{}}}", param.name);
        let rust_param_name = super::utils::to_valid_rust_field_name(&param.name);
        let ident = format_ident!("{}", rust_param_name);
        placeholders.push(placeholder.clone());
        replacements.push(quote! {
            ::oapi_universal_gen::urlencode(#ident)
        });
        format_string = format_string.replace(&placeholder, "{}");
    }

    let format_args = if replacements.is_empty() {
        quote! { #format_string }
    } else {
        quote! { format!(#format_string, #(#replacements),*) }
    };

    (format_args, replacements)
}

/// Builds query parameter expressions from parameter definitions.
///
/// Generates Rust code that creates query parameter tuples from
/// the method parameters.
///
/// # Arguments
///
/// * `params` - The list of parameters
///
/// # Returns
///
/// A vector of token streams for query parameter construction
pub fn build_query_params(params: &[ParameterInfo]) -> Vec<proc_macro2::TokenStream> {
    params
        .iter()
        .filter(|p| p.param_location == ParameterLocation::Query)
        .map(|p| {
            let name = &p.name;
            let rust_name = super::utils::to_valid_rust_field_name(name);
            let ident = format_ident!("{}", rust_name);
            if p.required {
                quote! {
                    Some((String::from(#name), #ident.to_string()))
                }
            } else {
                quote! {
                    match #ident.as_ref() {
                        Some(v) => Some((String::from(#name), v.to_string())),
                        None => None,
                    }
                }
            }
        })
        .collect()
}

/// Builds header handling code from header parameters.
///
/// Generates Rust code that adds header parameters to requests.
///
/// # Arguments
///
/// * `params` - The list of parameters
///
/// # Returns
///
/// A vector of token streams for header handling
pub fn build_header_handling(params: &[ParameterInfo]) -> Vec<proc_macro2::TokenStream> {
    params
        .iter()
        .filter(|p| p.param_location == ParameterLocation::Header)
        .map(|p| {
            let name = &p.name;
            let rust_name = super::utils::to_valid_rust_field_name(name);
            if p.required {
                quote! {
                    request.with_header(#name, #rust_name.to_string());
                }
            } else {
                quote! {
                    if let Some(ref value) = #rust_name {
                        request.with_header(#name, value.to_string());
                    }
                }
            }
        })
        .collect()
}

/// Generates the method body implementation.
///
/// Creates the complete method implementation including parameter
/// destructuring, request construction, and response handling.
///
/// # Arguments
///
/// * `path` - The API path
/// * `method_info` - The method metadata
///
/// # Returns
///
/// A token stream containing the complete method implementation
pub fn generate_method_body(path: &str, method_info: &MethodInfo) -> proc_macro2::TokenStream {
    let fn_name = format_ident!("{}", method_info.name);
    let mut array = Vec::new();

    if let Some(ref d) = method_info.description {
        let dd = format!(" {}", d);
        array.push(quote! { #[doc = #dd]});
        array.push(quote! { #[doc = ""]});
    }
    let endpoint_doc = format!(
        " Autogenerated `{:?}` request to `{}`.",
        &method_info.http_method, path
    );
    array.push(quote! { #[doc = #endpoint_doc]});
    array.push(quote! { #[doc = ""]});
    if !method_info.parameters.is_empty() {
        array.push(quote! { #[doc = " # Arguments"]});
        array.push(quote! { #[doc = ""]});
    }
    for p in &method_info.parameters {
        let location_str = match p.param_location {
            ParameterLocation::Path => "path",
            ParameterLocation::Query => "query",
            ParameterLocation::Header => "header",
            ParameterLocation::Cookie => "cookie",
        };
        let param_name = super::utils::to_valid_rust_field_name(&p.name);
        let optional = if p.required { "mandatory" } else { "optional" };
        if let Some(ref description) = p.description {
            let dd = format!(
                " - `{}`- {} `{}` parameter - {}",
                param_name, optional, location_str, description
            );
            array.push(quote! { #[doc = #dd]});
        } else {
            let dd = format!(
                " - `{}`- {} `{}` parameter",
                param_name, optional, location_str
            );
            array.push(quote! { #[doc = #dd]});
        }
    }

    let mut param_decls: Vec<_> = method_info
        .parameters
        .iter()
        .map(|p| {
            let param_name = format_ident!("{}", super::utils::to_valid_rust_field_name(&p.name));
            let param_ty = type_to_tokens(&p.ty);
            if p.required {
                quote! { #param_name: #param_ty }
            } else {
                quote! { #param_name: Option<#param_ty> }
            }
        })
        .collect();

    if let Some(body_type_name) = &method_info.request_body_type {
        let body_ty = type_to_tokens(body_type_name);
        if method_info.request_body_required {
            param_decls.push(quote! { body: #body_ty });
        } else {
            param_decls.push(quote! { body: Option<#body_ty> });
        }
    }

    let return_type = match &method_info.response_type {
        Some(rt) => {
            let rt_type = type_to_tokens(rt);
            quote! { Result<#rt_type, Self::RequesterErrorType> }
        }
        None => quote! { Result<(), Self::RequesterErrorType> },
    };

    let where_clause = quote! { where Self::RequesterErrorType: From<OapiRequesterError> };

    let http_method = match method_info.http_method {
        crate::RequestType::Get => quote! { RequestType::Get },
        crate::RequestType::Post => quote! { RequestType::Post },
        crate::RequestType::Put => quote! { RequestType::Put },
        crate::RequestType::Patch => quote! { RequestType::Patch },
        crate::RequestType::Delete => quote! { RequestType::Delete },
    };

    let (path_format, _path_replacements) = format_path_with_params(path, &method_info.parameters);
    let query_exprs = build_query_params(&method_info.parameters);
    let header_handling = build_header_handling(&method_info.parameters);

    let query_collect = if query_exprs.is_empty() {
        quote! {
            let query_params: Vec<(String, String)> = Vec::new();
        }
    } else {
        quote! {
            let query_params: Vec<(String, String)> = vec![#(#query_exprs),*]
                .into_iter()
                .flatten()
                .collect();
        }
    };

    let request_creation = if method_info.request_body_type.is_some() {
        if method_info.request_body_required {
            quote! {
                let request = self.create_request_with_body(#http_method, &uri, &body)?;
            }
        } else {
            quote! {
                let request = if let Some(b) = &body {
                    self.create_request_with_body(#http_method, &uri, b)?
                } else {
                    self.create_request(#http_method, &uri)?
                };
            }
        }
    } else {
        quote! {
            let request = self.create_request(#http_method, &uri)?;
        }
    };

    let response_handling = match &method_info.response_type {
        Some(rt) => {
            let rt_type = type_to_tokens(rt);
            quote! {
                let content = response.response_content().await
                    .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
                Ok(serde_json::from_str::<#rt_type>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?)
            }
        }
        None => quote! { Ok(()) },
    };

    quote! {
        #(#array)*
        fn #fn_name(&self, #(#param_decls),*) -> impl Future<Output = #return_type> #where_clause {
            async move {
                #query_collect

                let uri = ::oapi_universal_gen::UrlBuilder::build(#path_format, &query_params);

                #request_creation

                #(#header_handling)*

                let response = request.send_request().await?;

                if response.is_client_error() || response.is_server_error() {
                    return Err(OapiRequesterError::ClientOrServerError(response.status()).into());
                }

                #response_handling
            }
        }
    }
}
