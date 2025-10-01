# Relay GraphQL spec helpers

[![⚒️ Build and test](https://github.com/solution10/graphql-relay-helpers/actions/workflows/branch-test.yml/badge.svg)](https://github.com/solution10/graphql-relay-helpers/actions/workflows/branch-test.yml)

WIP - real documentation coming soon, this is an experiment for now.

## Development Todo

The library is being actively developed - here's the WIP status:

- [x] Add support for `Connection` and `Edge` derive macros
- [x] Add support for opaque cursor generation
  - [x] Add `OffsetCursor` implementation
  - [ ] Add `StringCursor` implementation  
- [ ] Add `PageInfo` struct
- [ ] Add support for identifiers
- [ ] Add support for `CursorProvider` to make generating cursors on Connections better.
- [ ] Add example Juniper application
- [ ] Publish crate
