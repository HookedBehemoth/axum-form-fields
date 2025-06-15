use std::{fmt::Debug, str::FromStr};

use crate::{Descriptor, FormField, selectable::Selectable};

/// Represents a multi-select input [`<input type="checkbox">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/checkbox).
#[derive(Debug)]
pub struct MultiSelect<T: Selectable + Debug> {
    pub keys: Vec<T::Key>,
    pub options: Vec<T>,
}

impl<T: Selectable + Debug> Descriptor for MultiSelect<T> {
    type Value = Vec<T>;

    fn render(field: &FormField<Self>) -> maud::Markup {
        let Self { keys, options } = &field.descriptor;
        maud::html! {
            fieldset {
                legend { (field.display_name) }
                @for option in options {
                    @let key = option.key();
                    @let selected = keys.contains(&key);
                    @let display_value = option.display_value();
                    label {
                        input
                            type="checkbox"
                            name=(field.field_name)
                            checked[selected]
                            value=(key.to_string()) {}
                        (display_value)
                    }
                }
            }
        }
    }

    fn parse(&mut self, value: &str) {
        if value.is_empty() {
            return;
        }

        // Parse key from input text
        let Ok(key) = FromStr::from_str(value) else {
            return;
        };

        self.keys.push(key);
    }

    fn has_value(&self) -> bool {
        !self.keys.is_empty()
    }

    fn value(&self) -> Result<Self::Value, &'static str> {
        let keys = &self.keys;

        let options = &self.options;

        Ok(options
            .iter()
            .filter(|&kv| keys.contains(&kv.key()))
            .cloned()
            .collect())
    }

    fn load(&mut self, value: Self::Value) {
        self.keys = value.iter().map(|v| v.key()).collect();
    }
}
