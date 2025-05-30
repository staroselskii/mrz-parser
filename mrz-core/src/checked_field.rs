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
}

impl<T> CheckedField<T> {
    /// Creates a new `CheckedField` with a value and an optional checksum error.
    pub fn new(value: T, error: Option<MRZChecksumError>) -> Self {
        CheckedField { value, error }
    }

    /// Creates a new `CheckedField` with a value and no checksum error.
    pub fn new_valid(value: T) -> Self {
        CheckedField { value, error: None }
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
    /// preserving the existing checksum error (if any).
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> CheckedField<U> {
        CheckedField {
            value: f(self.value),
            error: self.error,
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
        }
    }

    /// Maps the checksum error using the given function, preserving the value.
    pub fn map_error<F, E>(self, f: F) -> CheckedField<T>
    where
        F: FnOnce(MRZChecksumError) -> E,
        E: Into<MRZChecksumError>,
    {
        CheckedField {
            value: self.value,
            error: self.error.map(|e| f(e).into()),
        }
    }

    /// Returns the inner value as a byte slice if the value type supports it.
    pub fn as_bytes(&self) -> &[u8]
    where
        T: AsRef<[u8]>,
    {
        self.value.as_ref()
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
