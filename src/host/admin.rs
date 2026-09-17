//! Administrative utilities for host interaction in http-wasm guest plugins.
//!
//! This module provides functions to enable host capabilities and read runtime
//! configuration.
use crate::host::{Bytes, feature, handler};

/// Enables one or more host features and returns the host's full feature bitflag.
///
/// Combine feature flags with bitwise OR (e.g., `BufferRequest | BufferResponse`)
/// to enable multiple capabilities in a single call. The return value is the
/// complete bitflag of features the host supports, so check it against your
/// request to see which features were actually enabled.
///
/// Calling this during `handle_request` enables features only for the current
/// request; call it before returning from `handle_request`, or during
/// initialization to fail fast on hosts that lack features your plugin requires.
pub fn enable(feature: feature::Feature) -> i32 {
    handler::enable_feature(feature.into())
}

/// Returns the raw configuration bytes provided by the host.
///
/// The host controls the configuration payload; interpret it according to your
/// plugin's configuration format (for example, JSON or protobuf). Note that the
/// payload is guest-specific and not necessarily UTF-8 encoded, and that a host
/// which fails to retrieve the configuration will trap.
///
/// Costs one host call into the shared 2048-byte buffer plus one heap allocation
/// for the returned [`Bytes`] (none if the configuration is empty).
pub fn config() -> Bytes {
    Bytes::from(handler::get_config())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admin_config() {
        let config = config();
        // The mock returns JSON-like config
        let config_str = String::from_utf8_lossy(&config);
        assert!(config_str.contains("config"));
        assert!(config_str.contains("test1"));
    }

    #[test]
    fn admin_enable_feature() {
        // Should not panic - mock handles feature enablement
        let _result = enable(feature::BufferRequest);
    }

    #[test]
    fn admin_enable_combined_features() {
        let combined = feature::BufferRequest | feature::BufferResponse;
        let _result = enable(combined);
    }
}
