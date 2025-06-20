use crate::{BaseField, FieldParseResult, maybe_extract_attribute, to_quote::ToQuote};

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(multiselect))]
struct MultiSelectAttributes {
    #[deluxe(flatten)]
    base: BaseField,
    options: Vec<syn::Expr>,
}

pub(crate) fn try_parse(
    field: &mut syn::Field,
    ident: &syn::Ident,
    field_type: &syn::Type,
    required: bool,
) -> deluxe::Result<Option<FieldParseResult>> {
    if let Some(attrs) = maybe_extract_attribute::<_, MultiSelectAttributes>(field)? {
        let syn::Type::Path(ty) = field_type else {
            return Ok(None);
        };
        let Some(segment) = ty.path.segments.last() else {
            return Ok(None);
        };
        if segment.ident != "Vec" {
            return Ok(None);
        }
        let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
            return Ok(None);
        };
        let Some(syn::GenericArgument::Type(field_type)) = args.args.first() else {
            return Ok(None);
        };

        let help_text = attrs.base.help_text.to_quote();
        let options = attrs.options.iter().collect::<Vec<_>>();
        Ok(Some(FieldParseResult {
            ident: ident.clone(),
            required,
            display_name: attrs.base.display_name,
            field_name: attrs.base.field_name,
            help_text,
            field_type: quote::quote! {
                form_fields::elements::MultiSelect::<#field_type>
            },
            initializer: quote::quote! {
                form_fields::elements::MultiSelect::<#field_type> {
                    options: vec![ #( #options ),* ],
                }
            },
        }))
    } else {
        Ok(None)
    }
}
