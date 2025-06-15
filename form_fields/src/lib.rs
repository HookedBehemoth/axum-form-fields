pub mod elements;
pub mod from_form;
pub mod selectable;
pub mod validation_value;

pub trait Descriptor: Sized {
    type Value;

    fn render(field: &FormField<Self>) -> maud::Markup;
    fn parse(&mut self, value: &str);
    fn has_value(&self) -> bool;
    fn value(&self) -> Result<Self::Value, &'_ str>;
    fn load(&mut self, value: Self::Value);
}

#[derive(Debug)]
pub struct FormField<T: Descriptor> {
    pub display_name: &'static str,
    pub field_name: &'static str,
    pub descriptor: T,
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
