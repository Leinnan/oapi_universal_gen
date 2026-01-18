//! A utility library for HTTP requests and URL building.
//!
//! This library provides helper traits for making HTTP requests using the `ehttp` crate
//! and a builder pattern for constructing URLs with query parameters.
//!
//! # Features
//!
//! - **HTTP Request Helpers**: Trait implementations for making HTTP requests (GET, POST, PUT, PATCH, DELETE)
//! - **Response Helpers**: Trait implementations for inspecting HTTP responses
//! - **URL Builder**: A builder for constructing URLs with properly encoded query parameters
//!
//! # Example
//!
//! ```ignore
//! use oapi_universal_gen::{RequestHelperExt, UrlBuilder};
//!
//! let url = UrlBuilder::new("https://api.example.com/users")
//!     .with_query("page", "1")
//!     .with_query("limit", "10")
//!     .to_uri();
//!
//! let request = ehttp::Request::get_request("https://api.example.com/users").unwrap();
//! ```

use serde::Serialize;
use std::borrow::Borrow;

/// Module containing HTTP request/response helper implementations for various backends.
pub mod backends;

#[cfg(feature = "oapi_gen")]
/// Module containing OpenAPI generator utilities.
pub mod oapi_gen;

/// Represents the HTTP request method type.
///
/// This enum defines the supported HTTP methods that can be used
/// when creating requests through the [`OapiRequester`] trait.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RequestType {
    /// HTTP GET request - retrieves a resource without a body
    Get,
    /// HTTP POST request - creates a new resource with a body
    Post,
    /// HTTP PUT request - replaces an existing resource with a body
    Put,
    /// HTTP PATCH request - partially updates a resource with a body
    Patch,
    /// HTTP DELETE request - removes a resource
    Delete,
}

#[derive(Debug)]
pub enum OapiRequesterError {
    ClientOrServerError(u16),
    ResponseContentError,
    SerializationError,
}

impl std::fmt::Display for OapiRequesterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OapiRequesterError::ClientOrServerError(status) => {
                write!(f, "HTTP error: {}", status)
            }
            OapiRequesterError::ResponseContentError => {
                write!(f, "Failed to get response content")
            }
            OapiRequesterError::SerializationError => {
                write!(f, "JSON serialization/deserialization error")
            }
        }
    }
}

impl std::error::Error for OapiRequesterError {}

pub trait FromHttpError {
    fn from_http_error(status_code: u16) -> Self;
}

impl<T: From<OapiRequesterError>> FromHttpError for T {
    fn from_http_error(status_code: u16) -> Self {
        OapiRequesterError::ClientOrServerError(status_code).into()
    }
}

impl From<serde_json::Error> for OapiRequesterError {
    fn from(_e: serde_json::Error) -> Self {
        OapiRequesterError::SerializationError
    }
}

impl From<OapiRequesterError> for String {
    fn from(e: OapiRequesterError) -> Self {
        e.to_string()
    }
}

/// A trait for creating HTTP requests with a configuration.
///
/// This trait provides a unified interface for creating HTTP requests
/// based on a configuration that provides the base URL. Implementors
/// must define the associated types for the requester, error type,
/// and configuration.
///
/// # Associated Types
///
/// - `RequesterErrorType`: The error type returned when creating requests
/// - `Requester`: The request helper type implementing [`RequestHelperExt`]
/// - `Configuration`: The configuration type implementing [`ConfigExt`]
///
/// # Example
///
/// ```ignore
/// use oapi_universal_gen::{OapiRequester, RequestType, ConfigExt, RequestHelperExt};
///
/// struct MyClient {
///     config: MyConfig,
/// }
///
/// impl OapiRequester for MyClient {
///     type RequesterErrorType = String;
///     type Requester = ehttp::Request;
///     type Configuration = MyConfig;
///
///     fn get_configuration(&self) -> &Self::Configuration {
///         &self.config
///     }
/// }
///
/// struct MyConfig;
///
/// impl ConfigExt for MyConfig {
///     fn get_base_url(&self) -> &str {
///         "https://api.example.com"
///     }
/// }
/// ```
pub trait OapiRequester: Sized {
    /// The error type returned when creating requests.
    type RequesterErrorType;

    /// The request helper type that implements [`RequestHelperExt`].
    ///
    /// This type is used to construct and send HTTP requests.
    type Requester: RequestHelperExt<ResponseError = Self::RequesterErrorType>;

    /// The configuration type that provides base URL and other settings.
    ///
    /// This type must implement [`ConfigExt`] to provide the base URL.
    type Configuration: ConfigExt;

    /// Returns a reference to the configuration.
    ///
    /// # Returns
    ///
    /// A reference to the implementor's configuration
    fn get_configuration(&self) -> &Self::Configuration;

    /// Creates an HTTP request for the specified path using the default method.
    ///
    /// This is a convenience method that calls [`Self::create_request_with_body`]
    /// with `None` as the body. Use this for GET and DELETE requests that
    /// don't require a request body.
    ///
    /// # Arguments
    ///
    /// * `request_type` - The HTTP method to use
    /// * `path` - The API path to append to the base URL
    ///
    /// # Returns
    ///
    /// A Result containing the request or an error
    fn create_request(
        &self,
        request_type: RequestType,
        path: impl AsRef<str>,
    ) -> Result<Self::Requester, Self::RequesterErrorType> {
        self.create_request_with_body(request_type, path, &None::<String>)
    }

    /// Creates an HTTP request with a body for the specified path.
    ///
    /// This method constructs a URL by joining the base URL from the
    /// configuration with the provided path, then creates the appropriate
    /// HTTP request based on the request type.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The body type, which must be serializable to JSON
    ///
    /// # Arguments
    ///
    /// * `request_type` - The HTTP method to use
    /// * `path` - The API path to append to the base URL
    /// * `body` - A reference to the serializable body
    ///
    /// # Returns
    ///
    /// A Result containing the request or an error
    fn create_request_with_body<T>(
        &self,
        request_type: RequestType,
        path: impl AsRef<str>,
        body: &T,
    ) -> Result<Self::Requester, Self::RequesterErrorType>
    where
        T: ?Sized + serde::Serialize,
    {
        let uri = UrlBuilder::new(self.get_configuration().get_base_url()).join(path);
        let mut request = match request_type {
            RequestType::Get => Self::Requester::get_request(uri),
            RequestType::Post => Self::Requester::post_request(uri, body),
            RequestType::Put => Self::Requester::put_request(uri, body),
            RequestType::Patch => Self::Requester::patch_request(uri, body),
            RequestType::Delete => Self::Requester::delete_request(uri, body),
        }?;
        self.get_configuration().add_default_headers(&mut request);
        Ok(request)
    }
}

/// A trait for providing configuration settings.
///
/// This trait provides access to configuration properties needed
/// for making HTTP requests, such as the base URL for the API.
///
/// # Example
///
/// ```ignore
/// use oapi_universal_gen::ConfigExt;
///
/// struct ApiConfig {
///     base_url: String,
/// }
///
/// impl ConfigExt for ApiConfig {
///     fn get_base_url(&self) -> &str {
///         &self.base_url
///     }
/// }
/// ```
pub trait ConfigExt {
    /// Returns the base URL for API requests.
    ///
    /// This URL is used as the foundation when constructing
    /// request URLs by joining it with the provided path.
    ///
    /// # Returns
    ///
    /// The base URL as a string slice
    fn get_base_url(&self) -> &str;

    /// Adds the default headers for API requests.
    ///
    /// These headers are added to every request made using this configuration.
    ///
    /// # Returns
    ///
    /// * `requester` - A mutable reference to the request helper
    #[allow(unused_variables)]
    fn add_default_headers(&self, requester: &mut impl RequestHelperExt) {}
}

/// A trait providing helper methods for inspecting HTTP responses.
///
/// This trait extends `ehttp::Response` with convenient methods to extract
/// response metadata such as content type, content body, and error status checks.
pub trait ResponseHelperExt: Sized {
    fn response_content_type(&self) -> String;

    fn response_content(self) -> impl Future<Output = Option<String>>;

    fn status(&self) -> u16;

    fn is_client_error(&self) -> bool;

    fn is_server_error(&self) -> bool;
}
/// A trait providing helper methods for constructing and sending HTTP requests.
///
/// This trait provides a fluent API for building HTTP requests with headers
/// and convenience methods for common HTTP methods (GET, POST, PUT, PATCH, DELETE).
///
/// # Associated Types
///
/// - `Response`: The type of the response returned by the request
/// - `Error`: The type of error that can occur when sending the request
pub trait RequestHelperExt: Sized {
    /// The type of the response returned by this request.
    type Response: ResponseHelperExt;

    /// The type of error that can occur when sending this request.
    type ResponseError;

    /// Adds a header to the request.
    ///
    /// # Arguments
    ///
    /// * `key` - The header name
    /// * `value` - The header value
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    fn with_header(&mut self, key: impl ToString, value: impl ToString) -> &mut Self;

    /// Sends the request asynchronously.
    ///
    /// # Returns
    ///
    /// A future that resolves to the response or an error
    fn send_request(self) -> impl Future<Output = Result<Self::Response, Self::ResponseError>>;

    /// Creates a GET request for the specified URL.
    ///
    /// # Type Parameters
    ///
    /// * `Z` - A type that can be converted to a String (e.g., `&str`, `String`)
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to request
    ///
    /// # Returns
    ///
    /// A Result containing the request or an error
    fn get_request<Z>(url: Z) -> Result<Self, Self::ResponseError>
    where
        Z: ToString;

    /// Creates a POST request with a JSON body for the specified URL.
    ///
    /// # Type Parameters
    ///
    /// * `Y` - A serializable type for the request body
    /// * `Z` - A type that can be converted to a String for the URL
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to request
    /// * `body` - A reference to the serializable body
    ///
    /// # Returns
    ///
    /// A Result containing the request or an error
    fn post_request<Y, Z>(url: Z, body: &Y) -> Result<Self, Self::ResponseError>
    where
        Y: ?Sized + Serialize,
        Z: ToString;

    /// Creates a PUT request with a JSON body for the specified URL.
    ///
    /// # Type Parameters
    ///
    /// * `Y` - A serializable type for the request body
    /// * `Z` - A type that can be converted to a String for the URL
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to request
    /// * `body` - A reference to the serializable body
    ///
    /// # Returns
    ///
    /// A Result containing the request or an error
    fn put_request<Y, Z>(url: Z, body: &Y) -> Result<Self, Self::ResponseError>
    where
        Y: ?Sized + Serialize,
        Z: ToString;

    /// Creates a PATCH request with a JSON body for the specified URL.
    ///
    /// # Type Parameters
    ///
    /// * `Y` - A serializable type for the request body
    /// * `Z` - A type that can be converted to a String for the URL
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to request
    /// * `body` - A reference to the serializable body
    ///
    /// # Returns
    ///
    /// A Result containing the request or an error
    fn patch_request<Y, Z>(url: Z, body: &Y) -> Result<Self, Self::ResponseError>
    where
        Y: ?Sized + Serialize,
        Z: ToString;

    /// Creates a DELETE request with a JSON body for the specified URL.
    ///
    /// # Type Parameters
    ///
    /// * `Y` - A serializable type for the request body
    /// * `Z` - A type that can be converted to a String for the URL
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to request
    /// * `body` - A reference to the serializable body
    ///
    /// # Returns
    ///
    /// A Result containing the request or an error
    fn delete_request<Y, Z>(url: Z, body: &Y) -> Result<Self, Self::ResponseError>
    where
        Y: ?Sized + Serialize,
        Z: ToString;
}

/// A builder for constructing URLs with query parameters.
///
/// This struct provides a fluent API for building URLs by appending
/// query parameters. All query parameters are properly URL-encoded.
///
/// # Fields
///
/// - `path`: The base URL path
/// - `query`: A vector of query parameter key-value pairs
///
/// # Example
///
/// ```ignore
/// use oapi_universal_gen::UrlBuilder;
///
/// let url = UrlBuilder::new("https://api.example.com/users")
///     .with_query("page", "1")
///     .with_query("search", "john doe")
///     .to_uri();
/// ```
///
/// # Output
///
/// ```text
/// https://api.example.com/users?page=1&search=john+doe
/// ```
pub struct UrlBuilder {
    /// The base URL path
    path: String,
    /// Query parameter key-value pairs
    query: Vec<(String, String)>,
}

impl ToString for UrlBuilder {
    fn to_string(&self) -> String {
        self.to_uri()
    }
}

impl UrlBuilder {
    /// Creates a new UrlBuilder with the specified base path.
    ///
    /// # Arguments
    ///
    /// * `path` - The base URL path (can be any type implementing `ToString`)
    ///
    /// # Example
    ///
    /// ```ignore
    /// let builder = UrlBuilder::new("https://example.com/api");
    /// ```
    pub fn new(path: impl ToString) -> Self {
        UrlBuilder {
            path: path.to_string(),
            query: vec![],
        }
    }

    /// Joins the current path with a new path segment.
    ///
    /// # Arguments
    ///
    /// * `path` - The path segment to join (can be any type implementing `AsRef<str>`)
    ///
    /// # Example
    ///
    /// ```ignore
    /// let builder = UrlBuilder::new("https://example.com/api");
    /// let builder = builder.join("users");
    /// ```
    pub fn join(mut self, path: impl AsRef<str>) -> Self {
        let path_str = path.as_ref();

        // Normalize base: remove all trailing slashes to avoid duplicates
        while self.path.ends_with('/') {
            self.path.pop();
        }

        // Add single separator
        self.path.push('/');

        // Normalize appended path: remove all leading slashes
        self.path.push_str(path_str.trim_start_matches('/'));

        self
    }

    /// Builds a URL string from a path and an iterator of query parameters.
    ///
    /// This is a convenience function that creates a UrlBuilder, adds all
    /// query parameters, and returns the resulting URL string.
    ///
    /// # Type Parameters
    ///
    /// * `I` - An iterator of query parameter tuples
    /// * `T` - The item type borrowed from the iterator
    /// * `K` - The key type (must implement `AsRef<str>`)
    /// * `V` - The value type (must implement `AsRef<str>`)
    ///
    /// # Arguments
    ///
    /// * `path` - The base URL path
    /// * `query` - An iterator of (key, value) tuples
    ///
    /// # Example
    ///
    /// ```ignore
    /// let url = UrlBuilder::build(
    ///     "https://example.com",
    ///     &[("page", "1"), ("limit", "10")]
    /// );
    /// ```
    pub fn build<I, T, K, V>(path: impl ToString, query: I) -> String
    where
        I: IntoIterator<Item = T>,
        T: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let mut p = Self::new(path);
        for item in query {
            let (k, v) = item.borrow();
            p = p.with_query(k.as_ref(), v.as_ref());
        }
        p.to_uri()
    }

    /// Adds a query parameter to the URL.
    ///
    /// This method appends a key-value pair to the list of query parameters.
    /// Both the key and value are URL-encoded automatically.
    ///
    /// # Arguments
    ///
    /// * `key` - The query parameter name
    /// * `value` - The query parameter value
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    ///
    /// # Example
    ///
    /// ```ignore
    /// let url = UrlBuilder::new("https://example.com")
    ///     .with_query("name", "John Smith")
    ///     .with_query("filter", "active")
    ///     .to_uri();
    /// ```
    pub fn with_query(mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        self.query
            .push((key.as_ref().to_string(), value.as_ref().to_string()));
        self
    }

    /// Converts the UrlBuilder to a URI string.
    ///
    /// This method constructs the final URL by combining the base path
    /// with the query parameters, properly encoded and joined with `?`.
    ///
    /// # Returns
    ///
    /// The complete URL as a String
    ///
    /// # Example
    ///
    /// ```ignore
    /// let builder = UrlBuilder::new("https://api.example.com");
    /// builder.with_query("page", "2");
    /// let uri = builder.to_uri();
    /// assert_eq!(uri, "https://api.example.com?page=2");
    /// ```
    pub fn to_uri(&self) -> String {
        let mut path = self.path.clone();
        if !self.query.is_empty() {
            let mut serializer = url::form_urlencoded::Serializer::new(String::new());
            for (k, v) in &self.query {
                serializer.append_pair(k, v);
            }
            path.push('?');
            path.push_str(&serializer.finish());
        }
        path
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}
