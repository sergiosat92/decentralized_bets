//! 🌐 HTTP CLIENT MODULE FOR ASYNC REQUESTS WITH LOGGING AND ERROR HANDLING
//!
//! This module provides a generic async HTTP client function to send requests with optional headers and JSON bodies.
//! It handles request timeouts, response status codes, and logs errors with tracing.

use reqwest::{Method, Client, header::HeaderMap, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use core::fmt;
use std::time::Duration;


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
pub async fn send_request<T, R>(
    url: &str,
    method: Method,
    headers: Option<&HeaderMap>,
    body: Option<&T>,
    timeout_sec: Option<u64>,
) -> Result<Option<R>, String>
where
    T: Serialize + fmt::Debug,
    R: DeserializeOwned + fmt::Debug,
{
    let client = Client::builder()
        .timeout(Duration::from_secs(timeout_sec.unwrap_or(5)))
        .build()
        .map_err(|e| {
            println!("❌ Error creating HTTP client: {}", e);
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
            println!("❌ Error sending request: {}", e);
            return Err(format!("❌ Error sending request: {}", e));
        }
    };

    // Handle response status codes
    match response.status() {
        StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED => {
            match response.json::<R>().await {
                Ok(data) => Ok(Some(data)),
                Err(e) => {
                    println!("❌ Error deserializing response: {}", e);
                    Err(format!("❌ Error deserializing response: {}", e))
                },
            }
        }
        StatusCode::NO_CONTENT => {
            println!("✅ Request succeeded with no content");
            Ok(None)
        },
        status => {
            let text = response.text().await.unwrap_or_default();
            println!("❌ Request failed with status {}: {}", status, text);
            Err(format!(
                "Request failed with status {}: {}",
                status, text
            ))
        }
    }
}
