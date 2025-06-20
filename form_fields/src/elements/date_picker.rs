use std::str::FromStr;

use chrono::NaiveDate;

use crate::{Descriptor, FormField, validation_value::Value};

/// Represents a date picker input [`<input type="date">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/date).
#[derive(Debug)]
pub struct DatePicker {
    pub min: Option<NaiveDate>,
    pub max: Option<NaiveDate>,
}

impl Descriptor for DatePicker {
    type Value = NaiveDate;
    type Intermediate = Value<NaiveDate>;

    fn render(field: &FormField<Self>) -> maud::Markup {
        let Self { min, max } = &field.descriptor;
        let value = field.intermediate.inner();
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

    fn parse(&self, value: &str, intermediate: &mut Self::Intermediate) {
        if value.is_empty() {
            *intermediate = Value::None;
            return;
        }

        let Ok(parsed_value) = NaiveDate::from_str(value) else {
            *intermediate = Value::Failure(value.to_string(), "Invalid date".to_string());
            return;
        };

        *intermediate = Value::Success(parsed_value)
    }

    fn validate(&self, intermediate: &Self::Intermediate) -> Result<Self::Value, &'_ str> {
        let value = intermediate.inner().ok_or("Value is required")?;

        if let Some(min) = &self.min {
            if value < min {
                return Err("Value is less than min");
            }
        }

        if let Some(max) = &self.max {
            if value > max {
                return Err("Value exceeds max");
            }
        }

        Ok(*value)
    }

    fn load(&self, value: Self::Value) -> Self::Intermediate {
        Value::Success(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        let date_picker = DatePicker {
            min: NaiveDate::from_ymd_opt(2020, 1, 1),
            max: NaiveDate::from_ymd_opt(2023, 12, 31),
        };
        let mut intermediate = Value::None;
        date_picker.parse("2022-05-15", &mut intermediate);
        assert_eq!(intermediate.inner(), NaiveDate::from_ymd_opt(2022, 5, 15).as_ref());
        
        date_picker.parse("1800-01-01", &mut intermediate);
        assert_eq!(intermediate.inner(), NaiveDate::from_ymd_opt(1800, 1, 1).as_ref());
        
        date_picker.parse("2022-18-08", &mut intermediate);
        assert!(matches!(intermediate, Value::Failure(_, _)));

        date_picker.parse("invalid-date", &mut intermediate);
        assert!(matches!(intermediate, Value::Failure(_, _)));
    }

    #[test]
    fn validate() {
        let date_picker = DatePicker {
            min: NaiveDate::from_ymd_opt(2020, 1, 1),
            max: NaiveDate::from_ymd_opt(2023, 12, 31),
        };

        let expected = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        assert_eq!(date_picker.validate(&Value::Success(expected)), Ok(expected));
        let expected = NaiveDate::from_ymd_opt(2022, 5, 15).unwrap();
        assert_eq!(date_picker.validate(&Value::Success(expected)), Ok(expected));
        let expected = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
        assert_eq!(date_picker.validate(&Value::Success(expected)), Ok(expected));
        assert_eq!(date_picker.validate(&Value::Success(NaiveDate::from_ymd_opt(2019, 12, 31).unwrap())), Err("Value is less than min"));
        assert_eq!(date_picker.validate(&Value::Success(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())), Err("Value exceeds max"));
        assert_eq!(date_picker.validate(&Value::None), Err("Value is required"));
    }
}