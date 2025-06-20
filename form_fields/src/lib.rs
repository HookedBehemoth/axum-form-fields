pub mod elements;
pub mod from_form;
#[cfg(feature = "multipart")]
pub mod multipart;
pub mod selectable;
#[cfg(feature = "urlencoded")]
pub mod urlencoded;
pub mod validation_value;

pub trait Intermediate: Sized {
    fn has_value(&self) -> bool;
}

impl<T> Intermediate for Option<T> {
    fn has_value(&self) -> bool {
        self.is_some()
    }
}

impl<T> Intermediate for validation_value::Value<T> {
    fn has_value(&self) -> bool {
        !self.is_none()
    }
}

impl<T> Intermediate for Vec<T> {
    fn has_value(&self) -> bool {
        !self.is_empty()
    }
}

/// A trait that describes a form field input element.
/// Stores and validates data posted from a form.
pub trait Descriptor: Sized {
    type Value;
    type Intermediate: Intermediate;

    /// Renders the form field as HTML markup.
    /// Preserves previous input values, even if they are invalid.
    /// Should not try to render error or help messages.
    fn render(field: &FormField<Self>) -> maud::Markup;

    /// Parses the input value from a string.
    /// If the value is empty, it should set the internal state to `None` or equivalent.
    /// If the value has been parsed before, it should overwrite the previous value, or
    /// extend it, if the descriptor supports multiple values.
    fn parse(&self, value: &str, intermediate: &mut Self::Intermediate);

    /// Returns the value of the descriptor.
    /// If the value is invalid, it should return an error message.
    fn validate(&self, intermediate: &Self::Intermediate) -> Result<Self::Value, &'_ str>;

    /// Loads a value into the descriptor.
    /// This is useful for pre-filling the form with existing data from e.g. a database.
    fn load(&self, value: Self::Value) -> Self::Intermediate;
}

/// A struct that represents a form field with its metadata and descriptor.
/// Values shared between every form field type are stored here, while input
/// specific data is stored in the `descriptor` field, which implements the `Descriptor` trait.
#[derive(Debug, Default)]
pub struct FormField<T: Descriptor> {
    pub display_name: &'static str,
    pub field_name: &'static str,
    pub descriptor: T,
    pub intermediate: T::Intermediate,
    pub required: bool,
    pub error: Option<String>,
    pub help_text: Option<&'static str>,
}

impl<T: Descriptor> maud::Render for FormField<T> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div {
                (T::render(&self))
                @if let Some(help) = &self.help_text {
                    div class="help-text" { (help) }
                }
                (self.render_error())
            }
        }
    }
}

impl<T: Descriptor> FormField<T> {
    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
    }

    fn render_error(&self) -> maud::Markup {
        if let Some(ref error) = self.error {
            maud::html! {
                div class="error" { (error) }
            }
        } else {
            maud::html! {}
        }
    }
}

#[cfg(feature = "derive")]
extern crate form_fields_macro;

#[cfg(feature = "derive")]
pub use form_fields_macro::{FromForm, Selectable};
