//! Main code generation logic for OpenAPI specifications.
//!
//! This module provides the entry points for generating Rust code from
//! OpenAPI specifications. It orchestrates the entire generation process
//! including schema parsing, type generation, and method implementation.

use openapiv3_1::{Components, OpenApi, Schema, Type, schema::Types};
use quote::{format_ident, quote};
use std::collections::HashSet;

use super::methods::generate_methods;
use super::schemas::{
    extract_discriminator_info, extract_enum_values, generate_struct_fields,
    generate_struct_fields_for_tagged_variant, is_enum_schema, property_type,
};
use super::types::{InlineEnumInfo, InlineStructInfo};
use super::utils::{enum_value_to_variant_name, to_pascal_case, to_valid_enum_variant};

#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    pub generate_doc_comments: bool,
    pub generate_display_trait: bool,
    pub serde_rename_all_snake_case: bool,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        GeneratorConfig {
            generate_doc_comments: true,
            generate_display_trait: true,
            serde_rename_all_snake_case: true,
        }
    }
}

fn make_schema_doc(name: &str, description: Option<String>) -> String {
    if let Some(desc) = description {
        if !desc.is_empty() {
            return desc;
        }
    }
    format!("Generated from schema '{}'.", name)
}

pub struct Generator<'a> {
    pub openapi: &'a OpenApi,
    pub config: GeneratorConfig,
    pub components: Components,
    pub inline_structs: Vec<InlineStructInfo>,
    pub inline_enums: Vec<InlineEnumInfo>,
    pub enum_defs: Vec<proc_macro2::TokenStream>,
    pub struct_defs: Vec<proc_macro2::TokenStream>,
    pub inline_input_structs: Vec<proc_macro2::TokenStream>,
    pub inline_enum_defs: Vec<proc_macro2::TokenStream>,
    pub inline_struct_defs: Vec<proc_macro2::TokenStream>,
    pub methods: Vec<proc_macro2::TokenStream>,
    pub seen_enum_names: HashSet<String>,
    pub seen_struct_names: HashSet<String>,
}

impl<'a> Generator<'a> {
    pub fn new(openapi: &'a OpenApi) -> Self {
        Generator {
            openapi,
            config: GeneratorConfig::default(),
            components: openapi.components.clone().unwrap_or_default(),
            inline_structs: Vec::new(),
            inline_enums: Vec::new(),
            enum_defs: Vec::new(),
            struct_defs: Vec::new(),
            inline_input_structs: Vec::new(),
            inline_enum_defs: Vec::new(),
            inline_struct_defs: Vec::new(),
            methods: Vec::new(),
            seen_enum_names: HashSet::new(),
            seen_struct_names: HashSet::new(),
        }
    }

    pub fn with_config(openapi: &'a OpenApi, config: GeneratorConfig) -> Self {
        Generator {
            openapi,
            config,
            components: openapi.components.clone().unwrap_or_default(),
            inline_structs: Vec::new(),
            inline_enums: Vec::new(),
            enum_defs: Vec::new(),
            struct_defs: Vec::new(),
            inline_input_structs: Vec::new(),
            inline_enum_defs: Vec::new(),
            inline_struct_defs: Vec::new(),
            methods: Vec::new(),
            seen_enum_names: HashSet::new(),
            seen_struct_names: HashSet::new(),
        }
    }

    pub fn generate_code(&mut self) -> String {
        self.debug_state();
        self.generate_enums();
        self.generate_structs();
        self.generate_inline_types();
        self.generate_methods();
        self.assemble_output()
    }

    fn debug_state(&self) {
        println!("=== Generator State ===");
        println!("Inline structs: {}", self.inline_structs.len());
        println!("Inline enums: {}", self.inline_enums.len());
        println!("Enum defs: {}", self.enum_defs.len());
        println!("Struct defs: {}", self.struct_defs.len());
        println!("Inline input structs: {}", self.inline_input_structs.len());
        println!("Inline enum defs: {}", self.inline_enum_defs.len());
        println!("Inline struct defs: {}", self.inline_struct_defs.len());
        println!("Methods: {}", self.methods.len());
        println!("Seen enums: {}", self.seen_enum_names.len());
        println!("Seen structs: {}", self.seen_struct_names.len());
        println!("======================");
    }

    pub fn print_statistics(&self) {
        println!("\n=== Generation Statistics ===");
        println!(
            "Component Schemas Enums: {}",
            self.count_component_enum_schemas()
        );
        println!(
            "Component Schemas Structs: {}",
            self.count_component_struct_schemas()
        );
        println!(
            "Total Inline Structs Generated: {}",
            self.inline_struct_defs.len()
        );
        println!(
            "Total Inline Enums Generated: {}",
            self.inline_enum_defs.len()
        );
        println!("Total Methods Generated: {}", self.methods.len());
        println!("=============================\n");
    }

    fn count_component_enum_schemas(&self) -> usize {
        self.components
            .schemas
            .iter()
            .filter(|(_, schema_or_ref)| is_enum_schema(schema_or_ref))
            .count()
    }

    fn count_component_struct_schemas(&self) -> usize {
        self.components
            .schemas
            .iter()
            .filter(|(_, schema_or_ref)| !is_enum_schema(schema_or_ref))
            .count()
    }

    fn generate_enums(&mut self) {
        let enum_schemas: Vec<_> = self
            .components
            .schemas
            .iter()
            .filter(|(_, schema_or_ref)| is_enum_schema(schema_or_ref))
            .collect();

        for (name, schema) in enum_schemas {
            let schema_ref = match schema {
                Schema::Object(box_obj) => box_obj.as_ref(),
                _ => continue,
            };
            let enum_name = format_ident!("{}", to_pascal_case(name.as_str()));
            let d = make_schema_doc(name.as_str(), Some(schema_ref.description.clone()));
            let doc = quote! { #[doc = #d] };

            if let Some((tag_field, variants)) =
                extract_discriminator_info(&self.components, schema_ref)
            {
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
                        let enum_name_str = to_pascal_case(name);
                        let variant_parent_name = if !ref_name.is_empty() {
                            ref_name.clone()
                        } else {
                            format!("{}{}", enum_name_str, variant_name)
                        };
                        let (fields, has_fields) = generate_struct_fields_for_tagged_variant(
                            &self.components,
                            &variant_schema,
                            &tag_field,
                            &mut self.inline_structs,
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
                self.enum_defs.push(quote! {
                    #doc
                    #[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
                    #[serde(tag = #tag_field_str)]
                    pub enum #enum_name {
                        #(#variant_defs)*
                    }
                });
            } else {
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

                    self.enum_defs.push(quote! {
                        #doc
                        #[derive(Debug, Clone, Serialize, Deserialize)]
                        pub enum #enum_name {
                            #(#variants)*
                        }
                    });
                } else {
                    let variants = enum_values.into_iter().map(|val| {
                        let variant_name = enum_value_to_variant_name(&val);
                        quote! { #variant_name, }
                    });

                    self.enum_defs.push(quote! {
                        #doc
                        #[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
                        #[serde(rename_all = "snake_case")]
                        pub enum #enum_name {
                            #(#variants)*
                        }
                    });
                }
            }
        }
    }

    fn generate_structs(&mut self) {
        let struct_schemas: Vec<_> = self
            .components
            .schemas
            .iter()
            .filter(|(_, schema_or_ref)| !is_enum_schema(schema_or_ref))
            .collect();

        for (name, schema_or_ref) in struct_schemas {
            let schema_ref = match schema_or_ref {
                Schema::Object(box_obj) => box_obj.as_ref(),
                _ => continue,
            };

            if let Some((tag_field, _variants)) =
                extract_discriminator_info(&self.components, schema_ref)
            {
                let enum_name_str = to_pascal_case(name);
                let description = if schema_ref.description.is_empty() {
                    None
                } else {
                    Some(schema_ref.description.clone())
                };
                self.inline_enums.push(InlineEnumInfo {
                    name: enum_name_str.clone(),
                    schema: schema_ref.clone(),
                    description,
                    is_tagged: true,
                    tag_field: Some(tag_field),
                });
                let enum_ident = format_ident!("{}", enum_name_str);
                let doc = quote! { #[doc = {}] };
                self.struct_defs.push(quote! {
                    #doc
                    #[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
                    pub enum #enum_ident {}
                });
                continue;
            }

            let schema_name = format_ident!("{}", to_pascal_case(name.as_str()));
            let d = make_schema_doc(name.as_str(), Some(schema_ref.description.clone()));
            let doc = quote! { #[doc = #d] };

            if let Some(Types::Single(Type::Array)) = &schema_ref.schema_type {
                if let Some(items) = &schema_ref.items {
                    let item_type = property_type(
                        &self.components,
                        items,
                        &mut self.inline_enums,
                        &mut self.inline_structs,
                        Some(name),
                        "Item",
                        true,
                    );
                    self.struct_defs.push(quote! {
                        #doc
                        #[derive(Debug, Clone, Serialize, Deserialize)]
                        pub struct #schema_name(pub Vec<#item_type>);
                    });
                    continue;
                }
            }

            let fields = generate_struct_fields(
                &self.components,
                schema_or_ref,
                &mut self.inline_enums,
                &mut self.inline_structs,
                Some(name),
            );
            let struct_tokenstream = quote! {
                #doc
                #[derive(Debug, Clone, Serialize, Deserialize)]
                pub struct #schema_name {
                    #(#fields)*
                }
            };
            self.struct_defs.push(struct_tokenstream);
        }
    }

    fn generate_inline_types(&mut self) {
        let (inline_input_structs, methods) = generate_methods(
            self.openapi,
            &self.components,
            &mut self.inline_enums,
            &mut self.inline_structs,
        );

        self.inline_input_structs = inline_input_structs;
        self.methods = methods;

        let mut seen_enum_names = std::mem::take(&mut self.seen_enum_names);
        let mut seen_struct_names = std::mem::take(&mut self.seen_struct_names);

        self.inline_enum_defs = self
            .inline_enums
            .iter()
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
                    let discriminator_info =
                        extract_discriminator_info(&self.components, &enum_info.schema);
                    if discriminator_info.is_none() {
                        return std::iter::once(quote! {});
                    }
                    let (_, variants) = discriminator_info.unwrap();

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
                                &self.components,
                                &variant_schema,
                                tag_field,
                                &mut self.inline_structs,
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
                    std::iter::once(quote! {
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

                    std::iter::once(quote! {
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

        self.inline_struct_defs = self
            .inline_structs
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

        self.seen_enum_names = seen_enum_names;
        self.seen_struct_names = seen_struct_names;
    }

    fn generate_methods(&mut self) {
        use super::schemas::{add_parameter, extract_request_body_type, extract_response_type};

        self.inline_input_structs = Vec::new();
        self.methods = Vec::new();

        for (path, path_item) in self.openapi.paths.paths.iter() {
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
                    let method_name = format!(
                        "{}_{}",
                        super::utils::clean_path_for_method_name(path),
                        super::utils::method_suffix(&http_method)
                    );

                    let mut parameters: Vec<super::types::ParameterInfo> = Vec::new();

                    for param in &path_params {
                        add_parameter(&mut parameters, param);
                    }
                    if let Some(params) = &op.parameters {
                        for param in params {
                            add_parameter(&mut parameters, param);
                        }
                    }

                    let (request_body_type, request_body_required) = op
                        .request_body
                        .as_ref()
                        .map(|rb| {
                            extract_request_body_type(
                                &openapiv3_1::RefOr::T(rb.clone()),
                                &self.components,
                                &mut self.inline_input_structs,
                                &mut self.inline_enums,
                                &mut self.inline_structs,
                                &method_name,
                            )
                        })
                        .unwrap_or((None, false));

                    let response_type = extract_response_type(
                        &op.responses,
                        &self.components,
                        &mut self.inline_input_structs,
                        &mut self.inline_enums,
                        &method_name,
                        &mut self.inline_structs,
                    );

                    let description = op.description.clone().or(op.summary.clone());

                    let method_info = super::types::MethodInfo {
                        name: method_name,
                        http_method,
                        parameters,
                        request_body_type,
                        request_body_required,
                        response_type,
                        description,
                    };

                    let method_body = self.generate_method_body(path, &method_info);
                    self.methods.push(method_body);
                }
            }
        }
    }

    fn generate_method_body(
        &self,
        path: &str,
        method_info: &super::types::MethodInfo,
    ) -> proc_macro2::TokenStream {
        use super::methods::{build_header_handling, build_query_params, format_path_with_params};
        use super::utils::type_to_tokens;

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
                super::types::ParameterLocation::Path => "path",
                super::types::ParameterLocation::Query => "query",
                super::types::ParameterLocation::Header => "header",
                super::types::ParameterLocation::Cookie => "cookie",
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
                let param_name =
                    format_ident!("{}", super::utils::to_valid_rust_field_name(&p.name));
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

        let (path_format, _path_replacements) =
            format_path_with_params(path, &method_info.parameters);
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

    fn assemble_output(&self) -> String {
        let enum_defs = &self.enum_defs;
        let struct_defs = &self.struct_defs;
        let inline_input_structs = &self.inline_input_structs;
        let inline_enum_defs = &self.inline_enum_defs;
        let inline_struct_defs = &self.inline_struct_defs;
        let methods = &self.methods;

        let raw_tokens = quote! {
            use serde::{Serialize, Deserialize};
            use oapi_universal_gen::*;
            use std::future::Future;

            #( #enum_defs )*

            #( #struct_defs )*

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
        self.print_statistics();
        formatted
    }
}

pub fn generate_from_json(json: &str) -> String {
    let openapi: OpenApi = serde_json::from_str(json).expect("Could not deserialize input");
    generate_code(&openapi)
}

pub fn generate_openapi_spec() {
    let data = include_str!("../../schemas/other.json");
    let output = generate_from_json(data);
    println!("{}", output);
}

fn generate_code(openapi: &OpenApi) -> String {
    let mut generator = Generator::new(openapi);
    generator.generate_code()
}
