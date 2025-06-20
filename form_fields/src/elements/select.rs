use std::{fmt::Debug, str::FromStr};

use crate::{Descriptor, FormField, selectable::Selectable};

/// Represents a select input [`<select>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select).
#[derive(Debug)]
pub struct Select<T: Selectable + Debug> {
    pub default_value: Option<T>,
    pub options: Vec<T>,
    pub placeholder: String,
}

impl<T: Selectable + Debug> Descriptor for Select<T> {
    type Value = T;
    type Intermediate = Option<T::Key>;

    fn render(field: &FormField<Self>) -> maud::Markup {
        let Self {
            default_value,
            options,
            placeholder,
        } = &field.descriptor;
        let default = default_value.as_ref().map(|v| v.key());
        let selected = field.intermediate.as_ref().or(default.as_ref());
        let has_value = field.intermediate.is_some();
        maud::html! {
            label for=(field.field_name) { (field.display_name) }
            select name=(field.field_name) required[field.required] {
                option value="" disabled[field.required] selected[!has_value] { (placeholder) }
                @for option in options {
                    @let key = option.key();
                    @let display_value = option.display_value();
                    option
                        value=(key.to_string())
                        selected[selected == Some(&key)] { (display_value) }
                }
            }
        }
    }

    fn parse(&self, value: &str, key: &mut Self::Intermediate) {
        if value.is_empty() {
            *key = None;
            return;
        }

        // Parse key from input text
        *key = FromStr::from_str(value).ok();
    }

    fn validate(&self, intermediate: &Self::Intermediate) -> Result<Self::Value, &'_ str> {
        let key = intermediate.as_ref().ok_or("Value is required")?;

        // Check if the key is valid
        let options = &self.options;

        options
            .iter()
            .find(|&option| &option.key() == key)
            .cloned()
            .ok_or("Invalid value")
    }

    fn load(&self, value: Self::Value) -> Self::Intermediate {
        Some(value.key())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn parse() {
        let radio = Select {
            options: vec![
                "option1".to_string(),
                "option2".to_string(),
                "option3".to_string(),
            ],
            default_value: None,
            placeholder: String::new(),
        };
        let mut intermediate = None;
        radio.parse("option2", &mut intermediate);
        assert_eq!(intermediate, Some("option2".to_string()));
        radio.parse("", &mut intermediate);
        assert_eq!(intermediate, None);
    }

    #[test]
    fn validate() {
        let radio = Select {
            options: vec![
                "option1".to_string(),
                "option2".to_string(),
                "option3".to_string(),
            ],
            default_value: None,
            placeholder: String::new(),
        };
        let mut intermediate = Some("option2".to_string());
        let value = radio.validate(&intermediate).unwrap();
        assert_eq!(value, "option2".to_string());

        intermediate = Some("invalid_option".to_string());
        assert!(radio.validate(&intermediate).is_err());
    }
}
