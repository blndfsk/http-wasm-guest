//! Feature management for http-wasm host capabilities.
//!
//! This module provides functionality to enable optional features in the http-wasm host
//! environment. Features control behavior like body buffering and trailer handling.
//!
//! # Available Features
//!
//! - [`BufferRequest`] - Buffer request bodies in memory for multiple reads
//! - [`BufferResponse`] - Buffer response bodies to allow modification
//! - [`Trailers`] - Enable HTTP trailer support
//!
//! # Usage
//!
//! Features should be enabled during module initialization, typically in your `main()` function:
//!
//! ```no_run
//! use http_wasm_guest::host::feature::{enable, BufferRequest, BufferResponse};
//!
//! fn main() {
//!     // Enable single feature
//!     enable(BufferRequest);
//!
//!     // Enable multiple features
//!     enable(BufferRequest | BufferResponse);
//!
//!     // Register your plugin...
//! }
//! ```
//!
//! # Performance Considerations
//!
//! - `BufferRequest` and `BufferResponse` consume additional memory
//! - Enable only the features you actually need
//! - Some features may not be supported by all hosts

use super::handler;
use std::ops::BitOr;

/// Feature flag for buffering request bodies.
///
/// When enabled, the host will buffer the entire request body in memory
/// before calling the guest handler. This allows the guest to read the
/// complete request body multiple times.
///
/// See the [http-wasm specification](https://http-wasm.io/http-handler-abi/#features)
/// for more details.
#[allow(non_upper_case_globals, non_snake_case)]
pub const BufferRequest: Feature = Feature(1);

/// Feature flag for buffering response bodies.
///
/// When enabled, the host will buffer the entire response body in memory,
/// allowing the guest to modify the response body during response processing.
///
/// See the [http-wasm specification](https://http-wasm.io/http-handler-abi/#features)
/// for more details.
#[allow(non_upper_case_globals, non_snake_case)]
pub const BufferResponse: Feature = Feature(2);

/// Feature flag for handling HTTP trailers.
///
/// When enabled, the guest can access and modify HTTP trailers (headers that
/// come after the body in chunked transfer encoding).
///
/// See the [http-wasm specification](https://http-wasm.io/http-handler-abi/#features)
/// for more details.
#[allow(non_upper_case_globals, non_snake_case)]
pub const Trailers: Feature = Feature(4);

/// Represents a feature flag that can be enabled on the host.
///
/// Features control optional behavior in the http-wasm host environment.
/// Multiple features can be combined using the bitwise OR operator (`|`).
///
/// # Examples
///
/// ```no_run
/// use http_wasm_guest::host::feature::{enable, BufferRequest, BufferResponse};
///
/// // Enable a single feature
/// enable(BufferRequest);
///
/// // Enable multiple features
/// enable(BufferRequest | BufferResponse);
/// ```
///
/// See the [http-wasm specification](https://http-wasm.io/http-handler-abi/#enable_features)
/// for more details.
#[derive(Debug, PartialEq)]
pub struct Feature(pub i32);
impl BitOr for Feature {
    type Output = Feature;

    fn bitor(self, rhs: Self) -> Feature {
        Feature(self.0 | rhs.0)
    }
}

/// Enables the specified features on the host.
///
/// This function must be called during module initialization (typically in `main()`)
/// to enable optional features provided by the host environment.
///
/// # Parameters
///
/// - `feature`: The feature or combination of features to enable
///
/// # Returns
///
/// Returns an `i32` indicating the result of the feature enable operation.
/// The exact meaning depends on the host implementation.
///
/// # Examples
///
/// ```no_run
/// use http_wasm_guest::host::feature::{enable, BufferRequest, BufferResponse};
///
/// fn main() {
///     // Enable request body buffering
///     enable(BufferRequest);
///
///     // Enable multiple features at once
///     enable(BufferRequest | BufferResponse);
/// }
/// ```
///
/// # Notes
///
/// - Features should be enabled before registering the guest plugin
/// - Not all hosts support all features - check your host's documentation
/// - Some features may have performance implications
///
/// See the [http-wasm specification](https://http-wasm.io/http-handler-abi/#enable_features)
/// for more details.
pub fn enable(feature: Feature) -> i32 {
    handler::enable_feature(feature.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_or() {
        assert_eq!(BufferRequest | BufferResponse, Feature(3));
    }
}
