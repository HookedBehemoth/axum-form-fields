use crate::{Descriptor, FormField};

#[derive(Debug)]
pub struct TextField {
    pub value: Option<String>,
    pub placeholder: Option<String>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

impl Descriptor for TextField {
    type Value = String;

    fn render(field: &FormField<Self>) -> maud::Markup {
        let self_ = &field.descriptor;
        maud::html! {
            label for=(field.field_name) { (field.display_name) }
            input
                type="text"
                name=(field.field_name)
                value=[field.descriptor.value.as_deref()]
                placeholder=[self_.placeholder.as_deref()]
                minlength=[self_.min_length]
                maxlength=[self_.max_length]
                required[field.required] {}
        }
    }

    fn parse(&mut self, value: &str) {
        if value.is_empty() {
            self.value = None;
            return;
        }

        self.value = Some(value.to_string());
    }

    fn has_value(&self) -> bool {
        self.value.is_some()
    }

    fn value(&self) -> Result<Self::Value, &'static str> {
        let value = self.value.as_ref().ok_or("Value is required")?;

        if let Some(max_length) = self.max_length {
            if value.len() > max_length {
                return Err("Value exceeds max length");
            }
        }

        Ok(value.clone())
    }

    fn load(&mut self, value: Self::Value) {
        self.value = Some(value);
    }
}
