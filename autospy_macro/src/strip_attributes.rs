use crate::{edit, inspect};
use syn::{Attribute, ItemTrait, Signature, TraitItem};

pub fn strip_attributes(mut item_trait: ItemTrait) -> ItemTrait {
    item_trait
        .items
        .iter_mut()
        .for_each(strip_attributes_from_item);
    item_trait
}

fn strip_attributes_from_item(item: &mut TraitItem) {
    match item {
        TraitItem::Fn(function) => {
            strip_autospy_attributes(&mut function.attrs);
            strip_attributes_from_signature(&mut function.sig);
        }
        TraitItem::Type(_type) => strip_autospy_attributes(&mut _type.attrs),
        _ => todo!(),
    }
}

fn strip_autospy_attributes(attributes: &mut Vec<Attribute>) {
    attributes.retain(|attribute| !inspect::is_autospy_attribute(attribute));
}

pub fn strip_attributes_from_signature(signature: &mut Signature) {
    for argument in edit::non_self_signature_arguments_mut(signature) {
        strip_autospy_attributes(&mut argument.attrs);
    }
}
