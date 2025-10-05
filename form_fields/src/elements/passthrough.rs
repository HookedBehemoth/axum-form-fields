
use crate::{Descriptor, FormField};

/// Doesn't represent any specific HTML input type. Instead, it simply passes the value through as-is.
/// This is useful for custom or complex types that don't fit into standard HTML input types.
#[derive(Debug, Default)]
pub struct Passthrough<T> {
    _marker: std::marker::PhantomData<T>,
}

impl Descriptor for Passthrough<String> {
    type Value = String;
    type Intermediate = Option<String>;

    fn render(_field: &FormField<Self>) -> maud::Markup {
        panic!("Passthrough cannot be rendered directly");
    }

    fn parse(&self, value: &str, intermediate: &mut Self::Intermediate) {
        if value.is_empty() {
            *intermediate = None;
            return;
        }

        *intermediate = Some(value.to_string());
    }

    fn validate(&self, intermediate: &Self::Intermediate) -> Result<Self::Value, &'_ str> {
        let value = intermediate.as_ref().ok_or("Value is required")?;

        Ok(value.clone())
    }

    fn load(&self, value: Self::Value) -> Self::Intermediate {
        Some(value)
    }
}

impl Descriptor for Passthrough<Vec<String>> {
    type Value = Vec<String>;
    type Intermediate = Vec<String>;

    fn render(_field: &FormField<Self>) -> maud::Markup {
        panic!("Passthrough cannot be rendered directly");
    }

    fn parse(&self, value: &str, intermediate: &mut Self::Intermediate) {
        if value.is_empty() {
            return;
        }

        intermediate.push(value.to_string());
    }

    fn validate(&self, intermediate: &Self::Intermediate) -> Result<Self::Value, &'_ str> {
        Ok(intermediate.clone())
    }

    fn load(&self, value: Self::Value) -> Self::Intermediate {
        value.clone()
    }
}
