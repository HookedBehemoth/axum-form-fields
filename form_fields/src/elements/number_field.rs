use std::{fmt::Display, str::FromStr};

use crate::{Descriptor, FormField, validation_value::Value};

/// Represents a number input field [`<input type="number">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/number).
#[derive(Debug)]
pub struct NumberField<T: PartialOrd> {
    pub value: Value<T>,
    pub min: Option<T>,
    pub max: Option<T>,
}

impl<T: PartialOrd + maud::Render + Display + Copy + FromStr> Descriptor for NumberField<T> {
    type Value = T;

    fn render(field: &FormField<Self>) -> maud::Markup {
        let Self { value, min, max } = &field.descriptor;

        maud::html! {
            label for=(field.field_name) { (field.display_name) }
            input
                type="number"
                name=(field.field_name)
                value=[value.map(|v| v.to_string())]
                min=[min]
                max=[max]
                required[field.required] {}
        }
    }

    fn parse(&mut self, value: &str) {
        if value.is_empty() {
            self.value = Value::None;
            return;
        }

        let Ok(parsed_value) = T::from_str(value) else {
            self.value = Value::Failure(value.to_string(), "Invalid number".to_string());
            return;
        };

        self.value = Value::Success(parsed_value);
    }

    fn has_value(&self) -> bool {
        match &self.value {
            Value::Success(_) => true,
            Value::Failure(_, _) => false,
            Value::None => false,
        }
    }

    fn value(&self) -> Result<Self::Value, &'_ str> {
        let value = *match &self.value {
            Value::Success(v) => v,
            Value::Failure(_, err) => return Err(err.as_str()),
            Value::None => return Err("Value is required"),
        };

        if let Some(min) = self.min {
            if value < min {
                return Err("Value is less than min");
            }
        }

        if let Some(max) = self.max {
            if value > max {
                return Err("Value exceeds max");
            }
        }

        Ok(value)
    }

    fn load(&mut self, value: Self::Value) {
        self.value = Value::Success(value);
    }
}
