use serde::{Serialize, Deserialize};
use crate::VerityError;

/// Basis points: 0-10000. Used for shares, modifiers, confidence, rates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BasisPoints(u16);

impl BasisPoints {
    pub fn new(value: u16) -> Result<Self, VerityError> {
        if value > 10000 {
            return Err(VerityError::BasisPointsOutOfRange(value));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> u16 {
        self.0
    }
}

impl std::fmt::Display for BasisPoints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} bps", self.0)
    }
}

/// Validate that a set of basis point shares sums to exactly 10000.
pub fn validate_shares(shares: &[BasisPoints]) -> Result<(), VerityError> {
    let sum: u32 = shares.iter().map(|bp| u32::from(bp.value())).sum();
    if sum != 10000 {
        return Err(VerityError::SharesSumInvalid(sum));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_range() {
        assert!(BasisPoints::new(0).is_ok());
        assert!(BasisPoints::new(5000).is_ok());
        assert!(BasisPoints::new(10000).is_ok());
    }

    #[test]
    fn test_out_of_range() {
        assert!(BasisPoints::new(10001).is_err());
        assert!(BasisPoints::new(u16::MAX).is_err());
    }

    #[test]
    fn test_shares_sum_valid() {
        let shares = vec![
            BasisPoints::new(6000).unwrap(),
            BasisPoints::new(4000).unwrap(),
        ];
        assert!(validate_shares(&shares).is_ok());
    }

    #[test]
    fn test_shares_sum_invalid() {
        let shares = vec![
            BasisPoints::new(6000).unwrap(),
            BasisPoints::new(3000).unwrap(),
        ];
        assert!(validate_shares(&shares).is_err());
    }
}
