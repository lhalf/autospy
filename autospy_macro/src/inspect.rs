use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemTrait, TraitItem, TraitItemConst, TraitItemFn};

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
