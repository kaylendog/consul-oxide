//! # consul-oxide
//!
//! An asynchronous Rust client for the [Consul HTTP API](https://developer.hashicorp.com/consul/api-docs).
//!
//! ## Overview
//!
//! Consul is a service mesh solution providing a full featured control plane
//! with service discovery, configuration, and segmentation functionality. For
//! more information on what Consul is, read the [documentation][1].
//!
//! This crate provides an asynchronous Rust client for interacting with
//! a local Consul agent via its HTTP API, This allows for developers
//! to write applications that leverage Consul's service mesh functionality.
//!
//! ## Supported Features
//!
//! The feature set of this crate is a little bland as of time of writing, but
//! is intended to be enough to enable an application to declare itself and
//! access other service definitions.
//!
//! The following features are supported by
//! this crate:
//!
//! - [x] Agent API
//! - [x] Catalog API
//! - [x] Health API
//!
//! This crate is still under active development and more features will be added
//! to support the more advanced features of the Consul HTTP API.
//!
//! ## Usage
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! consul-oxide = "0.1"
//! ```
//!
//! Then, add the following to your crate root:
//!
//! ```rust
//! use consul_oxide::Client;
//! ```
//!
//! The client can be initialized with a `consul_oxide::Config` struct, or
//! by using the `consul_oxide::Config::from_env()` method to load the
//! configuration from environment variables.
//!
//! ## Example
//!
//! ```
//! let client = consul_oxide::Client::new(consul_oxide::Config::from_env());
//! client.catalog.list_nodes().await.unwrap();
//! ```
//!
//! [1]: https://www.consul.io/docs

use std::{env, sync::Arc};

use agent::Agent;
use reqwest::header::HeaderMap;

pub mod agent;
pub mod catalog;
pub mod common;
pub mod health;
mod http;

use catalog::Catalog;
use health::Health;

/// Type alias for `Result` with the error type `consul_oxide::Error`.
pub type Result<T> = std::result::Result<T, Error>;

/// An enumeration of errors that can occur when interacting with the Consul
/// HTTP API.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error occured in the HTTP client.
    #[error("An error occured in the HTTP client")]
    HttpError(#[from] reqwest::Error),
    /// The specified envioronment variable was not found.
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),
}

/// The main entry point for interacting with the Consul HTTP API.
pub struct Client {
    /// Provides access to the Consul Catalog API.
    pub catalog: Catalog,
    /// Provides access to the Consul Health API.
    pub health: Health,
    /// Provides access to the Consul Agent API.
    pub agent: Agent,
}

impl Client {
    /// Create a new `Client` from the given `Config`.
    pub fn new(config: Config) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert("X-Consul-Token", config.token.parse().unwrap());
        // create reqwest client with custom headers
        let client = reqwest::Client::builder()
            .user_agent("consul-oxide")
            .default_headers(headers)
            .build()
            .map_err(Error::HttpError)?;
        // use arc to avoid cloning the client
        let client = Arc::new(client);
        let config = Arc::new(config);
        // construct submodules
        let catalog = Catalog::new(client.clone(), config.clone());
        let health = Health::new(client.clone(), config.clone());
        let agent = Agent::new(client, config);
        // return the client
        Ok(Self { catalog, health, agent })
    }
}

/// Configuration for the `Client`.
pub struct Config {
    /// The address of the Consul server. This is the address that the client
    /// will connect to when making requests to the Consul HTTP API.
    pub address: String,
    /// The access token to use when making requests to the Consul HTTP API.
    pub token: String,
}

impl Config {
    /// Manually create a new `Config` with the given address and token.
    pub fn new(address: String, token: String) -> Self {
        Self { address, token }
    }

    /// Create a new `Config` from environment variables. This reads the
    /// `CONSUL_HTTP_ADDR` and `CONSUL_HTTP_TOKEN` environment variables,
    /// as specified in the [Consul HTTP API documentation][1]
    ///
    /// [1]: https://developer.hashicorp.com/consul/api-docs
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            address: env::var("CONSUL_HTTP_ADDR")
                .map_err(|_| Error::MissingEnvVar("CONSUL_HTTP_ADDR".to_string()))?,
            token: env::var("CONSUL_HTTP_TOKEN")
                .map_err(|_| Error::MissingEnvVar("CONSUL_HTTP_TOKEN".to_string()))?,
        })
    }
}
