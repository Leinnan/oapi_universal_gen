use oapi_universal_gen::UrlBuilder;

#[test]
fn test_join_no_slashes() {
    let builder = UrlBuilder::new("https://example.com");
    let uri = builder.join("api").to_uri();
    assert_eq!(uri, "https://example.com/api");
}

#[test]
fn test_join_base_slash() {
    let builder = UrlBuilder::new("https://example.com/");
    let uri = builder.join("api").to_uri();
    assert_eq!(uri, "https://example.com/api");
}

#[test]
fn test_join_path_slash() {
    let builder = UrlBuilder::new("https://example.com");
    let uri = builder.join("/api").to_uri();
    assert_eq!(uri, "https://example.com/api");
}

#[test]
fn test_join_both_slash() {
    let builder = UrlBuilder::new("https://example.com/");
    let uri = builder.join("/api").to_uri();
    assert_eq!(uri, "https://example.com/api");
}

#[test]
fn test_join_empty_path() {
    let builder = UrlBuilder::new("https://example.com");
    let uri = builder.join("").to_uri();
    // This is debatable. "https://example.com/" is probably expected if we treat it as a directory join.
    // But commonly APIs might treat trailing slash differently.
    // Given the current implementation: `if !ends_with('/') push('/')`, then append path.
    // "https://example.com" + "" -> "https://example.com/"
    assert_eq!(uri, "https://example.com/");
}

#[test]
fn test_join_base_slash_empty_path() {
    let builder = UrlBuilder::new("https://example.com/");
    let uri = builder.join("").to_uri();
    assert_eq!(uri, "https://example.com/");
}

#[test]
fn test_join_multiple_slashes_in_path() {
    // "https://example.com" + "//api" -> "https://example.com//api" currently
    let builder = UrlBuilder::new("https://example.com");
    let uri = builder.join("//api").to_uri();
    // If we want strict single slash, this should fail if I assert single slash.
    // Let's assume we want to fix it.
    assert_eq!(uri, "https://example.com/api");
}

#[test]
fn test_join_multiple_slashes_in_base() {
    // "https://example.com//" + "api" -> "https://example.com//api" currently
    let builder = UrlBuilder::new("https://example.com//");
    let uri = builder.join("api").to_uri();
    assert_eq!(uri, "https://example.com/api");
}

#[test]
fn test_chained_joins() {
    let builder = UrlBuilder::new("https://example.com");
    let uri = builder
        .join("/api/")
        .join("/v1/")
        .join("/users/")
        .join("/list")
        .to_uri();
    assert_eq!(uri, "https://example.com/api/v1/users/list");
}
