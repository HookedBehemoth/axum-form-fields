/// A sum type to allow restoring a value for form fields where the validation
/// failed to provide a more user-friendly error flow.
#[derive(Debug)]
pub enum Value<T> {
    Success(T),
    Failure(String, String),
    None,
}

impl<T> Value<T> {
    /// Maps the value into a string representation to be displayed again.
    pub fn map(&self, f: impl FnOnce(&T) -> String) -> Option<String> {
        match self {
            Value::Success(value) => Some(f(value)),
            Value::Failure(value, _) => Some(value.clone()),
            Value::None => None,
        }
    }

    /// Checks if the value has been set.
    pub fn is_none(&self) -> bool {
        matches!(self, Value::None)
    }

    /// Extracts the success value if it exists.
    pub fn inner(&self) -> Option<&T> {
        match self {
            Value::Success(value) => Some(value),
            Value::Failure(_, _) => None,
            Value::None => None,
        }
    }
}
