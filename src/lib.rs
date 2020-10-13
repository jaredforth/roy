use reqwest::{Response, header};

/// The HTTP client
pub struct Client {
    /// API base URL
    pub base_url: String,
    /// The reqwest wrapper
    client: reqwest::Client,
}

impl Client {
    /// Create a new instance of a `Client`
    ///
    /// ## Usage:
    /// ```
    /// use roy::Client;
    ///
    /// let c = Client::new("https://httpbin.org".to_string());
    /// assert_eq!(c.base_url, "https://httpbin.org");
    /// ```
    pub fn new(base_url: String) -> Client {
        Client {
            base_url,
            client: reqwest::Client::new()
        }
    }
    /// Create a new instance of an authenticated `Client`
    ///
    /// ## Usage:
    /// ```
    /// use roy::Client;
    /// use tokio_test::block_on;
    ///
    /// let c = Client::new_auth("https://httpbin.org".to_string(), "");
    /// assert_eq!(block_on(c.get("/bearer")).is_some(), true);
    /// ```
    pub fn new_auth(base_url: String, auth_token: &'static str) -> Client {
        let mut headers = header::HeaderMap::new();
        headers.insert(header::AUTHORIZATION, header::HeaderValue::from_static(auth_token));
        Client {
            base_url,
            client: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .unwrap()
        }
    }
    /// Generic function to POST data to an endpoint
    ///
    /// ## Usage:
    /// ```
    /// use roy::Client;
    /// use tokio_test::block_on;
    ///
    /// let c = Client::new("https://httpbin.org".to_string());
    /// assert_eq!(block_on(c.post("/post", "{data}")).is_some(), true);
    /// ```
    pub async fn post<T: serde::ser::Serialize + std::fmt::Debug>(&self, endpoint: &str, data: T) -> Option<Response> {
        let res = self.client.post(&format_url(endpoint))
            .json(&data)
            .send()
            .await.ok()?;
        Some(res)
    }
    /// Generic function to DELETE to an endpoint
    ///
    /// ## Usage:
    /// ```
    /// use roy::Client;
    /// use tokio_test::block_on;
    ///
    /// let c = Client::new("https://httpbin.org".to_string());
    /// assert_eq!(block_on(c.delete("/delete")).is_some(), true);
    /// ```
    pub async fn delete(&self, endpoint: &str) -> Option<Response> {
        let res = self.client.delete(&format_url(endpoint))
            .send()
            .await.ok()?;
        Some(res)
    }
    /// Generic function to PATCH data to an endpoint
    ///
    /// ## Usage:
    /// ```
    /// use roy::Client;
    /// use tokio_test::block_on;
    ///
    /// let c = Client::new("https://httpbin.org".to_string());
    /// assert_eq!(block_on(c.patch("/patch", "{data}")).is_some(), true);
    /// ```
    pub async fn patch<T: serde::ser::Serialize + std::fmt::Debug>(&self, endpoint: &str, data: T) -> Option<Response> {
        let res = self.client.patch(&format_url(endpoint))
            .json(&data)
            .send()
            .await.ok()?;
        Some(res)
    }
    /// Generic function to PUT data to an endpoint
    ///
    /// ## Usage:
    /// ```
    /// use roy::Client;
    /// use tokio_test::block_on;
    ///
    /// let c = Client::new("https://httpbin.org".to_string());
    /// assert_eq!(block_on(c.put("/put", "{data}")).is_some(), true);
    /// ```
        pub async fn put<T: serde::ser::Serialize + std::fmt::Debug>(&self, endpoint: &str, data: T) -> Option<Response> {
            let res = self.client.put(&format_url(endpoint))
                .json(&data)
                .send()
                .await.ok()?;
            Some(res)
        }
    /// Generic function to send a GET request to an endpoint
    ///
    /// ## Usage:
    /// ```
    /// use roy::Client;
    /// use tokio_test::block_on;
    ///
    /// let c = Client::new("https://httpbin.org".to_string());
    /// assert_eq!(block_on(c.get("/get")).is_some(), true);
    /// ```
    pub async fn get(&self, endpoint: &str) -> Option<Response> {
        let res = self.client.get(&format_url(endpoint))
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
/// use roy::format_url;
///
///
/// assert_eq!(format_url("test"), "https://rivaldata.tech/test")
/// ```
pub fn format_url(endpoint: &str) -> String {
    format!("https://rivaldata.tech/{}", endpoint)
}