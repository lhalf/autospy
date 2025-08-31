use crate::attribute;
use proc_macro2::Ident;
use syn::{Expr, FnArg, Pat, PatType, TraitItemFn, Type};

#[derive(PartialEq, Debug)]
pub struct SpyArgument {
    pub name: Ident,
    pub into_type: Option<Type>,
    pub with_expression: Option<Expr>,
    pub dereferenced_type: Type,
    pub dereference_count: u8,
}

pub fn spy_arguments(function: &TraitItemFn) -> impl Iterator<Item = SpyArgument> {
    non_self_function_arguments(function).filter_map(spy_argument)
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

fn spy_argument(argument: &PatType) -> Option<SpyArgument> {
    let name = match *argument.pat {
        Pat::Ident(ref pat_ident) => pat_ident.ident.clone(),
        _ => return None,
    };

    if is_argument_marked_as_ignore(argument) {
        return None;
    }

    let (dereferenced_type, dereference_count) = remove_references(&argument.ty);

    Some(SpyArgument {
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

#[cfg(test)]
mod tests {
    use super::{SpyArgument, spy_arguments};
    use syn::{TraitItemFn, parse_quote};

    #[test]
    fn no_arguments() {
        let input: TraitItemFn = parse_quote! {
            fn foo(&self);
        };

        assert_eq!(0, spy_arguments(&input).count());
    }

    #[test]
    fn single_argument() {
        let input: TraitItemFn = parse_quote! {
            fn foo(&self, bar: bool);
        };

        let expected = SpyArgument {
            name: parse_quote! { bar },
            into_type: None,
            with_expression: None,
            dereferenced_type: parse_quote! { bool },
            dereference_count: 0,
        };

        assert_eq!(vec![expected], spy_arguments(&input).collect::<Vec<_>>());
    }
}
