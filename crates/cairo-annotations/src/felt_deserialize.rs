use serde::de::Error;
use serde::{Deserialize, Deserializer};
use starknet_types_core::felt::Felt;

/// Custom deserialization function for [`Felt`] to allow legacy support for old `snforge` serialization.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Felt, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum FeltDeserialize {
        StandardDeserialization(Felt),
        Decimal(String),
    }

    match FeltDeserialize::deserialize(deserializer)? {
        FeltDeserialize::StandardDeserialization(felt) => Ok(felt),
        FeltDeserialize::Decimal(s) => Felt::from_dec_str(&s).map_err(Error::custom),
    }
}
