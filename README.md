# Juniper Relay GraphQL spec helpers

[![⚒️ Build and test](https://github.com/solution10/graphql-relay-helpers/actions/workflows/branch-test.yml/badge.svg)](https://github.com/solution10/graphql-relay-helpers/actions/workflows/branch-test.yml)

WIP - real documentation coming soon, this is an experiment for now.

## Development Todo

The library is being actively developed - here's the WIP status:

- [x] Add support for `Connection` and `Edge` derive macros
- [x] Add support for opaque cursor generation
  - [x] Add `OffsetCursor` implementation
  - [x] Add `StringCursor` implementation  
- [x] Add `PageInfo` struct
- [x] Add support for identifiers
- [ ] Add support for `CursorProvider` to make generating cursors on Connections better.
- [x] Add example Axum & Juniper application
- [ ] Add cursor scalar serialization
- [ ] rename crate
- [ ] Publish crate
