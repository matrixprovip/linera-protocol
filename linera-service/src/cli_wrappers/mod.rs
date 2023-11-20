// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Helper module to call the binaries of `linera-service` with appropriate command-line
//! arguments.

#[cfg(feature = "kubernetes")]
/// How to run docker operations
mod docker;
#[cfg(feature = "kubernetes")]
/// How to run helm operations
mod helm;
#[cfg(feature = "kubernetes")]
/// How to run kind operations
mod kind;
#[cfg(feature = "kubernetes")]
/// How to run kubectl operations
mod kubectl;
#[cfg(feature = "kubernetes")]
/// How to run Linera validators locally as a Kubernetes deployment.
pub mod local_kubernetes_net;
/// How to run Linera validators locally as native processes.
pub mod local_net;
#[cfg(feature = "kubernetes")]
/// Util functions for the wrappers
mod util;
/// How to run a linera wallet and its GraphQL service.
mod wallet;

pub use wallet::{ApplicationWrapper, ClientWrapper, Faucet, NodeService};

use anyhow::Result;
use async_trait::async_trait;

/// The information needed to start a Linera net of a particular kind.
#[async_trait]
pub trait LineraNetConfig {
    type Net: LineraNet + Sized + Send + Sync + 'static;

    async fn instantiate(self) -> Result<(Self::Net, ClientWrapper)>;

    #[cfg(any(test, feature = "test"))]
    fn get_network(&self) -> Network;
}

/// A running Linera net.
#[async_trait]
pub trait LineraNet {
    async fn ensure_is_running(&mut self) -> Result<()>;

    fn make_client(&mut self) -> ClientWrapper;

    async fn terminate(mut self) -> Result<()>;

    #[cfg(any(test, feature = "test"))]
    async fn terminate_server(&mut self, i: usize, j: usize) -> Result<()>;

    #[cfg(any(test, feature = "test"))]
    async fn start_server(&mut self, i: usize, j: usize) -> Result<()>;

    #[cfg(any(test, feature = "test"))]
    async fn generate_validator_config(&mut self, i: usize) -> Result<()>;

    async fn start_validator(&mut self, i: usize) -> Result<()>;

    #[cfg(any(test, feature = "test"))]
    fn validator_name(&self, i: usize) -> Option<&String>;

    #[cfg(any(test, feature = "test"))]
    fn remove_validator(&mut self, i: usize) -> Result<()>;
}

/// Network protocol in use outside and inside a Linera net.
#[derive(Copy, Clone)]
pub enum Network {
    Grpc,
    Simple,
}

impl Network {
    fn internal(&self) -> &'static str {
        match self {
            Network::Grpc => "{ Grpc = \"ClearText\" }",
            Network::Simple => "{ Simple = \"Tcp\" }",
        }
    }

    fn external(&self) -> &'static str {
        match self {
            Network::Grpc => "{ Grpc = \"ClearText\" }",
            Network::Simple => "{ Simple = \"Tcp\" }",
        }
    }

    fn external_short(&self) -> &'static str {
        match self {
            Network::Grpc => "grpc",
            Network::Simple => "tcp",
        }
    }
}
