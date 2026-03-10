use serde::{Serialize, Deserialize};
use crate::{BasisPoints, VerityError};

/// All money in XAP is integer minor units. Never floating point.
/// Invariant 7: Money uses integer minor units only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    pub amount_minor_units: i64,
    pub currency: Currency,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Currency {
    USD,
    EUR,
    GBP,
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Currency::USD => write!(f, "USD"),
            Currency::EUR => write!(f, "EUR"),
            Currency::GBP => write!(f, "GBP"),
        }
    }
}

impl Money {
    pub fn new(amount_minor_units: i64, currency: Currency) -> Self {
        Self { amount_minor_units, currency }
    }

    pub fn add(&self, other: &Money) -> Result<Money, VerityError> {
        if self.currency != other.currency {
            return Err(VerityError::CurrencyMismatch(
                self.currency.clone(),
                other.currency.clone(),
            ));
        }
        let amount = self.amount_minor_units
            .checked_add(other.amount_minor_units)
            .ok_or(VerityError::ArithmeticOverflow)?;
        Ok(Money::new(amount, self.currency.clone()))
    }

    pub fn subtract(&self, other: &Money) -> Result<Money, VerityError> {
        if self.currency != other.currency {
            return Err(VerityError::CurrencyMismatch(
                self.currency.clone(),
                other.currency.clone(),
            ));
        }
        let amount = self.amount_minor_units
            .checked_sub(other.amount_minor_units)
            .ok_or(VerityError::ArithmeticOverflow)?;
        Ok(Money::new(amount, self.currency.clone()))
    }

    /// Split by basis points. Shares must sum to 10000.
    /// Deterministic remainder allocation: first payee gets remainder.
    pub fn split_bps(&self, shares_bps: &[BasisPoints]) -> Result<Vec<Money>, VerityError> {
        crate::validate_shares(shares_bps)?;

        let total = self.amount_minor_units;
        let mut results: Vec<Money> = shares_bps
            .iter()
            .map(|bp| {
                let amount = total * i64::from(bp.value()) / 10000;
                Money::new(amount, self.currency.clone())
            })
            .collect();

        let distributed: i64 = results.iter().map(|m| m.amount_minor_units).sum();
        let remainder = total - distributed;
        if remainder != 0 {
            if let Some(first) = results.first_mut() {
                first.amount_minor_units += remainder;
            }
        }

        Ok(results)
    }

    /// Apply a basis point modifier (e.g., 9788 bps = 97.88%)
    pub fn apply_modifier_bps(&self, modifier_bps: BasisPoints) -> Money {
        let amount = self.amount_minor_units * i64::from(modifier_bps.value()) / 10000;
        Money::new(amount, self.currency.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_same_currency() {
        let a = Money::new(500, Currency::USD);
        let b = Money::new(300, Currency::USD);
        let result = a.add(&b).unwrap();
        assert_eq!(result.amount_minor_units, 800);
        assert_eq!(result.currency, Currency::USD);
    }

    #[test]
    fn test_add_different_currency_fails() {
        let a = Money::new(500, Currency::USD);
        let b = Money::new(300, Currency::EUR);
        assert!(a.add(&b).is_err());
    }

    #[test]
    fn test_subtract() {
        let a = Money::new(500, Currency::USD);
        let b = Money::new(300, Currency::USD);
        let result = a.subtract(&b).unwrap();
        assert_eq!(result.amount_minor_units, 200);
    }

    #[test]
    fn test_overflow_detection() {
        let a = Money::new(i64::MAX, Currency::USD);
        let b = Money::new(1, Currency::USD);
        assert!(a.add(&b).is_err());
    }

    #[test]
    fn test_split_bps() {
        let total = Money::new(10000, Currency::USD);
        let shares = vec![
            BasisPoints::new(6000).unwrap(),
            BasisPoints::new(2500).unwrap(),
            BasisPoints::new(1500).unwrap(),
        ];
        let splits = total.split_bps(&shares).unwrap();
        assert_eq!(splits[0].amount_minor_units, 6000);
        assert_eq!(splits[1].amount_minor_units, 2500);
        assert_eq!(splits[2].amount_minor_units, 1500);
    }

    #[test]
    fn test_split_remainder_goes_to_first() {
        let total = Money::new(1000, Currency::USD);
        let shares = vec![
            BasisPoints::new(3333).unwrap(),
            BasisPoints::new(3333).unwrap(),
            BasisPoints::new(3334).unwrap(),
        ];
        let splits = total.split_bps(&shares).unwrap();
        let sum: i64 = splits.iter().map(|m| m.amount_minor_units).sum();
        assert_eq!(sum, 1000);
        // First payee gets remainder
        assert_eq!(splits[0].amount_minor_units, 334);
        assert_eq!(splits[1].amount_minor_units, 333);
        assert_eq!(splits[2].amount_minor_units, 333);
    }

    #[test]
    fn test_apply_modifier_bps() {
        let m = Money::new(10000, Currency::USD);
        let modifier = BasisPoints::new(9788).unwrap();
        let result = m.apply_modifier_bps(modifier);
        assert_eq!(result.amount_minor_units, 9788);
    }
}
