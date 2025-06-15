use chrono::Datelike;
use proc_macro2::TokenStream;

pub(crate) trait ToQuote {
    fn to_quote(&self) -> TokenStream;
}

impl<T> ToQuote for Option<T>
where
    T: quote::ToTokens,
{
    fn to_quote(&self) -> TokenStream {
        match self {
            Some(value) => quote::quote! { Some(#value) },
            None => quote::quote! { None },
        }
    }
}

pub(crate) trait ToQuoteAs {
    fn to_quote_as(&self, ident: &syn::Type) -> TokenStream;
}

impl ToQuoteAs for Option<isize> {
    fn to_quote_as(&self, ident: &syn::Type) -> TokenStream {
        match self {
            Some(value) => quote::quote! { Some(#value as #ident) },
            None => quote::quote! { None },
        }
    }
}

impl ToQuoteAs for Option<chrono::NaiveDate> {
    fn to_quote_as(&self, ident: &syn::Type) -> TokenStream {
        match self {
            Some(value) => {
                let year = value.year();
                let month = value.month();
                let day = value.day();

                quote::quote! { Some(chrono::NaiveDate::from_ymd(#year, #month, #day) as #ident) }
            }
            None => quote::quote! { None },
        }
    }
}
