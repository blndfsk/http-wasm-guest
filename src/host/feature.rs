use super::handler;
use std::ops::BitOr;

/**
 * https://http-wasm.io/http-handler-abi/#features
 */
#[allow(non_upper_case_globals, non_snake_case)]
pub const BufferRequest: Feature = Feature(1);

#[allow(non_upper_case_globals, non_snake_case)]
pub const BufferResponse: Feature = Feature(2);

#[allow(non_upper_case_globals, non_snake_case)]
pub const Trailers: Feature = Feature(4);

/**
 * enables the specified Features on the host.
 *
 * https://http-wasm.io/http-handler-abi/#enable_features
 *
 */
#[derive(Debug, PartialEq)]
pub struct Feature(pub i32);
impl BitOr for Feature {
    type Output = Feature;

    fn bitor(self, rhs: Self) -> Feature {
        Feature(self.0 | rhs.0)
    }
}
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
