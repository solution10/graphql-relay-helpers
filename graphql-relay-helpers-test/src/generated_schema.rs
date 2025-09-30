#[cfg(test)]
mod tests {
    use assertor::{assert_that, StringAssertion};
    use juniper::{EmptyMutation, EmptySubscription, FieldResult, GraphQLObject, RootNode};
    use graphql_relay_helpers::{RelayConnection, PageInfo};

    // ---- Define the types ----

    #[derive(Debug, GraphQLObject, Clone, Eq, PartialEq, RelayConnection)]
    struct User {
        name: String,
    }

    #[derive(Debug, GraphQLObject, Clone, Eq, PartialEq, RelayConnection)]
    struct Post {
        title: String,
    }

    // ----- Build the query root ----

    struct QueryRoot;

    #[juniper::graphql_object()]
    impl QueryRoot {
        fn get_users() -> FieldResult<UserRelayConnection> {
            Ok(UserRelayConnection {
                count: 12,
                edges: vec![
                    UserRelayEdge {
                        node: User {
                            name: "Lune".to_owned()
                        },
                        cursor: None
                    },
                    UserRelayEdge {
                        node: User {
                            name: "Sciel".to_owned()
                        },
                        cursor: Some("some-string".to_owned())
                    }
                ],
                page_info: PageInfo {
                    start_cursor: None,
                    end_cursor: None,
                    has_prev_page: false,
                    has_next_page: false
                }
            })
        }

        fn get_posts() -> FieldResult<PostRelayConnection> {
            Ok(PostRelayConnection {
                count: 0,
                edges: vec![],
                page_info: PageInfo {
                    start_cursor: None,
                    end_cursor: None,
                    has_prev_page: false,
                    has_next_page: false
                }
            })
        }
    }

    // ---- Build the schema ----

    type Schema = RootNode<QueryRoot, EmptyMutation, EmptySubscription>;
    fn build_schema() -> Schema {
        Schema::new(QueryRoot, EmptyMutation::new(), EmptySubscription::new())
    }

    #[test]
    fn print_schema_for_debugging() {
        let schema_document = build_schema();
        let schema_sdl = schema_document.as_sdl();
        println!("{}", schema_sdl);
    }

    #[test]
    fn connection_info_generated() {
        let schema_document = build_schema();
        let schema_sdl = schema_document.as_sdl();

        assert_that!(schema_sdl).contains("type UserConnection");
        assert_that!(schema_sdl).contains("Connection type for User.");

        assert_that!(schema_sdl).contains("type PostConnection");
        assert_that!(schema_sdl).contains("Connection type for Post.");
    }

    #[test]
    fn edge_info_generated() {
        let schema_document = build_schema();
        let schema_sdl = schema_document.as_sdl();

        assert_that!(schema_sdl).contains("type UserEdge");
        assert_that!(schema_sdl).contains("Edge type for User.");

        assert_that!(schema_sdl).contains("type PostEdge");
        assert_that!(schema_sdl).contains("Edge type for Post.");
    }

    #[test]
    fn pagination_info_generated() {
        let schema_document = build_schema();
        let schema_sdl = schema_document.as_sdl();

        assert_that!(schema_sdl).contains("type PageInfo");
        assert_that!(schema_sdl).contains("Pagination information");
    }
}
