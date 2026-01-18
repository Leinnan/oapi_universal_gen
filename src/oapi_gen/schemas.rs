//! Schema processing and type generation from OpenAPI specifications.
//!
//! This module provides functions for analyzing OpenAPI schemas and
//! generating corresponding Rust types. It handles enum detection,
//! struct field generation, property type resolution, and discriminator
//! analysis for tagged unions.

use openapiv3_1::{
    Components, Object, Ref, RefOr, Schema, Type, path::Parameter, request_body::RequestBody,
    schema::Types,
};
use quote::{format_ident, quote};
use std::cell::RefCell;
use std::rc::Rc;

use super::types::{InlineEnumInfo, InlineStructInfo, ParameterInfo};
use super::utils::{extract_ref_name, is_field_required, to_pascal_case, to_valid_rust_field_name};

/// Determines if a schema represents an enum type.
///
/// An enum schema is one that:
/// - Has an `any_of` definition (oneOf/anyOf patterns)
/// - Is a String, Integer, Number, or Boolean with enum_values
///
/// # Arguments
///
/// * `schema` - The schema to check
///
/// # Returns
///
/// `true` if the schema represents an enum
pub fn is_enum_schema(schema: &Schema) -> bool {
    let obj: &Object = match schema {
        Schema::Object(box_obj) => box_obj.as_ref(),
        _ => {
            return false;
        }
    };
    if let Some(any_of) = &obj.any_of {
        return !any_of.is_empty();
    }
    if let Some(Types::Single(Type::String)) = obj.schema_type {
        return obj.enum_values.as_ref().map_or(false, |v| !v.is_empty());
    }
    if let Some(Types::Single(Type::Integer)) = obj.schema_type {
        return obj.enum_values.as_ref().map_or(false, |v| !v.is_empty());
    }
    if let Some(Types::Single(Type::Number)) = obj.schema_type {
        return obj.enum_values.as_ref().map_or(false, |v| !v.is_empty());
    }
    if let Some(Types::Single(Type::Boolean)) = obj.schema_type {
        return obj.enum_values.as_ref().map_or(false, |v| !v.is_empty());
    }
    false
}

/// Extracts enum values from a schema.
///
/// # Arguments
///
/// * `obj` - The object schema to extract values from
///
/// # Returns
///
/// A vector of JSON values representing the enum variants
pub fn extract_enum_values(obj: &Object) -> Vec<serde_json::Value> {
    if let Some(values) = &obj.enum_values {
        return values.clone();
    }
    if let Some(any_of) = &obj.any_of {
        return any_of
            .iter()
            .filter_map(|schema| {
                if let Schema::Object(box_schema) = schema {
                    if !box_schema.reference.is_empty() {
                        let ref_location = &box_schema.reference;
                        Some(serde_json::Value::String(extract_ref_name(ref_location)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
    }
    vec![]
}

/// Extracts discriminator information for tagged enums.
///
/// Analyzes an `any_of` schema to find tagged union patterns where
/// each variant has a `const` field that serves as the discriminator.
///
/// # Arguments
///
/// * `components` - The OpenAPI components for resolving references
/// * `obj` - The object schema to analyze
///
/// # Returns
///
/// Some with (tag_field, variants) if it's a tagged enum, None otherwise
pub fn extract_discriminator_info(
    components: &Components,
    obj: &Object,
) -> Option<(String, Vec<(String, String, Object)>)> {
    let any_of = obj.any_of.as_ref()?;
    if any_of.is_empty() {
        return None;
    }

    let mut tag_field = None;
    let mut variants = Vec::new();

    for schema in any_of {
        let mut box_schema = match schema {
            Schema::Object(box_schema) => box_schema.as_ref(),
            _ => return None,
        };
        let ref_name = if !box_schema.reference.is_empty() {
            let name = extract_ref_name(&box_schema.reference);
            if let Some(Schema::Object(s)) = components.schemas.get(&name) {
                box_schema = s.as_ref();
                Some(name)
            } else {
                return None;
            }
        } else {
            None
        };

        let props = match &box_schema.schema_type {
            Some(Types::Single(Type::Object)) => &box_schema.properties,
            _ => return None,
        };

        let mut found_tag = None;
        for (name, prop) in props {
            if let Schema::Object(box_prop) = prop {
                let object_prop = box_prop.as_ref();
                if let Some(const_val) = &object_prop.const_value {
                    if tag_field.is_none() {
                        tag_field = Some(name.clone());
                    } else if tag_field.as_ref() != Some(name) {
                        return None;
                    }
                    found_tag = Some((name.clone(), const_val.clone()));
                    break;
                }
            }
        }

        if let Some((_name, const_val)) = found_tag {
            variants.push((
                const_val.as_str().unwrap_or_default().to_string(),
                ref_name.unwrap_or_default(),
                box_schema.clone(),
            ));
        } else {
            return None;
        }
    }

    Some((tag_field?, variants))
}

/// Generates struct fields for a tagged enum variant.
///
/// Creates field definitions for all properties except the discriminator
/// field itself.
///
/// # Arguments
///
/// * `components` - The OpenAPI components
/// * `schema` - The variant schema
/// * `tag_field` - The discriminator field name
/// * `inline_structs` - Collector for inline struct definitions
/// * `parent_name` - Optional parent name for nested struct naming
///
/// # Returns
///
/// A tuple of (field token streams, whether there are any fields)
pub fn generate_struct_fields_for_tagged_variant(
    components: &Components,
    schema: &Object,
    tag_field: &str,
    inline_structs: &mut Vec<InlineStructInfo>,
    parent_name: Option<&str>,
) -> (Vec<proc_macro2::TokenStream>, bool) {
    let fields: Vec<_> = schema
        .properties
        .iter()
        .filter(|(name, _)| *name != tag_field)
        .map(|(name, prop)| {
            let rust_name = to_valid_rust_field_name(name);
            let field_name = format_ident!("{}", rust_name);
            let is_required = is_field_required(name, &schema.required);
            let ty = property_type(
                components,
                prop,
                &mut Vec::new(),
                inline_structs,
                parent_name,
                name,
                is_required,
            );
            let serde_attr = if is_required {
                if rust_name.as_str() != name {
                    quote! { #[serde(rename = #name)] }
                } else {
                    quote! {}
                }
            } else {
                if rust_name.as_str() != name {
                    quote! { #[serde(default, skip_serializing_if = "Option::is_none", rename = #name)] }
                } else {
                    quote! { #[serde(default, skip_serializing_if = "Option::is_none")] }
                }
            };
            quote! {
                #serde_attr
                #field_name: #ty,
            }
        })
        .collect();
    let has_fields = !fields.is_empty();
    (fields, has_fields)
}

/// Gets the enum type name for a schema with enum values.
///
/// # Arguments
///
/// * `schema` - The schema to check
///
/// # Returns
///
/// Some with the enum type name, or None
pub fn get_enum_type_name(schema: &Object) -> Option<String> {
    if let Some(values) = &schema.enum_values {
        if !values.is_empty() {
            return Some(if schema.title.is_empty() {
                "StringEnum".to_string()
            } else {
                schema.title.clone()
            });
        }
    }
    None
}

/// Converts a schema type to a Rust type string.
///
/// # Arguments
///
/// * `schema` - The schema to convert
///
/// # Returns
///
/// A string representation of the Rust type
pub fn schema_type_to_string(schema: &Object) -> String {
    if let Some(enum_name) = get_enum_type_name(schema) {
        return enum_name;
    }
    if let Some(Types::Single(schema_kind)) = &schema.schema_type {
        match schema_kind {
            Type::String => "String".to_string(),
            Type::Integer => {
                if schema.format == "int32" {
                    "i32".to_string()
                } else {
                    "i64".to_string()
                }
            }
            Type::Number => "f64".to_string(),
            Type::Boolean => "bool".to_string(),
            Type::Array => "Vec<serde_json::Value>".to_string(),
            Type::Object => {
                if !schema.title.is_empty() {
                    schema.title.clone()
                } else {
                    "serde_json::Value".to_string()
                }
            }
            _ => "serde_json::Value".to_string(),
        }
    } else {
        "object".to_string()
    }
}

/// Extracts the parameter type from a parameter definition.
///
/// # Arguments
///
/// * `pd` - The parameter definition
///
/// # Returns
///
/// A string representing the Rust type
pub fn extract_parameter_type(pd: &Parameter) -> String {
    match &pd.schema {
        Some(Schema::Object(box_schema)) => schema_type_to_string(box_schema.as_ref()),
        _ => "serde_json::Value".to_string(),
    }
}

/// Generates struct field definitions for an object schema.
///
/// # Arguments
///
/// * `components` - The OpenAPI components
/// * `schema` - The schema to generate fields from
/// * `inline_enums` - Collector for inline enum definitions
/// * `inline_structs` - Collector for inline struct definitions
/// * `parent_name` - Optional parent name for nested type naming
///
/// # Returns
///
/// A vector of token streams representing the field definitions
pub fn generate_struct_fields(
    components: &Components,
    schema: &Schema,
    inline_enums: &mut Vec<InlineEnumInfo>,
    inline_structs: &mut Vec<InlineStructInfo>,
    parent_name: Option<&str>,
) -> Vec<proc_macro2::TokenStream> {
    let obj: &Object = match schema {
        Schema::Object(box_obj) => box_obj.as_ref(),
        _ => return vec![],
    };
    obj.properties
        .iter()
        .flat_map(|(name, prop)| {
            let rust_name = to_valid_rust_field_name(name);
            let field_name = format_ident!("{}", rust_name);
            let is_required = is_field_required(name, &obj.required);
            let ty = property_type(
                components,
                prop,
                inline_enums,
                inline_structs,
                parent_name,
                name,
                is_required,
            );
            let doc: Option<proc_macro2::TokenStream> = match prop {
                Schema::Object(box_prop) => {
                    let prop_ref = box_prop.as_ref();
                    let desc = &prop_ref.description;
                    if !desc.is_empty() {
                        Some(quote! { #[doc = #desc] })
                    } else {
                        None
                    }
                }
                _ => None,
            };
            let serde_attr = if is_required {
                if rust_name.as_str() != name {
                    quote! { #[serde(rename = #name)] }
                } else {
                    quote! {}
                }
            } else {
                if rust_name.as_str() != name {
                    quote! { #[serde(default, skip_serializing_if = "Option::is_none", rename = #name)] }
                } else {
                    quote! { #[serde(default, skip_serializing_if = "Option::is_none")] }
                }
            };
            vec![
                doc.into_iter().collect::<Vec<_>>(),
                vec![serde_attr],
                vec![quote! { pub #field_name: #ty, }],
            ]
        })
        .flatten()
        .collect()
}

/// Determines the Rust type for a property schema.
///
/// This is the core type resolution function that maps OpenAPI schema
/// types to Rust types, handling references, enums, arrays, objects,
/// and primitives.
///
/// # Arguments
///
/// * `components` - The OpenAPI components for resolving references
/// * `schema` - The property schema
/// * `inline_enums` - Collector for inline enum definitions
/// * `inline_structs` - Collector for inline struct definitions
/// * `parent_name` - Optional parent name for nested type naming
/// * `field_name` - The field name for naming nested types
/// * `is_required` - Whether the field is required
///
/// # Returns
///
/// A token stream representing the Rust type
pub fn property_type(
    components: &Components,
    schema: &Schema,
    inline_enums: &mut Vec<InlineEnumInfo>,
    inline_structs: &mut Vec<InlineStructInfo>,
    parent_name: Option<&str>,
    field_name: &str,
    is_required: bool,
) -> proc_macro2::TokenStream {
    match schema {
        Schema::Object(box_schema) => {
            if !box_schema.reference.is_empty() {
                let ref_name = extract_ref_name(&box_schema.reference);
                let ident = format_ident!("{}", ref_name);
                if is_required {
                    quote! { #ident }
                } else {
                    quote! { Option<#ident> }
                }
            } else if let Some(enum_name) = get_enum_type_name(box_schema) {
                let description = if box_schema.description.is_empty() {
                    None
                } else {
                    Some(box_schema.description.clone())
                };
                inline_enums.push(InlineEnumInfo {
                    name: enum_name.clone(),
                    schema: box_schema.as_ref().clone(),
                    description,
                    is_tagged: false,
                    tag_field: None,
                });
                let ident = format_ident!("{}", enum_name);
                if is_required {
                    quote! { #ident }
                } else {
                    quote! { Option<#ident> }
                }
            } else if let Some((tag_field, _variants)) =
                extract_discriminator_info(components, box_schema)
            {
                let description = if box_schema.description.is_empty() {
                    None
                } else {
                    Some(box_schema.description.clone())
                };
                let enum_name_str = to_pascal_case(&tag_field);
                inline_enums.push(InlineEnumInfo {
                    name: enum_name_str.clone(),
                    schema: box_schema.as_ref().clone(),
                    description,
                    is_tagged: true,
                    tag_field: Some(tag_field),
                });
                let enum_name = format_ident!("{}", enum_name_str);
                if is_required {
                    quote! { #enum_name }
                } else {
                    quote! { Option<#enum_name> }
                }
            } else {
                match &box_schema.schema_type {
                    Some(Types::Single(Type::String)) => {
                        if is_required {
                            quote! { String }
                        } else {
                            quote! { Option<String> }
                        }
                    }
                    Some(Types::Single(Type::Integer)) => {
                        let format = &box_schema.format;
                        let ty = match format.as_str() {
                            "int32" => quote! { i32 },
                            "int64" => quote! { i64 },
                            _ => quote! { i64 },
                        };
                        if is_required {
                            ty
                        } else {
                            quote! { Option<#ty> }
                        }
                    }
                    Some(Types::Single(Type::Number)) => {
                        if is_required {
                            quote! { f64 }
                        } else {
                            quote! { Option<f64> }
                        }
                    }
                    Some(Types::Single(Type::Boolean)) => {
                        if is_required {
                            quote! { bool }
                        } else {
                            quote! { Option<bool> }
                        }
                    }
                    Some(Types::Single(Type::Array)) => {
                        let inner = box_schema.items.as_ref();
                        let inner_ty = if let Some(item_ref) = inner {
                            property_type(
                                components,
                                item_ref,
                                inline_enums,
                                inline_structs,
                                None,
                                "Item",
                                true,
                            )
                        } else {
                            quote! { String }
                        };
                        if is_required {
                            quote! { Vec<#inner_ty> }
                        } else {
                            quote! { Option<Vec<#inner_ty>> }
                        }
                    }
                    Some(Types::Single(Type::Object)) => {
                        if box_schema.properties.is_empty() {
                            if is_required {
                                quote! { serde_json::Value }
                            } else {
                                quote! { Option<serde_json::Value> }
                            }
                        } else {
                            let base_name = match parent_name {
                                Some(parent) => {
                                    format!(
                                        "{}{}",
                                        to_pascal_case(parent),
                                        to_pascal_case(field_name)
                                    )
                                }
                                None => "InlineObject".to_string(),
                            };
                            let mut struct_name = base_name.clone();
                            let mut counter = 0;
                            loop {
                                let exists = inline_structs.iter().any(|s| s.name == struct_name);
                                if !exists {
                                    break;
                                }
                                counter += 1;
                                struct_name = format!("{}{}", base_name, counter);
                            }
                            let fields: Vec<_> = box_schema.properties.iter()
                                .map(|(name, prop)| {
                                    let rust_name = to_valid_rust_field_name(name);
                                    let field_name = format_ident!("{}", rust_name);
                                    let is_required = is_field_required(name, &box_schema.required);
                                    let ty = property_type(
                                        components,
                                        prop,
                                        inline_enums,
                                        inline_structs,
                                        Some(&struct_name),
                                        name,
                                        is_required,
                                    );
                                    let serde_attr = if is_required {
                                        if rust_name.as_str() != name {
                                            quote! { #[serde(rename = #name)] }
                                        } else {
                                            quote! {}
                                        }
                                    } else {
                                        if rust_name.as_str() != name {
                                            quote! { #[serde(default, skip_serializing_if = "Option::is_none", rename = #name)] }
                                        } else {
                                            quote! { #[serde(default, skip_serializing_if = "Option::is_none")] }
                                        }
                                    };
                                    quote! {
                                        #serde_attr
                                        pub #field_name: #ty,
                                    }
                                })
                                .collect();
                            inline_structs.push(InlineStructInfo {
                                name: struct_name.clone(),
                                fields: fields.clone(),
                            });
                            let ident = format_ident!("{}", struct_name);
                            if is_required {
                                quote! { #ident }
                            } else {
                                quote! { Option<#ident> }
                            }
                        }
                    }
                    _ => {
                        if is_required {
                            quote! { serde_json::Value }
                        } else {
                            quote! { Option<serde_json::Value> }
                        }
                    }
                }
            }
        }
        _ => {
            if is_required {
                quote! { serde_json::Value }
            } else {
                quote! { Option<serde_json::Value> }
            }
        }
    }
}

/// Extracts the request body type from a request body definition.
///
/// # Arguments
///
/// * `rb` - The request body reference or type
/// * `components` - The OpenAPI components
/// * `inline_structs` - Collector for inline struct definitions
/// * `inline_enums` - Collector for inline enum definitions
/// * `main_inline_structs` - Main collector for inline structs
///
/// # Returns
///
/// Some with the body type name, or None
pub fn extract_request_body_type(
    rb: &RefOr<RequestBody>,
    components: &Components,
    inline_structs: &mut Vec<proc_macro2::TokenStream>,
    inline_enums: &mut Vec<InlineEnumInfo>,
    main_inline_structs: &Rc<RefCell<Vec<InlineStructInfo>>>,
) -> Option<String> {
    let mut nested_inline_structs: Vec<InlineStructInfo> = Vec::new();
    let result = match rb {
        RefOr::T(request_body) => {
            let mut result = None;
            for (media_type, mt) in &request_body.content {
                if media_type == "application/json" || media_type == "*/*" {
                    if let Some(schema) = &mt.schema {
                        result = match schema {
                            Schema::Object(box_schema) => match &box_schema.schema_type {
                                Some(Types::Single(Type::Object)) => {
                                    let struct_name =
                                        generate_inline_struct_name(inline_structs, "Request");
                                    let fields = box_schema.properties.iter().flat_map(|(name, prop)| {
                                        let rust_name = to_valid_rust_field_name(name);
                                        let field_name = format_ident!("{}", rust_name);
                                        let is_required = is_field_required(name, &box_schema.required);
                                        let ty = property_type(
                                            components,
                                            prop,
                                            inline_enums,
                                            &mut nested_inline_structs,
                                            None,
                                            name,
                                            is_required,
                                        );
                                        let doc: Option<proc_macro2::TokenStream> = match prop {
                                            Schema::Object(box_prop) => {
                                                let prop_ref = box_prop.as_ref();
                                                let desc = &prop_ref.description;
                                                if !desc.is_empty() {
                                                    Some(quote! { #[doc = #desc] })
                                                } else {
                                                    None
                                                }
                                            }
                                            _ => None,
                                        };
                                        let serde_attr = if is_required {
                                            if rust_name.as_str() != name {
                                                quote! { #[serde(rename = #name)] }
                                            } else {
                                                quote! {}
                                            }
                                        } else {
                                            if rust_name.as_str() != name {
                                                quote! { #[serde(default, skip_serializing_if = "Option::is_none", rename = #name)] }
                                            } else {
                                                quote! { #[serde(default, skip_serializing_if = "Option::is_none")] }
                                            }
                                        };
                                        vec![
                                            doc.into_iter().collect::<Vec<_>>(),
                                            vec![serde_attr],
                                            vec![quote! { pub #field_name: #ty, }],
                                        ]
                                    }).flatten().collect::<Vec<_>>();

                                    let struct_ident = format_ident!("{}", struct_name);
                                    inline_structs.push(quote! {
                                        #[derive(Debug, Clone, Serialize, Deserialize)]
                                        pub struct #struct_ident {
                                            #(#fields)*
                                        }
                                    });
                                    Some(struct_name)
                                }
                                Some(Types::Single(Type::Array)) => {
                                    Some("serde_json::Value".to_string())
                                }
                                _ => Some("serde_json::Value".to_string()),
                            },
                            _ => Some("serde_json::Value".to_string()),
                        };
                        break;
                    }
                }
            }
            result
        }
        RefOr::Ref(Ref { ref_location, .. }) => {
            if let Some(path) = ref_location.strip_prefix("#/components/requestBodies/") {
                let body_name = to_pascal_case(path);
                Some(body_name)
            } else {
                None
            }
        }
    };
    main_inline_structs
        .borrow_mut()
        .extend(nested_inline_structs);
    result
}

/// Generates a unique name for an inline struct.
///
/// # Arguments
///
/// * `inline_structs` - The collection of existing inline structs
/// * `prefix` - The prefix for the struct name
///
/// # Returns
///
/// A unique struct name
pub fn generate_inline_struct_name(
    inline_structs: &[proc_macro2::TokenStream],
    prefix: &str,
) -> String {
    let base_name = format!("{}Inline", prefix);
    let mut counter = 0;
    let mut name = base_name.clone();

    for ts in inline_structs {
        let s = quote::quote! { #ts }.to_string();
        if s.contains(&format!("struct {}", name)) {
            counter += 1;
            name = format!("{}{}", base_name, counter);
        }
    }

    name
}

/// Checks if a status code represents a success response.
///
/// # Arguments
///
/// * `status_code` - The status code string (e.g., "200", "2xx", "*")
///
/// # Returns
///
/// `true` if it's a success status code
pub fn is_success_status_code(status_code: &str) -> bool {
    if status_code == "*" {
        return true;
    }
    if let Ok(code) = status_code.parse::<u16>() {
        return code >= 200 && code < 300;
    }
    if status_code.starts_with('2') {
        return true;
    }
    false
}

/// Extracts the response type from a responses definition.
///
/// # Arguments
///
/// * `responses` - The responses definition
/// * `components` - The OpenAPI components
/// * `inline_structs` - Collector for inline struct definitions
/// * `inline_enums` - Collector for inline enum definitions
/// * `endpoint_name` - The endpoint name for naming response types
/// * `main_inline_structs` - Main collector for inline structs
///
/// # Returns
///
/// Some with the response type name, or None
pub fn extract_response_type(
    responses: &openapiv3_1::Responses,
    components: &openapiv3_1::Components,
    inline_structs: &mut Vec<proc_macro2::TokenStream>,
    inline_enums: &mut Vec<InlineEnumInfo>,
    endpoint_name: &str,
    main_inline_structs: &Rc<RefCell<Vec<InlineStructInfo>>>,
) -> Option<String> {
    for (status_code, response_ref) in &responses.responses {
        if !is_success_status_code(status_code) {
            continue;
        }
        if let RefOr::T(response) = response_ref {
            for (media_type, mt) in &response.content {
                if media_type == "application/json" || media_type == "*/*" {
                    if let Some(schema) = &mt.schema {
                        return Some(response_schema_to_string(
                            schema,
                            components,
                            inline_structs,
                            inline_enums,
                            endpoint_name,
                            main_inline_structs,
                        ));
                    }
                }
            }
        }
    }
    None
}

/// Converts a response schema to a Rust type string.
///
/// # Arguments
///
/// * `schema` - The response schema
/// * `components` - The OpenAPI components
/// * `inline_structs` - Collector for inline struct definitions
/// * `inline_enums` - Collector for inline enum definitions
/// * `endpoint_name` - The endpoint name for naming response types
/// * `main_inline_structs` - Main collector for inline structs
///
/// # Returns
///
/// A string representing the Rust type
pub fn response_schema_to_string(
    schema: &Schema,
    components: &Components,
    inline_structs: &mut Vec<proc_macro2::TokenStream>,
    inline_enums: &mut Vec<InlineEnumInfo>,
    endpoint_name: &str,
    main_inline_structs: &Rc<RefCell<Vec<InlineStructInfo>>>,
) -> String {
    let mut nested_inline_structs: Vec<InlineStructInfo> = Vec::new();
    let result = match schema {
        Schema::Object(box_schema) => {
            if !box_schema.reference.is_empty() {
                return extract_ref_name(&box_schema.reference);
            }
            match &box_schema.schema_type {
                Some(Types::Single(Type::Object)) => {
                    if box_schema.properties.is_empty() {
                        "serde_json::Value".to_string()
                    } else {
                        let struct_name =
                            format!("{}{}", to_pascal_case(endpoint_name), "Response");
                        let fields = box_schema.properties.iter().flat_map(|(name, prop)| {
                        let rust_name = to_valid_rust_field_name(name);
                        let field_name = format_ident!("{}", rust_name);
                        let is_required = is_field_required(name, &box_schema.required);
                        let ty = property_type(
                            components,
                            prop,
                            inline_enums,
                            &mut nested_inline_structs,
                            Some(&struct_name),
                            name,
                            is_required,
                        );
                        let doc: Option<proc_macro2::TokenStream> = match prop {
                            Schema::Object(box_prop) => {
                                let prop_ref = box_prop.as_ref();
                                let desc = &prop_ref.description;
                                if !desc.is_empty() {
                                    Some(quote! { #[doc = #desc] })
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        };
                        let serde_attr = if is_required {
                            if rust_name.as_str() != name {
                                quote! { #[serde(rename = #name)] }
                            } else {
                                quote! {}
                            }
                        } else {
                            if rust_name.as_str() != name {
                                quote! { #[serde(default, skip_serializing_if = "Option::is_none", rename = #name)] }
                            } else {
                                quote! { #[serde(default, skip_serializing_if = "Option::is_none")] }
                            }
                        };
                        vec![
                            doc.into_iter().collect::<Vec<_>>(),
                            vec![serde_attr],
                            vec![quote! { pub #field_name: #ty, }],
                        ]
                    }).flatten().collect::<Vec<_>>();

                        let struct_ident = format_ident!("{}", struct_name);
                        inline_structs.push(quote! {
                            #[derive(Debug, Clone, Serialize, Deserialize)]
                            pub struct #struct_ident {
                                #(#fields)*
                            }
                        });
                        struct_name
                    }
                }
                Some(Types::Single(Type::Array)) => {
                    let inner_type = box_schema
                        .items
                        .as_ref()
                        .map(|item| match item {
                            Schema::Object(inner_box_schema) => response_schema_to_string_item(
                                &inner_box_schema,
                                components,
                                inline_structs,
                                inline_enums,
                                endpoint_name,
                                main_inline_structs,
                            ),
                            _ => "serde_json::Value".to_string(),
                        })
                        .unwrap_or_else(|| "serde_json::Value".to_string());
                    format!("Vec<{}>", inner_type)
                }
                Some(Types::Single(Type::String)) => "String".to_string(),
                Some(Types::Single(Type::Integer)) => {
                    if box_schema.format == "int32" {
                        "i32".to_string()
                    } else if box_schema.format == "int64" {
                        "i64".to_string()
                    } else {
                        "i64".to_string()
                    }
                }
                Some(Types::Single(Type::Number)) => "f64".to_string(),
                Some(Types::Single(Type::Boolean)) => "bool".to_string(),
                _ => "serde_json::Value".to_string(),
            }
        }
        _ => "serde_json::Value".to_string(),
    };
    main_inline_structs
        .borrow_mut()
        .extend(nested_inline_structs);
    result
}

/// Converts a response array item schema to a Rust type string.
///
/// # Arguments
///
/// * `schema` - The item schema
/// * `components` - The OpenAPI components
/// * `inline_structs` - Collector for inline struct definitions
/// * `inline_enums` - Collector for inline enum definitions
/// * `endpoint_name` - The endpoint name for naming types
/// * `main_inline_structs` - Main collector for inline structs
///
/// # Returns
///
/// A string representing the Rust type
pub fn response_schema_to_string_item(
    schema: &Object,
    components: &Components,
    inline_structs: &mut Vec<proc_macro2::TokenStream>,
    inline_enums: &mut Vec<InlineEnumInfo>,
    endpoint_name: &str,
    main_inline_structs: &Rc<RefCell<Vec<InlineStructInfo>>>,
) -> String {
    if !schema.reference.is_empty() {
        return extract_ref_name(&schema.reference);
    }
    let mut nested_inline_structs: Vec<InlineStructInfo> = Vec::new();
    let result = match &schema.schema_type {
        Some(Types::Single(Type::Object)) => {
            if schema.properties.is_empty() {
                "serde_json::Value".to_string()
            } else {
                let struct_name = format!("{}{}", to_pascal_case(endpoint_name), "Response");
                let fields = schema.properties.iter().flat_map(|(name, prop)| {
                    let rust_name = to_valid_rust_field_name(name);
                    let field_name = format_ident!("{}", rust_name);
                    let is_required = is_field_required(name, &schema.required);
                    let ty = property_type(
                        components,
                        prop,
                        inline_enums,
                        &mut nested_inline_structs,
                        Some(&struct_name),
                        name,
                        is_required,
                    );
                    let doc: Option<proc_macro2::TokenStream> = match prop {
                        Schema::Object(box_prop) => {
                            let prop_ref = box_prop.as_ref();
                            let desc = &prop_ref.description;
                            if !desc.is_empty() {
                                Some(quote! { #[doc = #desc] })
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };
                    let is_required = is_field_required(name, &schema.required);
                    let serde_attr = if is_required {
                        if rust_name.as_str() != name {
                            quote! { #[serde(rename = #name)] }
                        } else {
                            quote! {}
                        }
                    } else {
                        if rust_name.as_str() != name {
                            quote! { #[serde(default, skip_serializing_if = "Option::is_none", rename = #name)] }
                        } else {
                            quote! { #[serde(default, skip_serializing_if = "Option::is_none")] }
                        }
                    };
                    vec![
                        doc.into_iter().collect::<Vec<_>>(),
                        vec![serde_attr],
                        vec![quote! { pub #field_name: #ty, }],
                    ]
                }).flatten().collect::<Vec<_>>();

                let struct_ident = format_ident!("{}", struct_name);
                inline_structs.push(quote! {
                    #[derive(Debug, Clone, Serialize, Deserialize)]
                    pub struct #struct_ident {
                        #(#fields)*
                    }
                });
                struct_name
            }
        }
        Some(Types::Single(Type::Array)) => {
            let inner_type = schema
                .items
                .as_ref()
                .map(|item| match item {
                    Schema::Object(inner_box_schema) => response_schema_to_string_item(
                        &inner_box_schema,
                        components,
                        inline_structs,
                        inline_enums,
                        endpoint_name,
                        main_inline_structs,
                    ),
                    _ => "serde_json::Value".to_string(),
                })
                .unwrap_or_else(|| "serde_json::Value".to_string());
            format!("Vec<{}>", inner_type)
        }
        Some(Types::Single(Type::String)) => "String".to_string(),
        Some(Types::Single(Type::Integer)) => {
            if schema.format == "int32" {
                "i32".to_string()
            } else if schema.format == "int64" {
                "i64".to_string()
            } else {
                "i64".to_string()
            }
        }
        Some(Types::Single(Type::Number)) => "f64".to_string(),
        Some(Types::Single(Type::Boolean)) => "bool".to_string(),
        _ => "serde_json::Value".to_string(),
    };
    main_inline_structs
        .borrow_mut()
        .extend(nested_inline_structs);
    result
}

/// Adds a parameter to the parameter collection.
///
/// # Arguments
///
/// * `params` - The parameter collection
/// * `param` - The parameter to add
pub fn add_parameter(params: &mut Vec<ParameterInfo>, param: &Parameter) {
    let param_location = param.parameter_in.clone().into();

    let param_type = extract_parameter_type(param);

    params.push(ParameterInfo {
        name: param.name.clone(),
        ty: param_type,
        required: param.required,
        description: param.description.clone(),
        param_location,
    });
}
