//! Host-side API surface for HTTP request/response interaction.
//!
//! This module provides access to request and response handles, header and body
//! manipulation utilities, logging, and feature configuration for `http-wasm`
//! guest plugins.
mod body;
mod bytes;
mod handler;
mod header;
mod request;
mod response;

pub mod admin;
pub mod feature;

pub use body::Body;
pub use bytes::Bytes;
pub use header::Header;
pub use request::Request;
pub use response::Response;
