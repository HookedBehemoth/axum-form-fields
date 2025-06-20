use crate::{Descriptor, FormField};

/// Represents a checkbox input [`<input type="checkbox">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/checkbox).
#[derive(Debug)]
pub struct Checkbox {
    pub required_true: bool,
    pub checked: bool,
}

impl Descriptor for Checkbox {
    type Value = bool;
    type Intermediate = Option<bool>;

    // NOTE: required for input type checkbox means that it has to be true. This isn't desireable.
    fn render(field: &FormField<Self>) -> maud::Markup {
        let Self {
            required_true,
            checked,
        } = &field.descriptor;
        let prechecked = field.intermediate.unwrap_or(*checked);
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

    fn parse(&self, value: &str, intermediate: &mut Self::Intermediate) {
        if value.is_empty() {
            *intermediate = None;
            return;
        }

        *intermediate = if value == "true" {
            Some(true)
        } else {
            Some(false)
        }
    }

    fn validate(&self, intermediate: &Self::Intermediate) -> Result<Self::Value, &'static str> {
        match intermediate {
            Some(true) => Ok(true),
            Some(false) if !self.required_true => Ok(false),
            None if !self.required_true => Ok(false),
            _ => Err("Checkbox is required"),
        }
    }

    fn load(&self, value: Self::Value) -> Self::Intermediate {
        Some(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        let checkbox = Checkbox {
            required_true: true,
            checked: false,
        };
        let mut intermediate = None;
        checkbox.parse("false", &mut intermediate);
        assert_eq!(intermediate, Some(false));
        checkbox.parse("true", &mut intermediate);
        assert_eq!(intermediate, Some(true));
        checkbox.parse("", &mut intermediate);
        assert_eq!(intermediate, None);
    }

    #[test]
    fn validate() {
        let checkbox = Checkbox {
            required_true: true,
            checked: false,
        };
        assert_eq!(checkbox.validate(&Some(true)), Ok(true));
        assert_eq!(checkbox.validate(&Some(false)), Err("Checkbox is required"));
        assert_eq!(checkbox.validate(&None), Err("Checkbox is required"));

        let checkbox = Checkbox {
            required_true: false,
            checked: false,
        };
        assert_eq!(checkbox.validate(&Some(true)), Ok(true));
        assert_eq!(checkbox.validate(&Some(false)), Ok(false));
        assert_eq!(checkbox.validate(&None), Ok(false));
    }
}
