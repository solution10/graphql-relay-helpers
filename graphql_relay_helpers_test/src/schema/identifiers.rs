use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Relay identifier types for this application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    Character,
    Location
}

impl Display for EntityType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityType::Character => { write!(f, "character") }
            EntityType::Location => { write!(f, "location") }
        }
    }
}

// Your type also needs to implement FromStr trait so that we can decode correctly.
impl FromStr for EntityType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
       match s {
            "character" => Ok(EntityType::Character),
            "enemy" => Ok(EntityType::Location),
            &_ => Err("Invalid type delimiter")
       }
    }
}
