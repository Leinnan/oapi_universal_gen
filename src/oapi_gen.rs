use crate::RequestType;
use openapiv3_1::{
    Components, Object, OpenApi, Ref, RefOr, Schema, Type,
    path::{Operation, Parameter, ParameterIn},
    request_body::RequestBody,
    schema::Types,
};
use quote::{format_ident, quote};
use serde_json;

pub fn generate_from_json(json: &str) -> String {
    let openapi: OpenApi = serde_json::from_str(json).expect("Could not deserialize input");
    generate_code(&openapi)
}

pub fn generate_openapi_spec() {
    let data = include_str!("../schemas/other.json");
    let output = generate_from_json(data);
    println!("{}", output);
}

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
        let d = schema_ref.description.clone();
        let doc = quote! { #[doc = #d] };

        // Check if this is a tagged enum
        if let Some((tag_field, variants)) = extract_discriminator_info(&cmp_2, schema_ref) {
            let variant_defs: Vec<_> = variants
                .into_iter()
                .map(|(tag_value, variant_schema)| {
                    let variant_name = to_valid_enum_variant(&tag_value);
                    let ident = format_ident!("{}", variant_name);
                    let (fields, has_fields) = generate_struct_fields_for_tagged_variant(
                        &cmp_2,
                        &variant_schema,
                        &tag_field,
                        &mut *inline_structs.borrow_mut(),
                    );
                    if !has_fields {
                        quote! { #ident, }
                    } else {
                        quote! {
                            #[display(#variant_name)]
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

    let mut inline_enums: Vec<InlineEnumInfo> = Vec::new();

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
            // if name.contains("Auth") {
            //     eprintln!("{} -> {:#?}", name, schema_ref);
            // }

            // Check if this schema is a tagged enum (anyOf with object variants with const tags)
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
            let d = schema_ref.description.clone();
            let doc = quote! { #[doc = #d] };
            let fields = generate_struct_fields(
                &cmp_2,
                schema_or_ref,
                &mut inline_enums,
                &mut *inline_structs.borrow_mut(),
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

    let (inline_input_structs, methods) = generate_methods(openapi, &cmp_2, &mut inline_enums);

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
            let doc = enum_info
                .description
                .as_ref()
                .map(|d| quote! { #[doc = #d] });

            if enum_info.is_tagged {
                let tag_field = enum_info.tag_field.as_ref().unwrap();
                let discriminator_info = extract_discriminator_info(&cmp_2, &enum_info.schema)?;
                let (_, variants) = discriminator_info;

                let variant_defs: Vec<_> = variants
                    .into_iter()
                    .map(|(tag_value, variant_schema)| {
                        let variant_name = to_valid_enum_variant(&tag_value);
                        let ident = format_ident!("{}", variant_name);
                        let (fields, has_fields) = generate_struct_fields_for_tagged_variant(
                            &cmp_2,
                            &variant_schema,
                            tag_field,
                            &mut *inline_structs.borrow_mut(),
                        );
                        if !has_fields {
                            quote! { #ident, }
                        } else {
                            quote! {
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

#[derive(Debug, Clone, PartialEq)]
enum ParameterLocation {
    Path,
    Query,
    Header,
    Cookie,
}

#[derive(Debug, Clone)]
struct ParameterInfo {
    name: String,
    ty: String,
    required: bool,
    description: Option<String>,
    param_location: ParameterLocation,
}

#[derive(Debug, Clone)]
struct MethodInfo {
    name: String,
    http_method: RequestType,
    parameters: Vec<ParameterInfo>,
    request_body_type: Option<String>,
    response_type: Option<String>,
    description: Option<String>,
}

fn format_path_with_params(
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
        let rust_param_name = to_valid_rust_field_name(&param.name);
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

fn build_query_params(params: &[ParameterInfo]) -> Vec<proc_macro2::TokenStream> {
    params
        .iter()
        .filter(|p| p.param_location == ParameterLocation::Query)
        .map(|p| {
            let name = &p.name;
            let rust_name = to_valid_rust_field_name(name);
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

fn build_header_handling(params: &[ParameterInfo]) -> Vec<proc_macro2::TokenStream> {
    params
        .iter()
        .filter(|p| p.param_location == ParameterLocation::Header)
        .map(|p| {
            let name = &p.name;
            let rust_name = to_valid_rust_field_name(name);
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

fn generate_method_body(path: &str, method_info: &MethodInfo) -> proc_macro2::TokenStream {
    let fn_name = format_ident!("{}", method_info.name);
    let mut docs = format!("ENDPOINT {:?} {}", &method_info.http_method, path);

    if let Some(ref d) = method_info.description {
        docs.push('\n');
        docs.push_str(d.as_str());
        docs.push('\n');
    }
    if !method_info.parameters.is_empty() {
        docs.push('\n');
        docs.push_str("# Arguments");
        docs.push('\n');
        docs.push('\n');
    }
    for p in &method_info.parameters {
        let location_str = match p.param_location {
            ParameterLocation::Path => "path",
            ParameterLocation::Query => "query",
            ParameterLocation::Header => "header",
            ParameterLocation::Cookie => "cookie",
        };
        let param_name = to_valid_rust_field_name(&p.name);
        if let Some(ref description) = p.description {
            docs.push_str(&format!(
                "- `{}` {} - {}\n",
                location_str, param_name, description
            ));
        } else {
            docs.push_str(&format!("- `{}` {}\n", location_str, param_name));
        }
    }

    let param_decls: Vec<_> = method_info
        .parameters
        .iter()
        .map(|p| {
            let param_name = format_ident!("{}", to_valid_rust_field_name(&p.name));
            let param_ty = type_to_tokens(&p.ty);
            if p.required {
                quote! { #param_name: #param_ty }
            } else {
                quote! { #param_name: Option<#param_ty> }
            }
        })
        .collect();

    let return_type = match &method_info.response_type {
        Some(rt) => {
            let rt_type = type_to_tokens(rt);
            quote! { Result<#rt_type, Self::RequesterErrorType> }
        }
        None => quote! { Result<(), Self::RequesterErrorType> },
    };

    let where_clause = quote! { where Self::RequesterErrorType: From<OapiRequesterError> };

    let http_method = match method_info.http_method {
        RequestType::Get => quote! { RequestType::Get },
        RequestType::Post => quote! { RequestType::Post },
        RequestType::Put => quote! { RequestType::Put },
        RequestType::Patch => quote! { RequestType::Patch },
        RequestType::Delete => quote! { RequestType::Delete },
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

    let request_body_fields: Vec<proc_macro2::TokenStream> = method_info
        .parameters
        .iter()
        .filter(|p| p.param_location != ParameterLocation::Path)
        .filter(|p| p.param_location != ParameterLocation::Query)
        .filter(|p| p.param_location != ParameterLocation::Header)
        .map(|p| {
            let name = &p.name;
            let rust_name = to_valid_rust_field_name(name);
            quote! { #name: #rust_name }
        })
        .collect();

    let request_creation = if method_info.request_body_type.is_some() {
        let body_type = method_info.request_body_type.as_ref().unwrap();
        let body_ty = type_to_tokens(body_type);

        quote! {
            let body: #body_ty = serde_json::from_value(serde_json::json!({
                #(#request_body_fields)*
            }))
            .map_err(|_e| OapiRequesterError::SerializationError.into())?;

            let request = self.create_request_with_body(#http_method, &uri, &body)?;
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
        #[doc = #docs]
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

fn generate_methods(
    openapi: &OpenApi,
    components: &Components,
    inline_enums: &mut Vec<InlineEnumInfo>,
) -> (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) {
    let mut inline_input_structs = Vec::new();
    let mut methods = Vec::new();

    for (path, path_item) in openapi.paths.paths.iter() {
        let path_params: Vec<Parameter> = path_item
            .parameters
            .as_ref()
            .map(|p| p.clone())
            .unwrap_or_default();

        let operations = [
            ("get", RequestType::Get, path_item.get.as_ref()),
            ("post", RequestType::Post, path_item.post.as_ref()),
            ("put", RequestType::Put, path_item.put.as_ref()),
            ("patch", RequestType::Patch, path_item.patch.as_ref()),
            ("delete", RequestType::Delete, path_item.delete.as_ref()),
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
                );

                let method_body = generate_method_body(path, &method_info);
                methods.push(method_body);
            }
        }
    }

    (inline_input_structs, methods)
}

fn build_method_info(
    path: &str,
    http_method: RequestType,
    operation: Option<&Operation>,
    path_params: &[Parameter],
    components: &Components,
    inline_structs: &mut Vec<proc_macro2::TokenStream>,
    inline_enums: &mut Vec<InlineEnumInfo>,
) -> MethodInfo {
    let method_name = format!(
        "{}_{}",
        clean_path_for_method_name(path),
        method_suffix(&http_method)
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
        let request_body_type = oper.request_body.as_ref().and_then(|rb| {
            extract_request_body_type(
                &RefOr::T(rb.clone()),
                components,
                inline_structs,
                inline_enums,
            )
        });

        let response_type = extract_response_type(
            &oper.responses,
            components,
            inline_structs,
            inline_enums,
            &method_name,
        );

        let description = oper.description.clone().or(oper.summary.clone());

        MethodInfo {
            name: method_name,
            http_method,
            parameters,
            request_body_type,
            response_type,
            description,
        }
    } else {
        MethodInfo {
            name: method_name,
            http_method,
            parameters,
            request_body_type: None,
            response_type: None,
            description: None,
        }
    }
}

fn add_parameter(params: &mut Vec<ParameterInfo>, param: &Parameter) {
    let param_location = match param.parameter_in {
        ParameterIn::Query => ParameterLocation::Query,
        ParameterIn::Path => ParameterLocation::Path,
        ParameterIn::Header => ParameterLocation::Header,
        ParameterIn::Cookie => ParameterLocation::Cookie,
    };

    let param_type = extract_parameter_type(param);

    params.push(ParameterInfo {
        name: param.name.clone(),
        ty: param_type,
        required: param.required,
        description: param.description.clone(),
        param_location,
    });
}

fn extract_parameter_type(pd: &Parameter) -> String {
    match &pd.schema {
        Some(Schema::Object(box_schema)) => schema_type_to_string(box_schema.as_ref()),
        _ => "serde_json::Value".to_string(),
    }
}

fn schema_type_to_string(schema: &Object) -> String {
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

fn extract_request_body_type(
    rb: &RefOr<RequestBody>,
    components: &Components,
    inline_structs: &mut Vec<proc_macro2::TokenStream>,
    inline_enums: &mut Vec<InlineEnumInfo>,
) -> Option<String> {
    let mut nested_inline_structs: Vec<InlineStructInfo> = Vec::new();
    match rb {
        RefOr::T(request_body) => {
            for (media_type, mt) in &request_body.content {
                if media_type == "application/json" || media_type == "*/*" {
                    if let Some(schema) = &mt.schema {
                        return match schema {
                            Schema::Object(box_schema) => match &box_schema.schema_type {
                                Some(Types::Single(Type::Object)) => {
                                    let struct_name =
                                        generate_inline_struct_name(inline_structs, "Request");
                                    let fields = box_schema.properties.iter().flat_map(|(name, prop)| {
                                    let rust_name = to_valid_rust_field_name(name);
                                    let field_name = format_ident!("{}", rust_name);
                                    let ty = property_type(components, prop, inline_enums, &mut nested_inline_structs);
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
                                        let serde_attr = if rust_name.as_str() != name {
                                            quote! { #[serde(default, skip_serializing_if = "Option::is_none", rename = #name)] }
                                        } else {
                                            quote! { #[serde(default, skip_serializing_if = "Option::is_none")] }
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
                    }
                }
            }
            None
        }
        RefOr::Ref(Ref { ref_location, .. }) => {
            if let Some(path) = ref_location.strip_prefix("#/components/requestBodies/") {
                let body_name = to_pascal_case(path);
                Some(body_name)
            } else {
                None
            }
        }
    }
}

fn extract_response_type(
    responses: &openapiv3_1::Responses,
    components: &openapiv3_1::Components,
    inline_structs: &mut Vec<proc_macro2::TokenStream>,
    inline_enums: &mut Vec<InlineEnumInfo>,
    endpoint_name: &str,
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
                        ));
                    }
                }
            }
        }
    }
    None
}

fn is_success_status_code(status_code: &str) -> bool {
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

fn response_schema_to_string(
    schema: &Schema,
    components: &Components,
    inline_structs: &mut Vec<proc_macro2::TokenStream>,
    inline_enums: &mut Vec<InlineEnumInfo>,
    endpoint_name: &str,
) -> String {
    let mut nested_inline_structs: Vec<InlineStructInfo> = Vec::new();
    match schema {
        Schema::Object(box_schema) => match &box_schema.schema_type {
            Some(Types::Single(Type::Object)) => {
                if box_schema.properties.is_empty() {
                    return "serde_json::Value".to_string();
                }
                let struct_name = format!("{}{}", to_pascal_case(endpoint_name), "Response");
                let fields = box_schema.properties.iter().flat_map(|(name, prop)| {
                    let rust_name = to_valid_rust_field_name(name);
                    let field_name = format_ident!("{}", rust_name);
                    let ty = property_type(components, prop, inline_enums, &mut nested_inline_structs);
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
                    let serde_attr = if rust_name.as_str() != name {
                        quote! { #[serde(default, skip_serializing_if = "Option::is_none", rename = #name)] }
                    } else {
                        quote! { #[serde(default, skip_serializing_if = "Option::is_none")] }
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
        },
        _ => "serde_json::Value".to_string(),
    }
}

fn response_schema_to_string_item(
    schema: &Object,
    components: &Components,
    inline_structs: &mut Vec<proc_macro2::TokenStream>,
    inline_enums: &mut Vec<InlineEnumInfo>,
    endpoint_name: &str,
) -> String {
    let mut nested_inline_structs: Vec<InlineStructInfo> = Vec::new();
    match &schema.schema_type {
        Some(Types::Single(Type::Object)) => {
            if schema.properties.is_empty() {
                return "serde_json::Value".to_string();
            }
            let struct_name = format!("{}{}", to_pascal_case(endpoint_name), "Response");
            let fields = schema.properties.iter().flat_map(|(name, prop)| {
                let rust_name = to_valid_rust_field_name(name);
                let field_name = format_ident!("{}", rust_name);
                let ty = property_type(components, prop, inline_enums, &mut nested_inline_structs);
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
                let serde_attr = if rust_name.as_str() != name {
                    quote! { #[serde(default, skip_serializing_if = "Option::is_none", rename = #name)] }
                } else {
                    quote! { #[serde(default, skip_serializing_if = "Option::is_none")] }
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
    }
}

fn generate_inline_struct_name(
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

fn clean_path_for_method_name(path: &str) -> String {
    let mut result = path.trim_start_matches("/").to_lowercase();

    while result.contains("{") && result.contains("}") {
        let start = result.find("{").unwrap();
        let end = result.find("}").unwrap();
        if start < end {
            let param_name = &result[start + 1..end];
            let snake_case_param = to_snake_case(param_name);
            result.replace_range(start..=end, &format!("_{}_", snake_case_param));
        } else {
            break;
        }
    }

    let normalized = result.replace("//", "/");

    let mut output = normalized
        .replace("/", "_")
        .replace("-", "_")
        .trim_end_matches('_')
        .to_string();

    while output.contains("__") {
        output = output.replace("__", "_");
    }

    output
}

fn method_suffix(method: &RequestType) -> String {
    match method {
        RequestType::Get => "get",
        RequestType::Post => "post",
        RequestType::Put => "put",
        RequestType::Patch => "patch",
        RequestType::Delete => "delete",
    }
    .to_string()
}

fn type_to_tokens(ty: &str) -> proc_macro2::TokenStream {
    if ty.contains("::") || ty.contains('<') {
        ty.parse().unwrap_or_else(|_| quote! { serde_json::Value })
    } else {
        let ident = format_ident!("{}", ty);
        quote! { #ident }
    }
}

fn is_enum_schema(schema: &Schema) -> bool {
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

fn extract_enum_values(obj: &Object) -> Vec<serde_json::Value> {
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

fn generate_struct_fields(
    components: &Components,
    schema: &Schema,
    inline_enums: &mut Vec<InlineEnumInfo>,
    inline_structs: &mut Vec<InlineStructInfo>,
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
            let ty = property_type(components, prop, inline_enums, inline_structs);
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
            let serde_attr = if rust_name.as_str() != name {
                quote! { #[serde(default, skip_serializing_if = "Option::is_none", rename = #name)] }
            } else {
                quote! { #[serde(default, skip_serializing_if = "Option::is_none")] }
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

fn to_valid_rust_field_name(name: &str) -> String {
    let rust_keywords = [
        "type", "mod", "use", "fn", "struct", "enum", "impl", "trait", "const", "static", "let",
        "mut", "ref", "return", "if", "else", "match", "for", "while", "loop", "break", "continue",
        "self", "super", "crate", "async", "await", "move", "where", "pub", "priv", "unsafe",
        "extern", "dyn", "abstract", "override", "final", "in",
    ];

    let mut result = name.replace('$', "dollar_");

    if rust_keywords.contains(&result.as_str()) {
        result = format!("{}_field", result);
    }

    to_snake_case(&result)
}

struct InlineEnumInfo {
    name: String,
    schema: Object,
    description: Option<String>,
    is_tagged: bool,
    tag_field: Option<String>,
}

struct InlineStructInfo {
    name: String,
    fields: Vec<proc_macro2::TokenStream>,
}

fn property_type(
    components: &Components,
    schema: &Schema,
    inline_enums: &mut Vec<InlineEnumInfo>,
    inline_structs: &mut Vec<InlineStructInfo>,
) -> proc_macro2::TokenStream {
    match schema {
        Schema::Object(box_schema) => {
            if !box_schema.reference.is_empty() {
                let ref_name = extract_ref_name(&box_schema.reference);
                let ident = format_ident!("{}", ref_name);
                return quote! { Option<#ident> };
            }
            if let Some(enum_name) = get_enum_type_name(box_schema) {
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
                return quote! { Option<#ident> };
            }
            if let Some((tag_field, _variants)) = extract_discriminator_info(components, box_schema)
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
                return quote! { Option<#enum_name> };
            }
            match &box_schema.schema_type {
                Some(Types::Single(Type::String)) => quote! { Option<String> },
                Some(Types::Single(Type::Integer)) => {
                    let format = &box_schema.format;
                    match format.as_str() {
                        "int32" => quote! { Option<i32> },
                        "int64" => quote! { Option<i64> },
                        _ => quote! { Option<i64> },
                    }
                }
                Some(Types::Single(Type::Number)) => quote! { Option<f64> },
                Some(Types::Single(Type::Boolean)) => quote! { Option<bool> },
                Some(Types::Single(Type::Array)) => {
                    let inner = box_schema.items.as_ref();
                    let inner_ty = if let Some(item_ref) = inner {
                        property_type(components, item_ref, inline_enums, inline_structs)
                    } else {
                        quote! { String }
                    };
                    quote! { Option<Vec<#inner_ty>> }
                }
                Some(Types::Single(Type::Object)) => {
                    if box_schema.properties.is_empty() {
                        quote! { Option<serde_json::Value> }
                    } else {
                        let base_name = "InlineObject";
                        let mut struct_name = base_name.to_string();
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
                                let ty = property_type(components, prop, inline_enums, inline_structs);
                                let serde_attr = if rust_name.as_str() != name {
                                    quote! { #[serde(default, skip_serializing_if = "Option::is_none", rename = #name)] }
                                } else {
                                    quote! { #[serde(default, skip_serializing_if = "Option::is_none")] }
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
                        quote! { Option<#ident> }
                    }
                }
                _ => quote! { Option<serde_json::Value> },
            }
        }
        _ => quote! { Option<serde_json::Value> },
    }
}

fn get_enum_type_name(schema: &Object) -> Option<String> {
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

fn extract_discriminator_info(
    components: &Components,
    obj: &Object,
) -> Option<(String, Vec<(String, Object)>)> {
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
        if !box_schema.reference.is_empty() {
            let ref_name = extract_ref_name(&box_schema.reference);
            eprintln!("Search for {}", ref_name);
            if let Some(Schema::Object(s)) = components.schemas.get(&ref_name) {
                box_schema = s.as_ref();
            } else {
                return None;
            }
        }

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
                box_schema.clone(),
            ));
        } else {
            return None;
        }
    }

    Some((tag_field?, variants))
}

fn generate_struct_fields_for_tagged_variant(
    components: &Components,
    schema: &Object,
    tag_field: &str,
    inline_structs: &mut Vec<InlineStructInfo>,
) -> (Vec<proc_macro2::TokenStream>, bool) {
    let fields: Vec<_> = schema
        .properties
        .iter()
        .filter(|(name, _)| *name != tag_field)
        .map(|(name, prop)| {
            let rust_name = to_valid_rust_field_name(name);
            let field_name = format_ident!("{}", rust_name);
            let ty = property_type(components, prop, &mut Vec::new(), inline_structs);
            let serde_attr = if rust_name.as_str() != name {
                quote! { #[serde(default, skip_serializing_if = "Option::is_none", rename = #name)] }
            } else {
                quote! { #[serde(default, skip_serializing_if = "Option::is_none")] }
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

fn extract_ref_name(reference: &str) -> String {
    if let Some(path) = reference.strip_prefix("#/components/schemas/") {
        to_pascal_case(path)
    } else {
        "serde_json::Value".to_string()
    }
}

fn enum_value_to_variant_name(value: &serde_json::Value) -> proc_macro2::Ident {
    match value {
        serde_json::Value::String(s) => {
            let variant_name = to_valid_enum_variant(s);
            format_ident!("{}", variant_name)
        }
        serde_json::Value::Number(n) => {
            let num_str = n.to_string().replace('.', "_");
            let variant_name = format!("V{}", num_str.replace('-', "Neg"));
            format_ident!("{}", variant_name)
        }
        serde_json::Value::Bool(b) => {
            let variant_name = if *b { "True" } else { "False" };
            format_ident!("{}", variant_name)
        }
        _ => format_ident!("Unknown"),
    }
}

fn to_valid_enum_variant(name: &str) -> String {
    let mut result = String::new();
    let mut first = true;
    for c in name.chars() {
        if c.is_alphanumeric() || c == '_' {
            if first && c.is_ascii_digit() {
                result.push('V');
            }
            if result.is_empty() {
                result.push(c.to_ascii_uppercase());
            } else {
                result.push(c);
            }
            first = false;
        } else if !result.is_empty()
            && result
                .chars()
                .last()
                .map(|last| last.is_alphanumeric())
                .unwrap_or(false)
        {
            result.push('_');
        }
    }
    if result.is_empty() {
        result = "Unknown".to_string();
    }
    result
}

fn to_snake_case(name: &str) -> String {
    let name = name.replace('-', "_");
    let mut result = String::new();
    let chars: Vec<char> = name.chars().collect();
    let n = chars.len();

    let mut i = 0;
    while i < n {
        if chars[i].is_uppercase() {
            if i > 0 {
                let next_upper_count = chars[i..n]
                    .iter()
                    .take_while(|&&c| c.is_uppercase())
                    .count();

                if next_upper_count > 1 {
                    if i > 0 {
                        result.push('_');
                    }
                    for j in i..i + next_upper_count {
                        result.push(chars[j].to_ascii_lowercase());
                    }
                    i += next_upper_count;
                    continue;
                }

                result.push('_');
                result.push(chars[i].to_ascii_lowercase());
            } else {
                result.push(chars[i].to_ascii_lowercase());
            }
        } else {
            result.push(chars[i]);
        }
        i += 1;
    }
    result
}

fn to_pascal_case(input: &str) -> String {
    input
        .split(|c: char| !c.is_alphanumeric())
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut chars = segment.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => {
                    let first = f.to_uppercase().collect::<String>();
                    let rest: String = chars.as_str().to_string();
                    if rest.chars().any(|c| c.is_uppercase()) {
                        first + &rest
                    } else {
                        first + &rest.to_lowercase()
                    }
                }
            }
        })
        .collect()
}
