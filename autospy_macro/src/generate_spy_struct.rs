use crate::inspect::AssociatedType;
use crate::{edit, inspect};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::visit_mut::VisitMut;
use syn::{ItemTrait, ReturnType, TraitItemFn, Type, TypeImplTrait};

pub fn generate_spy_struct(
    item_trait: &ItemTrait,
    associated_type: &Option<AssociatedType>,
) -> TokenStream {
    let visibility = &item_trait.vis;
    let spy_name = format_ident!("{}Spy", item_trait.ident);
    let spy_fields = generate_spy_fields(item_trait, associated_type);

    quote! {
        #[derive(Default, Clone)]
        #visibility struct #spy_name {
            #(#spy_fields),*
        }
    }
}

fn generate_spy_fields(
    item_trait: &ItemTrait,
    associated_type: &Option<AssociatedType>,
) -> impl Iterator<Item = TokenStream> {
    inspect::trait_functions(item_trait)
        .map(|function| function_as_spy_field(function, associated_type))
}

fn function_as_spy_field(
    function: &TraitItemFn,
    associated_type: &Option<AssociatedType>,
) -> TokenStream {
    let function_name = &function.sig.ident;

    let function = replace_associated_types(function.clone(), associated_type);

    let spy_argument_type =
        tuple_or_single(inspect::spyable_arguments(&function).map(argument_spy_type));

    let return_type = function_return_type(&function);

    quote! {
        pub #function_name: autospy::SpyFunction<#spy_argument_type, #return_type>
    }
}

fn replace_associated_types(
    function: TraitItemFn,
    associated_type: &Option<AssociatedType>,
) -> TraitItemFn {
    if let Some(associated_type) = associated_type {
        let mut modified_function = function.clone();
        let mut replacer = edit::AssociatedTypeReplacer {
            associated_type: associated_type.clone(),
        };
        replacer.visit_trait_item_fn_mut(&mut modified_function);
        modified_function
    } else {
        function
    }
}

fn argument_spy_type(argument: inspect::SpyableArgument) -> TokenStream {
    if let Some(into_type) = argument.into_type {
        return quote! { #into_type };
    }

    let dereferenced_type = &argument.dereferenced_type;
    match argument.dereferenced_type {
        Type::ImplTrait(TypeImplTrait { bounds, .. }) => quote! { Box<dyn #bounds> },
        _ => quote! { <#dereferenced_type as ToOwned>::Owned },
    }
}

fn tuple_or_single(mut items: impl Iterator<Item = TokenStream>) -> TokenStream {
    match (items.next(), items.next(), items) {
        (None, _, _) => quote! { () },
        (Some(first), None, _) => quote! { #first },
        (Some(first), Some(second), remainder) => quote! { ( #first , #second #(, #remainder)* ) },
    }
}

fn function_return_type(function: &TraitItemFn) -> TokenStream {
    // specifying the return attribute takes precedence over associated type
    if let Some(specified_return_type) = inspect::get_return_attribute_type(&function.attrs) {
        return specified_return_type;
    }

    match &function.sig.output {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_arrow, return_type) => return_type.to_token_stream(),
    }
}

#[cfg(test)]
mod tests {
    use crate::generate_spy_struct::generate_spy_struct;
    use quote::{ToTokens, quote};
    use syn::ItemTrait;

    #[test]
    fn empty_generated_struct() {
        let input: ItemTrait = syn::parse2(quote! {
            trait Example {}
        })
        .unwrap();

        let expected = quote! {
            #[derive(Default, Clone)]
            struct ExampleSpy {}
        };

        let actual = generate_spy_struct(&input, &None);

        assert_eq!(actual.to_token_stream().to_string(), expected.to_string());
    }

    #[test]
    fn generated_spy_struct_retains_trait_visibility() {
        let input: ItemTrait = syn::parse2(quote! {
            pub trait Example {
                fn foo(&self);
            }
        })
        .unwrap();

        let expected = quote! {
            #[derive(Default, Clone)]
            pub struct ExampleSpy {
                pub foo: autospy::SpyFunction<(), ()>
            }
        };

        let actual = generate_spy_struct(&input, &None);

        assert_eq!(actual.to_token_stream().to_string(), expected.to_string());
    }

    #[test]
    fn generated_spy_struct_captures_owned_return_values() {
        let input: ItemTrait = syn::parse2(quote! {
            trait Example {
                fn foo(&self) -> String;
            }
        })
        .unwrap();

        let expected = quote! {
            #[derive(Default, Clone)]
            struct ExampleSpy {
                pub foo: autospy::SpyFunction<(), String>
            }
        };

        let actual = generate_spy_struct(&input, &None);

        assert_eq!(actual.to_token_stream().to_string(), expected.to_string());
    }

    #[test]
    fn generated_spy_struct_captures_reference_arguments_as_owned() {
        let input: ItemTrait = syn::parse2(quote! {
            trait Example {
                fn foo(&self, argument: &str);
            }
        })
        .unwrap();

        let expected = quote! {
            #[derive(Default, Clone)]
            struct ExampleSpy {
                pub foo: autospy::SpyFunction< < str as ToOwned > :: Owned , () >
            }
        };

        let actual = generate_spy_struct(&input, &None);

        assert_eq!(actual.to_token_stream().to_string(), expected.to_string());
    }
}
