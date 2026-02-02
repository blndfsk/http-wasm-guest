use super::handler;
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
/// Feature flag value used to configure host capabilities.
pub struct Feature(pub i32);
impl BitOr for Feature {
    type Output = Feature;

    fn bitor(self, rhs: Self) -> Feature {
        Feature(self.0 | rhs.0)
    }
}
/// Enables one or more host features and returns the host result code.
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
