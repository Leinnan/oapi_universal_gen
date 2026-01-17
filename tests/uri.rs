use oapi_universal_gen::UrlBuilder;

/// Tests that UrlBuilder correctly constructs URLs with query parameters.
#[test]
fn test_build_url() {
    let url = UrlBuilder::build(
        "https://example.com",
        &[("param1", "value1"), ("param2", "value2")],
    );
    assert_eq!(url, "https://example.com?param1=value1&param2=value2");
}

/// Tests that UrlBuilder correctly URL-encodes spaces in query parameters.
#[test]
fn test_build_url_with_space() {
    let url = UrlBuilder::build(
        "https://example.com",
        &[("param1 dsa", "value1"), ("param2", "value2")],
    );
    assert_eq!(url, "https://example.com?param1+dsa=value1&param2=value2");
}

#[test]
fn test_building_url() {
    let url = UrlBuilder::new("https://example.com/testings");
    let url2 = UrlBuilder::new("https://example.com").join("testings");
    assert_eq!(url.to_uri(), url2.to_string());
}

#[test]
fn test_building_url_with_headers() {
    let url = UrlBuilder::new("https://example.com/testings")
        .with_query("first", "value")
        .join("second");
    let url2 = UrlBuilder::new("https://example.com")
        .with_query("first", "value")
        .join("testings")
        .join("second");
    assert_eq!(url.to_uri(), url2.to_string());
}

#[test]
#[cfg(feature = "oapi_gen")]
fn test_building_url_with_openapi() {
    oapi_universal_gen::oapi_gen::generate_openapi_spec();
}
