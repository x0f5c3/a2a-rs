//! gRPC adapter for the A2A protocol
//!
//! This module provides gRPC transport implementation for the A2A protocol
//! according to the official A2A Protocol v0.3.0 specification.
//!
//! The implementation includes:
//! - gRPC server adapter
//! - gRPC client adapter
//! - Protocol buffer message conversions
//!
//! # Features
//!
//! This module is only available when the `grpc` feature is enabled.

#[cfg(feature = "grpc")]
pub mod client;
#[cfg(feature = "grpc")]
pub mod server;
#[cfg(feature = "grpc")]
pub mod convert;

#[cfg(feature = "grpc")]
pub use client::GrpcClient;
#[cfg(feature = "grpc")]
pub use server::GrpcServer;

/// Re-export the generated protobuf types
#[cfg(feature = "grpc")]
pub mod proto {
    tonic::include_proto!("a2a.v1");
}
