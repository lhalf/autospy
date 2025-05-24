use crate::attribute;
use proc_macro2::TokenStream;
use syn::{Ident, ItemTrait, TraitItem, TraitItemType};

#[derive(Clone)]
pub struct AssociatedType {
    pub name: Ident,
    pub r#type: TokenStream,
}

pub fn get_associated_types(item_trait: &ItemTrait) -> Option<AssociatedType> {
    item_trait
        .items
        .iter()
        .find_map(associated_types)
        .and_then(associated_type_from_attribute)
}

fn associated_types(item: &TraitItem) -> Option<&TraitItemType> {
    match item {
        TraitItem::Type(trait_type) => Some(trait_type),
        _ => None,
    }
}

fn associated_type_from_attribute(trait_item: &TraitItemType) -> Option<AssociatedType> {
    let r#type = attribute::associated_type(&trait_item.attrs)?.clone();
    Some(AssociatedType {
        name: trait_item.ident.clone(),
        r#type,
    })
}
