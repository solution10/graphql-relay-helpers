//! Library to help with working with the Relay specification. Contains derive macros for generating
//! connection and edge structs, as well as structs for the Pagination information and handling
//! of Relay cursors and Identifiers.
//!
//! # Pagination helpers
//!
//! The library contains a few helper functions to help with pagination.
//!
//! ## PageInfo
//!
//! The PageInfo struct is a ready to use GraphQLObject that conforms to the Relay spec.
//!
//! ## Cursors
//!
//! Relay requires edges and pagination info to contain opaque strings called "cursors".
//! This library provides a few built-in cursors, but you can also implement your own.
//!
//! The most simple cursor is the OffsetCursor, which is just an offset and a limit, similar to
//! SQL LIMIT and OFFSET.
//!
//! ```
//! # use graphql_relay_helpers::{Cursor, OffsetCursor};
//! #
//! # fn cursors() {
//! let cursor = OffsetCursor { offset: 1, first: 10 };
//!
//! // Encode the cursor into a string of format "offset:1:10"
//! let cursor_string = cursor.to_raw_string();
//!
//! // Encode the raw string into a base64 encoded string
//! let encoded_string = cursor.to_encoded_string();
//!
//! // You can also decode the cursor from the base64 encoded string
//! let decoded_cursor = OffsetCursor::from_encoded_string(&encoded_string).unwrap();
//! #
//! # }
//! ```
//!
//! Implementing your own cursor is as simple as implementing the `Cursor` trait.
//!
//! ## Cursor providers
//!
//! Relay requires edges and pagination info to contain cursors, which can be annoying to generate
//! and add to the connection.
//!
//! `CursorProvider` is a trait that allows you to easily generate cursors for each of the items
//! in the result set.
//!
//! todo().
//!
//! # Code Generation
//!
//! ## RelayConnection
//!
//! Derive macro to generate the connection and edge structs for a given object.
//!
//! The object that this is applied to must also `#[derive(juniper::GraphQLObject)]` otherwise you'll
//! get a compilation error.
//!
//! Given the following struct:
//!
//! ```
//! use juniper::GraphQLObject;
//! # use graphql_relay_helpers_codegen::{RelayConnection};
//! # use graphql_relay_helpers::PageInfo;
//!
//! #[derive(Debug, GraphQLObject, RelayConnection, Clone, Eq, PartialEq)]
//! struct PlayableCharacter {
//!     pub name: String,
//!     pub theme_song: String,
//! }
//! ```
//!
//! The `RelayConnection` macro will generate two additional structs:
//!
//! ```rust
//! # use juniper::GraphQLObject;
//! # use graphql_relay_helpers::PageInfo;
//! #
//! # #[derive(GraphQLObject)]
//! # struct PlayableCharacter {
//! #     pub name: String,
//! #     pub theme_song: String,
//! # }
//! #
//! // Generated structs:
//! #[derive(GraphQLObject)]
//! struct PlayableCharacterRelayConnection {
//!     count: i32,
//!     edges: Vec<PlayableCharacterRelayEdge>,
//!     page_info: PageInfo
//! }
//!
//! #[derive(GraphQLObject)]
//! struct PlayableCharacterRelayEdge {
//!     cursor: String,
//!     node: PlayableCharacter,
//! }
//!
//! ```
//!
//! With the following types generated for the GraphQL schema:
//!
//! ```graphql
//! type PlayableCharacterConnection {
//!     count: Int!
//!     edges: [PlayableCharacterEdge]!
//!     pageInfo: PageInfo!
//! }
//!
//! type PlayableCharacterEdge {
//! 	cursor: String!
//! 	node: PlayableCharacter!
//! }
//! ```
//!
//!

mod pagination;
mod cursors;
mod cursor_errors;

// From other crates in the workspace:
pub use graphql_relay_helpers_codegen::*;

// From this crate:
pub use pagination::*;
pub use cursors::*;
pub use cursor_errors::*;
