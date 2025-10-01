/// Trait to implement when building a Relay cursor provider.
///
/// Cursor providers are how we generate cursors for each of the individual items
/// within the result set, without needing to do a pass and build them manually.
pub trait CursorProvider {
    fn get_cursor(&self, ) -> String;
}
