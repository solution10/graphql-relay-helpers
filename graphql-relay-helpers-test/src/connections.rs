#[cfg(test)]
mod tests {
    use juniper::GraphQLObject;
    use graphql_relay_helpers::RelayConnection;

    #[derive(Debug, RelayConnection, GraphQLObject)]
    struct User {
        name: String,
    }

    #[test]
    fn connection_types_are_generated() {
        let conn = UserRelayConnection {
            count: 12,
            edges: vec![]
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
