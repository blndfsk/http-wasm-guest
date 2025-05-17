use crate::host::handler;

#[allow(non_upper_case_globals, non_snake_case)]
/**
 * https://http-wasm.io/http-handler-abi/#features
 */
pub mod Feature {
    use crate::Feature;
    pub const BufferRequest: Feature = Feature(1);
    pub const BufferResponse: Feature = Feature(2);
    pub const Trailers: Feature = Feature(4);
}
/**
 * enables the specified Features on the host.
 *
 * https://http-wasm.io/http-handler-abi/#enable_features
 *
 */
pub fn enable_feature(feature: crate::Feature) -> i32 {
    handler::enable_feature(feature.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_or() {
        assert_eq!(
            Feature::BufferRequest | Feature::BufferResponse,
            crate::Feature(3)
        );
    }
}
