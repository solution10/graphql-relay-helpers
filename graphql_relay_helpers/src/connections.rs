#[cfg(test)]
mod tests {
    use juniper::GraphQLObject;
    use graphql_relay_helpers_codegen::RelayConnection;
    use crate::PageInfo;

    #[derive(Debug, RelayConnection, GraphQLObject, Clone, Eq, PartialEq)]
    struct User {
        name: String,
    }

    #[test]
    fn connection_types_are_generated() {
        let conn = UserRelayConnection {
            count: 12,
            edges: vec![],
            page_info: PageInfo {
                start_cursor: None,
                end_cursor: None,
                has_prev_page: false,
                has_next_page: false
            }
        };

        assert_eq!(conn.count, 12);
        assert_eq!(conn.edges.len(), 0);
    }

    #[test]
    fn edge_types_are_generated() {
        let edge = UserRelayEdge {
            node: User {
                name: "Lune".to_owned(),
            },
            cursor: Some("some-string".to_owned())
        };
        assert_eq!(edge.node.name, "Lune");
        assert_eq!(edge.cursor, Some("some-string".to_owned()));
    }
}
