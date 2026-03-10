use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use crate::VerityError;

/// Canonical timestamps. Always UTC. Never local time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalTimestamp(DateTime<Utc>);

impl CanonicalTimestamp {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn from_rfc3339(s: &str) -> Result<Self, VerityError> {
        let dt = DateTime::parse_from_rfc3339(s)
            .map_err(|e| VerityError::InvalidTimestamp(e.to_string()))?;
        Ok(Self(dt.with_timezone(&Utc)))
    }

    pub fn to_rfc3339(&self) -> String {
        self.0.to_rfc3339()
    }

    pub fn elapsed_since(&self, earlier: &CanonicalTimestamp) -> chrono::Duration {
        self.0 - earlier.0
    }

    pub fn is_expired(&self) -> bool {
        self.0 < Utc::now()
    }

    pub fn inner(&self) -> &DateTime<Utc> {
        &self.0
    }
}

impl Serialize for CanonicalTimestamp {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_rfc3339())
    }
}

impl<'de> Deserialize<'de> for CanonicalTimestamp {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        CanonicalTimestamp::from_rfc3339(&s).map_err(serde::de::Error::custom)
    }
}

impl std::fmt::Display for CanonicalTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_rfc3339())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now_produces_valid() {
        let ts = CanonicalTimestamp::now();
        let s = ts.to_rfc3339();
        assert!(CanonicalTimestamp::from_rfc3339(&s).is_ok());
    }

    #[test]
    fn test_rfc3339_roundtrip() {
        let input = "2026-03-15T14:30:00+00:00";
        let ts = CanonicalTimestamp::from_rfc3339(input).unwrap();
        let output = ts.to_rfc3339();
        let ts2 = CanonicalTimestamp::from_rfc3339(&output).unwrap();
        assert_eq!(ts, ts2);
    }

    #[test]
    fn test_invalid_timestamp() {
        assert!(CanonicalTimestamp::from_rfc3339("not-a-date").is_err());
    }

    #[test]
    fn test_expiry_check() {
        let past = CanonicalTimestamp::from_rfc3339("2020-01-01T00:00:00+00:00").unwrap();
        assert!(past.is_expired());

        let future = CanonicalTimestamp::from_rfc3339("2099-01-01T00:00:00+00:00").unwrap();
        assert!(!future.is_expired());
    }

    #[test]
    fn test_elapsed_since() {
        let t1 = CanonicalTimestamp::from_rfc3339("2026-03-15T14:00:00+00:00").unwrap();
        let t2 = CanonicalTimestamp::from_rfc3339("2026-03-15T14:30:00+00:00").unwrap();
        let elapsed = t2.elapsed_since(&t1);
        assert_eq!(elapsed.num_minutes(), 30);
    }
}
