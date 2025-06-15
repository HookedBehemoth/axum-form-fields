use std::{fmt::Debug, str::FromStr};

use crate::{Descriptor, FormField, selectable::Selectable};

/// Represents a select input [`<select>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select).
#[derive(Debug)]
pub struct Select<T: Selectable + Debug> {
    pub key: Option<T::Key>,
    pub default_value: Option<T>,
    pub options: Vec<T>,
    pub placeholder: String,
}

impl<T: Selectable + Debug> Descriptor for Select<T> {
    type Value = T;

    fn render(field: &FormField<Self>) -> maud::Markup {
        let Self {
            key,
            default_value,
            options,
            placeholder,
        } = &field.descriptor;
        let default = default_value.as_ref().map(|v| v.key());
        let selected = key.as_ref().or(default.as_ref());
        maud::html! {
            label for=(field.field_name) { (field.display_name) }
            select name=(field.field_name) required[field.required] {
                option value="" disabled[field.required] selected[!key.is_some()] { (placeholder) }
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
