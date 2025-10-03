use juniper::GraphQLObject;
use uuid::Uuid;
use graphql_relay_helpers::{RelayConnection, RelayIdentifier, PageInfo};

use crate::schema::identifiers::{EntityType};

/// "Database" row for a character.
#[derive(Clone)]
pub struct CharacterRow {
    pub id: Uuid,
    pub name: String,
}

/// GraphQL type for a character.
#[derive(GraphQLObject, RelayConnection, Debug, Eq, PartialEq, Clone)]
pub struct Character {
    pub id: RelayIdentifier<Uuid, EntityType>,
    pub name: String,
}


// ----------- Test data ------------------

pub fn get_character_test_data() -> Vec<CharacterRow> {
    vec![
        CharacterRow {
            id: Uuid::new_v4(),
            name: "Lune".to_string()
        },
        CharacterRow {
            id: Uuid::new_v4(),
            name: "Sciel".to_string()
        },
        CharacterRow {
            id: Uuid::new_v4(),
            name: "Maelle".to_string()
        },
        CharacterRow {
            id: Uuid::new_v4(),
            name: "Gustave".to_string()
        },
        CharacterRow {
            id: Uuid::new_v4(),
            name: "Monoco".to_string()
        },
    ]
}
