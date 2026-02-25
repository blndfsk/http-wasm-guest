//! Administrative utilities for configuring host features and config
//!
//! This module provides functions to enable host capabilities, read runtime
//! configuration
use crate::host::{Bytes, feature, handler};

/// Enables one or more host features and returns the host result code.
///
/// Combine feature flags with bitwise OR (e.g., `BufferRequest | BufferResponse`)
/// to enable multiple capabilities in a single call. The return value is the
/// host-provided status code.
pub fn enable(feature: feature::Feature) -> i32 {
    handler::enable_feature(feature.into())
}

/// Returns the raw configuration bytes provided by the host.
///
/// The host controls the configuration payload; interpret it according to your
/// plugin's configuration format (for example, JSON or protobuf).
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
        let config_str = config.to_str().unwrap();
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
