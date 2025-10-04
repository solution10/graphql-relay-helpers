use juniper_relay_helpers::{Cursor, OffsetCursor, PageInfo, PageRequest};

/// Struct that holds metadata about the response that can be used in the CursorProvider
#[derive(Debug, Clone)]
pub struct PaginationMetadata {
    /// The total number of items in the result set:
    pub total_count: i32,

    /// The current PageInfo, if any:
    pub page_request: Option<PageRequest>,
}

/// Trait to implement when building a Relay cursor provider.
///
/// Cursor providers are how we generate cursors for each of the individual items
/// within the result set, without needing to do a pass and build them manually.
///
pub trait CursorProvider {
    /// Build a cursor instance for the given item, with helper metadata etc.
    ///
    /// `metadata` is information about the current resultset we're building for.
    /// `item_idx` is the index of the item we're building a cursor for.
    /// `item` is the item itself.
    fn get_cursor_for_item<T>(
        &self,
        metadata: &PaginationMetadata,
        item_idx: i32,
        item: &T
    ) -> impl Cursor;

    /// Builds the `PageInfo` to return to the RelayConnection
    fn get_page_info<T>(&self, metadata: &PaginationMetadata, items: &Vec<T>) -> PageInfo;
}


// -------------- OffsetCursorProvider ---------------

/// Built-in cursor provider that can handle Offset cursors. Serves as a reference implementation for
/// your own cursor providers too.
pub struct OffsetCursorProvider;
impl CursorProvider for OffsetCursorProvider {
    fn get_cursor_for_item<T>(&self, metadata: &PaginationMetadata, item_idx: i32, _item: &T) -> impl Cursor {
        let default_cursor = OffsetCursor::default();
        let current_cursor = match &metadata.page_request {
            Some(pr) => match pr.parsed_cursor() {
                Ok(c) => c.unwrap_or(default_cursor),
                Err(_) => default_cursor
            },
            None => default_cursor
        };

        OffsetCursor {
            offset: current_cursor.offset + item_idx,
            first: current_cursor.first
        }
    }

    fn get_page_info<T>(&self, metadata: &PaginationMetadata, items: &Vec<T>) -> PageInfo {
        let default_cursor = OffsetCursor::default();
        let current_cursor = match &metadata.page_request {
            Some(pr) => match pr.parsed_cursor() {
                Ok(c) => c.unwrap_or(default_cursor),
                Err(_) => default_cursor
            },
            None => default_cursor
        };

        let has_next_page = items.len() > 0
            && (current_cursor.offset + items.len() as i32) < metadata.total_count;

        let last_index = items.len() - 1;

        PageInfo {
            has_prev_page: current_cursor.offset > 0,
            has_next_page,
            start_cursor: if items.len() > 0 {
                Some(
                    self.get_cursor_for_item(metadata, 0, &items[0])
                        .to_encoded_string()
                )
            } else {
                None
            },
            end_cursor: if items.len() > 0 {
                Some(
                    self.get_cursor_for_item(metadata, last_index as i32, &items[last_index])
                        .to_encoded_string()
                )
            } else {
                None
            },
        }
    }
}

impl OffsetCursorProvider {
    pub fn new() -> Self {
        OffsetCursorProvider
    }
}


#[cfg(test)]
mod tests {
    mod offset_cursor_provider {
        use crate::{OffsetCursorProvider, PaginationMetadata, CursorProvider, Cursor, OffsetCursor, PageRequest};

        struct Location {
            name: String
        }

        fn data() -> Vec<Location> {
            vec![
                Location { name: "Lumiére".to_owned() },
                Location { name: "Flying Waters".to_owned() }
            ]
        }

        /// Mimics a "complete" request - no `first` and no `after` with the total result set returned
        /// as part of the payload.
        #[test]
        fn test_page_info_no_request() {
            let p = OffsetCursorProvider::new();
            let pi = p.get_page_info(&PaginationMetadata {
                total_count: 2,
                page_request: None
            }, &data());

            assert_eq!(pi.has_prev_page, false);
            assert_eq!(pi.has_next_page, false);
            assert_eq!(pi.start_cursor, Some(OffsetCursor { offset: 0, first: None }.to_encoded_string()));
            assert_eq!(pi.end_cursor, Some(OffsetCursor { offset: 1, first: None }.to_encoded_string()));
        }

        /// Verifies what happens when there's a mismatch between the total count and the number of items
        /// returned from the query. Should still say there's a next page as the result set may be short.
        #[test]
        fn test_page_info_no_request_mismatch_results_count() {
            let p = OffsetCursorProvider::new();
            let pi = p.get_page_info(&PaginationMetadata {
                total_count: 27,
                page_request: None
            }, &data());

            assert_eq!(pi.has_prev_page, false);
            assert_eq!(pi.has_next_page, true);
            assert_eq!(pi.start_cursor, Some(OffsetCursor { offset: 0, first: None }.to_encoded_string()));
            assert_eq!(pi.end_cursor, Some(OffsetCursor { offset: 1, first: None }.to_encoded_string()));
        }

        /// Mimics a first page request - there's no `after` but there is a provided `first`
        #[test]
        fn test_page_info_has_request_first_page() {
            let p = OffsetCursorProvider::new();
            let pi = p.get_page_info(&PaginationMetadata {
                total_count: 27,
                page_request: Some(
                    PageRequest {
                        first: Some(10),
                        after: None
                    }
                )
            }, &data());

            assert_eq!(pi.has_prev_page, false);
            assert_eq!(pi.has_next_page, true);
            assert_eq!(pi.start_cursor, Some(OffsetCursor { offset: 0, first: None }.to_encoded_string()));
            assert_eq!(pi.end_cursor, Some(OffsetCursor { offset: 1, first: None }.to_encoded_string()));
        }

        /// Mimics a subsequent page request - there's an `after` and a `first`
        /// TODO: I think this is actually an off-by-one error :yikes:
        #[test]
        fn test_page_info_has_request_subsequent_page() {
            let p = OffsetCursorProvider::new();
            let pi = p.get_page_info(&PaginationMetadata {
                total_count: 27,
                page_request: Some(
                    PageRequest {
                        first: Some(10),
                        after: Some(OffsetCursor { offset: 9, first: Some(10) }.to_encoded_string())
                    }
                )
            }, &data());

            assert_eq!(pi.has_prev_page, true);
            assert_eq!(pi.has_next_page, true);
            assert_eq!(pi.start_cursor, Some(OffsetCursor { offset: 9, first: Some(10) }.to_encoded_string()));
            assert_eq!(pi.end_cursor, Some(OffsetCursor { offset: 10, first: Some(10) }.to_encoded_string()));
        }
    }
}

