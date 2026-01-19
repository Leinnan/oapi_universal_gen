//! Main code generation logic for OpenAPI specifications.
//!
//! This module provides the entry points for generating Rust code from
//! OpenAPI specifications. It orchestrates the entire generation process
//! including schema parsing, type generation, and method implementation.

use openapiv3_1::{OpenApi, Schema, Type, schema::Types};
use quote::{format_ident, quote};

use super::methods::generate_methods;
use super::schemas::{
    extract_discriminator_info, extract_enum_values, generate_struct_fields,
    generate_struct_fields_for_tagged_variant, is_enum_schema, property_type,
};
use super::types::{InlineEnumInfo, InlineStructInfo};
use super::utils::{enum_value_to_variant_name, to_pascal_case, to_valid_enum_variant};

/// Generates a documentation string for a schema.
///
/// If a description is provided, it is used directly. Otherwise, a descriptive
/// comment is generated based on the schema name.
///
/// # Arguments
///
/// * `name` - The name of the schema
/// * `description` - Optional description from the OpenAPI schema
///
/// # Returns
///
/// A string suitable for use as a doc comment
fn make_schema_doc(name: &str, description: Option<String>) -> String {
    if let Some(desc) = description {
        if !desc.is_empty() {
            return desc;
        }
    }
    format!("Generated from schema '{}'.", name)
}

/// Generates Rust code from an OpenAPI JSON string.
///
/// This is the main entry point for code generation. It takes a JSON
/// string representing an OpenAPI specification and returns the
/// generated Rust code as a string.
///
/// # Arguments
///
/// * `json` - A JSON string containing the OpenAPI specification
///
/// # Returns
///
/// A string containing the generated Rust code
///
/// # Panics
///
/// Panics if the JSON cannot be parsed or if code generation fails
pub fn generate_from_json(json: &str) -> String {
    let openapi: OpenApi = serde_json::from_str(json).expect("Could not deserialize input");
    generate_code(&openapi)
}

/// Generates and prints the OpenAPI spec code.
///
/// This function is typically used for debugging or generating
/// example code from a schema file.
pub fn generate_openapi_spec() {
    let data = include_str!("../../schemas/other.json");
    let output = generate_from_json(data);
    println!("{}", output);
}

/// Main code generation function.
///
/// This function orchestrates the entire code generation process:
/// 1. Parses component schemas into enums and structs
/// 2. Generates inline types for anonymous schemas
/// 3. Generates method signatures for all endpoints
/// 4. Assembles everything into a complete Rust module
///
/// # Arguments
///
/// * `openapi` - The parsed OpenAPI specification
///
/// # Returns
///
/// A formatted string containing the complete generated Rust code
fn generate_code(openapi: &OpenApi) -> String {
    let cmp = openapi.components.clone().unwrap_or_default();
    let cmp_2 = cmp.clone();

    let mut inline_enums: Vec<InlineEnumInfo> = Vec::new();
    let inline_structs = std::rc::Rc::new(std::cell::RefCell::new(Vec::<InlineStructInfo>::new()));

    let enum_schemas: Vec<_> = cmp
        .schemas
        .iter()
        .filter(|(_, schema_or_ref)| is_enum_schema(schema_or_ref))
        .collect();

    let enums = enum_schemas.iter().flat_map(|(name, schema)| {
        let schema_ref = match schema {
            Schema::Object(box_obj) => box_obj.as_ref(),
            _ => return None,
        };
        let enum_name = format_ident!("{}", to_pascal_case(name.as_str()));
        let d = make_schema_doc(name.as_str(), Some(schema_ref.description.clone()));
        let doc = quote! { #[doc = #d] };

        if let Some((tag_field, variants)) = extract_discriminator_info(&cmp_2, schema_ref) {
            let variant_defs: Vec<_> = variants
                .into_iter()
                .map(|(tag_value, ref_name, variant_schema)| {
                    let variant_name = to_valid_enum_variant(&tag_value);
                    let ident = format_ident!("{}", variant_name);
                    let rename_attr = if variant_name != tag_value {
                        quote! { #[serde(rename = #tag_value)] }
                    } else {
                        quote! {}
                    };
                    let docs = if !variant_schema.description.is_empty() {
                        format!(" '{}' variant. {}", name, variant_schema.description)
                    } else {
                        format!(" '{}' variant.", name)
                    };
                    let docs_atr = quote! { #[doc = #docs] };
                    let enum_name = to_pascal_case(name);
                    let variant_parent_name = if !ref_name.is_empty() {
                        ref_name.clone()
                    } else {
                        format!("{}{}", enum_name, variant_name)
                    };
                    let (fields, has_fields) = generate_struct_fields_for_tagged_variant(
                        &cmp_2,
                        &variant_schema,
                        &tag_field,
                        &mut *inline_structs.borrow_mut(),
                        Some(&variant_parent_name),
                        true,
                    );
                    if !has_fields {
                        quote! {
                            #rename_attr
                            #docs_atr
                            #ident,
                        }
                    } else {
                        quote! {
                            #rename_attr
                            #[display(#variant_name)]
                            #docs_atr
                            #ident { #(#fields)* },
                        }
                    }
                })
                .collect();

            let tag_field_str = tag_field.as_str();
            return Some(quote! {
                #doc
                #[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
                #[serde(tag = #tag_field_str)]
                pub enum #enum_name {
                    #(#variant_defs)*
                }
            });
        }

        let enum_values = extract_enum_values(schema_ref);

        if schema_ref.any_of.is_some() {
            let variants = enum_values.iter().map(|val| match val {
                serde_json::Value::String(ref_name) => {
                    let variant_name = format_ident!("{}", ref_name);
                    let type_name = format_ident!("{}", ref_name);
                    quote! { #variant_name(#type_name), }
                }
                _ => quote! { Unknown, },
            });

            Some(quote! {
                #doc
                #[derive(Debug, Clone, Serialize, Deserialize)]
                pub enum #enum_name {
                    #(#variants)*
                }
            })
        } else {
            let variants = enum_values.into_iter().map(|val| {
                let variant_name = enum_value_to_variant_name(&val);
                quote! { #variant_name, }
            });

            Some(quote! {
                #doc
                #[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
                #[serde(rename_all = "snake_case")]
                pub enum #enum_name {
                    #(#variants)*
                }
            })
        }
    });

    let struct_schemas: Vec<_> = cmp
        .schemas
        .iter()
        .filter(|(_, schema_or_ref)| !is_enum_schema(schema_or_ref))
        .collect();

    let structs: Vec<_> = struct_schemas
        .iter()
        .flat_map(|(name, schema_or_ref)| {
            let schema_ref = match schema_or_ref {
                Schema::Object(box_obj) => box_obj.as_ref(),
                _ => return None,
            };

            if let Some((tag_field, _variants)) = extract_discriminator_info(&cmp_2, schema_ref) {
                let enum_name_str = to_pascal_case(name);
                let description = if schema_ref.description.is_empty() {
                    None
                } else {
                    Some(schema_ref.description.clone())
                };
                inline_enums.push(InlineEnumInfo {
                    name: enum_name_str.clone(),
                    schema: schema_ref.clone(),
                    description,
                    is_tagged: true,
                    tag_field: Some(tag_field),
                });
                let enum_ident = format_ident!("{}", enum_name_str);
                let doc = quote! { #[doc = {}] };
                return Some(quote! {
                    #doc
                    #[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
                    pub enum #enum_ident {}
                });
            }

            let schema_name = format_ident!("{}", to_pascal_case(name.as_str()));
            let d = make_schema_doc(name.as_str(), Some(schema_ref.description.clone()));
            let doc = quote! { #[doc = #d] };

            if let Some(Types::Single(Type::Array)) = &schema_ref.schema_type {
                if let Some(items) = &schema_ref.items {
                    let item_type = property_type(
                        &cmp_2,
                        items,
                        &mut inline_enums,
                        &mut *inline_structs.borrow_mut(),
                        Some(name),
                        "Item",
                        true,
                    );
                    return Some(quote! {
                        #doc
                        #[derive(Debug, Clone, Serialize, Deserialize)]
                        pub struct #schema_name(pub Vec<#item_type>);
                    });
                }
            }

            let fields = generate_struct_fields(
                &cmp_2,
                schema_or_ref,
                &mut inline_enums,
                &mut *inline_structs.borrow_mut(),
                Some(name),
            );
            Some(quote! {
                #doc
                #[derive(Debug, Clone, Serialize, Deserialize)]
                pub struct #schema_name {
                    #(#fields)*
                }
            })
        })
        .collect();

    let (inline_input_structs, methods) =
        generate_methods(openapi, &cmp_2, &mut inline_enums, &inline_structs);

    let mut seen_enum_names = std::collections::HashSet::new();
    let inline_enum_defs: Vec<_> = inline_enums
        .into_iter()
        .filter(|enum_info| {
            if seen_enum_names.contains(&enum_info.name) {
                false
            } else {
                seen_enum_names.insert(enum_info.name.clone());
                true
            }
        })
        .flat_map(|enum_info| {
            let enum_name = format_ident!("{}", enum_info.name);
            let d = make_schema_doc(&enum_info.name, enum_info.description.clone());
            let doc = quote! { #[doc = #d] };

            if enum_info.is_tagged {
                let tag_field = enum_info.tag_field.as_ref().unwrap();
                let discriminator_info = extract_discriminator_info(&cmp_2, &enum_info.schema)?;
                let (_, variants) = discriminator_info;

                let variant_defs: Vec<_> = variants
                    .into_iter()
                    .map(|(tag_value, ref_name, variant_schema)| {
                        let variant_name = to_valid_enum_variant(&tag_value);
                        let ident = format_ident!("{}", variant_name);
                        let rename_attr = if variant_name != tag_value {
                            quote! { #[serde(rename = #tag_value)] }
                        } else {
                            quote! {}
                        };
                        let variant_parent_name = if !ref_name.is_empty() {
                            ref_name.clone()
                        } else {
                            format!("{}{}", enum_info.name, variant_name)
                        };
                        let (fields, has_fields) = generate_struct_fields_for_tagged_variant(
                            &cmp_2,
                            &variant_schema,
                            tag_field,
                            &mut *inline_structs.borrow_mut(),
                            Some(&variant_parent_name),
                            true,
                        );
                        if !has_fields {
                            quote! {
                                #rename_attr
                                #ident,
                            }
                        } else {
                            quote! {
                                #rename_attr
                                #[display(#variant_name)]
                                #ident { #(#fields)* },
                            }
                        }
                    })
                    .collect();

                let tag_field_str = tag_field.as_str();
                Some(quote! {
                    #doc
                    #[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
                    #[serde(tag = #tag_field_str)]
                    pub enum #enum_name {
                        #(#variant_defs)*
                    }
                })
            } else {
                let variants = extract_enum_values(&enum_info.schema)
                    .into_iter()
                    .map(|val| {
                        let variant_name = enum_value_to_variant_name(&val);
                        quote! { #variant_name, }
                    });

                Some(quote! {
                    #doc
                    #[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
                    #[serde(rename_all = "snake_case")]
                    pub enum #enum_name {
                        #(#variants)*
                    }
                })
            }
        })
        .collect();

    let mut seen_struct_names = std::collections::HashSet::new();
    let inline_struct_defs: Vec<proc_macro2::TokenStream> = inline_structs
        .borrow_mut()
        .drain(..)
        .filter(|struct_info| {
            if seen_struct_names.contains(&struct_info.name) {
                false
            } else {
                seen_struct_names.insert(struct_info.name.clone());
                true
            }
        })
        .map(|struct_info| {
            let struct_ident = format_ident!("{}", struct_info.name);
            let fields = struct_info.fields;
            quote! {
                #[derive(Debug, Clone, Serialize, Deserialize)]
                pub struct #struct_ident {
                    #(#fields)*
                }
            }
        })
        .collect();

    let raw_tokens = quote! {
        use serde::{Serialize, Deserialize};
        use oapi_universal_gen::*;
        use std::future::Future;

        #( #enums )*

        #( #structs )*

        #( #inline_input_structs )*

        #( #inline_enum_defs )*

        #( #inline_struct_defs )*

        pub trait ApiService: ::oapi_universal_gen::OapiRequester {
            #( #methods )*
        }
    };

    let raw_tokens_clone = raw_tokens.clone();
    let syntax_tree = match syn::parse2(raw_tokens) {
        Ok(tree) => tree,
        Err(e) => {
            eprintln!("Syn parse error: {:?}", e);
            let raw_code = raw_tokens_clone.to_string();
            std::fs::write("generated_code_debug.txt", &raw_code).ok();
            eprintln!(
                "Generated code written to generated_code_debug.txt ({} chars)",
                raw_code.len()
            );
            eprintln!(
                "First 3000 chars:\n{}",
                &raw_code[..raw_code.len().min(3000)]
            );
            if let Some(pos) = raw_code.find("pub fn") {
                eprintln!(
                    "\nLast few functions before error:\n{}",
                    &raw_code[pos..pos.saturating_add(1000)]
                );
            }
            panic!("Failed to parse generated code: {}", e);
        }
    };
    let formatted = prettyplease::unparse(&syntax_tree);
    formatted
}
