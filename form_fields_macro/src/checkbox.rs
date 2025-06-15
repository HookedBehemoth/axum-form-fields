use crate::{BaseField, FieldParseResult, maybe_extract_attribute, to_quote::ToQuote};

// Example #[checkbox(checked = true, required_true = true)]
#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(checkbox))]
struct CheckboxAttributes {
    #[deluxe(flatten)]
    base: BaseField,
    checked: bool,
    required_true: Option<bool>,
}

pub(crate) fn try_parse(
    field: &mut syn::Field,
    ident: &syn::Ident,
    _field_type: &syn::Type,
    required: bool,
) -> deluxe::Result<Option<FieldParseResult>> {
    if let Some(attrs) = maybe_extract_attribute::<_, CheckboxAttributes>(field)? {
        let help_text = attrs.base.help_text.to_quote();
        let checked = attrs.checked;
        let required_true = attrs.required_true.unwrap_or(false);
        Ok(Some(FieldParseResult {
            ident: ident.clone(),
            required,
            display_name: attrs.base.display_name,
            field_name: attrs.base.field_name,
            help_text,
            field_type: quote::quote! {
                form_fields::elements::Checkbox
            },
            initializer: quote::quote! {
                form_fields::elements::Checkbox {
                    value: None,
                    checked: #checked,
                    required_true: #required_true,
                }
            },
        }))
    } else {
        Ok(None)
    }
}
