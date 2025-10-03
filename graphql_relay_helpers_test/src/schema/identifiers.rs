use std::fmt::{Display, Formatter};
use std::str::FromStr;
use graphql_relay_helpers::IdentifierTypeDiscriminator;

/// Relay identifier types for this application
#[derive(Debug, Clone, Copy, PartialEq, Eq, IdentifierTypeDiscriminator)]
pub enum EntityType {
    Character,
    Location
}
