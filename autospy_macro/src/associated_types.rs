use crate::inspect;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{ItemTrait, TraitItem, TraitItemType};

#[derive(Clone)]
pub struct AssociatedType {
    pub name: TokenStream,
    pub _type: TokenStream,
}

pub fn get_associated_types(item_trait: &ItemTrait) -> Option<AssociatedType> {
    item_trait
        .items
        .iter()
        .find_map(associated_types)
        .and_then(associated_type_attribute)
}

fn associated_types(item: &TraitItem) -> Option<TraitItemType> {
    match item {
        TraitItem::Type(trait_type) => Some(trait_type.clone()),
        _ => None,
    }
}

fn associated_type_attribute(trait_item: TraitItemType) -> Option<AssociatedType> {
    match trait_item.attrs.iter().find_map(inspect::autospy_attribute) {
        Some(associated_type) => Some(AssociatedType {
            name: trait_item.ident.to_token_stream(),
            _type: associated_type.clone(),
        }),
        None => None,
    }
}
