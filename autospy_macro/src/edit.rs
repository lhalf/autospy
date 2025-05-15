use syn::{FnArg, ItemTrait, PatType, Signature, TraitItem, TraitItemFn, parse_quote};

use crate::inspect;

pub fn strip_no_spy_from_trait(mut item_trait: ItemTrait) -> ItemTrait {
    trait_functions_mut(&mut item_trait).for_each(strip_no_spy_from_function);
    item_trait
}

pub fn ignore_no_spy_arguments_in_signature(signature: &mut Signature) {
    for argument in non_self_function_arguments_mut(signature)
        .filter(|argument| inspect::is_argument_marked_as_no_spy(argument))
    {
        strip_no_spy_from_argument(argument);
        rename_argument_to_ignored(argument);
    }
}

fn strip_no_spy_from_function(function: &mut TraitItemFn) {
    non_self_function_arguments_mut(&mut function.sig).for_each(strip_no_spy_from_argument);
}

fn strip_no_spy_from_argument(argument: &mut PatType) {
    argument
        .attrs
        .retain(|attribute| !super::inspect::is_no_spy_attribute(attribute));
}

fn rename_argument_to_ignored(argument: &mut PatType) {
    argument.pat = parse_quote! { _ };
}

fn trait_functions_mut(item_trait: &mut ItemTrait) -> impl Iterator<Item = &mut TraitItemFn> {
    item_trait.items.iter_mut().filter_map(|item| match item {
        TraitItem::Fn(function) => Some(function),
        _ => None,
    })
}

fn non_self_function_arguments_mut(
    signature: &mut Signature,
) -> impl Iterator<Item = &mut PatType> {
    signature.inputs.iter_mut().filter_map(|input| match input {
        FnArg::Typed(argument) => Some(argument),
        _ => None,
    })
}
