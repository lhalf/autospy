use proc_macro2::TokenStream;
use syn::parse::Parser;
use syn::{
    Attribute, Expr, ExprLit, Lit, Meta, MetaNameValue, Token, Type, TypePath, parse::Parse,
    punctuated::Punctuated,
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

pub fn has_use_default_attribute(attributes: &[Attribute]) -> bool {
    autospy_attributes(attributes).any(|attribute| attribute.to_string() == "use_default")
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
    let name_values = Punctuated::<Meta, Token![,]>::parse_terminated
        .parse2(autospy_attribute(attribute)?)
        .ok()?
        .into_iter()
        .filter_map(|meta| match meta {
            Meta::NameValue(name_value) => Some(name_value),
            _ => None,
        });

    Some(name_values)
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
        Meta::List(meta_list) if meta_list.path.is_ident("autospy") => {
            Some(meta_list.tokens.clone())
        }
        Meta::List(meta_list) if meta_list.path.is_ident("cfg_attr") => {
            let inner = Punctuated::<Meta, Token![,]>::parse_terminated
                .parse2(meta_list.tokens.clone())
                .ok()?;

            if inner.len() == 2 {
                if let Some(Meta::List(inner)) = inner.into_iter().nth(1) {
                    if inner.path.is_ident("autospy") {
                        return Some(inner.tokens.clone());
                    }
                }
            }
            None
        }
        _ => None,
    }
}
