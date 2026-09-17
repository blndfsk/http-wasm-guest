//! Host feature flags for configuring runtime capabilities.
//!
//! Use these flags to enable host-supported functionality such as buffering
//! request/response bodies or accessing trailers. Combine multiple flags with
//! bitwise OR and pass the result to `admin::enable`.
use std::ops::{BitOr, BitOrAssign};

#[allow(non_upper_case_globals, non_snake_case)]
/// Enables host-side buffering of the request body so multiple readers can use it.
///
/// When enabled, the host buffers the HTTP request body as it is read, so both
/// your plugin and any downstream handler can read it. Without this flag, reading
/// the body consumes the stream and the next handler may panic when it tries to
/// read what was already read.
///
/// Enable it before returning from `handle_request` for the current request (or
/// during initialization to fail fast on hosts that don't support it).
///
/// Trade-offs:
/// - Increased memory usage proportional to request size
/// - Potential latency from buffering large payloads
///
/// Use this flag only when your plugin must read or modify the request body.
pub const BufferRequest: Feature = Feature(1);
#[allow(non_upper_case_globals, non_snake_case)]
/// Enables host-side buffering of the response produced by the next handler.
///
/// When enabled, the host defers the response produced by the next handler so
/// `handle_response` can inspect and overwrite its status code, body, or trailers.
/// This is required for plugins that rewrite, filter, or analyze full response
/// payloads.
///
/// Because the response is sent later than usual, expect timing differences when
/// enabled. As with [`BufferRequest`], enable it before returning from
/// `handle_request` (or during initialization to fail fast).
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
/// - Not all hosts or upstream servers support trailers; hosts that don't report
///   this bit as unavailable and trap if you try to set trailer values
/// - Trailers are only applicable to chunked or streaming responses
pub const Trailers: Feature = Feature(4);

/// Bitflag wrapper used to configure host capabilities.
///
/// Combine flags with `|` and pass the result to `admin::enable`.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Feature(i32);

impl From<Feature> for i32 {
    fn from(val: Feature) -> Self {
        val.0
    }
}

impl BitOr for Feature {
    type Output = Feature;

    fn bitor(self, rhs: Self) -> Feature {
        Feature(self.0 | rhs.0)
    }
}

impl BitOrAssign for Feature {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
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
    fn feature_combine_with_bitorassign() {
        let mut f = BufferRequest;
        f |= BufferResponse;
        assert_eq!(f, Feature(3));
    }

    #[test]
    fn feature_combine_all() {
        let combined = BufferRequest | BufferResponse | Trailers;
        assert_eq!(combined, Feature(7));
    }

    #[test]
    fn feature_debug_output() {
        let f = Feature(42);
        let debug_str = format!("{:?}", f);
        assert!(debug_str.contains("42"));
    }

    #[test]
    fn feature_partial_eq() {
        assert_eq!(Feature(5), Feature(5));
        assert_ne!(Feature(1), Feature(2));
    }

    #[test]
    fn feature_into_i32() {
        let f = Feature(123);
        let val: i32 = f.into();
        assert_eq!(val, 123);
    }
}
