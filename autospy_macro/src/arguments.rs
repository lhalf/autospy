use crate::attribute;
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use std::collections::HashMap;
use syn::{Expr, FnArg, GenericParam, Generics, Pat, PatType, TraitItemFn, Type, parse_quote};

#[derive(PartialEq, Debug)]
pub struct SpyArgument {
    pub name: Ident,
    pub into_type: Option<Type>,
    pub with_expression: Option<Expr>,
    pub dereferenced_type: Type,
    pub dereference_count: u8,
}

pub fn spy_arguments(function: &TraitItemFn) -> impl Iterator<Item = SpyArgument> {
    non_self_function_arguments(function)
        .filter_map(|argument| spy_argument(generics_map(&function.sig.generics), argument))
}

pub fn is_argument_marked_as_ignore(argument: &PatType) -> bool {
    argument.attrs.iter().any(attribute::is_ignore_attribute)
}

fn generics_map(generics: &Generics) -> HashMap<Ident, TokenStream> {
    let parameter_generics = generics.params.iter().filter_map(|param| match param {
        GenericParam::Type(type_param) => Some((
            type_param.ident.clone(),
            type_param.bounds.to_token_stream(),
        )),
        _ => None,
    });

    let where_clause_generics = generics
        .where_clause
        .iter()
        .flat_map(|where_clause| where_clause.predicates.iter())
        .filter_map(|predicate| {
            let type_predicate = match predicate {
                syn::WherePredicate::Type(type_predicate) => type_predicate,
                _ => return None,
            };
            let type_path = match &type_predicate.bounded_ty {
                Type::Path(type_path)
                    if type_path.qself.is_none() && type_path.path.segments.len() == 1 =>
                {
                    type_path
                }
                _ => return None,
            };
            let ident = type_path.path.segments[0].ident.clone();
            let bounds = type_predicate.bounds.to_token_stream();
            Some((ident, bounds))
        });

    parameter_generics.chain(where_clause_generics).collect()
}

fn non_self_function_arguments(function: &TraitItemFn) -> impl Iterator<Item = &PatType> {
    function.sig.inputs.iter().filter_map(|input| match input {
        FnArg::Typed(argument) => Some(argument),
        _ => None,
    })
}

fn spy_argument(
    generics_map: HashMap<Ident, TokenStream>,
    argument: &PatType,
) -> Option<SpyArgument> {
    let name = match *argument.pat {
        Pat::Ident(ref pat_ident) => pat_ident.ident.clone(),
        _ => return None,
    };

    if is_argument_marked_as_ignore(argument) {
        return None;
    }

    let (mut dereferenced_type, dereference_count) = remove_references(&argument.ty);

    if let Type::Path(type_path) = &dereferenced_type
        && type_path.qself.is_none()
        && type_path.path.segments.len() == 1
        && let Some(bounds) = generics_map.get(&type_path.path.segments[0].ident)
    {
        dereferenced_type = parse_quote! { impl #bounds }
    }

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

    #[test]
    fn ignore_attribute_argument() {
        let input: TraitItemFn = parse_quote! {
            fn foo(&self, #[autospy(ignore)] bar: bool);
        };

        assert_eq!(0, spy_arguments(&input).count());
    }

    #[test]
    fn reference_argument() {
        let input: TraitItemFn = parse_quote! {
            fn foo(&self, bar: &u32);
        };

        let expected = SpyArgument {
            name: parse_quote! { bar },
            into_type: None,
            with_expression: None,
            dereferenced_type: parse_quote! { u32 },
            dereference_count: 1,
        };

        assert_eq!(vec![expected], spy_arguments(&input).collect::<Vec<_>>());
    }

    #[test]
    fn double_reference_argument() {
        let input: TraitItemFn = parse_quote! {
            fn foo(&self, bar: &&String);
        };

        let expected = SpyArgument {
            name: parse_quote! { bar },
            into_type: None,
            with_expression: None,
            dereferenced_type: parse_quote! { String },
            dereference_count: 2,
        };

        assert_eq!(vec![expected], spy_arguments(&input).collect::<Vec<_>>());
    }

    #[test]
    fn into_attribute_argument() {
        let input: TraitItemFn = parse_quote! {
            fn foo(&self, #[autospy(into = "Ipv4Addr")] ip: [u8; 4]);
        };

        let expected = SpyArgument {
            name: parse_quote! { ip },
            into_type: Some(parse_quote! { Ipv4Addr }),
            with_expression: None,
            dereferenced_type: parse_quote! { [u8; 4] },
            dereference_count: 0,
        };

        assert_eq!(vec![expected], spy_arguments(&input).collect::<Vec<_>>());
    }

    #[test]
    fn impl_argument_function() {
        let input: TraitItemFn = parse_quote! {
            fn foo(&self, argument: impl ToString + 'static);
        };

        let expected = SpyArgument {
            name: parse_quote! { argument },
            into_type: None,
            with_expression: None,
            dereferenced_type: parse_quote! { impl ToString + 'static },
            dereference_count: 0,
        };

        assert_eq!(vec![expected], spy_arguments(&input).collect::<Vec<_>>());
    }

    #[test]
    fn generic_function() {
        let input: TraitItemFn = parse_quote! {
            fn foo<T: ToString + 'static>(&self, argument: T);
        };

        let expected = SpyArgument {
            name: parse_quote! { argument },
            into_type: None,
            with_expression: None,
            dereferenced_type: parse_quote! { impl ToString + 'static },
            dereference_count: 0,
        };

        assert_eq!(vec![expected], spy_arguments(&input).collect::<Vec<_>>());
    }

    #[test]
    fn generic_function_with_where_clause() {
        let input: TraitItemFn = parse_quote! {
            fn foo<T>(&self, argument: T) where T: ToString + 'static;
        };

        let expected = SpyArgument {
            name: parse_quote! { argument },
            into_type: None,
            with_expression: None,
            dereferenced_type: parse_quote! { impl ToString + 'static },
            dereference_count: 0,
        };

        assert_eq!(vec![expected], spy_arguments(&input).collect::<Vec<_>>());
    }
}
