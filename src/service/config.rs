//! FuwaNe - Config

use std::fmt::{ Formatter, Result as FmtResult };

use serde::{ Serialize, Deserialize, Serializer, Deserializer, de::Visitor };
use bitflags::bitflags;


bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Intents: u8 {
        const CLIENT_DISCONNECT = 0b0001;
    }
}

impl Serialize for Intents {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        serializer.serialize_u8(self.bits())
    }
}

struct IntentsVisitor;
impl<'de> Visitor<'de> for IntentsVisitor {
    type Value = Intents;
    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("Must be integer u8 type.")
    }
    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where E: serde::de::Error, {
        Ok(Intents::from_bits(v).unwrap())
    }
}

impl<'de> Deserialize<'de> for Intents {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        deserializer.deserialize_u8(IntentsVisitor)
    }
}


#[derive(Serialize, Deserialize)]
pub struct Config {
    pub wasi: bool,
    pub intents: Intents
}