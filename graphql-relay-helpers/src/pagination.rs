use juniper::GraphQLObject;

/// Represents the Relay spec pagination object
/// <https://relay.dev/docs/guides/graphql-server-specification/>
///
#[derive(Debug, GraphQLObject, Eq, PartialEq, Clone)]
#[graphql(description = "Pagination information")]
pub struct PageInfo {
    /// Indicates whether there is a page following this current one
    #[graphql(description = "Indicates whether there is a page following this current one")]
    pub has_next_page: bool,

    /// Indicates whether there is a page preceding this one
    #[graphql(description = "Indicates whether there is a page preceding this one")]
    pub has_prev_page: bool,

    /// An opaque cursor that when passed to after: in a query will return the previous page of
    /// results.
    #[graphql(description = "An opaque cursor that when passed to after: in a query will return the previous page of results.")]
    pub start_cursor: Option<String>,

    /// An opaque cursor that when passed to after: in a query will return the following page of
    /// results.
    #[graphql(description = "An opaque cursor that when passed to after: in a query will return the following page of results.")]
    pub end_cursor: Option<String>,
}
