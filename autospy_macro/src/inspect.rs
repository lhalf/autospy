use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    Attribute, FnArg, Ident, ItemTrait, Meta, MetaList, MetaNameValue, Pat, PatType, TraitItem,
    TraitItemFn, Type,
};

const AUTOSPY_TOKEN: &str = "autospy";
const IGNORE_TOKEN: &str = "ignore";
const RETURNS_TOKEN: &str = "returns";
const INTO_TOKEN: &str = "into";

pub fn trait_functions(item_trait: &ItemTrait) -> impl Iterator<Item = &TraitItemFn> {
    item_trait.items.iter().filter_map(|item| match item {
        TraitItem::Fn(function) => Some(function),
        _ => None,
    })
}

pub fn spyable_arguments(function: &TraitItemFn) -> impl Iterator<Item = SpyableArgument> {
    non_self_function_arguments(function).filter_map(spyable_argument)
}

pub struct SpyableArgument {
    pub name: Ident,
    pub into_type: Option<TokenStream>,
    pub dereferenced_type: Type,
    pub dereference_count: u8,
}

pub fn is_autospy_attribute(attribute: &Attribute) -> bool {
    autospy_attribute(attribute).is_some()
}

pub fn is_argument_marked_as_ignore(argument: &PatType) -> bool {
    argument.attrs.iter().any(is_ignore_attribute)
}

pub fn get_return_attribute_type(attributes: &[Attribute]) -> Option<TokenStream> {
    attributes
        .iter()
        .find_map(autospy_attribute)
        .and_then(returns_attribute_type)
}

fn is_ignore_attribute(attribute: &Attribute) -> bool {
    match autospy_attribute(attribute) {
        Some(tokens) => tokens.to_string() == IGNORE_TOKEN,
        None => false,
    }
}

fn autospy_attribute(attribute: &Attribute) -> Option<&TokenStream> {
    match &attribute.meta {
        Meta::List(MetaList { path, tokens, .. }) if path.is_ident(AUTOSPY_TOKEN) => Some(tokens),
        _ => None,
    }
}

fn returns_attribute_type(tokens: &TokenStream) -> Option<TokenStream> {
    match syn::parse2::<MetaNameValue>(tokens.clone()) {
        Ok(MetaNameValue { path, value, .. }) if path.is_ident(RETURNS_TOKEN) => {
            Some(value.to_token_stream())
        }
        _ => None,
    }
}

fn non_self_function_arguments(function: &TraitItemFn) -> impl Iterator<Item = &PatType> {
    function.sig.inputs.iter().filter_map(|input| match input {
        FnArg::Typed(argument) => Some(argument),
        _ => None,
    })
}

fn spyable_argument(argument: &PatType) -> Option<SpyableArgument> {
    let name = match *argument.pat {
        Pat::Ident(ref pat_ident) => pat_ident.ident.clone(),
        _ => return None,
    };

    if is_argument_marked_as_ignore(argument) {
        return None;
    }

    let (dereferenced_type, dereference_count) = remove_references(&argument.ty);

    Some(SpyableArgument {
        name,
        into_type: argument_attribute_into_type(argument),
        dereferenced_type,
        dereference_count,
    })
}

fn argument_attribute_into_type(argument: &PatType) -> Option<TokenStream> {
    argument
        .attrs
        .iter()
        .find_map(autospy_attribute)
        .and_then(into_attribute_type)
}

fn into_attribute_type(tokens: &TokenStream) -> Option<TokenStream> {
    match syn::parse2::<MetaNameValue>(tokens.clone()) {
        Ok(MetaNameValue { path, value, .. }) if path.is_ident(INTO_TOKEN) => {
            Some(value.to_token_stream())
        }
        _ => None,
    }
}

fn remove_references(argument_type: &Type) -> (Type, u8) {
    match argument_type {
        Type::Reference(referenced_argument) => {
            let (dereferenced_argument, count) = remove_references(&referenced_argument.elem);
            (dereferenced_argument, count + 1)
        }
        argument_type => (argument_type.clone(), 0),
    }
}
