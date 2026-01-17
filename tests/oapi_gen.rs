use oapi_universal_gen::oapi_gen::generate_from_json;
use std::fs;

fn load_fixture(name: &str) -> String {
    fs::read_to_string(format!("tests/fixtures/{}.json", name)).unwrap()
}

#[test]
fn test_minimal_spec() {
    let json = load_fixture("minimal");
    let output = generate_from_json(&json);
    assert!(output.contains("pub trait ApiService: ::oapi_universal_gen::OapiRequester"));
    let parsed = syn::parse_file(&output);
    assert!(
        parsed.is_ok(),
        "Generated code from opencode.json is not valid Rust"
    );
}

#[test]
fn test_string_enum_generation() {
    let json = load_fixture("enums_only");
    let output = generate_from_json(&json);

    assert!(output.contains("pub enum StringStatus"));
    assert!(output.contains("Pending"));
    assert!(output.contains("Active"));
    assert!(output.contains("Completed"));
    assert!(output.contains("Failed"));
    let parsed = syn::parse_file(&output);
    assert!(
        parsed.is_ok(),
        "Generated code from opencode.json is not valid Rust"
    );
}

#[test]
fn test_integer_enum_generation() {
    let json = load_fixture("enums_only");
    let output = generate_from_json(&json);

    assert!(output.contains("pub enum IntegerPriority"));
    assert!(output.contains("V1"));
    assert!(output.contains("V2"));
    assert!(output.contains("V3"));
    assert!(output.contains("V4"));
    assert!(output.contains("V5"));
    let parsed = syn::parse_file(&output);
    assert!(
        parsed.is_ok(),
        "Generated code from opencode.json is not valid Rust"
    );
}

#[test]
fn test_negative_integer_enum() {
    let json = load_fixture("enums_only");
    let output = generate_from_json(&json);

    assert!(output.contains("pub enum NegativeNumbers"));
    assert!(output.contains("VNeg1"));
    assert!(output.contains("VNeg2"));
    assert!(output.contains("VNeg3"));
    let parsed = syn::parse_file(&output);
    assert!(
        parsed.is_ok(),
        "Generated code from opencode.json is not valid Rust"
    );
}

#[test]
fn test_mixed_integer_enum() {
    let json = load_fixture("enums_only");
    let output = generate_from_json(&json);

    assert!(output.contains("pub enum MixedNumbers"));
    assert!(output.contains("VNeg5"));
    assert!(output.contains("V0"));
    assert!(output.contains("V5"));
}

#[test]
fn test_number_enum_generation() {
    let json = load_fixture("enums_only");
    let output = generate_from_json(&json);

    assert!(output.contains("pub enum NumberScore"));
    assert!(output.contains("V0_5"));
    assert!(output.contains("V1_0"));
    assert!(output.contains("V1_5"));
    assert!(output.contains("V2_0"));
    let parsed = syn::parse_file(&output);
    assert!(
        parsed.is_ok(),
        "Generated code from opencode.json is not valid Rust"
    );
}

#[test]
fn test_boolean_enum_generation() {
    let json = load_fixture("enums_only");
    let output = generate_from_json(&json);

    assert!(output.contains("pub enum BooleanFlag"));
    assert!(output.contains("True"));
    assert!(output.contains("False"));
    let parsed = syn::parse_file(&output);
    assert!(
        parsed.is_ok(),
        "Generated code from opencode.json is not valid Rust"
    );
}

#[test]
fn test_string_with_spaces_enum() {
    let json = load_fixture("enums_only");
    let output = generate_from_json(&json);

    assert!(output.contains("pub enum StringWithSpaces"));
    assert!(output.contains("Hello_world"));
    assert!(output.contains("Foo_bar"));
    assert!(output.contains("Baz_qux"));
    let parsed = syn::parse_file(&output);
    assert!(
        parsed.is_ok(),
        "Generated code from opencode.json is not valid Rust"
    );
}

#[test]
fn test_numeric_start_enum() {
    let json = load_fixture("enums_only");
    let output = generate_from_json(&json);

    assert!(output.contains("pub enum NumericStart"));
    assert!(output.contains("V123abc"));
    assert!(output.contains("V42answers"));
    let parsed = syn::parse_file(&output);
    assert!(
        parsed.is_ok(),
        "Generated code from opencode.json is not valid Rust"
    );
}

#[test]
fn test_struct_generation() {
    let json = load_fixture("structs_only");
    let output = generate_from_json(&json);

    assert!(output.contains("pub struct SimpleUser"));
    assert!(output.contains("pub id: Option<String>"));
    assert!(output.contains("pub name: Option<String>"));
    assert!(output.contains("pub age: Option<i32>"));
    let parsed = syn::parse_file(&output);
    assert!(
        parsed.is_ok(),
        "Generated code from opencode.json is not valid Rust"
    );
}

#[test]
fn test_struct_with_all_types() {
    let json = load_fixture("structs_only");
    let output = generate_from_json(&json);

    assert!(output.contains("pub struct UserWithAllTypes"));
    assert!(output.contains("pub string_field: Option<String>"));
    assert!(output.contains("pub int_field: Option<i64>"));
    assert!(output.contains("pub int32_field: Option<i32>"));
    assert!(output.contains("pub int64_field: Option<i64>"));
    assert!(output.contains("pub number_field: Option<f64>"));
    assert!(output.contains("pub bool_field: Option<bool>"));
    assert!(output.contains("pub array_field: Option<Vec<Option<String>>>"));
    let parsed = syn::parse_file(&output);
    assert!(
        parsed.is_ok(),
        "Generated code from opencode.json is not valid Rust"
    );
}

#[test]
fn test_snake_case_field_names() {
    let json = load_fixture("structs_only");
    let output = generate_from_json(&json);

    assert!(output.contains("pub my_field_name: Option<String>"));
    assert!(output.contains("pub another_field: Option<i64>"));
    let parsed = syn::parse_file(&output);
    assert!(
        parsed.is_ok(),
        "Generated code from opencode.json is not valid Rust"
    );
}

#[test]
fn test_kebab_case_to_snake_case() {
    let json = load_fixture("structs_only");
    let output = generate_from_json(&json);

    assert!(output.contains("pub my_field_name: Option<String>"));
    assert!(output.contains("pub another_field: Option<i64>"));
    let parsed = syn::parse_file(&output);
    assert!(
        parsed.is_ok(),
        "Generated code from opencode.json is not valid Rust"
    );
}

#[test]
fn test_nested_object_reference() {
    let json = load_fixture("structs_only");
    let output = generate_from_json(&json);

    assert!(output.contains("pub object_field: Option<SimpleUser>"));
    let parsed = syn::parse_file(&output);
    assert!(
        parsed.is_ok(),
        "Generated code from opencode.json is not valid Rust"
    );
}

#[test]
fn test_from_original_schema() {
    let json = load_fixture("other");
    let output = generate_from_json(&json);

    let parsed = syn::parse_file(&output);
    assert!(
        parsed.is_ok(),
        "Generated code from opencode.json is not valid Rust"
    );
    assert!(output.contains("pub trait ApiService: ::oapi_universal_gen::OapiRequester"));
    assert!(output.contains("fn v1_task_task_get"));
    assert!(output.contains("task: String"));
    assert!(output.contains("fn v1_tasks_get(&self)"));
    assert!(output.contains("pub struct Task"));
    assert!(output.contains("pub state: Option<String>"));
}

#[test]
fn test_api_put_method() {
    let json = load_fixture("full_api");
    let output = generate_from_json(&json);

    assert!(!output.contains("fn users_user_id_put(&self);"));
    let parsed = syn::parse_file(&output);
    assert!(
        parsed.is_ok(),
        "Generated code from opencode.json is not valid Rust"
    );
}

#[test]
fn test_api_delete_method() {
    let json = load_fixture("full_api");
    let output = generate_from_json(&json);

    assert!(!output.contains("fn users_user_id_delete(&self);"));
    let parsed = syn::parse_file(&output);
    assert!(
        parsed.is_ok(),
        "Generated code from opencode.json is not valid Rust"
    );
}

#[test]
fn test_comprehensive_api_generates_valid_rust() {
    let json = load_fixture("comprehensive_api");
    let output = generate_from_json(&json);
    eprintln!(
        "Generated code:\n{}",
        output
            .lines()
            .filter(|l| l.contains("fn "))
            .take(20)
            .collect::<Vec<_>>()
            .join("\n")
    );
    let parsed = syn::parse_file(&output);
    assert!(
        parsed.is_ok(),
        "Generated code is not valid Rust: {:?}",
        parsed.err()
    );
}

#[test]
fn test_empty_components() {
    let json = r#"{
        "openapi": "3.1.0",
        "info": {"title": "Test", "version": "1.0.0"},
        "paths": {}
    }"#;
    let output = generate_from_json(json);
    assert!(output.contains("pub trait ApiService: ::oapi_universal_gen::OapiRequester"));
    assert!(!output.contains("pub enum"));
    assert!(!output.contains("pub struct"));
    let parsed = syn::parse_file(&output);
    assert!(
        parsed.is_ok(),
        "Generated code from opencode.json is not valid Rust"
    );
}

#[test]
fn test_snake_case_serde_attr() {
    let json = load_fixture("enums_only");
    let output = generate_from_json(&json);

    assert!(output.contains(r#"#[serde(rename_all = "snake_case")]"#));
    let parsed = syn::parse_file(&output);
    assert!(
        parsed.is_ok(),
        "Generated code from opencode.json is not valid Rust"
    );
}

#[test]
fn test_struct_serde_attr() {
    let json = load_fixture("structs_only");
    let output = generate_from_json(&json);

    assert!(output.contains(r#"#[serde(default, skip_serializing_if = "Option::is_none")]"#));
    let parsed = syn::parse_file(&output);
    assert!(
        parsed.is_ok(),
        "Generated code from opencode.json is not valid Rust"
    );
}

#[test]
fn test_doc_comments_generated() {
    let json = r#"{
        "openapi": "3.1.0",
        "info": {"title": "Test", "version": "1.0.0"},
        "paths": {
            "/test": {
                "get": {
                    "operationId": "test_get",
                    "description": "This is a test endpoint",
                    "responses": {"200": {"description": "ok"}}
                }
            }
        },
        "components": {
            "schemas": {
                "TestSchema": {
                    "type": "object",
                    "description": "A test schema description",
                    "properties": {
                        "name": {"type": "string", "description": "The name field"}
                    },
                    "required": []
                }
            }
        }
    }"#;
    let output = generate_from_json(json);

    assert!(output.contains("///A test schema description"));
    assert!(output.contains("///The name field"));

    let parsed = syn::parse_file(&output);
    assert!(
        parsed.is_ok(),
        "Generated code from opencode.json is not valid Rust"
    );
}

#[test]
fn test_integration_with_real_schema() {
    let json = load_fixture("other");
    let output = generate_from_json(&json);

    let parsed = syn::parse_file(&output);
    assert!(
        parsed.is_ok(),
        "Generated code from other.json is not valid Rust"
    );
}

#[test]
fn test_opencode() {
    let json = load_fixture("opencode");
    let output = generate_from_json(&json);
    eprintln!("{}", output);
    let parsed = syn::parse_file(&output);
    assert!(
        parsed.is_ok(),
        "Generated code from opencode.json is not valid Rust"
    );
}

#[test]
fn test_any_of_schema_generation() {
    let json = load_fixture("opencode");
    let output = generate_from_json(&json);

    assert!(
        output.contains("pub enum Message"),
        "Should generate Message enum"
    );
    assert!(
        output.contains("UserMessage(UserMessage)"),
        "Should have UserMessage tuple variant"
    );
    assert!(
        output.contains("AssistantMessage(AssistantMessage)"),
        "Should have AssistantMessage tuple variant"
    );

    let parsed = syn::parse_file(&output);
    assert!(
        parsed.is_ok(),
        "Generated code is not valid Rust: {:?}",
        parsed.err()
    );
}

#[test]
fn test_path_params_urlencoded() {
    let json = load_fixture("comprehensive_api");
    let output = generate_from_json(&json);
    assert!(output.contains("::urlencode"));
    assert!(output.contains(r#""/session/{}/command""#));
}

#[test]
fn test_query_params_in_url() {
    let json = load_fixture("comprehensive_api");
    let output = generate_from_json(&json);
    assert!(output.contains("UrlBuilder::build"));
    assert!(output.contains("query_params"));
}

#[test]
fn test_api_get_method() {
    let json = load_fixture("full_api");
    let output = generate_from_json(&json);
    let method_lines: Vec<_> = output
        .lines()
        .filter(|l| l.contains("fn users") || l.contains("fn items"))
        .collect();
    eprintln!(
        "Generated methods ({}):\n{}",
        method_lines.len(),
        method_lines.join("\n")
    );
    eprintln!("Output sample:\n{}", &output[..500]);

    assert!(output.contains("fn users_get(&self)"));
    assert!(
        output.contains("user_id: String"),
        "Expected users_user_id_get with user_id param"
    );
    assert!(output.contains("id: String"));
    assert!(output.contains("::urlencode"));
    let parsed = syn::parse_file(&output);
    assert!(
        parsed.is_ok(),
        "Generated code from opencode.json is not valid Rust"
    );
}

#[test]
fn test_request_body_handling() {
    let json = load_fixture("comprehensive_api");
    let output = generate_from_json(&json);
    assert!(output.contains("create_request_with_body"));
    assert!(output.contains("CommandRequest"));
}

#[test]
fn test_response_body_parsing() {
    let json = load_fixture("comprehensive_api");
    let output = generate_from_json(&json);
    assert!(output.contains("serde_json::from_str"));
    assert!(output.contains("CommandResult"));
}

#[test]
fn test_delete_method_returns_unit() {
    let json = load_fixture("comprehensive_api");
    let output = generate_from_json(&json);
    assert!(output.contains("fn users_user_id_delete"));
    assert!(output.contains("Ok(())"));
}

#[test]
fn test_list_items_with_query_params() {
    let json = load_fixture("comprehensive_api");
    let output = generate_from_json(&json);
    assert!(output.contains("fn items_get"));
    assert!(output.contains("limit: Option<i32>"));
    assert!(output.contains("offset: Option<i32>"));
}

#[test]
fn test_comprehensive_api_has_all_methods() {
    let json = load_fixture("comprehensive_api");
    let output = generate_from_json(&json);
    assert!(output.contains("fn session_sessionid_command_post"));
    assert!(output.contains("fn users_user_id_get"));
    assert!(output.contains("fn users_user_id_delete"));
    assert!(output.contains("fn items_get"));
}

#[test]
fn test_parameter_docs_generated() {
    let json = load_fixture("with_parameters");
    let output = generate_from_json(&json);

    assert!(output.contains("Maximum number of users to return"));
    assert!(output.contains("Number of users to skip"));
    assert!(output.contains("Search query for filtering users"));
    assert!(output.contains("The user ID"));
    assert!(output.contains("Include additional user details"));

    let parsed = syn::parse_file(&output);
    assert!(
        parsed.is_ok(),
        "Generated code is not valid Rust: {:?}",
        parsed.err()
    );
}
