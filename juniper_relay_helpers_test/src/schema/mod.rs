pub use crate::schema::character::{
    Character, CharacterRelayConnection, CharacterRelayEdge, CharacterRow,
};
pub use crate::schema::identifiers::EntityType;
pub use crate::schema::location::{Location, LocationRelayConnection, LocationRow};
use juniper::{EmptyMutation, EmptySubscription, FieldResult, RootNode};
use juniper_relay_helpers::{
    OffsetCursor, OffsetCursorProvider, PageInfo, PageRequest, RelayConnection, RelayEdge,
    RelayIdentifier,
};

mod character;
mod identifiers;
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
    /// This method shows how you can manually build up the resulting structs without using
    /// cursor providers or any of the other fancy stuff.
    async fn characters(ctx: &Context) -> FieldResult<CharacterRelayConnection> {
        Ok(CharacterRelayConnection {
            count: ctx.characters.len() as i32,
            edges: ctx
                .characters
                .iter()
                .enumerate()
                .map(|(idx, row)| {
                    CharacterRelayEdge::new(
                        Character {
                            id: RelayIdentifier::new(row.id, EntityType::Character),
                            name: row.name.clone(),
                        },
                        OffsetCursor {
                            offset: idx as i32,
                            first: Some(10),
                        },
                    )
                })
                .collect(),
            page_info: PageInfo {
                has_next_page: false,
                has_prev_page: false,
                start_cursor: None,
                end_cursor: None,
            },
        })
    }

    /// Queries for all locations in the "database"
    /// This method makes use of cursor providers and the shortcut methods to show how much you can
    /// hand off to the library:
    async fn locations(
        first: Option<i32>,
        after: Option<OffsetCursor>,
        ctx: &Context,
    ) -> FieldResult<LocationRelayConnection> {
        let mut nodes = ctx
            .locations
            .iter()
            .map(|row| Location::from(row.clone()))
            .collect::<Vec<Location>>();

        if let Some(after) = &after {
            nodes = nodes.split_off(after.offset as usize + 1);
        }

        if let Some(first) = first {
            nodes.truncate(first as usize);
        }

        Ok(LocationRelayConnection::new(
            &nodes,
            ctx.locations.len() as i32,
            OffsetCursorProvider::new(),
            Some(PageRequest::new(first, after)),
        ))
    }
}

// ---------- Schema -------------

pub type Schema = RootNode<QueryRoot, EmptyMutation<Context>, EmptySubscription<Context>>;
