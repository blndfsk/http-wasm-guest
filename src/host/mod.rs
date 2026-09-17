//! Host-side API surface for HTTP request/response interaction.
//!
//! This module provides access to request and response handles, header and body
//! manipulation utilities, logging, and feature configuration for `http-wasm`
//! guest plugins.
//!
//! # Memory and cost model
//!
//! All host reads are written into a single shared 2048-byte guest buffer, so
//! receiving a field that fits in it costs no allocation. Fields larger than the
//! buffer are fetched with one extra host call against an exactly-sized heap
//! allocation (capped at just under 16 MB).
//!
//! Because the shared buffer is reused for every subsequent host call, no API in
//! this module returns a zero-copy view of host data: returned values are owned
//! copies (`Bytes`, `Vec<Bytes>`, ...), and the heap cost of each is documented on
//! the individual methods.
mod body;
mod handler;
mod header;
mod request;
mod response;

pub mod admin;
pub mod feature;
pub mod log;

pub use body::Body;
pub use header::Header;
pub use request::Request;
pub use response::Response;

/// Owned byte buffer returned by most read APIs in this module.
///
/// This re-exports [`bytes::Bytes`] from the `bytes` crate: a reference-counted,
/// heap-backed byte slice that is cheap to clone (refcount bump), derefs to
/// `[u8]`, compares against `str`/`&[u8]`, and can be borrowed as UTF-8 via
/// `std::str::from_utf8` without copying. Values returned by this module own
/// their data because they are copied out of the shared guest buffer, which is
/// reused for every host call.
#[doc(no_inline)]
pub use bytes::Bytes;
