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
    /// let c = Client::new_auth("https://httpbin.org".to_string(), "".to_string());
    /// assert_eq!(block_on(c.get("/bearer", false)).is_some(), true);
    /// ```
    pub fn new_auth(base_url: String, auth_token: String) -> Client {
        let mut headers = header::HeaderMap::new();
        match header::HeaderValue::from_str(auth_token.as_str()) {
            Ok(val) => {
                headers.insert(header::AUTHORIZATION, val);
            }
            Err(e) => {
                println!("{:?}", e)
            }
        }
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
    /// Make a request to the specified endpoint with a specified request method.
    ///
    /// ## Usage:
    /// ```
    /// use roy::{Client, RequestMethod};
    /// use tokio_test::block_on;
    ///
    /// let c = Client::new("https://httpbin.org".to_string());
    ///
    /// assert!(block_on(c.request("/get", RequestMethod::GET, None)).is_some());
    /// assert!(block_on(c.request("/post", RequestMethod::POST, Some("{}"))).is_some());
    /// assert!(block_on(c.request("/patch", RequestMethod::PATCH, Some("{}"))).is_some());
    /// assert!(block_on(c.request("/put", RequestMethod::PUT, Some("{}"))).is_some());
    /// assert!(block_on(c.request("/delete", RequestMethod::DELETE, None)).is_some());
    /// ```
    pub async fn request(&self, endpoint: &str, method: RequestMethod, data: Option<&str>) -> Option<Response> {
        // Extract data from Option
        let d = match data {
            Some(d) => d,
            None => ""
        };
        match method {
            RequestMethod::GET => {
                self.get(endpoint, false).await
            }
            RequestMethod::POST => {
                self.post(endpoint, d).await
            }
            RequestMethod::PUT => {
                self.put(endpoint, d).await
            }
            RequestMethod::PATCH => {
                self.patch(endpoint, d).await
            }
            RequestMethod::DELETE => {
                self.delete(endpoint).await
            }
        }
    }
}

/// Lists the possible HTTP request methods that can be used.
///
/// #### MDN Description:
///
/// HTTP defines a set of request methods to indicate the desired action to be performed for a given resource.
/// Although they can also be nouns, these request methods are sometimes referred to as HTTP verbs.
///
pub enum RequestMethod {
    /// The GET method requests a representation of the specified resource. Requests using GET should only retrieve data.
    GET,
    /// The POST method is used to submit an entity to the specified resource, often causing a change in state or side effects on the server.
    POST,
    /// The PUT method replaces all current representations of the target resource with the request payload.
    PUT,
    /// The PATCH method is used to apply partial modifications to a resource.
    PATCH,
    /// The DELETE method deletes the specified resource.
    DELETE
}