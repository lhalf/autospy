use syn::{FnArg, Ident, Pat, PatType, TraitItemFn, Type};

pub struct SpyableArgument {
    pub name: Ident,
    pub dereferenced_type: Type,
    pub dereference_count: u8,
}

pub fn spyable_arguments(function: &TraitItemFn) -> impl Iterator<Item = SpyableArgument> {
    non_self_function_arguments(function).filter_map(spyable_argument)
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

    let (dereferenced_type, dereference_count) = remove_references(&argument.ty);

    Some(SpyableArgument {
        name,
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
