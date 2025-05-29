use crate::associated_types::AssociatedSpyTypes;
use crate::{attribute, edit, generate, inspect};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::visit_mut::VisitMut;
use syn::{ItemStruct, ItemTrait, ReturnType, TraitItemFn, Type, TypeImplTrait};

pub fn generate_spy_struct(
    item_trait: &ItemTrait,
    associated_spy_types: &AssociatedSpyTypes,
) -> ItemStruct {
    let visibility = &item_trait.vis;
    let spy_name = format_ident!("{}Spy", item_trait.ident);
    let spy_fields = generate_spy_fields(item_trait, associated_spy_types);

    syn::parse2(quote! {
        #[cfg(any(test, not(feature = "test")))]
        #[derive(Default, Clone)]
        #visibility struct #spy_name {
            #(#spy_fields),*
        }
    })
    .expect("invalid generated spy struct")
}

fn generate_spy_fields(
    item_trait: &ItemTrait,
    associated_spy_types: &AssociatedSpyTypes,
) -> impl Iterator<Item = TokenStream> {
    inspect::trait_functions(item_trait)
        .map(|function| function_as_spy_field(function, associated_spy_types))
}

fn function_as_spy_field(
    function: &TraitItemFn,
    associated_spy_types: &AssociatedSpyTypes,
) -> TokenStream {
    let function_name = &function.sig.ident;

    let function = replace_associated_types(function.clone(), associated_spy_types);

    let spy_argument_type =
        generate::tuple_or_single(inspect::spyable_arguments(&function).map(argument_spy_type));

    let return_type = function_return_type(&function);

    quote! {
        pub #function_name: autospy::SpyFunction<#spy_argument_type, #return_type>
    }
}

fn replace_associated_types(
    mut function: TraitItemFn,
    associated_spy_types: &AssociatedSpyTypes,
) -> TraitItemFn {
    edit::AssociatedTypeReplacer {
        associated_spy_types,
    }
    .visit_trait_item_fn_mut(&mut function);
    function
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

fn function_return_type(function: &TraitItemFn) -> TokenStream {
    // specifying the return attribute takes precedence over associated type
    if let Some(specified_return_type) = attribute::return_type(&function.attrs) {
        return specified_return_type.to_token_stream();
    }

    match &function.sig.output {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_arrow, return_type) => return_type.to_token_stream(),
    }
}

#[cfg(test)]
mod tests {
    use crate::associated_types::AssociatedSpyTypes;
    use crate::generate_spy_struct::generate_spy_struct;
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::{ItemStruct, ItemTrait};

    #[test]
    fn empty_generated_struct() {
        let input: ItemTrait = syn::parse2(quote! {
            trait Example {}
        })
        .unwrap();

        let expected: ItemStruct = syn::parse2(quote! {
            #[cfg(any(test, not(feature = "test")))]
            #[derive(Default, Clone)]
            struct ExampleSpy {}
        })
        .unwrap();

        assert_eq!(
            expected,
            generate_spy_struct(&input, &AssociatedSpyTypes::new())
        );
    }

    #[test]
    fn generated_spy_struct_retains_trait_visibility() {
        let input: ItemTrait = syn::parse2(quote! {
            pub trait Example {
                fn foo(&self);
            }
        })
        .unwrap();

        let expected: ItemStruct = syn::parse2(quote! {
            #[cfg(any(test, not(feature = "test")))]
            #[derive(Default, Clone)]
            pub struct ExampleSpy {
                pub foo: autospy::SpyFunction<(), ()>
            }
        })
        .unwrap();

        assert_eq!(
            expected,
            generate_spy_struct(&input, &AssociatedSpyTypes::new())
        );
    }

    #[test]
    fn generated_spy_struct_handles_owned_return_values() {
        let input: ItemTrait = syn::parse2(quote! {
            trait Example {
                fn foo(&self) -> String;
            }
        })
        .unwrap();

        let expected: ItemStruct = syn::parse2(quote! {
            #[cfg(any(test, not(feature = "test")))]
            #[derive(Default, Clone)]
            struct ExampleSpy {
                pub foo: autospy::SpyFunction<(), String>
            }
        })
        .unwrap();

        assert_eq!(
            expected,
            generate_spy_struct(&input, &AssociatedSpyTypes::new())
        );
    }

    #[test]
    fn generated_spy_struct_captures_reference_arguments_as_owned() {
        let input: ItemTrait = syn::parse2(quote! {
            trait Example {
                fn foo(&self, argument: &str);
            }
        })
        .unwrap();

        let expected: ItemStruct = syn::parse2(quote! {
            #[cfg(any(test, not(feature = "test")))]
            #[derive(Default, Clone)]
            struct ExampleSpy {
                pub foo: autospy::SpyFunction< < str as ToOwned > :: Owned , () >
            }
        })
        .unwrap();

        assert_eq!(
            expected,
            generate_spy_struct(&input, &AssociatedSpyTypes::new())
        );
    }

    #[test]
    fn generated_spy_struct_captures_associated_type_arguments() {
        let input: ItemTrait = syn::parse2(quote! {
            trait Example {
                #[autospy(String)]
                type Item;
                fn foo(&self, argument: Self::Item);
            }
        })
        .unwrap();

        let expected: ItemStruct = syn::parse2(quote! {
            #[cfg(any(test, not(feature = "test")))]
            #[derive(Default, Clone)]
            struct ExampleSpy {
                pub foo: autospy::SpyFunction< < String as ToOwned > :: Owned , () >
            }
        })
        .unwrap();

        assert_eq!(
            expected,
            generate_spy_struct(
                &input,
                &associated_spy_types(quote! { Item }, quote! { String })
            )
        );
    }

    #[test]
    fn generated_spy_struct_handles_associated_type_returns() {
        let input: ItemTrait = syn::parse2(quote! {
            trait Example {
                #[autospy(String)]
                type Item;
                fn foo(&self) -> Self::Item;
            }
        })
        .unwrap();

        let expected: ItemStruct = syn::parse2(quote! {
            #[cfg(any(test, not(feature = "test")))]
            #[derive(Default, Clone)]
            struct ExampleSpy {
                pub foo: autospy::SpyFunction< (), String >
            }
        })
        .unwrap();

        assert_eq!(
            expected,
            generate_spy_struct(
                &input,
                &associated_spy_types(quote! { Item }, quote! { String })
            )
        );
    }

    fn associated_spy_types(ident: TokenStream, r#type: TokenStream) -> AssociatedSpyTypes {
        [(syn::parse2(ident).unwrap(), syn::parse2(r#type).unwrap())]
            .into_iter()
            .collect()
    }
}
