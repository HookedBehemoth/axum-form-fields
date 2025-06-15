use core::panic;
use proc_macro2::TokenStream;
use selectable::selectable;
use syn::parse_macro_input;
use syn::{DeriveInput, Field, GenericArgument, Ident, PathArguments, Type, spanned::Spanned};

mod checkbox;
mod date_picker;
mod multiselect;
mod number_field;
mod radio_button;
mod select;
mod selectable;
mod text_field;
pub(crate) mod to_quote;

/// Derive macro for generating form field specifications from a struct.
/// 
/// This macro generates a secondary struct with the same fields as the original struct, 
/// but each field is wrapped in a `form_fields::FormField` type. The generated struct 
/// provides functionality for rendering, parsing, validating, and loading form data.
/// 
/// ### Supported Attributes
/// The macro supports the following attributes, which correspond to specific HTML input types:
/// 
/// #### `#[text_field]`
/// - **Description**: Represents a text input field.
/// - **HTML Input Type**: [`<input type="text">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/text)
/// - **Parameters**:
///   - `display_name`: A string to display as the label for the field.
///   - `max_length`: Maximum number of characters allowed in the input.
///   - `min_length`: Minimum number of characters required in the input.
///   - `placeholder`: Placeholder text displayed inside the input field.
/// 
/// #### `#[number_field]`
/// - **Description**: Represents a number input field.
/// - **HTML Input Type**: [`<input type="number">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/number)
/// - **Parameters**:
///   - `display_name`: A string to display as the label for the field.
///   - `min`: Minimum value allowed for the input.
///   - `max`: Maximum value allowed for the input.
/// 
/// #### `#[date_select]`
/// - **Description**: Represents a date picker input field.
/// - **HTML Input Type**: [`<input type="date">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/date)
/// - **Parameters**:
///   - `display_name`: A string to display as the label for the field.
///   - `min`: Minimum date allowed (formatted as `YYYY-MM-DD`).
///   - `max`: Maximum date allowed (formatted as `YYYY-MM-DD`).
/// 
/// #### `#[checkbox]`
/// - **Description**: Represents a checkbox input field.
/// - **HTML Input Type**: [`<input type="checkbox">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/checkbox)
/// - **Parameters**:
///   - `display_name`: A string to display as the label for the field.
///   - `checked`: Whether the checkbox is pre-checked.
///   - `required_true`: Whether the checkbox must be checked to pass validation.
///   - `help_text`: Additional text to display as help for the field.
/// 
/// #### `#[radio_button]`
/// - **Description**: Represents a radio button input field.
/// - **HTML Input Type**: [`<input type="radio">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/radio)
/// - **Parameters**:
///   - `display_name`: A string to display as the label for the field.
///   - `options`: A list of selectable options.
///   - `default_value`: The default selected option.
/// 
/// #### `#[select]`
/// - **Description**: Represents a dropdown select input field.
/// - **HTML Input Type**: [`<select>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select)
/// - **Parameters**:
///   - `display_name`: A string to display as the label for the field.
///   - `options`: A list of selectable options.
///   - `default_value`: The default selected option.
///   - `placeholder`: Placeholder text displayed when no option is selected.
/// 
/// #### `#[multiselect]`
/// - **Description**: Represents a multi-select input field.
/// - **HTML Input Type**: [`<select multiple>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select#attr-multiple)
/// - **Parameters**:
///   - `display_name`: A string to display as the label for the field.
///   - `options`: A list of selectable options.
/// 
/// ### Example Usage
/// ```rust
/// #[derive(Debug, FromForm)]
/// struct Test {
///     #[text_field(display_name = "Required Text", max_length = 50)]
///     pub text: String,
/// 
///     #[number_field(display_name = "Age", min = 0, max = 120)]
///     pub age: u8,
/// 
///     #[checkbox(display_name = "Accept Terms", required_true)]
///     pub accept_terms: bool,
/// }
/// ```
/// 
/// This will generate a `TestFormSpec` struct with fields wrapped in `form_fields::FormField`.
/// The generated struct can be used for rendering, parsing, and validating form data.
#[proc_macro_derive(
    FromForm,
    attributes(
        text_field,
        number_field,
        date_select,
        checkbox,
        radio_button,
        select,
        multiselect
    )
)]
pub fn from_form(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match form_derive(item.into()) {
        Ok(result) => result.into(),
        Err(err) => {
            let err = err.to_compile_error();
            quote::quote_spanned! {err.span() => #err}.into()
        }
    }
}

#[proc_macro_derive(Selectable)]
pub fn selectable_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let ident = input.ident.clone();

    let expanded = selectable(&ident);

    expanded.into()
}

fn extract_fields(ast: &mut DeriveInput) -> deluxe::Result<Vec<FieldParseResult>> {
    let data = match &mut ast.data {
        syn::Data::Struct(data) => data,
        _ => panic!("FromForm can only be derived for structs"),
    };

    let mut fields = Vec::new();

    for field in data.fields.iter_mut() {
        let field = parse_field(field)?;
        fields.push(field);
    }

    Ok(fields)
}

fn generate_struct(name: &Ident, fields: &[FieldParseResult]) -> TokenStream {
    let field_names: Vec<&Ident> = fields.iter().map(|f| &f.ident).collect();
    let field_types: Vec<&TokenStream> = fields.iter().map(|f| &f.field_type).collect();

    let text = quote::quote! {
        #[derive(Debug)]
        pub struct #name {
            #(
                pub #field_names: form_fields::FormField<#field_types>,
            )*
        }
    };

    text
}

fn generate_from_request(
    newtype: &Ident,
    original: &Ident,
    fields: &[FieldParseResult],
) -> TokenStream {
    let idents: Vec<&Ident> = fields.iter().map(|f| &f.ident).collect();

    let text = quote::quote! {
        impl form_fields::from_form::FormSpecable for #original {
            type Spec = #newtype;
        }

        impl form_fields::from_form::FormSpec for #newtype {
            fn generate_spec() -> Self {
                Self::new()
            }

            fn parse_field(&mut self, name: &str, value: &str) -> bool {
                #(
                    if name == stringify!(#idents) {
                        form_fields::Descriptor::parse(&mut self.#idents.descriptor, value);
                        true
                    } else
                )*
                {
                    false
                }
            }
        }
    };

    text
}

fn generate_impl(newtype: &Ident, origin: &Ident, fields: &[FieldParseResult]) -> TokenStream {
    let display_names: Vec<String> = fields
        .iter()
        .map(|f| {
            f.display_name
                .clone()
                .unwrap_or_else(|| f.ident.to_string())
        })
        .collect();
    let field_names: Vec<String> = fields
        .iter()
        .map(|f| f.field_name.clone().unwrap_or_else(|| f.ident.to_string()))
        .collect();
    let idents: Vec<&Ident> = fields.iter().map(|f| &f.ident).collect();
    let help_text: Vec<&TokenStream> = fields.iter().map(|f| &f.help_text).collect();
    let initializers: Vec<&TokenStream> = fields.iter().map(|f| &f.initializer).collect();
    let required: Vec<bool> = fields.iter().map(|f| f.required).collect();

    let required_fields: Vec<_> = fields
        .iter()
        .filter(|f| f.required)
        .map(|f| &f.ident)
        .collect();
    let other_fields: Vec<_> = fields
        .iter()
        .filter(|f| !f.required)
        .map(|f| &f.ident)
        .collect();

    let text = quote::quote! {
        impl #newtype {
            fn new() -> Self {
                #(
                    let #idents = form_fields::FormField {
                        display_name: #display_names,
                        field_name: #field_names,
                        descriptor: #initializers,
                        required: #required,
                        error: None,
                        help_text: #help_text,
                    };
                )*

                Self {
                    #(#idents,)*
                }
            }

            fn inner(&mut self) -> Option<#origin> {
                #(
                    let #required_fields = match form_fields::Descriptor::value(&self.#required_fields.descriptor) {
                        Ok(value) => Some(value),
                        Err(err) => {
                            self.#required_fields.error = Some(err.to_string());
                            None
                        }
                    };
                )*
                #(
                    let #other_fields = if !form_fields::Descriptor::has_value(&self.#other_fields.descriptor) {
                        None
                    } else {
                        match form_fields::Descriptor::value(&self.#other_fields.descriptor) {
                            Ok(value) => Some(value),
                            Err(err) => {
                                self.#other_fields.error = Some(err.to_string());
                                None
                            }
                        }
                    };
                )*

                #(
                    let #required_fields = #required_fields?;
                )*

                if #(self.#idents.error.is_some())||* {
                    return None;
                }

                // Unwrap required fields, pass optional fields.
                Some(#origin {
                    #(#required_fields: #required_fields,)*
                    #(#other_fields: #other_fields,)*
                })
            }

            fn valid(&self) -> Option<()> {
                if #(self.#idents.error.is_some())||* {
                    return None;
                }

                Some(())
            }

            fn load(&mut self, input: #origin) {
                #(
                    form_fields::Descriptor::load(&mut self.#required_fields.descriptor, input.#required_fields);
                )*
                #(
                    if let Some(value) = input.#other_fields {
                        form_fields::Descriptor::load(&mut self.#other_fields.descriptor, value);
                    }
                )*
            }
        }
    };

    text
}

fn form_derive(item: TokenStream) -> deluxe::Result<TokenStream> {
    let mut ast: DeriveInput = syn::parse2(item)?;

    let origin = ast.ident.clone();
    let newtype = quote::format_ident!("{}{}", origin, "FormSpec");

    let fields = extract_fields(&mut ast)?;

    let r#struct = generate_struct(&newtype, &fields);
    let r#from_request = generate_from_request(&newtype, &origin, &fields);
    let r#impl = generate_impl(&newtype, &origin, &fields);

    let text = quote::quote! {
        #r#struct

        #r#from_request

        #r#impl
    };

    Ok(text)
}

pub(crate) struct FieldParseResult {
    ident: Ident,
    required: bool,
    display_name: Option<String>,
    field_name: Option<String>,
    help_text: TokenStream,
    field_type: TokenStream,
    initializer: TokenStream,
}

#[derive(deluxe::ParseMetaItem)]
pub(crate) struct BaseField {
    #[deluxe(default)]
    display_name: Option<String>,
    #[deluxe(default)]
    field_name: Option<String>,
    #[deluxe(default)]
    help_text: Option<String>,
}

fn parse_field(field: &mut Field) -> deluxe::Result<FieldParseResult> {
    let ident = field.ident.as_ref().unwrap().clone();

    let (required, field_type) = extract_option_inner(&field.ty)?;

    if let Some(number_field) = number_field::try_parse(field, &ident, &field_type, required)? {
        return Ok(number_field);
    }

    if let Some(text_field) = text_field::try_parse(field, &ident, &field_type, required)? {
        return Ok(text_field);
    }

    if let Some(date_picker) = date_picker::try_parse(field, &ident, &field_type, required)? {
        return Ok(date_picker);
    }

    if let Some(checkbox) = checkbox::try_parse(field, &ident, &field_type, required)? {
        return Ok(checkbox);
    }

    if let Some(radio_button) = radio_button::try_parse(field, &ident, &field_type, required)? {
        return Ok(radio_button);
    }

    if let Some(select) = select::try_parse(field, &ident, &field_type, required)? {
        return Ok(select);
    }

    if let Some(multiselect) = multiselect::try_parse(field, &ident, &field_type, required)? {
        return Ok(multiselect);
    }

    Err(syn::Error::new(
        field.span(),
        "Requires attribute [text_field], [number_field] or [checkbox]",
    ))
}

fn extract_option_inner(ty: &Type) -> deluxe::Result<(bool, Type)> {
    let Type::Path(type_path) = ty else {
        return deluxe::Result::Err(syn::Error::new(ty.span(), "Expected a type path"));
    };

    let Some(segment) = type_path.path.segments.first() else {
        return deluxe::Result::Err(syn::Error::new(
            type_path.span(),
            "Expected a type path with at least one segment",
        ));
    };

    if segment.ident != "Option" {
        // panic!("Expected type to be an Option, found: {}", segment.ident);
        // If it's not an Option, return the type as is
        return Ok((true, ty.clone()));
    }

    let PathArguments::AngleBracketed(args) = &segment.arguments else {
        return deluxe::Result::Err(syn::Error::new(
            segment.span(),
            "Expected angle bracketed arguments for Option",
        ));
    };

    let Some(GenericArgument::Type(inner_type)) = args.args.first() else {
        return deluxe::Result::Err(syn::Error::new(
            args.span(),
            "Expected a type argument for Option",
        ));
    };

    Ok((false, inner_type.clone()))
}

// https://github.com/jf2048/deluxe/issues/24#issuecomment-2518421372
pub(crate) fn maybe_extract_attribute<T, R>(t: &mut T) -> deluxe::Result<Option<R>>
where
    T: deluxe::HasAttributes,
    R: deluxe::ExtractAttributes<T>,
{
    let mut have_attr = false;
    for attr in t.attrs() {
        if R::path_matches(attr.meta.path()) {
            have_attr = true;
        }
    }
    if !have_attr {
        return Ok(None);
    }
    Ok(Some(R::extract_attributes(t)?))
}
