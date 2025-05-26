use crate::attribute;
use syn::{Ident, ItemTrait, TraitItem, TraitItemType, TypePath};

// Vec rather than HashMap so that ordering is preserved.
// Probably more efficent anyway because never very many of them. But this has not been performance tested.
pub type AssociatedSpyTypes = Vec<(Ident, TypePath)>;

pub fn get_associated_types(item_trait: &ItemTrait) -> AssociatedSpyTypes {
    item_trait
        .items
        .iter()
        .filter_map(associated_types)
        .filter_map(associated_type_name_and_spy_type)
        .collect()
}

fn associated_types(item: &TraitItem) -> Option<&TraitItemType> {
    match item {
        TraitItem::Type(trait_type) => Some(trait_type),
        _ => None,
    }
}

fn associated_type_name_and_spy_type(trait_item: &TraitItemType) -> Option<(Ident, TypePath)> {
    Some((
        trait_item.ident.clone(),
        attribute::associated_type(&trait_item.attrs)?,
    ))
}
