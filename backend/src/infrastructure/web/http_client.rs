//! 🌐 HTTP CLIENT MODULE FOR ASYNC REQUESTS WITH LOGGING AND ERROR HANDLING
//!
//! This module provides a generic async HTTP client function to send requests with optional headers and JSON bodies.
//! It handles request timeouts, response status codes, and logs errors with tracing.

use reqwest::{Method, Client, header::HeaderMap, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tracing::Level;
use std::time::Duration;

use crate::infrastructure::observability::logs::logger;

/// Sends an HTTP request asynchronously with optional headers and JSON body.
/// Returns the deserialized response data if available.
///
/// # Parameters
/// - `url`: The target URL as string slice.
/// - `method`: HTTP method (GET, POST, etc.).
/// - `headers`: Optional HTTP headers to include.
/// - `body`: Optional JSON serializable body to send.
/// - `timeout_sec`: Request timeout in seconds.
///
/// # Returns
/// - `Ok(Some(T))` if response contains JSON body successfully deserialized to `T`.
/// - `Ok(None)` if response has no content or body cannot be deserialized.
/// - `Err(String)` if request fails or returns error status.
///
/// # Logging
/// Errors and failures are logged at error level with detailed messages.
pub async fn send_request<T>(
    url: &str,
    method: Method,
    headers: Option<&HeaderMap>,
    body: Option<&T>,
    timeout_sec: u64,
) -> Result<Option<T>, String>
where
    T: DeserializeOwned + Serialize,
{
    let client = Client::builder()
        .timeout(Duration::from_secs(timeout_sec))
        .build()
        .map_err(|e| {
            logger(
                Level::ERROR,
                "send_request",
                &format!("❌ Error creating HTTP client: {}", e),
            );
            format!("Error creating HTTP client: {}", e)
        })?;

    let mut request_builder = client.request(method, url);

    // Add headers if present
    if let Some(headers_map) = headers {
        request_builder = request_builder.headers(headers_map.clone());
    }

    // Add JSON body if present
    if let Some(body_data) = body {
        request_builder = request_builder.json(body_data);
    }

    let response = match request_builder.send().await {
        Ok(resp) => resp,
        Err(e) => {
            logger(
                Level::ERROR,
                "send_request",
                &format!("❌ Error sending request: {}", e),
            );
            return Err(format!("❌ Error sending request: {}", e));
        }
    };

    // Handle response status codes
    match response.status() {
        StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED => {
            match response.json::<T>().await {
                Ok(data) => Ok(Some(data)),
                Err(_) => Ok(None),
            }
        }
        StatusCode::NO_CONTENT => Ok(None),
        status => {
            let text = response.text().await.unwrap_or_default();

            logger(
                Level::ERROR,
                "send_request",
                &format!("❌ Request failed: {} - {}", status, text),
            );

            Err(format!(
                "Request failed with status {}: {}",
                status, text
            ))
        }
    }
}
