use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Result;

#[proc_macro_attribute]
pub fn super_enum(attr: TokenStream, tokens: TokenStream) -> TokenStream {
    match super_enum_internal(attr, tokens) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

fn super_enum_internal(
    attr: impl Into<TokenStream2>,
    tokens: impl Into<TokenStream2>,
) -> Result<TokenStream2> {
    Ok(quote!())
}
