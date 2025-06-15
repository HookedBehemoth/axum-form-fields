pub(crate) fn selectable(ident: &proc_macro2::Ident) -> proc_macro2::TokenStream {
    quote::quote! {
        impl form_fields::selectable::Selectable for #ident {
            type Key = Self;
            type DisplayValue = Self;
            fn key(&self) -> Self::Key {
                *self
            }
            fn display_value(&self) -> Self::DisplayValue {
                *self
            }
        }
    }
}
