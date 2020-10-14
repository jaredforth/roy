[![Build Status](https://travis-ci.com/jaredforth/roy.svg?branch=master)](https://travis-ci.com/jaredforth/roy)
[![Build status](https://ci.appveyor.com/api/projects/status/gffkkb1qwafutmii?svg=true)](https://ci.appveyor.com/project/jaredforth/roy)
[![Crate](https://img.shields.io/crates/v/roy.svg)](https://crates.io/crates/roy)
[![API](https://docs.rs/roy/badge.svg)](https://docs.rs/roy)
![Crates.io](https://img.shields.io/crates/l/roy)
![GitHub top language](https://img.shields.io/github/languages/top/jaredforth/roy)
![Crates.io](https://img.shields.io/crates/d/roy)

# roy

- [roy](#roy)
  - [About](#about)
  - [Installation](#installation)
  - [Usage](#usage)
    - [GET](#get)
    - [POST](#post)
    - [PATCH](#patch)
    - [PUT](#put)
    - [DELETE](#delete)
  - [License](#license)
  - [Why the name Roy?](#why-the-name-roy)

## About

This crate is a high-level library for consuming RESTful APIs.

Documentation:

- [API Reference](https://docs.rs/roy)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
roy = "0.1"
```

## Usage

To use any of the methods on the `roy::Client` struct, it has to be instantiated as follows:

```rust
use roy::Client; // import into scope
let c = roy::Client::new("https://httpbin.org".to_string());  // Instantiate `Client` with your API's base URL
```

All of the examples below assume that the struct has already been instantiated.

### GET

```rust
c.get("/get", false); // Make a GET request to your endpoint
```

### POST

```rust
c.post("/post", "{some: data}"); // Make a POST request to your endpoint
```

### PATCH

```rust
c.patch("/patch", "{some: data}"); // Make a PATCH request to your endpoint
```

### PUT

```rust
c.put("/put", "{some: data}"); // Make a PUT request to your endpoint
```

### DELETE

```rust
c.delete("/delete"); // Make a DELETE request to your endpoint
```

## License

**roy** is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.

## Why the name Roy?

This crate is named after [Roy Fielding](https://en.wikipedia.org/wiki/Roy_Fielding), who defined Representational State Transfer. According to [Wikipedia](https://en.wikipedia.org/wiki/Representational_state_transfer#History): 

> Roy Fielding defined REST in his 2000 PhD dissertation "Architectural Styles and the Design of Network-based Software Architectures" at UC Irvine. He developed the REST architectural style in parallel with HTTP 1.1 of 1996–1999, based on the existing design of HTTP 1.0 of 1996.
