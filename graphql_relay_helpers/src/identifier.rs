use std::fmt::Display;
use std::str::FromStr;
use base64::prelude::*;
use juniper::{GraphQLScalar, ParseScalarResult, ParseScalarValue, ScalarToken, ScalarValue};

const SEGMENT_DELIMITER: &str = "::";

/// Relay identifiers need to be globally unique. It's often useful to have a type delimiter in there
/// too, so this is a struct to help with that!
///
/// This struct can serialize down to a GraphQLScalar of type ID in Juniper, so you can simply build
/// and return it as part of your field resolvers / DTOs.
#[derive(Debug, GraphQLScalar, Clone, Eq, PartialEq, Hash)]
#[graphql(
    name = "ID",
    to_output_with = Self::to_output,
    from_input_with = Self::from_input,
    parse_token_with = Self::parse_token
)]
pub struct RelayIdentifier<T, TD> where T: Display, T: FromStr, TD: Display, TD: FromStr {
    pub id: T,
    pub type_delimiter: TD,
}

/// Implement Display
impl<T, TD> Display for RelayIdentifier<T, TD> where T: Display, T: FromStr, TD: Display, TD: FromStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.type_delimiter, SEGMENT_DELIMITER, self.id)
    }
}

/// Common implementation for all identifiers
impl<T, TD> RelayIdentifier<T, TD> where T: Display, T: FromStr, TD: Display, TD: FromStr {
    /// General constructor
    pub fn new(id: T, type_delimiter: TD) -> Self {
        Self { id, type_delimiter }
    }

    pub fn to_encoded_string(&self) -> String {
        BASE64_URL_SAFE.encode(self.to_string())
    }

    // ---------- GraphQLScalar implementation ----------

    pub fn to_output(&self) -> juniper::ID {
        juniper::ID::from(self.to_encoded_string())
    }

    pub fn from_input(input: &str) -> Result<Self, Box<str>> {
        // Input is a base64 encoded string, so we need to decode it first
        let decoded_bytes = BASE64_URL_SAFE.decode(input)
            .map_err(|err| format!("Invalid base64 encoding: {}", err))?;

        let decoded_string = String::from_utf8(decoded_bytes)
            .map_err(|err| format!("Invalid UTF-8 encoding: {}", err))?;

        let parts = decoded_string.split(SEGMENT_DELIMITER).collect::<Vec<&str>>();

        if parts.len() != 2 {
            return Err("Invalid Relay identifier".into());
        }

        let identifier_part = T::from_str(parts[1])
            .map_err(|_| "Invalid identifier")?;

        let type_delimiter_part = TD::from_str(parts[0])
            .map_err(|_| "Invalid type delimiter")?;

        Ok(Self::new(identifier_part, type_delimiter_part))
    }

    fn parse_token<S: ScalarValue>(value: ScalarToken<'_>) -> ParseScalarResult<S> {
        <String as ParseScalarValue<S>>::from_str(value)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::{Display, Formatter};
    use std::str::FromStr;
    use base64::Engine;
    use base64::prelude::BASE64_URL_SAFE;
    use uuid::Uuid;
    use crate::identifier::RelayIdentifier;

    #[derive(PartialEq, Eq, Debug)]
    enum TestTypeDelimiter {
        Character,
        Weapon
    }
    impl Display for TestTypeDelimiter {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                TestTypeDelimiter::Character => { write!(f, "character") }
                TestTypeDelimiter::Weapon => { write!(f, "weapon") }
            }
        }
    }
    impl FromStr for TestTypeDelimiter {
        type Err = &'static str;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "character" => Ok(TestTypeDelimiter::Character),
                "weapon" => Ok(TestTypeDelimiter::Weapon),
                &_ => Err("Invalid type delimiter")
            }
        }
    }

    #[test]
    fn test_string_identifiers() {
        let id = RelayIdentifier { id: "123".to_string(), type_delimiter: TestTypeDelimiter::Character };
        assert_eq!(id.to_string(), "character::123");
    }

    #[test]
    fn test_i32_identifiers() {
        let id = RelayIdentifier::new(123, TestTypeDelimiter::Weapon);
        assert_eq!(id.to_string(), "weapon::123");
    }

    #[test]
    fn test_uuid_identifiers() {
        let uuid = Uuid::new_v4();
        let id = RelayIdentifier::new(uuid, TestTypeDelimiter::Character);
        assert_eq!(id.to_string(), format!("character::{}", uuid));
    }

    #[test]
    fn test_to_output() {
        let id = RelayIdentifier::new("123".to_string(), TestTypeDelimiter::Character);
        let output = id.to_output();
        assert_eq!(output.to_string(), "Y2hhcmFjdGVyOjoxMjM=");
    }

    #[test]
    fn test_from_input_string() {
        let input = "Y2hhcmFjdGVyOjoxMjM=";
        let identifier = RelayIdentifier::<String, TestTypeDelimiter>::from_input(input).unwrap();
        assert_eq!(identifier.id, "123");
        assert_eq!(identifier.type_delimiter, TestTypeDelimiter::Character);
    }

    #[test]
    fn test_from_input_i32() {
        let input = "d2VhcG9uOjoxMjM=";
        let identifier = RelayIdentifier::<i32, TestTypeDelimiter>::from_input(input).unwrap();
        assert_eq!(identifier.id, 123);
        assert_eq!(identifier.type_delimiter, TestTypeDelimiter::Weapon);
    }

    #[test]
    fn test_from_input_uuid() {
        let input = "Y2hhcmFjdGVyOjo3Mzk2YWEyZi0wM2RmLTQyZDYtYWFlMS1jZjBlOTE4MmYwZDI=";
        let identifier = RelayIdentifier::<Uuid, TestTypeDelimiter>::from_input(input).unwrap();
        assert_eq!(identifier.id, Uuid::parse_str("7396aa2f-03df-42d6-aae1-cf0e9182f0d2").unwrap());
        assert_eq!(identifier.type_delimiter, TestTypeDelimiter::Character);
    }

    #[test]
    fn test_from_invalid_base64() {
        let input = "Y2hhcmFjdGVyOjo3Mzk2YWEyZi0wM2RmLTQyZDYtYWFlMS1jZjBlOTE4MmYwZDI";
        let result = RelayIdentifier::<Uuid, TestTypeDelimiter>::from_input(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Invalid base64 encoding: Invalid padding");
    }

    #[test]
    fn test_from_invalid_utf8() {
        let input = BASE64_URL_SAFE.encode(vec![0x80]);
        let result = RelayIdentifier::<Uuid, TestTypeDelimiter>::from_input(&input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Invalid UTF-8 encoding: invalid utf-8 sequence of 1 bytes from index 0");
    }

    #[test]
    fn test_invalid_identifier_format() {
        let input = BASE64_URL_SAFE.encode("character//123".to_string());
        let result = RelayIdentifier::<String, TestTypeDelimiter>::from_input(&input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Invalid Relay identifier");
    }
}
