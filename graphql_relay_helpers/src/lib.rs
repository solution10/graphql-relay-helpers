//! Library to help with working with the Relay specification. Contains derive macros for generating
//! connection and edge structs, as well as structs for the Pagination information and handling
//! of Relay cursors and Identifiers.
//!
//! # Macros
//!
//! ## RelayConnection
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
//!
//! # #[derive(GraphQLObject)]
//! # struct PlayableCharacter {
//! #     pub name: String,
//! #     pub theme_song: String,
//! # }
//!
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

mod pagination;

// From other crates in the workspace:
pub use graphql_relay_helpers_codegen::*;

// From this crate:
pub use pagination::*;
