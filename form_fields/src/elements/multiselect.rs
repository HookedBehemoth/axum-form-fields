use std::{fmt::Debug, str::FromStr};

use crate::{selectable::Selectable, Descriptor, FormField};

/// Represents a multi-select input [`<input type="checkbox">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/checkbox).
#[derive(Debug)]
pub struct MultiSelect<T: Selectable + Debug> {
    pub options: Vec<T>,
}

impl<T: Selectable + Debug> Descriptor for MultiSelect<T> {
    type Value = Vec<T>;
    type Intermediate = Vec<T::Key>;

    fn render(field: &FormField<Self>) -> maud::Markup {
        let Self { options } = &field.descriptor;
        let keys = &field.intermediate;
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

    fn parse(&self, value: &str, intermediate: &mut Self::Intermediate) {
        if value.is_empty() {
            return;
        }

        // Parse key from input text
        let Ok(key) = FromStr::from_str(value) else {
            return;
        };

        intermediate.push(key);
    }

    fn validate(&self, keys: &Self::Intermediate) -> Result<Self::Value, &'_ str> {
        let options = &self.options;

        // Check if all keys are valid
        for key in keys {
            if !options.iter().any(|option| &option.key() == key) {
                return Err("Invalid option selected");
            }
        }

        // Return the selected options
        Ok(options
            .iter()
            .filter(|&option| keys.contains(&option.key()))
            .cloned()
            .collect())
    }

    fn load(&self, value: Self::Value) -> Self::Intermediate {
        value.iter().map(|v| v.key()).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        let multiselect = MultiSelect::<String> {
            options: vec![],
        };
        let mut intermediate = vec![];
        multiselect.parse("option1", &mut intermediate);
        assert_eq!(intermediate, vec!["option1"]);
        multiselect.parse("", &mut intermediate);
        assert_eq!(intermediate, vec!["option1"]);
        multiselect.parse("option2", &mut intermediate);
        assert_eq!(intermediate, vec!["option1", "option2"]);
    }

    #[test]
    fn validate() {
        let options = vec!["option1".to_string(), "option2".to_string()];
        let multiselect = MultiSelect { options };

        // Valid selection
        let keys = vec!["option1".to_string(), "option2".to_string()];
        assert_eq!(
            multiselect.validate(&keys),
            Ok(vec!["option1".to_string(), "option2".to_string()])
        );

        // Invalid selection
        let keys = vec!["option3".to_string()];
        assert_eq!(multiselect.validate(&keys), Err("Invalid option selected"));

        // Partial correct selection
        let keys = vec!["option1".to_string(), "option3".to_string()];
        assert_eq!(multiselect.validate(&keys), Err("Invalid option selected"));

        // Empty selection
        let keys: Vec<String> = vec![];
        assert_eq!(multiselect.validate(&keys), Ok(vec![]));
    }
}