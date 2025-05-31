use crate::MRZChecksumError;

/// A wrapper for MRZ fields that may be accompanied by a checksum validation error.
///
/// `CheckedField<T>` stores a parsed value of type `T` along with an optional
/// `MRZChecksumError` indicating whether the field failed checksum verification.
///
/// This structure is useful for retaining the original field value regardless
/// of whether its checksum was valid, enabling more robust error handling and
/// potential OCR correction workflows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedField<T> {
    value: T,
    error: Option<MRZChecksumError>,
    check_digit: Option<u8>,
}

impl<T> CheckedField<T> {
    /// Creates a new `CheckedField` with a value, an optional checksum error, and a raw check digit character.
    /// If the raw check character is b'&lt;', the check digit is set to None.
    pub fn new(value: T, error: Option<MRZChecksumError>, raw_check_char: u8) -> Self {
        let check_digit = if raw_check_char == b'<' {
            None
        } else {
            Some(raw_check_char)
        };
        CheckedField {
            value,
            error,
            check_digit,
        }
    }

    /// Returns `true` if the field is valid (i.e. has no checksum error).
    pub fn is_valid(&self) -> bool {
        self.error.is_none()
    }

    /// Returns a reference to the checksum error, if any.
    pub fn error(&self) -> Option<&MRZChecksumError> {
        self.error.as_ref()
    }

    /// Returns a reference to the underlying value.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Consumes the `CheckedField`, returning the inner value.
    pub fn into_inner(self) -> T {
        self.value
    }

    /// Transforms the inner value using the given function,
    /// preserving the existing checksum error (if any) and check digit.
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> CheckedField<U> {
        CheckedField {
            value: f(self.value),
            error: self.error,
            check_digit: self.check_digit,
        }
    }

    /// Returns true if the field has a checksum error.
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    /// Returns a reference to the inner value and error.
    pub fn as_ref(&self) -> CheckedField<&T> {
        CheckedField {
            value: &self.value,
            error: self.error.clone(),
            check_digit: self.check_digit,
        }
    }

    /// Maps the checksum error using the given function, preserving the value and check digit.
    pub fn map_error<F, E>(self, f: F) -> CheckedField<T>
    where
        F: FnOnce(MRZChecksumError) -> E,
        E: Into<MRZChecksumError>,
    {
        CheckedField {
            value: self.value,
            error: self.error.map(|e| f(e).into()),
            check_digit: self.check_digit,
        }
    }

    /// Returns the check digit, if any.
    pub fn check_digit(&self) -> Option<u8> {
        self.check_digit
    }

    /// Sets the check digit.
    pub fn set_check_digit(&mut self, digit: u8) {
        self.check_digit = Some(digit);
    }
}

impl<const N: usize> CheckedField<[u8; N]> {
    /// Returns the length of value plus one for the check digit.
    pub const fn len_with_check() -> usize {
        N + 1
    }

    /// Returns the value bytes combined with the check digit as a heapless::Vec<u8>.
    pub fn as_slice_with_check(&self) -> heapless::Vec<u8, 32> {
        let mut result = heapless::Vec::<u8, 32>::new();
        dbg!(&self.check_digit);
        result.extend_from_slice(&self.value).ok();
        result.push(self.check_digit.unwrap_or(b'<')).ok();
        result
    }
}

impl<const N: usize> CheckedField<heapless::String<N>> {
    /// Returns the length of value plus one for the check digit.
    pub const fn len_with_check() -> usize {
        N + 1
    }

    /// Returns the value bytes combined with the check digit as a heapless::Vec<u8>.
    pub fn as_slice_with_check(&self) -> heapless::Vec<u8, 32> {
        let mut result = heapless::Vec::<u8, 32>::new();
        dbg!(&self.check_digit);
        result.extend_from_slice(self.value.as_bytes()).ok();
        result.push(self.check_digit.unwrap_or(b'<')).ok();
        result
    }
}

impl<T: core::fmt::Display> core::fmt::Display for CheckedField<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.error {
            Some(e) => write!(f, "{} (invalid: {})", self.value, e),
            None => write!(f, "{}", self.value),
        }
    }
}
