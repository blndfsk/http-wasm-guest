use super::handler;
use std::ops::BitOr;
#[allow(non_upper_case_globals, non_snake_case)]
/// Enables request body buffering in the host.
pub const BufferRequest: Feature = Feature(1);
#[allow(non_upper_case_globals, non_snake_case)]
/// Enables response body buffering in the host.
pub const BufferResponse: Feature = Feature(2);
#[allow(non_upper_case_globals, non_snake_case)]
/// Enables HTTP trailer support in the host.
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
