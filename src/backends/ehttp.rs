//! Implementations of request/response helper traits for the `ehttp` crate.
//!
//! This module provides concrete implementations of [`ResponseHelperExt`] and
//! [`RequestHelperExt`] for `ehttp::Response` and `ehttp::Request` respectively.

use crate::{RequestHelperExt, ResponseHelperExt};
/// Implementation of [`ResponseHelperExt`] for [`ehttp::Response`].
impl ResponseHelperExt for ehttp::Response {
    /// Returns the Content-Type header value of the response.
    ///
    /// Defaults to `"application/json"` if the header is not present.
    fn response_content_type(&self) -> String {
        self.headers
            .get("Content-Type")
            .unwrap_or(&"application/json".to_string())
            .to_string()
    }

    /// Returns the response body as a String, if present.
    ///
    /// Attempts to decode the response body as text.
    async fn response_content(self) -> Option<String> {
        self.text().map(|text| text.to_string())
    }

    /// Checks if the response indicates a client error (4xx status codes).
    fn is_client_error(&self) -> bool {
        self.status >= 400 && self.status < 500
    }

    /// Checks if the response indicates a server error (5xx status codes).
    fn is_server_error(&self) -> bool {
        self.status >= 500 && self.status < 600
    }
}

/// Implementation of [`RequestHelperExt`] for [`ehttp::Request`].
///
/// This implementation provides HTTP request methods for the `ehttp` library,
/// supporting GET, POST, PUT, PATCH, and DELETE requests with JSON body serialization.
impl RequestHelperExt for ehttp::Request {
    /// The response type for ehttp requests.
    type Response = ehttp::Response;

    /// The error type for ehttp requests.
    type ResponseError = ehttp::Error;

    /// Adds a header to the request.
    ///
    /// Inserts or updates the specified header key-value pair.
    fn with_header(&mut self, key: impl ToString, value: impl ToString) -> &mut Self {
        self.headers.insert(key, value);
        self
    }

    /// Sends the request asynchronously.
    ///
    /// Uses `ehttp::fetch_async` to perform the network request.
    async fn send_request(self) -> Result<Self::Response, Self::ResponseError> {
        ehttp::fetch_async(self).await
    }

    /// Creates a GET request for the specified URL.
    fn get_request<Z>(url: Z) -> Result<Self, Self::ResponseError>
    where
        Z: ToString,
    {
        Ok(ehttp::Request::get(url))
    }

    /// Creates a POST request with a JSON-encoded body.
    ///
    /// The body is serialized to JSON using `serde_json`.
    fn post_request<Y, Z>(url: Z, body: &Y) -> Result<Self, Self::ResponseError>
    where
        Y: ?Sized + serde::Serialize,
        Z: ToString,
    {
        let body = serde_json::to_vec(body).map_err(|e| ehttp::Error::from(e.to_string()))?;
        Ok(ehttp::Request::post(url, body))
    }

    /// Creates a PUT request with a JSON-encoded body.
    ///
    /// The body is serialized to JSON using `serde_json`.
    fn put_request<Y, Z>(url: Z, body: &Y) -> Result<Self, Self::ResponseError>
    where
        Y: ?Sized + serde::Serialize,
        Z: ToString,
    {
        let body = serde_json::to_vec(body).map_err(|e| ehttp::Error::from(e.to_string()))?;
        Ok(ehttp::Request {
            method: "PUT".to_string(),
            ..ehttp::Request::post(url, body)
        })
    }

    /// Creates a PATCH request with a JSON-encoded body.
    ///
    /// The body is serialized to JSON using `serde_json`.
    fn patch_request<Y, Z>(url: Z, body: &Y) -> Result<Self, Self::ResponseError>
    where
        Y: ?Sized + serde::Serialize,
        Z: ToString,
    {
        let body = serde_json::to_vec(body).map_err(|e| ehttp::Error::from(e.to_string()))?;
        Ok(ehttp::Request {
            method: "PATCH".to_string(),
            ..ehttp::Request::post(url, body)
        })
    }

    /// Creates a DELETE request with a JSON-encoded body.
    ///
    /// The body is serialized to JSON using `serde_json`.
    fn delete_request<Y, Z>(url: Z, body: &Y) -> Result<Self, Self::ResponseError>
    where
        Y: ?Sized + serde::Serialize,
        Z: ToString,
    {
        let body = serde_json::to_vec(body).map_err(|e| ehttp::Error::from(e.to_string()))?;
        Ok(ehttp::Request {
            method: "DELETE".to_string(),
            ..ehttp::Request::post(url, body)
        })
    }
}
