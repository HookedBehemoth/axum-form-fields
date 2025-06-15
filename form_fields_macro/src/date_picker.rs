use std::str::FromStr;

use crate::{
    BaseField, FieldParseResult, maybe_extract_attribute,
    to_quote::{ToQuote, ToQuoteAs},
};

// Example #[date_select(min = "2023-01-01", max = "2023-12-31")]
#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(date_select))]
struct DatePickerAttributes {
    #[deluxe(flatten)]
    base: BaseField,
    min: Option<String>,
    max: Option<String>,
}

pub(crate) fn try_parse(
    field: &mut syn::Field,
    ident: &syn::Ident,
    field_type: &syn::Type,
    required: bool,
) -> deluxe::Result<Option<FieldParseResult>> {
    if let Some(attrs) = maybe_extract_attribute::<_, DatePickerAttributes>(field)? {
        let help_text = attrs.base.help_text.to_quote();
        let min = attrs
            .min
            .and_then(|min| chrono::NaiveDate::from_str(&min).ok());
        let max = attrs
            .max
            .and_then(|min| chrono::NaiveDate::from_str(&min).ok());

        let min = min.to_quote_as(field_type);
        let max = max.to_quote_as(field_type);
        Ok(Some(FieldParseResult {
            ident: ident.clone(),
            required,
            display_name: attrs.base.display_name,
            field_name: attrs.base.field_name,
            help_text,
            field_type: quote::quote! {
                form_fields::elements::DatePicker
            },
            initializer: quote::quote! {
                form_fields::elements::DatePicker {
                    value: form_fields::validation_value::Value::None,
                    min: #min,
                    max: #max,
                }
            },
        }))
    } else {
        Ok(None)
    }
}
