use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum RatingError {
    #[snafu(display("Rating must be between 0 and 5, but was {rating}"))]
    InvalidRating { rating: f32 },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rating(u8);

impl Rating {
    pub fn new(rating: f32) -> Result<Self, RatingError> {
        ensure!((0.0..=5.0).contains(&rating), InvalidRatingSnafu { rating });
        let adjusted_rating = rating * 10.0;
        let rating_candidate = adjusted_rating.round() as u8;
        Ok(Self(rating_candidate))
    }

    pub fn rating(&self) -> f32 {
        self.0 as f32 / 10.0
    }
}

#[cfg(test)]
mod rating_tests {
    use super::*;

    mod proptests {
        use super::*;
        use proptest::num::f32::{ANY, NEGATIVE};
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn doesnt_crash(rating in ANY) {
                let _rating = Rating::new(rating);
            }

            #[test]
            fn rating_is_valid(rating in 0.0f32..=5.0f32) {
                let parsed_rating = Rating::new(rating);
                assert!(parsed_rating.is_ok());
                assert_eq!(parsed_rating.unwrap().rating(), (rating * 10.0).round() / 10.0);
            }

            #[test]
            fn negative_ratings_are_invalid(rating in NEGATIVE) {
                let parsed_rating = Rating::new(rating);
                assert!(parsed_rating.is_err());
            }

            #[test]
            fn ratings_over_five_are_invalid(rating in 5.0f32..) {
                let parsed_rating = Rating::new(rating);
                assert!(parsed_rating.is_err());
            }

        }
    }

    #[test]
    fn zero_rating_is_valid() {
        let rating = Rating::new(0.0);
        assert!(rating.is_ok());
        assert_eq!(rating.unwrap().rating(), 0.0);
    }

    #[test]
    fn long_valid_decimal_is_valid() {
        let rating = Rating::new(4.999999);
        assert!(rating.is_ok());
        assert_eq!(rating.unwrap().rating(), 5.0);
    }

    #[test]
    fn long_invalid_float_is_invalid() {
        let rating = Rating::new(5.000001);
        assert!(rating.is_err());
    }

    #[test]
    fn five_rating_is_valid() {
        let rating = Rating::new(5.0);
        assert!(rating.is_ok());
        assert_eq!(rating.unwrap().rating(), 5.0);
    }
}
