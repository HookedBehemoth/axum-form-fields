use crate::{BaseField, FieldParseResult, maybe_extract_attribute, to_quote::ToQuote};

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(radio_button))]
struct RadioButtonAttributes {
    #[deluxe(flatten)]
    base: BaseField,
    options: Vec<syn::Expr>,
    default_value: syn::Expr,
}

pub(crate) fn try_parse(
    field: &mut syn::Field,
    ident: &syn::Ident,
    field_type: &syn::Type,
    required: bool,
) -> deluxe::Result<Option<FieldParseResult>> {
    if let Some(attrs) = maybe_extract_attribute::<_, RadioButtonAttributes>(field)? {
        let help_text = attrs.base.help_text.to_quote();
        let options = attrs.options;
        let default_value = attrs.default_value;
        Ok(Some(FieldParseResult {
            ident: ident.clone(),
            required,
            display_name: attrs.base.display_name,
            field_name: attrs.base.field_name,
            help_text,
            field_type: quote::quote! {
                form_fields::elements::RadioButton::<#field_type>
            },
            initializer: quote::quote! {
                form_fields::elements::RadioButton::<#field_type> {
                    key: None,
                    options: vec![ #( #options ),* ],
                    default_value: #default_value,
                }
            },
        }))
    } else {
        Ok(None)
    }
}
