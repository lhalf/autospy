use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    Attribute, Expr, ExprLit, Lit, Meta, MetaList, MetaNameValue, Token, Type, TypePath,
    parse::Parse, punctuated::Punctuated,
};

pub fn is_autospy_attribute(attribute: &Attribute) -> bool {
    autospy_attribute(attribute).is_some()
}

pub fn is_ignore_attribute(attribute: &Attribute) -> bool {
    match autospy_attribute(attribute) {
        Some(tokens) => tokens.to_string() == "ignore",
        None => false,
    }
}

pub fn associated_type(attributes: &[Attribute]) -> Option<TypePath> {
    Some(
        syn::parse2(autospy_attributes(attributes).next()?.clone())
            .expect("invalid associated type"),
    )
}

pub fn associated_const(attributes: &[Attribute]) -> Option<Expr> {
    syn::parse2::<Expr>(autospy_attributes(attributes).next()?.clone()).ok()
}

pub fn into_type(attributes: &[Attribute]) -> Option<Type> {
    key_value_autospy_attributes(attributes)
        .find_map(|name_value| matching_meta_name_value(name_value, "into"))
        .map(parse_literal_expression::<Type>)
}

pub fn with_expression(attributes: &[Attribute]) -> Option<Expr> {
    key_value_autospy_attributes(attributes)
        .find_map(|name_value| matching_meta_name_value(name_value, "with"))
        .map(parse_literal_expression::<Expr>)
}

pub fn return_type(attributes: &[Attribute]) -> Option<Type> {
    key_value_autospy_attributes(attributes)
        .find_map(|name_value| matching_meta_name_value(name_value, "returns"))
        .map(parse_literal_expression::<Type>)
}

fn matching_meta_name_value(name_value: MetaNameValue, expected_path: &str) -> Option<Expr> {
    match name_value {
        MetaNameValue { path, value, .. } if path.is_ident(expected_path) => Some(value),
        _ => None,
    }
}

fn key_value_autospy_attributes(attributes: &[Attribute]) -> impl Iterator<Item = MetaNameValue> {
    attributes
        .iter()
        .filter(|a| is_autospy_attribute(a))
        .filter_map(autospy_attribute_key_values)
        .flatten()
}

fn autospy_attribute_key_values(
    attribute: &Attribute,
) -> Option<impl Iterator<Item = MetaNameValue>> {
    let name_values: Punctuated<MetaNameValue, Token![,]> = attribute
        .parse_args_with(Punctuated::parse_separated_nonempty)
        .ok()?;
    Some(name_values.into_iter())
}

fn parse_literal_expression<T: Parse>(expression: Expr) -> T {
    match expression {
        Expr::Lit(ExprLit {
            lit: Lit::Str(literal),
            ..
        }) => literal.parse().expect("could not parse autospy value"),
        _ => panic!("invalid autospy value"),
    }
}

fn autospy_attributes(attributes: &[Attribute]) -> impl Iterator<Item = TokenStream> {
    attributes.iter().filter_map(autospy_attribute)
}

fn autospy_attribute(attribute: &Attribute) -> Option<TokenStream> {
    match &attribute.meta {
        Meta::List(MetaList { path, tokens, .. }) if path.is_ident("autospy") => {
            Some(tokens.clone())
        }
        Meta::List(MetaList { path, tokens, .. }) if path.is_ident("cfg_attr") => {
            let tokens = tokens.to_string();
            tokens
                .strip_prefix("test , autospy (")
                .and_then(|rest| rest.strip_suffix(')'))
                .filter(|tokens| !tokens.is_empty())
                .map(|tokens| tokens.to_token_stream())
        }
        _ => None,
    }
}
