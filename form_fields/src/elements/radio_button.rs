use std::{fmt::Debug, str::FromStr};

use crate::{Descriptor, FormField, selectable::Selectable};

/// Represents a radio button input [`<input type="radio">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/radio).
#[derive(Debug)]
pub struct RadioButton<T: Selectable + Debug> {
    pub key: Option<T::Key>,
    pub options: Vec<T>,
    pub default_value: T,
}

impl<T: Selectable + Debug> Descriptor for RadioButton<T> {
    type Value = T;

    fn render(field: &FormField<Self>) -> maud::Markup {
        let Self {
            key,
            options,
            default_value,
        } = &field.descriptor;
        let default = default_value.key();
        let selected = key.as_ref().unwrap_or(&default);
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

    fn parse(&mut self, value: &str) {
        if value.is_empty() {
            self.key = None;
            return;
        }

        // Parse key from input text
        self.key = FromStr::from_str(value).ok();
    }

    fn has_value(&self) -> bool {
        self.key.is_some()
    }

    fn value(&self) -> Result<Self::Value, &'static str> {
        let key = self.key.as_ref().ok_or("Value is required")?;

        // Check if the key is valid
        let options = &self.options;

        options
            .iter()
            .find(|&kv| &kv.key() == key)
            .cloned()
            .ok_or("Invalid value")
    }

    fn load(&mut self, value: Self::Value) {
        self.key = Some(value.key());
    }
}
