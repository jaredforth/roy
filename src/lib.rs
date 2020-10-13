use reqwest::Response;

/// The HTTP client
pub struct Client {
    pub base_url: String,
    client: reqwest::Client
}

impl Client {
    /// Create a new instance of `Client`
    pub fn new(base_url: String) -> Client {
        Client {
            base_url,
            client: reqwest::Client::new()
        }
    }
    /// Generic function to POST data to an endpoint
    pub async fn post_api<T: serde::ser::Serialize + std::fmt::Debug>(&self, endpoint: &str, data: T) -> Option<Response> {
        let res = self.client.post(&format_url(endpoint))
            .header("Authorization", "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoiYWRtaW5fdXNlciJ9.Re2Ci5f4u9Xtp6p0bKy71UPtMC94JS43IVDhX7fHZTE")
            .json(&data)
            .send()
            .await.ok()?;
        Some(res)
    }
    /// Generic function to DELETE to an endpoint
    pub async fn delete_api(&self, endpoint: &str) -> Option<Response> {
        let res = self.client.delete(&format_url(endpoint))
            .header("Authorization", "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoiYWRtaW5fdXNlciJ9.Re2Ci5f4u9Xtp6p0bKy71UPtMC94JS43IVDhX7fHZTE")
            .send()
            .await.ok()?;
        Some(res)
    }
    /// Generic function to PATCH data to an endpoint
    pub async fn patch_api<T: serde::ser::Serialize + std::fmt::Debug>(&self, endpoint: &str, data: T) -> Option<Response> {
        let res = self.client.patch(&format_url(endpoint))
            .header("Authorization", "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoiYWRtaW5fdXNlciJ9.Re2Ci5f4u9Xtp6p0bKy71UPtMC94JS43IVDhX7fHZTE")
            .json(&data)
            .send()
            .await.ok()?;
        Some(res)
    }
    /// Generic function to send a GET request to an endpoint
    pub async fn get_api(&self, endpoint: &str) -> Option<Response> {
        let res = self.client.get(endpoint)
            .send()
            .await.ok()?;
        Some(res)
    }
}

/// Format a URL
///
/// ## Usage:
///
/// ```
/// use polliwog::client::format_url;
///
/// assert_eq!(format_url("test"), "https://rivaldata.tech/test")
/// ```
pub fn format_url(endpoint: &str) -> String {
    format!("https://rivaldata.tech/{}", endpoint)
}