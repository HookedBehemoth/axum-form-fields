#[derive(Debug)]
pub enum Value<T> {
    Success(T),
    Failure(String, String),
    None,
}

impl<T> Value<T> {
    pub fn map(&self, f: impl FnOnce(&T) -> String) -> Option<String> {
        match self {
            Value::Success(value) => Some(f(value)),
            Value::Failure(value, _) => Some(value.clone()),
            Value::None => None,
        }
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Value::None)
    }

    pub fn inner(&self) -> Option<&T> {
        match self {
            Value::Success(value) => Some(value),
            Value::Failure(_, _) => None,
            Value::None => None,
        }
    }
}
