use juniper::{EmptyMutation, EmptySubscription, FieldResult, RootNode};
use juniper_relay_helpers::{OffsetCursor, PageInfo, RelayIdentifier};
pub use crate::schema::character::{Character, CharacterRelayConnection, CharacterRelayEdge, CharacterRow};
pub use crate::schema::identifiers::EntityType;
pub use crate::schema::location::{Location, LocationRelayConnection, LocationRelayEdge, LocationRow};

mod identifiers;
mod character;
mod location;

pub use crate::schema::character::get_character_test_data;
pub use crate::schema::location::get_location_test_data;

// ---------- Context -------------

#[derive(Clone)]
pub struct Context {
    pub characters: Vec<CharacterRow>,
    pub locations: Vec<LocationRow>,
}
impl juniper::Context for Context {}

// --------- QueryRoot ------------

pub struct QueryRoot;

#[juniper::graphql_object(context = Context)]
impl QueryRoot {
    /// Queries for all characters in the "database"
    async fn characters(ctx: &Context) -> FieldResult<CharacterRelayConnection> {
        Ok(CharacterRelayConnection {
            count: ctx.characters.len() as i32,
            edges: ctx.characters.iter().enumerate().map(|(idx, row)| {
                    CharacterRelayEdge {
                        node: Character {
                            id: RelayIdentifier::new(row.id, EntityType::Character),
                            name: row.name.clone()
                        },
                        cursor: Some(
                            OffsetCursor { offset: idx as i32, first: 10 }.to_string()
                        )
                    }
                }).collect()
            ,
            page_info: PageInfo {
                has_next_page: false,
                has_prev_page: false,
                start_cursor: None,
                end_cursor: None
            }
        })
    }

    /// Queries for all locations in the "database"
    async fn locations(ctx: &Context) -> FieldResult<LocationRelayConnection> {
        Ok(LocationRelayConnection {
            count: ctx.locations.len() as i32,
            edges: ctx.locations.iter().enumerate().map(|(idx, row)| {
                LocationRelayEdge {
                    node: Location {
                        // Note that this is a _string_ as the identifier type, not Uuid!
                        id: RelayIdentifier::new(row.id.clone(), EntityType::Location),
                        name: row.name.clone()
                    },
                    cursor: Some(
                        OffsetCursor { offset: idx as i32, first: 10 }.to_string()
                    )
                }
            }).collect()
            ,
            page_info: PageInfo {
                has_next_page: false,
                has_prev_page: false,
                start_cursor: None,
                end_cursor: None
            }
        })
    }
}

// ---------- Schema -------------

pub type Schema = RootNode<QueryRoot, EmptyMutation<Context>, EmptySubscription<Context>>;
