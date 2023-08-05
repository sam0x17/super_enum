use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{parse::Nothing, parse2, parse_macro_input, parse_quote, Attribute, ItemEnum, Result};

#[proc_macro_attribute]
pub fn super_enum(attr: TokenStream, tokens: TokenStream) -> TokenStream {
    match super_enum_internal(attr, tokens) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

fn attribute_helper(ident: Ident, attr: TokenStream, tokens: TokenStream) -> TokenStream {
    let mut item_enum = parse_macro_input!(tokens as ItemEnum);
    let attr: TokenStream2 = attr.into();
    let attr: Attribute = if attr.is_empty() {
        parse_quote!(#[#ident])
    } else {
        parse_quote!(#[#ident(#attr)])
    };
    item_enum.attrs.push(attr);
    item_enum.to_token_stream().into()
}

#[proc_macro_attribute]
pub fn fields(attr: TokenStream, tokens: TokenStream) -> TokenStream {
    attribute_helper(parse_quote!(fields), attr, tokens)
}

#[proc_macro_attribute]
pub fn aggregate(attr: TokenStream, tokens: TokenStream) -> TokenStream {
    attribute_helper(parse_quote!(aggregate), attr, tokens)
}

fn super_enum_internal(
    attr: impl Into<TokenStream2>,
    tokens: impl Into<TokenStream2>,
) -> Result<TokenStream2> {
    parse2::<Nothing>(attr.into())?;
    let item_enum = parse2::<ItemEnum>(tokens.into())?;
    println!(
        "attrs: {:?}",
        item_enum
            .attrs
            .iter()
            .map(|attr| attr.to_token_stream().to_string())
            .collect::<Vec<_>>()
    );
    Ok(quote!())
}
