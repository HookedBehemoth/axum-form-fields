pub mod checkbox;
#[cfg(feature = "chrono")]
pub mod date_picker;
pub mod multiselect;
pub mod number_field;
pub mod passthrough;
pub mod password_field;
pub mod radio_button;
pub mod select;
pub mod text_field;

pub type TextField = text_field::TextField;
pub type PasswordField = password_field::PasswordField;
pub type NumberField<T> = number_field::NumberField<T>;
#[cfg(feature = "chrono")]
pub type DatePicker = date_picker::DatePicker;
pub type Checkbox = checkbox::Checkbox;
pub type RadioButton<T> = radio_button::RadioButton<T>;
pub type Select<T> = select::Select<T>;
pub type MultiSelect<T> = multiselect::MultiSelect<T>;
pub type Passthrough<T> = passthrough::Passthrough<T>;
