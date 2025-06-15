use std::str::FromStr;

use chrono::NaiveDate;

use crate::{Descriptor, FormField, validation_value::Value};

#[derive(Debug)]
pub struct DatePicker {
    pub value: Value<NaiveDate>,
    pub min: Option<NaiveDate>,
    pub max: Option<NaiveDate>,
}

impl Descriptor for DatePicker {
    type Value = NaiveDate;

    fn render(field: &FormField<Self>) -> maud::Markup {
        let Self { value, min, max } = &field.descriptor;
        maud::html! {
            label for=(field.field_name) { (field.display_name) }
            input
                type="date"
                name=(field.field_name)
                value=[value.map(|v| v.format("%Y-%m-%d").to_string())]
                min=[min.map(|v| v.format("%Y-%m-%d").to_string())]
                max=[max.map(|v| v.format("%Y-%m-%d").to_string())]
                required[field.required] {}
        }
    }

    fn parse(&mut self, value: &str) {
        if value.is_empty() {
            self.value = Value::None;
            return;
        }

        let Ok(parsed_value) = NaiveDate::from_str(value) else {
            self.value = Value::Failure(value.to_string(), "Invalid date".to_string());
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
