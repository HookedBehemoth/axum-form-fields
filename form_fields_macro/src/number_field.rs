use crate::{
    BaseField, FieldParseResult, maybe_extract_attribute,
    to_quote::{ToQuote, ToQuoteAs},
};

// Example #[number_field(min = 0, max = 120)]
#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(number_field))]
struct NumberFieldAttributes {
    #[deluxe(flatten)]
    base: BaseField,
    min: Option<isize>,
    max: Option<isize>,
}

pub(crate) fn try_parse(
    field: &mut syn::Field,
    ident: &syn::Ident,
    field_type: &syn::Type,
    required: bool,
) -> deluxe::Result<Option<FieldParseResult>> {
    if let Some(attrs) = maybe_extract_attribute::<_, NumberFieldAttributes>(field)? {
        let help_text = attrs.base.help_text.to_quote();
        let min = attrs.min.to_quote_as(field_type);
        let max = attrs.max.to_quote_as(field_type);
        Ok(Some(FieldParseResult {
            ident: ident.clone(),
            required,
            display_name: attrs.base.display_name,
            field_name: attrs.base.field_name,
            help_text,
            field_type: quote::quote! {
                form_fields::elements::NumberField::<#field_type>
            },
            initializer: quote::quote! {
                form_fields::elements::NumberField::<#field_type> {
                    min: #min,
                    max: #max,
                }
            },
        }))
    } else {
        Ok(None)
    }
}
