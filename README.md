# consul-oxide

An asynchronous Rust client for the [Consul HTTP API](https://developer.hashicorp.com/consul/api-docs).

## Overview

Consul is a service mesh solution providing a full featured control plane with service discovery, configuration, and segmentation functionality. For more information on what Consul is, read the [documentation][1]. This crate provides an asynchronous Rust client for interacting with a local Consul agent via its HTTP API, This allows for developers to write applications that leverage Consul's service mesh functionality.

## Supported Features

The feature set of this crate is a little bland as of time of writing, but is intended to be enough to enable an application to declare itself and access other service definitions.

The following features are supported by this crate:

- Agent API
- Catalog API
- Health API

This crate is still under active development and more features will be added to support the more advanced features of the Consul HTTP API.

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
consul-oxide = "0.1"
```

Then, add the following to your crate root:

```rust
use consul_oxide::Client;
```

The client can be initialized with a `consul_oxide::Config` struct, or by using the `consul_oxide::Config::from_env()` method to load the configuration from environment variables.

## Example

```rust
let client = consul_oxide::Client::new(consul_oxide::Config::from_env());
client.catalog.list_nodes().await.unwrap();
```

[1]: https://www.consul.io/docs

## License

`consul-oxide` is licensed under a dual MIT/Apache-2.0 license. See the [`LICENSE-MIT`](LICENSE-MIT) and [`LICENSE-APACHE`](LICENSE-APACHE) file for more information.
