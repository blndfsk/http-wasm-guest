//! Host-side API surface for HTTP request/response interaction.
//!
//! This module provides access to request and response handles, header and body
//! manipulation utilities, logging, and feature configuration for `http-wasm`
//! guest plugins.
//!
//! Reads never hand out a zero-copy view of host data: returned values are
//! always owned copies. The heap cost of each is documented on the individual
//! methods.
mod body;
mod handler;
mod header;
mod request;
mod response;

pub mod admin;
pub mod feature;
pub mod log;

pub use body::Body;
pub use body::BodyIter;
pub use header::Header;
pub use request::Request;
pub use response::Response;

/// The byte type used throughout this crate's API: a re-export of
/// [`bytes::Bytes`], so its methods and traits apply as documented in the
/// `bytes` crate.
#[doc(no_inline)]
pub use bytes::Bytes;
