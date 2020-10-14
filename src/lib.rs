// Copyright 2020 Jared Forth.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! **roy** is a high-level library for consuming RESTful APIs.
//!
//! ## Usage:
//! ```
//! use roy::Client; // import into scope
//! let c = roy::Client::new("https://httpbin.org".to_string());  // Instantiate `Client` with your API's base URL
//! c.get("/get", false); // Make a GET request to your endpoint
//! ```
//!
//! **roy** also has methods that support POST, PUT, PATCH, and DELETE http verbs,
//! all of which are used in the same way as the `get()` method.

use reqwest::{Response, header};

/// The HTTP client
pub struct Client {
    /// API base URL
    pub base_url: String,
    /// The reqwest wrapper
    client: reqwest::Client
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
    /// assert_eq!(block_on(c.get("/bearer", false)).is_some(), true);
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
        let res = self.client.post(&self.format_url(endpoint))
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
        let res = self.client.delete(&self.format_url(endpoint))
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
        let res = self.client.patch(&self.format_url(endpoint))
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
        let res = self.client.put(&self.format_url(endpoint))
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
    /// assert_eq!(block_on(c.get("/get", false)).is_some(), true);
    /// ```
    pub async fn get(&self, endpoint: &str, single: bool) -> Option<Response> {
        let res;
        if single {
            res = self.client.get(&self.format_url(endpoint))
                .header("Accept", "application/vnd.pgrst.object+json")
                .send()
                .await.ok()?;
        } else {
            res = self.client.get(&self.format_url(endpoint))
                .send()
                .await.ok()?;
        }
        Some(res)
    }
    /// Generic function to send a GET request to an endpoint
    /// without formating to use the base url.
    ///
    /// Regardless of the value of `base_url`, this function
    /// will send a GET request to the absolute URL passed
    /// as the `url` parameter.
    ///
    /// ## Usage:
    /// ```
    /// use roy::Client;
    /// use tokio_test::block_on;
    ///
    /// let c = Client::new("https://doesnotexist.example.io".to_string());
    /// assert_eq!(block_on(c.get_abs("https://httpbin.org", false)).is_some(), true);
    /// ```
    pub async fn get_abs(&self, url: &str, single: bool) -> Option<Response> {
        let res;
        if single {
            res = self.client.get(url)
                .header("Accept", "application/vnd.pgrst.object+json")
                .send()
                .await.ok()?;
        } else {
            res = self.client.get(url)
                .send()
                .await.ok()?;
        }
        Some(res)
    }
    /// Format a URL
    ///
    /// ## Usage:
    /// ```
    /// use roy::Client;
    ///
    /// let c = Client::new("https://httpbin.org".to_string());
    /// assert_eq!(c.format_url("/test"), "https://httpbin.org/test")
    /// ```
    pub fn format_url(&self, endpoint: &str) -> String {
        format!("{}{}", self.base_url, endpoint)
    }
}