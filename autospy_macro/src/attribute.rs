use proc_macro2::TokenStream;
use syn::{Attribute, Expr, Meta, MetaList, MetaNameValue};

pub fn is_autospy_attribute(attribute: &Attribute) -> bool {
    autospy_attribute(attribute).is_some()
}

pub fn is_ignore_attribute(attribute: &Attribute) -> bool {
    match autospy_attribute(attribute) {
        Some(tokens) => tokens.to_string() == "ignore",
        None => false,
    }
}

pub fn associated_type(attributes: &[Attribute]) -> Option<&TokenStream> {
    autospy_attributes(attributes).next()
}

pub fn into_type(attributes: &[Attribute]) -> Option<Expr> {
    autospy_attributes(attributes).find_map(tokens_to_into_type)
}

pub fn return_type(attributes: &[Attribute]) -> Option<Expr> {
    autospy_attributes(attributes).find_map(tokens_to_returns_type)
}

fn tokens_to_into_type(tokens: &TokenStream) -> Option<Expr> {
    tokens_to_meta_name_value(tokens, "into")
}

fn tokens_to_returns_type(tokens: &TokenStream) -> Option<Expr> {
    tokens_to_meta_name_value(tokens, "returns")
}

fn tokens_to_meta_name_value(tokens: &TokenStream, expected_path: &str) -> Option<Expr> {
    match syn::parse2::<MetaNameValue>(tokens.clone()) {
        Ok(MetaNameValue { path, value, .. }) if path.is_ident(expected_path) => Some(value),
        _ => None,
    }
}

fn autospy_attributes(attributes: &[Attribute]) -> impl Iterator<Item = &TokenStream> {
    attributes.iter().filter_map(autospy_attribute)
}

fn autospy_attribute(attribute: &Attribute) -> Option<&TokenStream> {
    match &attribute.meta {
        Meta::List(MetaList { path, tokens, .. }) if path.is_ident("autospy") => Some(tokens),
        _ => None,
    }
}
