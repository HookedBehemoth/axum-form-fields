use crate::{BaseField, FieldParseResult, maybe_extract_attribute, to_quote::ToQuote};

// Example #[text_field(max_length = 5, placeholder = "Enter text")]
#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(text_field))]
struct TextFieldAttributes {
    #[deluxe(flatten)]
    base: BaseField,
    min_length: Option<usize>,
    max_length: Option<usize>,
    placeholder: Option<String>,
}

pub(crate) fn try_parse(
    field: &mut syn::Field,
    ident: &syn::Ident,
    _field_type: &syn::Type,
    required: bool,
) -> deluxe::Result<Option<FieldParseResult>> {
    if let Some(attrs) = maybe_extract_attribute::<_, TextFieldAttributes>(field)? {
        let help_text = attrs.base.help_text.to_quote();
        let min_length = attrs.min_length.to_quote();
        let max_length = attrs.max_length.to_quote();
        let placeholder = attrs.placeholder.to_quote();
        Ok(Some(FieldParseResult {
            ident: ident.clone(),
            required,
            display_name: attrs.base.display_name,
            field_name: attrs.base.field_name,
            help_text,
            field_type: quote::quote! {
                form_fields::elements::TextField
            },
            initializer: quote::quote! {
                form_fields::elements::TextField {
                    value: None,
                    min_length: #min_length,
                    max_length: #max_length,
                    placeholder: #placeholder,
                }
            },
        }))
    } else {
        Ok(None)
    }
}
