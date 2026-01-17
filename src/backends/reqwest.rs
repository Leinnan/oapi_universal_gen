//! Implementations of request/response helper traits for the `reqwest` crate.
//!
//! This module provides concrete implementations of [`ResponseHelperExt`] and
//! [`RequestHelperExt`] for `reqwest::Response` and `ReqwestHelper` respectively.

use crate::{RequestHelperExt, ResponseHelperExt};
use std::collections::HashMap;
use std::fmt;

/// A custom error type for reqwest requests that can contain serialization errors.
#[derive(Debug)]
pub enum ReqwestError {
    /// A reqwest network/HTTP error.
    Reqwest(reqwest::Error),
    /// A JSON serialization error.
    Serialization(serde_json::Error),
}

impl fmt::Display for ReqwestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReqwestError::Reqwest(e) => write!(f, "Reqwest error: {}", e),
            ReqwestError::Serialization(e) => write!(f, "Serialization error: {}", e),
        }
    }
}

impl std::error::Error for ReqwestError {}

impl From<reqwest::Error> for ReqwestError {
    fn from(e: reqwest::Error) -> Self {
        ReqwestError::Reqwest(e)
    }
}

impl From<serde_json::Error> for ReqwestError {
    fn from(e: serde_json::Error) -> Self {
        ReqwestError::Serialization(e)
    }
}

/// A helper wrapper for constructing reqwest requests.
///
/// This struct stores the components needed to build a reqwest request,
/// allowing for incremental header addition before sending.
pub struct ReqwestHelper {
    url: String,
    method: reqwest::Method,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

impl ReqwestHelper {
    /// Creates a new `ReqwestHelper` with the specified URL and method.
    pub fn new(url: String, method: reqwest::Method) -> Self {
        Self {
            url,
            method,
            headers: HashMap::new(),
            body: None,
        }
    }

    /// Creates a new `ReqwestHelper` with a JSON body.
    pub fn with_body(url: String, method: reqwest::Method, body: Vec<u8>) -> Self {
        Self {
            url,
            method,
            headers: HashMap::new(),
            body: Some(body),
        }
    }

    /// Builds the actual `reqwest::RequestBuilder` from the stored components.
    fn build_request(&self) -> reqwest::RequestBuilder {
        let client = reqwest::Client::new();
        let mut builder = client.request(self.method.clone(), &self.url);

        for (key, value) in &self.headers {
            builder = builder.header(key, value);
        }

        if let Some(body) = &self.body {
            builder = builder.body(body.clone());
        }

        builder
    }
}

impl Clone for ReqwestHelper {
    fn clone(&self) -> Self {
        Self {
            url: self.url.clone(),
            method: self.method.clone(),
            headers: self.headers.clone(),
            body: self.body.clone(),
        }
    }
}

/// Implementation of [`ResponseHelperExt`] for [`reqwest::Response`].
impl ResponseHelperExt for reqwest::Response {
    fn response_content_type(&self) -> String {
        self.headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string()
    }

    async fn response_content(self) -> Option<String> {
        self.text().await.ok()
    }

    fn status(&self) -> u16 {
        self.status().as_u16()
    }

    fn is_client_error(&self) -> bool {
        self.status().is_client_error()
    }

    fn is_server_error(&self) -> bool {
        self.status().is_server_error()
    }
}

/// Implementation of [`RequestHelperExt`] for [`ReqwestHelper`].
///
/// This implementation provides HTTP request methods for the `reqwest` library,
/// supporting GET, POST, PUT, PATCH, and DELETE requests with JSON body serialization.
impl RequestHelperExt for ReqwestHelper {
    /// The response type for reqwest requests.
    type Response = reqwest::Response;

    /// The error type for reqwest requests.
    type ResponseError = ReqwestError;

    /// Adds a header to the request.
    ///
    /// Inserts or updates the specified header key-value pair.
    fn with_header(&mut self, key: impl ToString, value: impl ToString) -> &mut Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    /// Sends the request asynchronously.
    ///
    /// Builds the request from components and sends it using reqwest.
    async fn send_request(self) -> Result<Self::Response, Self::ResponseError> {
        let builder = self.build_request();
        builder.send().await.map_err(ReqwestError::from)
    }

    /// Creates a GET request for the specified URL.
    fn get_request<Z>(url: Z) -> Result<Self, Self::ResponseError>
    where
        Z: ToString,
    {
        Ok(ReqwestHelper::new(url.to_string(), reqwest::Method::GET))
    }

    /// Creates a POST request with a JSON-encoded body.
    ///
    /// The body is serialized to JSON using `serde_json`.
    fn post_request<Y, Z>(url: Z, body: &Y) -> Result<Self, Self::ResponseError>
    where
        Y: ?Sized + serde::Serialize,
        Z: ToString,
    {
        let body_vec = serde_json::to_vec(body)?;
        Ok(ReqwestHelper::with_body(
            url.to_string(),
            reqwest::Method::POST,
            body_vec,
        ))
    }

    /// Creates a PUT request with a JSON-encoded body.
    ///
    /// The body is serialized to JSON using `serde_json`.
    fn put_request<Y, Z>(url: Z, body: &Y) -> Result<Self, Self::ResponseError>
    where
        Y: ?Sized + serde::Serialize,
        Z: ToString,
    {
        let body_vec = serde_json::to_vec(body)?;
        Ok(ReqwestHelper::with_body(
            url.to_string(),
            reqwest::Method::PUT,
            body_vec,
        ))
    }

    /// Creates a PATCH request with a JSON-encoded body.
    ///
    /// The body is serialized to JSON using `serde_json`.
    fn patch_request<Y, Z>(url: Z, body: &Y) -> Result<Self, Self::ResponseError>
    where
        Y: ?Sized + serde::Serialize,
        Z: ToString,
    {
        let body_vec = serde_json::to_vec(body)?;
        Ok(ReqwestHelper::with_body(
            url.to_string(),
            reqwest::Method::PATCH,
            body_vec,
        ))
    }

    /// Creates a DELETE request with a JSON-encoded body.
    ///
    /// The body is serialized to JSON using `serde_json`.
    fn delete_request<Y, Z>(url: Z, body: &Y) -> Result<Self, Self::ResponseError>
    where
        Y: ?Sized + serde::Serialize,
        Z: ToString,
    {
        let body_vec = serde_json::to_vec(body)?;
        Ok(ReqwestHelper::with_body(
            url.to_string(),
            reqwest::Method::DELETE,
            body_vec,
        ))
    }
}
