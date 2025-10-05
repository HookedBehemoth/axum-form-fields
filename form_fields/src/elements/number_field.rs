use std::{fmt::Display, str::FromStr};

use crate::{Descriptor, FormField, validation_value::Value};

/// Represents a number input field [`<input type="number">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/number).
#[derive(Debug)]
pub struct NumberField<T: PartialOrd> {
    pub min: Option<T>,
    pub max: Option<T>,
}

impl<T: PartialOrd + maud::Render + Display + Copy + FromStr> Descriptor for NumberField<T> {
    type Value = T;
    type Intermediate = Value<T>;

    fn render(field: &FormField<Self>) -> maud::Markup {
        let Self { min, max } = &field.descriptor;
        let value = field.intermediate.map(|v| v.to_string());

        maud::html! {
            label for=(field.field_name) { (field.display_name) }
            input
                type="number"
                name=(field.field_name)
                value=[value]
                min=[min]
                max=[max]
                required[field.required] {}
        }
    }

    fn parse(&self, value: &str, intermediate: &mut Self::Intermediate) {
        if value.is_empty() {
            *intermediate = Value::None;
            return;
        }

        let Ok(parsed_value) = T::from_str(value) else {
            *intermediate = Value::Failure(value.to_string(), "Invalid number".to_string());
            return;
        };

        *intermediate = Value::Success(parsed_value);
    }

    fn validate(&self, intermediate: &Self::Intermediate) -> Result<Self::Value, &'_ str> {
        let value = intermediate.inner().ok_or("Value is required")?;

        if let Some(min) = self.min
            && *value < min
        {
            return Err("Value is less than min");
        }

        if let Some(max) = self.max
            && *value > max
        {
            return Err("Value exceeds max");
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
        let number_field = NumberField {
            min: Some(10),
            max: Some(100),
        };
        let mut intermediate = Value::None;

        number_field.parse("50", &mut intermediate);
        assert!(matches!(intermediate, Value::Success(50)));

        number_field.parse("abc", &mut intermediate);
        assert!(matches!(intermediate, Value::Failure(_, _)));

        number_field.parse("", &mut intermediate);
        assert!(matches!(intermediate, Value::None));
    }

    #[test]
    fn validate() {
        let number_field = NumberField {
            min: Some(10),
            max: Some(100),
        };

        assert_eq!(number_field.validate(&Value::Success(10)), Ok(10));
        assert_eq!(number_field.validate(&Value::Success(50)), Ok(50));
        assert_eq!(number_field.validate(&Value::Success(100)), Ok(100));
        assert_eq!(
            number_field.validate(&Value::Success(5)),
            Err("Value is less than min")
        );
        assert_eq!(
            number_field.validate(&Value::Success(150)),
            Err("Value exceeds max")
        );
        assert_eq!(
            number_field.validate(&Value::None),
            Err("Value is required")
        );
    }
}
