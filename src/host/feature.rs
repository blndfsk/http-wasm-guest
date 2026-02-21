//! Host feature flags for configuring runtime capabilities.
//!
//! Use these flags to enable host-supported functionality such as buffering
//! request/response bodies or accessing trailers. Combine multiple flags with
//! bitwise OR and pass the result to `admin::enable`.
use std::ops::BitOr;

#[allow(non_upper_case_globals, non_snake_case)]
/// Enables buffering of the entire request body before your handler is invoked.
///
/// When enabled, the host reads the full request body into memory and then makes it
/// available to the guest. This allows multiple reads and supports request body
/// inspection or mutation workflows that need complete payload access.
///
/// Trade-offs:
/// - Increased memory usage proportional to request size
/// - Potential latency from buffering large payloads
///
/// Use this flag only when your plugin must read or modify the request body.
pub const BufferRequest: Feature = Feature(1);
#[allow(non_upper_case_globals, non_snake_case)]
/// Enables buffering of the entire response body before it is sent.
///
/// When enabled, the host collects the full response body so the guest can read and
/// modify it during response handling. This is required for plugins that rewrite,
/// filter, or analyze full response payloads.
///
/// Trade-offs:
/// - Increased memory usage proportional to response size
/// - Potential latency from buffering large payloads
///
/// Use this flag only when your plugin needs full response body access.
pub const BufferResponse: Feature = Feature(2);
#[allow(non_upper_case_globals, non_snake_case)]
/// Enables HTTP trailer support for chunked transfer encoding.
///
/// When enabled, the guest can access and modify trailer headers that arrive after
/// the body. Trailers are typically used to convey metadata computed during streaming,
/// such as checksums or signatures.
///
/// Notes:
/// - Not all hosts or upstream servers support trailers
/// - Trailers are only applicable to chunked or streaming responses
pub const Trailers: Feature = Feature(4);
#[derive(Debug, PartialEq)]
/// Bitflag wrapper used to configure host capabilities.
///
/// Combine flags with `|` and pass the result to `admin::enable`.
pub struct Feature(pub i32);
impl BitOr for Feature {
    type Output = Feature;

    fn bitor(self, rhs: Self) -> Feature {
        Feature(self.0 | rhs.0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feature_buffer_request() {
        assert_eq!(BufferRequest.0, 1);
    }

    #[test]
    fn feature_buffer_response() {
        assert_eq!(BufferResponse.0, 2);
    }

    #[test]
    fn feature_trailers() {
        assert_eq!(Trailers.0, 4);
    }

    #[test]
    fn feature_combine_with_bitor() {
        let combined = BufferRequest | BufferResponse;
        assert_eq!(combined, Feature(3));
    }

    #[test]
    fn feature_combine_all() {
        let combined = BufferRequest | BufferResponse | Trailers;
        assert_eq!(combined, Feature(7));
    }
}
