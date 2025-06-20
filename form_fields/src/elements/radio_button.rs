use std::{fmt::Debug, str::FromStr};

use crate::{Descriptor, FormField, selectable::Selectable};

/// Represents a radio button input [`<input type="radio">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/radio).
#[derive(Debug)]
pub struct RadioButton<T: Selectable + Debug> {
    pub options: Vec<T>,
    pub default_value: T,
}

impl<T: Selectable + Debug> Descriptor for RadioButton<T> {
    type Value = T;
    type Intermediate = Option<T::Key>;

    fn render(field: &FormField<Self>) -> maud::Markup {
        let Self {
            options,
            default_value,
        } = &field.descriptor;
        let default = default_value.key();
        let selected = field.intermediate.as_ref().unwrap_or(&default);
        maud::html! {
            label for=(field.field_name) { (field.display_name) }
            @for option in options {
                @let key = option.key();
                @let display_value = option.display_value();
                label {
                    input
                        type="radio"
                        name=(field.field_name)
                        value=(key.to_string())
                        checked[selected == &key]
                        required[field.required] {}
                    (display_value)
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

    fn validate(&self, key: &Self::Intermediate) -> Result<Self::Value, &'static str> {
        let key = key.as_ref().ok_or("Value is required")?;

        // Check if the key is valid
        let options = &self.options;

        options
            .iter()
            .find(|&kv| &kv.key() == key)
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
        let radio = RadioButton {
            options: vec![
                "option1".to_string(),
                "option2".to_string(),
                "option3".to_string(),
            ],
            default_value: "option1".to_string(),
        };
        let mut intermediate = None;
        radio.parse("option2", &mut intermediate);
        assert_eq!(intermediate, Some("option2".to_string()));
        radio.parse("", &mut intermediate);
        assert_eq!(intermediate, None);
    }

    #[test]
    fn validate() {
        let radio = RadioButton {
            options: vec![
                "option1".to_string(),
                "option2".to_string(),
                "option3".to_string(),
            ],
            default_value: "option1".to_string(),
        };
        let mut intermediate = Some("option2".to_string());
        let value = radio.validate(&intermediate).unwrap();
        assert_eq!(value, "option2".to_string());

        intermediate = Some("invalid_option".to_string());
        assert!(radio.validate(&intermediate).is_err());
    }
}