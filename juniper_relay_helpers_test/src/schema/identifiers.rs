use std::fmt::{Display, Formatter};
use std::str::FromStr;
use juniper_relay_helpers::IdentifierTypeDiscriminator;

/// Relay identifier types for this application
#[derive(Debug, Clone, Copy, PartialEq, Eq, IdentifierTypeDiscriminator)]
pub enum EntityType {
    Character,
    Location
}
