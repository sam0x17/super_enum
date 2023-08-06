use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
    parenthesized,
    parse::{Nothing, ParseStream},
    parse2, parse_macro_input, parse_quote,
    punctuated::Punctuated,
    Attribute, Error, Field, Ident, ItemEnum, Path, Result, Token, TypePath,
};

mod keywords {
    use syn::custom_keyword;

    custom_keyword!(fields);
    custom_keyword!(aggregate);
}

#[proc_macro_attribute]
pub fn super_enum(attr: TokenStream, tokens: TokenStream) -> TokenStream {
    match super_enum_internal(attr, tokens) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.into_compile_error().into(),
    }
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

enum HelperAttr {
    Aggregate(Punctuated<TypePath, Token![,]>),
    Fields(Punctuated<Field, Token![,]>),
    Regular(Attribute),
}

impl syn::parse::Parse for HelperAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(keywords::aggregate) {
            input.parse::<keywords::aggregate>()?;
            let content;
            parenthesized!(content in input);
            let paths = content.parse_terminated(TypePath::parse, Token![,])?;
            Ok(HelperAttr::Aggregate(paths))
        } else if input.peek(keywords::fields) {
            input.parse::<keywords::fields>()?;
            let content;
            parenthesized!(content in input);
            let fields = content.parse_terminated(Field::parse_named, Token![,])?;
            Ok(HelperAttr::Fields(fields))
        } else if input.peek(Ident) {
            let path = input.parse::<Path>()?;
            if input.is_empty() {
                return Ok(HelperAttr::Regular(parse_quote!(#[#path])));
            }
            let mut inner_tokens = TokenStream2::new();
            while !input.is_empty() {
                inner_tokens.extend([input.parse::<TokenStream2>()?]);
            }
            let attribute: Attribute = parse_quote!(#[#path #inner_tokens]);
            Ok(HelperAttr::Regular(attribute))
        } else {
            Err(Error::new(input.span(), "Expected `aggregate` or `fields`"))
        }
    }
}

impl ToTokens for HelperAttr {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            HelperAttr::Aggregate(contents) => {
                let contents = contents.into_iter();
                tokens.extend([quote!(#[aggregate(#(#contents),*)])])
            }
            HelperAttr::Fields(contents) => {
                let contents = contents.into_iter();
                tokens.extend([quote!(#[fields(#(#contents),*)])])
            }
            HelperAttr::Regular(attr) => tokens.extend([attr.to_token_stream()]),
        }
    }
}

#[test]
fn test_parse_helper_attr_aggregate() {
    assert!(matches!(
        parse2::<HelperAttr>(quote!(aggregate())).unwrap(),
        HelperAttr::Aggregate(_)
    ));
    assert!(matches!(
        parse2::<HelperAttr>(quote!(aggregate(an_ident))).unwrap(),
        HelperAttr::Aggregate(_)
    ));
    assert!(matches!(
        parse2::<HelperAttr>(quote!(aggregate(
            some::Long::complex_path,
            another::Thing,
            something_else
        )))
        .unwrap(),
        HelperAttr::Aggregate(_)
    ));
    assert!(parse2::<HelperAttr>(quote!(aggregate)).is_err());
    assert!(parse2::<HelperAttr>(quote!(aggregate[test])).is_err());
    assert!(matches!(
        parse2::<HelperAttr>(quote!(aggregates(test))).unwrap(),
        HelperAttr::Regular(_)
    ));
}

#[test]
fn test_parse_helper_attr_fields() {
    assert!(matches!(
        parse2::<HelperAttr>(quote!(fields())).unwrap(),
        HelperAttr::Fields(_)
    ));
    assert!(matches!(
        parse2::<HelperAttr>(quote!(fields(something: u32))).unwrap(),
        HelperAttr::Fields(_)
    ));
    assert!(matches!(
        parse2::<HelperAttr>(quote!(fields(
            a: usize,
            b: Option<u32>,
            c: bool
        )))
        .unwrap(),
        HelperAttr::Fields(_)
    ));
    assert!(matches!(
        parse2::<HelperAttr>(quote!(field)).unwrap(),
        HelperAttr::Regular(_)
    ));
    assert!(parse2::<HelperAttr>(quote!(fields[foo: Bar])).is_err());
    assert!(matches!(
        parse2::<HelperAttr>(quote!(field(test))).unwrap(),
        HelperAttr::Regular(_)
    ));
}

#[test]
fn test_parse_helper_regular() {
    assert!(matches!(
        parse2::<HelperAttr>(quote!(something)).unwrap(),
        HelperAttr::Regular(_)
    ));
    assert!(matches!(
        parse2::<HelperAttr>(quote!(cfg(test))).unwrap(),
        HelperAttr::Regular(_)
    ));
    assert!(matches!(
        parse2::<HelperAttr>(quote!(doc = "hello")).unwrap(),
        HelperAttr::Regular(_)
    ));
}
