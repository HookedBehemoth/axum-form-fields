use crate::{
    BaseField, FieldParseResult, maybe_extract_attribute,
    to_quote::ToQuote,
};

// Example #[number_field(min = 0, max = 120)]
#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(passthrough))]
struct PassthroughAttributes {
    #[deluxe(flatten)]
    base: BaseField,
}

pub(crate) fn try_parse(
    field: &mut syn::Field,
    ident: &syn::Ident,
    field_type: &syn::Type,
    required: bool,
) -> deluxe::Result<Option<FieldParseResult>> {
    if let Some(attrs) = maybe_extract_attribute::<_, PassthroughAttributes>(field)? {
        let help_text = attrs.base.help_text.to_quote();
        Ok(Some(FieldParseResult {
            ident: ident.clone(),
            required,
            display_name: attrs.base.display_name,
            field_name: attrs.base.field_name,
            help_text,
            field_type: quote::quote! {
                form_fields::elements::Passthrough::<#field_type>
            },
            initializer: quote::quote! {
                form_fields::elements::Passthrough::<#field_type>::default()
            },
        }))
    } else {
        Ok(None)
    }
}
