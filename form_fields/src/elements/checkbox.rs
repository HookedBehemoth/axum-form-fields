use crate::{Descriptor, FormField};

/// Represents a checkbox input [`<input type="checkbox">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/checkbox).
#[derive(Debug)]
pub struct Checkbox {
    pub value: Option<bool>,
    pub required_true: bool,
    pub checked: bool,
}

impl Descriptor for Checkbox {
    type Value = bool;

    // NOTE: required for input type checkbox means that it has to be true. This isn't desireable.
    fn render(field: &FormField<Self>) -> maud::Markup {
        let Self {
            value,
            required_true,
            checked,
        } = &field.descriptor;
        let prechecked = value.unwrap_or(*checked);
        maud::html! {
            label {
                input
                    type="checkbox"
                    name=(field.field_name)
                    value="true"
                    checked[prechecked]
                    required[*required_true] {}
                (field.display_name)
            }
        }
    }

    fn parse(&mut self, value: &str) {
        if value.is_empty() {
            self.value = None;
            return;
        }

        if value == "true" {
            self.value = Some(true);
        } else {
            self.value = Some(false);
        }
    }

    fn has_value(&self) -> bool {
        self.value.is_some() || self.checked
    }

    fn value(&self) -> Result<Self::Value, &'static str> {
        let value = self.value.unwrap_or(false);
        if !value && self.required_true {
            Err("Checkbox is required")
        } else {
            Ok(value)
        }
    }

    fn load(&mut self, value: Self::Value) {
        self.value = Some(value);
    }
}
