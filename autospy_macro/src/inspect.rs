use crate::attribute;
use syn::{
    Expr, FnArg, Ident, ItemTrait, Pat, PatType, TraitItem, TraitItemConst, TraitItemFn, Type,
};

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

pub fn spyable_arguments(function: &TraitItemFn) -> impl Iterator<Item = SpyableArgument> {
    non_self_function_arguments(function).filter_map(spyable_argument)
}

pub struct SpyableArgument {
    pub name: Ident,
    pub into_type: Option<Type>,
    pub with_expression: Option<Expr>,
    pub dereferenced_type: Type,
    pub dereference_count: u8,
}

pub fn is_argument_marked_as_ignore(argument: &PatType) -> bool {
    argument.attrs.iter().any(attribute::is_ignore_attribute)
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
        into_type: attribute::into_type(&argument.attrs),
        with_expression: attribute::with_expression(&argument.attrs),
        dereferenced_type,
        dereference_count,
    })
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
