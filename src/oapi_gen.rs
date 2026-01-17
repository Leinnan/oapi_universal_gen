use crate::RequestType;
use openapiv3::{OpenAPI, ReferenceOr, SchemaKind, Type};
use quote::{format_ident, quote};
use serde_json;

pub fn generate_from_json(json: &str) -> String {
    let openapi: OpenAPI = serde_json::from_str(json).expect("Could not deserialize input");
    generate_code(&openapi)
}

pub fn generate_openapi_spec() {
    let data = include_str!("../schemas/other.json");
    let output = generate_from_json(data);
    println!("{}", output);
}

fn generate_code(openapi: &OpenAPI) -> String {
    let cmp = openapi.components.clone().unwrap_or_default();

    let enum_schemas: Vec<_> = cmp
        .schemas
        .iter()
        .filter(|(_, schema)| is_enum_schema(*schema))
        .collect();

    let enums = enum_schemas.iter().flat_map(|(name, schema)| {
        let schema_ref = schema.as_item()?;
        let enum_name = format_ident!("{}", to_pascal_case(name.as_str()));
        let doc = schema_ref
            .schema_data
            .description
            .as_ref()
            .map(|d| quote! { #[doc = #d] });

        let variants = extract_enum_values(schema_ref).into_iter().map(|val| {
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
    });

    let mut inline_enums: Vec<InlineEnumInfo> = Vec::new();

    let struct_schemas: Vec<_> = cmp
        .schemas
        .iter()
        .filter(|(_, schema_or_ref)| !is_enum_schema(*schema_or_ref))
        .collect();

    let structs: Vec<_> = struct_schemas
        .iter()
        .flat_map(|(name, schema_or_ref)| {
            let schema_ref = schema_or_ref.as_item()?;
            let schema_name = format_ident!("{}", to_pascal_case(name.as_str()));
            let doc = schema_ref
                .schema_data
                .description
                .as_ref()
                .map(|d| quote! { #[doc = #d] });
            let fields = generate_struct_fields(schema_ref, &mut inline_enums);
            Some(quote! {
                #doc
                #[derive(Debug, Clone, Serialize, Deserialize)]
                pub struct #schema_name {
                    #(#fields)*
                }
            })
        })
        .collect();

    let (inline_input_structs, methods) = generate_methods(openapi, &mut inline_enums);

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
            // Write full generated code to file for debugging
            std::fs::write("generated_code_debug.txt", &raw_code).ok();
            eprintln!(
                "Generated code written to generated_code_debug.txt ({} chars)",
                raw_code.len()
            );
            eprintln!(
                "First 3000 chars:\n{}",
                &raw_code[..raw_code.len().min(3000)]
            );
            // Find position near error
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
    openapi: &OpenAPI,
    inline_enums: &mut Vec<InlineEnumInfo>,
) -> (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) {
    let mut inline_input_structs = Vec::new();
    let mut methods = Vec::new();

    for (path, path_item_ref) in openapi.paths.iter() {
        let path_item = match path_item_ref {
            ReferenceOr::Item(item) => item,
            ReferenceOr::Reference { .. } => continue,
        };

        let path_params: Vec<openapiv3::ReferenceOr<openapiv3::Parameter>> =
            path_item.parameters.iter().cloned().collect();

        let operations = [
            ("get", RequestType::Get, path_item.get.as_ref()),
            ("post", RequestType::Post, path_item.post.as_ref()),
            ("put", RequestType::Put, path_item.put.as_ref()),
            ("patch", RequestType::Patch, path_item.patch.as_ref()),
            ("delete", RequestType::Delete, path_item.delete.as_ref()),
        ];

        for (_method_name, http_method, operation) in operations {
            if let Some(op) = operation {
                let components = openapi.components.as_ref();
                let method_info = build_method_info(
                    path,
                    http_method,
                    op,
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
    operation: &openapiv3::Operation,
    path_params: &[openapiv3::ReferenceOr<openapiv3::Parameter>],
    components: Option<&openapiv3::Components>,
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
        if let openapiv3::ReferenceOr::Item(p) = param {
            add_parameter(&mut parameters, p);
        }
    }

    for param in &operation.parameters {
        if let openapiv3::ReferenceOr::Item(p) = param {
            add_parameter(&mut parameters, p);
        }
    }

    let request_body_type = operation
        .request_body
        .as_ref()
        .and_then(|rb| extract_request_body_type(rb, components, inline_structs, inline_enums));

    let response_type = extract_response_type(
        &operation.responses,
        components,
        inline_structs,
        inline_enums,
        &method_name,
    );

    let description = operation.description.clone().or(operation.summary.clone());

    MethodInfo {
        name: method_name,
        http_method,
        parameters,
        request_body_type,
        response_type,
        description,
    }
}

fn add_parameter(params: &mut Vec<ParameterInfo>, param: &openapiv3::Parameter) {
    let pd = match param {
        openapiv3::Parameter::Query { parameter_data, .. } => parameter_data,
        openapiv3::Parameter::Path { parameter_data, .. } => parameter_data,
        openapiv3::Parameter::Header { parameter_data, .. } => parameter_data,
        openapiv3::Parameter::Cookie { parameter_data, .. } => parameter_data,
    };

    let param_location = match param {
        openapiv3::Parameter::Query { .. } => ParameterLocation::Query,
        openapiv3::Parameter::Path { .. } => ParameterLocation::Path,
        openapiv3::Parameter::Header { .. } => ParameterLocation::Header,
        openapiv3::Parameter::Cookie { .. } => ParameterLocation::Cookie,
    };

    let param_type = extract_parameter_type(pd);

    params.push(ParameterInfo {
        name: pd.name.clone(),
        ty: param_type,
        required: pd.required,
        description: pd.description.clone(),
        param_location,
    });
}

fn extract_parameter_type(pd: &openapiv3::ParameterData) -> String {
    match &pd.format {
        openapiv3::ParameterSchemaOrContent::Schema(schema) => schema_type_to_string(schema),
        openapiv3::ParameterSchemaOrContent::Content(_) => "serde_json::Value".to_string(),
    }
}

fn schema_type_to_string(schema: &ReferenceOr<openapiv3::Schema>) -> String {
    match schema {
        ReferenceOr::Item(s) => {
            if let Some(enum_name) = get_enum_type_name(s) {
                return enum_name;
            }
            match &s.schema_kind {
                SchemaKind::Type(Type::String(_)) => "String".to_string(),
                SchemaKind::Type(Type::Integer(int_type)) => match &int_type.format {
                    openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::IntegerFormat::Int32) => {
                        "i32".to_string()
                    }
                    openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::IntegerFormat::Int64) => {
                        "i64".to_string()
                    }
                    _ => "i64".to_string(),
                },
                SchemaKind::Type(Type::Number(_)) => "f64".to_string(),
                SchemaKind::Type(Type::Boolean(_)) => "bool".to_string(),
                SchemaKind::Type(Type::Array(_)) => "Vec<serde_json::Value>".to_string(),
                SchemaKind::Type(Type::Object(_)) => {
                    if let Some(title) = &s.schema_data.title {
                        title.clone()
                    } else {
                        "serde_json::Value".to_string()
                    }
                }
                _ => "serde_json::Value".to_string(),
            }
        }
        ReferenceOr::Reference { reference } => extract_ref_name(reference),
    }
}

fn extract_request_body_type(
    rb: &ReferenceOr<openapiv3::RequestBody>,
    components: Option<&openapiv3::Components>,
    inline_structs: &mut Vec<proc_macro2::TokenStream>,
    inline_enums: &mut Vec<InlineEnumInfo>,
) -> Option<String> {
    match rb {
        ReferenceOr::Item(request_body) => {
            for (media_type, mt) in &request_body.content {
                if media_type == "application/json" || media_type == "*/*" {
                    if let Some(schema_ref) = mt.schema.as_ref() {
                        return match schema_ref {
                            ReferenceOr::Item(schema) => match &schema.schema_kind {
                                SchemaKind::Type(Type::Object(obj)) => {
                                    let struct_name =
                                        generate_inline_struct_name(inline_structs, "Request");
                                    let fields = obj.properties.iter().flat_map(|(name, prop)| {
                                            let rust_name = to_valid_rust_field_name(name);
                                            let field_name = format_ident!("{}", rust_name);
                                            let ty = property_type(prop, inline_enums);
                                            let doc = prop
                                                .as_item()
                                                .and_then(|s| s.schema_data.description.as_ref())
                                                .map(|d| quote! { #[doc = #d] });
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
                                SchemaKind::Type(Type::Array(_)) => {
                                    Some("serde_json::Value".to_string())
                                }
                                _ => Some("serde_json::Value".to_string()),
                            },
                            ReferenceOr::Reference { reference } => {
                                Some(extract_ref_name(reference))
                            }
                        };
                    }
                }
            }
            None
        }
        ReferenceOr::Reference { reference } => {
            if let Some(path) = reference.strip_prefix("#/components/requestBodies/") {
                let body_name = to_pascal_case(path);
                if let Some(rb) = components.and_then(|c| c.request_bodies.get(path)) {
                    extract_request_body_type(rb, components, inline_structs, inline_enums)
                } else {
                    Some(body_name)
                }
            } else {
                None
            }
        }
    }
}

fn extract_response_type(
    responses: &openapiv3::Responses,
    components: Option<&openapiv3::Components>,
    inline_structs: &mut Vec<proc_macro2::TokenStream>,
    inline_enums: &mut Vec<InlineEnumInfo>,
    endpoint_name: &str,
) -> Option<String> {
    for (status_code, response_ref) in &responses.responses {
        if !is_success_status_code(status_code) {
            continue;
        }
        if let ReferenceOr::Item(response) = response_ref {
            for (media_type, mt) in &response.content {
                if media_type == "application/json" || media_type == "*/*" {
                    if let Some(schema_ref) = mt.schema.as_ref() {
                        return Some(response_schema_to_string(
                            schema_ref,
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

fn is_success_status_code(status_code: &openapiv3::StatusCode) -> bool {
    match status_code {
        openapiv3::StatusCode::Code(code) => *code >= 200 && *code < 300,
        openapiv3::StatusCode::Range(code) => *code >= 200 && *code < 300,
    }
}

fn response_schema_to_string(
    schema: &ReferenceOr<openapiv3::Schema>,
    components: Option<&openapiv3::Components>,
    inline_structs: &mut Vec<proc_macro2::TokenStream>,
    inline_enums: &mut Vec<InlineEnumInfo>,
    endpoint_name: &str,
) -> String {
    match schema {
        ReferenceOr::Item(schema) => match &schema.schema_kind {
            SchemaKind::Type(Type::Object(obj)) => {
                if obj.properties.is_empty() {
                    return "serde_json::Value".to_string();
                }
                let struct_name = format!("{}{}", to_pascal_case(endpoint_name), "Response");
                let fields = obj.properties.iter().flat_map(|(name, prop)| {
                    let rust_name = to_valid_rust_field_name(name);
                    let field_name = format_ident!("{}", rust_name);
                    let ty = property_type(prop, inline_enums);
                    let doc = prop
                        .as_item()
                        .and_then(|s| s.schema_data.description.as_ref())
                        .map(|d| quote! { #[doc = #d] });
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
            SchemaKind::Type(Type::Array(arr)) => {
                let inner_type = arr
                    .items
                    .as_ref()
                    .map(|item| match item {
                        ReferenceOr::Item(box_schema) => response_schema_to_string_item(
                            &box_schema,
                            components,
                            inline_structs,
                            inline_enums,
                            endpoint_name,
                        ),
                        ReferenceOr::Reference { reference } => extract_ref_name(reference),
                    })
                    .unwrap_or_else(|| "serde_json::Value".to_string());
                format!("Vec<{}>", inner_type)
            }
            SchemaKind::Type(Type::String(_)) => "String".to_string(),
            SchemaKind::Type(Type::Integer(int_type)) => match &int_type.format {
                openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::IntegerFormat::Int32) => {
                    "i32".to_string()
                }
                openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::IntegerFormat::Int64) => {
                    "i64".to_string()
                }
                _ => "i64".to_string(),
            },
            SchemaKind::Type(Type::Number(_)) => "f64".to_string(),
            SchemaKind::Type(Type::Boolean(_)) => "bool".to_string(),
            _ => "serde_json::Value".to_string(),
        },
        ReferenceOr::Reference { reference } => extract_ref_name(reference),
    }
}

fn response_schema_to_string_item(
    schema: &openapiv3::Schema,
    components: Option<&openapiv3::Components>,
    inline_structs: &mut Vec<proc_macro2::TokenStream>,
    inline_enums: &mut Vec<InlineEnumInfo>,
    endpoint_name: &str,
) -> String {
    match &schema.schema_kind {
        SchemaKind::Type(Type::Object(obj)) => {
            if obj.properties.is_empty() {
                return "serde_json::Value".to_string();
            }
            let struct_name = format!("{}{}", to_pascal_case(endpoint_name), "Response");
            let fields = obj.properties.iter().flat_map(|(name, prop)| {
                let rust_name = to_valid_rust_field_name(name);
                let field_name = format_ident!("{}", rust_name);
                let ty = property_type(prop, inline_enums);
                let doc = prop
                    .as_item()
                    .and_then(|s| s.schema_data.description.as_ref())
                    .map(|d| quote! { #[doc = #d] });
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
        SchemaKind::Type(Type::Array(arr)) => {
            let inner_type = arr
                .items
                .as_ref()
                .map(|item| match item {
                    ReferenceOr::Item(box_schema) => response_schema_to_string_item(
                        &box_schema,
                        components,
                        inline_structs,
                        inline_enums,
                        endpoint_name,
                    ),
                    ReferenceOr::Reference { reference } => extract_ref_name(reference),
                })
                .unwrap_or_else(|| "serde_json::Value".to_string());
            format!("Vec<{}>", inner_type)
        }
        SchemaKind::Type(Type::String(_)) => "String".to_string(),
        SchemaKind::Type(Type::Integer(int_type)) => match &int_type.format {
            openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::IntegerFormat::Int32) => {
                "i32".to_string()
            }
            openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::IntegerFormat::Int64) => {
                "i64".to_string()
            }
            _ => "i64".to_string(),
        },
        SchemaKind::Type(Type::Number(_)) => "f64".to_string(),
        SchemaKind::Type(Type::Boolean(_)) => "bool".to_string(),
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

fn is_enum_schema(schema: &ReferenceOr<openapiv3::Schema>) -> bool {
    if let ReferenceOr::Item(s) = schema {
        if let SchemaKind::Type(Type::String(type_s)) = &s.schema_kind {
            return !type_s.enumeration.is_empty();
        }
        if let SchemaKind::Type(Type::Integer(type_i)) = &s.schema_kind {
            return !type_i.enumeration.is_empty();
        }
        if let SchemaKind::Type(Type::Number(type_n)) = &s.schema_kind {
            return !type_n.enumeration.is_empty();
        }
        if let SchemaKind::Type(Type::Boolean(type_b)) = &s.schema_kind {
            return !type_b.enumeration.is_empty();
        }
    }
    false
}

fn extract_enum_values(schema: &openapiv3::Schema) -> Vec<serde_json::Value> {
    match &schema.schema_kind {
        SchemaKind::Type(Type::String(s)) => s
            .enumeration
            .iter()
            .filter_map(|v| v.clone().map(serde_json::Value::String))
            .collect(),
        SchemaKind::Type(Type::Integer(s)) => s
            .enumeration
            .iter()
            .filter_map(|v| v.map(serde_json::Value::from))
            .collect(),
        SchemaKind::Type(Type::Number(s)) => s
            .enumeration
            .iter()
            .filter_map(|v| v.map(serde_json::Value::from))
            .collect(),
        SchemaKind::Type(Type::Boolean(s)) => s
            .enumeration
            .iter()
            .filter_map(|v| v.map(serde_json::Value::Bool))
            .collect(),
        _ => vec![],
    }
}

fn generate_struct_fields(
    schema: &openapiv3::Schema,
    inline_enums: &mut Vec<InlineEnumInfo>,
) -> Vec<proc_macro2::TokenStream> {
    match &schema.schema_kind {
        SchemaKind::Type(Type::Object(obj)) => obj
            .properties
            .iter()
            .flat_map(|(name, prop)| {
                let rust_name = to_valid_rust_field_name(name);
                let field_name = format_ident!("{}", rust_name);
                let ty = property_type(prop, inline_enums);
                let doc = prop
                    .as_item()
                    .and_then(|s| s.schema_data.description.as_ref())
                    .map(|d| quote! { #[doc = #d] });
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
            .collect(),
        _ => vec![],
    }
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
    schema: openapiv3::Schema,
    description: Option<String>,
}

fn property_type(
    prop: &ReferenceOr<Box<openapiv3::Schema>>,
    inline_enums: &mut Vec<InlineEnumInfo>,
) -> proc_macro2::TokenStream {
    match prop {
        ReferenceOr::Item(schema) => {
            if let Some(enum_name) = get_enum_type_name(schema) {
                let schema_ref = schema.as_ref();
                let description = schema_ref.schema_data.description.clone();
                inline_enums.push(InlineEnumInfo {
                    name: enum_name.clone(),
                    schema: schema_ref.clone(),
                    description,
                });
                let ident = format_ident!("{}", enum_name);
                return quote! { Option<#ident> };
            }
            match &schema.schema_kind {
                SchemaKind::Type(Type::String(_)) => quote! { Option<String> },
                SchemaKind::Type(Type::Integer(int_type)) => {
                    let format = match &int_type.format {
                        openapiv3::VariantOrUnknownOrEmpty::Item(
                            openapiv3::IntegerFormat::Int32,
                        ) => "int32",
                        openapiv3::VariantOrUnknownOrEmpty::Item(
                            openapiv3::IntegerFormat::Int64,
                        ) => "int64",
                        _ => "int64",
                    };
                    match format {
                        "int32" => quote! { Option<i32> },
                        "int64" => quote! { Option<i64> },
                        _ => quote! { Option<i64> },
                    }
                }
                SchemaKind::Type(Type::Number(_)) => quote! { Option<f64> },
                SchemaKind::Type(Type::Boolean(_)) => quote! { Option<bool> },
                SchemaKind::Type(Type::Array(arr)) => {
                    let inner = arr.items.as_ref();
                    let inner_ty = if let Some(item_ref) = inner {
                        property_type(item_ref, inline_enums)
                    } else {
                        quote! { String }
                    };
                    quote! { Option<Vec<#inner_ty>> }
                }
                SchemaKind::Type(Type::Object(_)) => {
                    let type_name = schema
                        .schema_data
                        .title
                        .clone()
                        .unwrap_or_else(|| "serde_json::Value".to_string());
                    if type_name == "serde_json::Value" {
                        quote! { Option<serde_json::Value> }
                    } else {
                        let ident = format_ident!("{}", type_name);
                        quote! { Option<#ident> }
                    }
                }
                _ => quote! { Option<serde_json::Value> },
            }
        }
        ReferenceOr::Reference { reference } => {
            let ref_name = extract_ref_name(reference);
            let ident = format_ident!("{}", ref_name);
            quote! { Option<#ident> }
        }
    }
}

fn get_enum_type_name(schema: &openapiv3::Schema) -> Option<String> {
    match &schema.schema_kind {
        SchemaKind::Type(Type::String(s)) if !s.enumeration.is_empty() => Some(
            schema
                .schema_data
                .title
                .clone()
                .unwrap_or_else(|| "StringEnum".to_string()),
        ),
        SchemaKind::Type(Type::Integer(s)) if !s.enumeration.is_empty() => Some(
            schema
                .schema_data
                .title
                .clone()
                .unwrap_or_else(|| "IntEnum".to_string()),
        ),
        SchemaKind::Type(Type::Number(s)) if !s.enumeration.is_empty() => Some(
            schema
                .schema_data
                .title
                .clone()
                .unwrap_or_else(|| "NumberEnum".to_string()),
        ),
        SchemaKind::Type(Type::Boolean(s)) if !s.enumeration.is_empty() => Some(
            schema
                .schema_data
                .title
                .clone()
                .unwrap_or_else(|| "BoolEnum".to_string()),
        ),
        _ => None,
    }
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
