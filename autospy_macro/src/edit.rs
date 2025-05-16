use syn::{Attribute, FnArg, ItemTrait, PatType, Signature, TraitItem, TraitItemFn, parse_quote};

use crate::inspect;

pub fn strip_attributes_from_trait(mut item_trait: ItemTrait) -> ItemTrait {
    trait_functions_mut(&mut item_trait).for_each(strip_attributes_from_function);
    item_trait
}

pub fn underscore_ignored_arguments_in_signature(signature: &mut Signature) {
    non_self_signature_arguments_mut(signature)
        .filter(|argument| inspect::is_argument_marked_as_ignore(argument))
        .for_each(rename_argument_to_underscore);
}

pub fn strip_attributes_from_signature(signature: &mut Signature) {
    for argument in non_self_signature_arguments_mut(signature) {
        strip_autospy_attributes(&mut argument.attrs);
    }
}

fn strip_attributes_from_function(function: &mut TraitItemFn) {
    strip_autospy_attributes(&mut function.attrs);
    strip_attributes_from_signature(&mut function.sig);
}

fn strip_autospy_attributes(attributes: &mut Vec<Attribute>) {
    attributes.retain(|attribute| !inspect::is_autospy_attribute(attribute));
}

fn rename_argument_to_underscore(argument: &mut PatType) {
    argument.pat = parse_quote! { _ };
}

fn trait_functions_mut(item_trait: &mut ItemTrait) -> impl Iterator<Item = &mut TraitItemFn> {
    item_trait.items.iter_mut().filter_map(|item| match item {
        TraitItem::Fn(function) => Some(function),
        _ => None,
    })
}

fn non_self_signature_arguments_mut(
    signature: &mut Signature,
) -> impl Iterator<Item = &mut PatType> {
    signature.inputs.iter_mut().filter_map(|input| match input {
        FnArg::Typed(argument) => Some(argument),
        _ => None,
    })
}
