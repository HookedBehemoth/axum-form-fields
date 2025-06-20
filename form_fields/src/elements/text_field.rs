use crate::{Descriptor, FormField};

/// Represents a text input field [`<input type="text">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/text).
#[derive(Debug)]
pub struct TextField {
    pub placeholder: Option<String>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

impl Descriptor for TextField {
    type Value = String;
    type Intermediate = Option<String>;

    fn render(field: &FormField<Self>) -> maud::Markup {
        let self_ = &field.descriptor;
        maud::html! {
            label for=(field.field_name) { (field.display_name) }
            input
                type="text"
                name=(field.field_name)
                value=[field.intermediate.as_deref()]
                placeholder=[self_.placeholder.as_deref()]
                minlength=[self_.min_length]
                maxlength=[self_.max_length]
                required[field.required] {}
        }
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

        if let Some(min_length) = self.min_length {
            if value.len() < min_length {
                return Err("Value is shorter than min length");
            }
        }

        if let Some(max_length) = self.max_length {
            if value.len() > max_length {
                return Err("Value exceeds max length");
            }
        }

        Ok(value.clone())
    }

    fn load(&self, value: Self::Value) -> Self::Intermediate {
        Some(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        let descriptor = TextField {
            placeholder: None,
            min_length: None,
            max_length: None,
        };
        let mut intermediate = None;
        descriptor.parse("Hello", &mut intermediate);
        assert_eq!(intermediate, Some("Hello".to_string()));
    }

    #[test]
    fn validate() {
        let descriptor = TextField {
            placeholder: None,
            min_length: Some(3),
            max_length: Some(10),
        };

        let mut intermediate = Some("Hello".to_string());
        assert_eq!(descriptor.validate(&intermediate), Ok("Hello".to_string()));

        intermediate = Some("Hi".to_string());
        assert!(matches!(descriptor.validate(&intermediate), Err(_)));

        intermediate = Some("This is a very long string".to_string());
        assert!(matches!(descriptor.validate(&intermediate), Err(_)));

        intermediate = None;
        assert!(matches!(descriptor.validate(&intermediate), Err(_)));
    }
}
