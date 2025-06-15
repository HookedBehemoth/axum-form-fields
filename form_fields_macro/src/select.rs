use crate::{BaseField, FieldParseResult, maybe_extract_attribute, to_quote::ToQuote};

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(select))]
struct SelectAttributes {
    #[deluxe(flatten)]
    base: BaseField,
    options: Vec<syn::Expr>,
    default_value: Option<syn::Expr>,
    placeholder: String,
}

pub(crate) fn try_parse(
    field: &mut syn::Field,
    ident: &syn::Ident,
    field_type: &syn::Type,
    required: bool,
) -> deluxe::Result<Option<FieldParseResult>> {
    if let Some(attrs) = maybe_extract_attribute::<_, SelectAttributes>(field)? {
        let help_text = attrs.base.help_text.to_quote();
        let options = attrs.options.iter().collect::<Vec<_>>();
        let placeholder = attrs.placeholder;
        let default_value = attrs.default_value.to_quote();
        Ok(Some(FieldParseResult {
            ident: ident.clone(),
            required,
            display_name: attrs.base.display_name,
            field_name: attrs.base.field_name,
            help_text,
            field_type: quote::quote! {
                form_fields::elements::Select::<#field_type>
            },
            initializer: quote::quote! {
                form_fields::elements::Select::<#field_type> {
                    key: None,
                    options: vec![ #( #options ),* ],
                    placeholder: #placeholder.to_string(),
                    default_value: #default_value,
                }
            },
        }))
    } else {
        Ok(None)
    }
}
