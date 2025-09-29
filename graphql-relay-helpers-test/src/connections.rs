#[cfg(test)]
mod tests {
    use graphql_relay_helpers::RelayConnection;

    #[derive(Debug, RelayConnection)]
    struct User {
        name: String,
    }

    #[test]
    fn connection_types_are_generated() {
        let conn = User_Connection {
            count: 12,
            edges: vec![]
        };

        assert_eq!(conn.count, 12);
        assert_eq!(conn.edges.len(), 0);
    }

    #[test]
    fn edge_types_are_generated() {
        let edge = User_Edge {
            node: User {
                name: "Lune".to_owned(),
            },
            cursor: Some("some-string".to_owned())
        };
        assert_eq!(edge.node.name, "Lune");
        assert_eq!(edge.cursor, Some("some-string".to_owned()));
    }
}
