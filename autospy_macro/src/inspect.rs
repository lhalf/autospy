use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemTrait, ReturnType, TraitItem, TraitItemConst, TraitItemFn, Type};

pub fn associated_consts(item_trait: &ItemTrait) -> impl Iterator<Item = &TraitItemConst> {
    item_trait.items.iter().filter_map(|item| match item {
        TraitItem::Const(associated_const) => Some(associated_const),
        _ => None,
    })
}

pub fn trait_functions(item_trait: &ItemTrait) -> impl Iterator<Item = &TraitItemFn> {
    item_trait.items.iter().filter_map(|item| match item {
        TraitItem::Fn(function) => Some(function),
        _ => None,
    })
}

pub fn owned_trait_functions(item_trait: ItemTrait) -> impl Iterator<Item = TraitItemFn> {
    item_trait.items.into_iter().filter_map(|item| match item {
        TraitItem::Fn(function) => Some(function),
        _ => None,
    })
}

pub fn cfg() -> TokenStream {
    match cfg!(feature = "test") {
        true => quote! { #[cfg(test)] },
        false => TokenStream::new(),
    }
}

pub fn has_function_with_no_lifetime_reference(item_trait: &ItemTrait) -> bool {
    trait_functions(item_trait).any(function_has_no_lifetime_reference)
}

fn function_has_no_lifetime_reference(function: &TraitItemFn) -> bool {
    matches!(
        &function.sig.output,
        ReturnType::Type(_, return_type)
            if matches!(return_type.as_ref(), Type::Reference(type_ref) if type_ref.lifetime.is_none())
    )
}
