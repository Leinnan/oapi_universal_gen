//! Utility functions for string transformations and name conversions.
//!
//! This module provides functions for converting OpenAPI identifiers
//! to valid Rust naming conventions including PascalCase for types
//! and snake_case for fields and variables.

use quote::{format_ident, quote};

/// Converts a string to PascalCase.
///
/// This is used for generating type names (structs, enums) from
/// OpenAPI schema names.
///
/// # Arguments
///
/// * `input` - The input string to convert
///
/// # Returns
///
/// A PascalCase string suitable for use as a Rust type name
pub fn to_pascal_case(input: &str) -> String {
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

/// Converts a string to snake_case.
///
/// This is used for generating field names and variable names
/// that are valid Rust identifiers.
///
/// # Arguments
///
/// * `name` - The input string to convert
///
/// # Returns
///
/// A snake_case string suitable for use as a Rust field or variable name
pub fn to_snake_case(name: &str) -> String {
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

/// Converts a string to a valid Rust field name.
///
/// This function handles Rust keywords by appending an underscore,
/// replaces special characters, and converts to snake_case.
///
/// # Arguments
///
/// * `name` - The input string to convert
///
/// # Returns
///
/// A valid Rust field name
pub fn to_valid_rust_field_name(name: &str) -> String {
    const RUST_KEYWORDS: &[&str] = &[
        "type", "mod", "use", "fn", "struct", "enum", "impl", "trait", "const", "static", "let",
        "mut", "ref", "return", "if", "else", "match", "for", "while", "loop", "break", "continue",
        "self", "super", "crate", "async", "await", "move", "where", "pub", "priv", "unsafe",
        "extern", "dyn", "abstract", "override", "final", "in",
    ];

    let mut result = name.replace('$', "dollar_");

    if RUST_KEYWORDS.contains(&result.as_str()) {
        result = format!("{}_field", result);
    }

    to_snake_case(&result)
}

/// Converts an enum value to a valid Rust variant name.
///
/// Handles strings, numbers, and booleans to produce valid enum
/// variant identifiers.
///
/// # Arguments
///
/// * `value` - The JSON value to convert
///
/// # Returns
///
/// A `proc_macro2::Ident` representing the valid variant name
pub fn enum_value_to_variant_name(value: &serde_json::Value) -> proc_macro2::Ident {
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

/// Converts a string to a valid Rust enum variant name.
///
/// Converts to PascalCase and handles special characters by
/// replacing them with underscores.
pub fn to_valid_enum_variant(name: &str) -> String {
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

/// Extracts the schema name from an OpenAPI reference.
///
/// # Arguments
///
/// * `reference` - The reference string (e.g., "#/components/schemas/User")
///
/// # Returns
///
/// The extracted schema name in PascalCase
pub fn extract_ref_name(reference: &str) -> String {
    if let Some(path) = reference.strip_prefix("#/components/schemas/") {
        to_pascal_case(path)
    } else {
        "serde_json::Value".to_string()
    }
}

/// Checks if a field is required based on the required array.
///
/// # Arguments
///
/// * `name` - The field name to check
/// * `required` - The list of required field names
///
/// # Returns
///
/// `true` if the field is required
pub fn is_field_required(name: &str, required: &[String]) -> bool {
    required.contains(&name.to_string())
}

/// Converts a type string to a token stream.
///
/// Handles both simple identifiers and complex types with generics.
///
/// # Arguments
///
/// * `ty` - The type string to convert
///
/// # Returns
///
/// A `proc_macro2::TokenStream` representing the type
pub fn type_to_tokens(ty: &str) -> proc_macro2::TokenStream {
    if ty.contains("::") || ty.contains('<') {
        ty.parse().unwrap_or_else(|_| quote! { serde_json::Value })
    } else {
        let ident = format_ident!("{}", ty);
        quote! { #ident }
    }
}

/// Converts a path to a valid method name.
///
/// Removes leading slashes, replaces path parameters with underscored
/// names, and normalizes the resulting string.
pub fn clean_path_for_method_name(path: &str) -> String {
    let mut result = path.trim_start_matches('/').to_lowercase();

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

/// Returns the suffix for a method based on HTTP method type.
pub fn method_suffix(method: &crate::RequestType) -> String {
    match method {
        crate::RequestType::Get => "get",
        crate::RequestType::Post => "post",
        crate::RequestType::Put => "put",
        crate::RequestType::Patch => "patch",
        crate::RequestType::Delete => "delete",
    }
    .to_string()
}
