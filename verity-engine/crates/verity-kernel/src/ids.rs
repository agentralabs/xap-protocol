use serde::{Serialize, Deserialize, Serializer, Deserializer};
use crate::VerityError;

macro_rules! define_id {
    ($name:ident, $prefix:expr, $hex_len:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: &str) -> Result<Self, VerityError> {
                let prefix = concat!($prefix, "_");
                if !value.starts_with(prefix) {
                    return Err(VerityError::InvalidIdFormat(format!(
                        "{} must start with '{}'",
                        stringify!($name),
                        prefix
                    )));
                }
                let hex_part = &value[prefix.len()..];
                if hex_part.len() != $hex_len {
                    return Err(VerityError::InvalidIdFormat(format!(
                        "{} hex part must be {} characters, got {}",
                        stringify!($name),
                        $hex_len,
                        hex_part.len()
                    )));
                }
                if !hex_part.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()) {
                    return Err(VerityError::InvalidIdFormat(format!(
                        "{} hex part must be lowercase hex",
                        stringify!($name)
                    )));
                }
                Ok(Self(value.to_string()))
            }

            pub fn generate() -> Self {
                let hex_bytes: Vec<u8> = (0..$hex_len / 2)
                    .map(|_| rand::random::<u8>())
                    .collect();
                let hex_str = hex::encode(&hex_bytes);
                Self(format!("{}_{}", $prefix, hex_str))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(&self.0)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let s = String::deserialize(deserializer)?;
                $name::new(&s).map_err(serde::de::Error::custom)
            }
        }
    };
}

define_id!(AgentId, "agent", 8);
define_id!(SettlementId, "stl", 8);
define_id!(ReceiptId, "rcpt", 8);
define_id!(VerityId, "vrt", 8);
define_id!(ContractId, "neg", 8);
define_id!(QueryId, "qry", 8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ids() {
        assert!(AgentId::new("agent_7f3a9b2c").is_ok());
        assert!(SettlementId::new("stl_4b7c9e2f").is_ok());
        assert!(ReceiptId::new("rcpt_6d1e8f3a").is_ok());
        assert!(VerityId::new("vrt_a1b2c3d4").is_ok());
        assert!(ContractId::new("neg_8a2f4c1d").is_ok());
        assert!(QueryId::new("qry_a1b2c3d4").is_ok());
    }

    #[test]
    fn test_invalid_prefix() {
        assert!(AgentId::new("user_7f3a9b2c").is_err());
        assert!(SettlementId::new("set_4b7c9e2f").is_err());
    }

    #[test]
    fn test_invalid_hex() {
        assert!(AgentId::new("agent_ZZZZZZZZ").is_err());
        assert!(AgentId::new("agent_7F3A9B2C").is_err()); // uppercase
    }

    #[test]
    fn test_invalid_length() {
        assert!(AgentId::new("agent_7f3a").is_err());
        assert!(AgentId::new("agent_7f3a9b2c00").is_err());
    }

    #[test]
    fn test_generate_produces_valid() {
        let id = AgentId::generate();
        assert!(AgentId::new(id.as_str()).is_ok());

        let id = VerityId::generate();
        assert!(VerityId::new(id.as_str()).is_ok());
    }

    #[test]
    fn test_display() {
        let id = AgentId::new("agent_7f3a9b2c").unwrap();
        assert_eq!(format!("{}", id), "agent_7f3a9b2c");
    }

    #[test]
    fn test_serde_roundtrip() {
        let id = AgentId::new("agent_7f3a9b2c").unwrap();
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"agent_7f3a9b2c\"");
        let parsed: AgentId = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, id);
    }
}
