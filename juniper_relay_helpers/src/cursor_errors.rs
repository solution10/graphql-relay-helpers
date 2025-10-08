use std::string::FromUtf8Error;

#[derive(Debug, Eq, PartialEq)]
pub enum CursorError {
    /// Returned when the cursor is invalid - wrong number of segments, mismatch of types, etc.
    InvalidCursor,

    /// Returned when the base64 encoding on the cursor is invalid.
    InvalidCursorEncoding,
}

impl std::fmt::Display for CursorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CursorError::InvalidCursor => write!(f, "Invalid cursor"),
            CursorError::InvalidCursorEncoding => write!(f, "Invalid cursor encoding"),
        }
    }
}

impl From<base64::DecodeError> for CursorError {
    fn from(_: base64::DecodeError) -> Self {
        CursorError::InvalidCursorEncoding
    }
}

impl From<FromUtf8Error> for CursorError {
    fn from(_: FromUtf8Error) -> Self {
        CursorError::InvalidCursorEncoding
    }
}

#[cfg(test)]
mod tests {
    use crate::cursor_errors::CursorError;

    #[test]
    fn display_types() {
        assert_eq!(format!("{}", CursorError::InvalidCursor), "Invalid cursor");
        assert_eq!(
            format!("{}", CursorError::InvalidCursorEncoding),
            "Invalid cursor encoding"
        );
    }

    #[test]
    fn from_utf8_error() {
        let error = String::from_utf8(vec![0x80]).unwrap_err();
        let cursor_error = CursorError::from(error);
        assert_eq!(cursor_error, CursorError::InvalidCursorEncoding);
    }

    #[test]
    fn base64_error() {
        let error = base64::DecodeError::InvalidPadding;
        let cursor_error = CursorError::from(error);
        assert_eq!(cursor_error, CursorError::InvalidCursorEncoding);
    }
}
