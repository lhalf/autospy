use crate::associated_types::AssociatedSpyTypes;
use crate::inspect::cfg;
use crate::{arguments, attribute, edit, generate, inspect};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::visit_mut::VisitMut;
use syn::{ItemStruct, ItemTrait, ReturnType, TraitItemFn, Type, TypeImplTrait, parse_quote};

pub fn generate_spy_struct(
    item_trait: &ItemTrait,
    associated_spy_types: &AssociatedSpyTypes,
) -> ItemStruct {
    let cfg = cfg();

    let visibility = &item_trait.vis;
    let spy_name = format_ident!("{}Spy", item_trait.ident);
    let generics = &item_trait.generics;
    let generics_where_clause = &generics.where_clause;

    let spy_fields = generate_spy_fields(item_trait, associated_spy_types);

    parse_quote! {
        #cfg
        #[derive(Clone)]
        #visibility struct #spy_name #generics #generics_where_clause {
            #(#spy_fields),*
        }
    }
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
    if attribute::has_use_default_attribute(&function.attrs) && function.default.is_some() {
        return TokenStream::new();
    }

    let function_name = &function.sig.ident;

    let function = replace_associated_types(function.clone(), associated_spy_types);

    let spy_argument_type =
        generate::tuple_or_single(arguments::spy_arguments(&function).map(argument_spy_type));

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

fn argument_spy_type(argument: arguments::SpyArgument) -> TokenStream {
    if let Some(into_type) = argument.into_type {
        return quote! { #into_type };
    }

    let dereferenced_type = &argument.dereferenced_type;
    match argument.dereferenced_type {
        Type::ImplTrait(TypeImplTrait { bounds, .. }) => quote! { Box<dyn #bounds> },
        _ if argument.dereference_count == 0 => quote! { #dereferenced_type },
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
    use syn::{ItemStruct, ItemTrait, parse_quote};

    #[test]
    fn empty_generated_struct() {
        let input: ItemTrait = parse_quote! {
            trait Example {}
        };

        let expected: ItemStruct = parse_quote! {
            #[cfg(test)]
            #[derive(Clone)]
            struct ExampleSpy {}
        };

        assert_eq!(
            expected,
            generate_spy_struct(&input, &AssociatedSpyTypes::new())
        );
    }

    #[test]
    fn generated_spy_struct_retains_trait_visibility() {
        let input: ItemTrait = parse_quote! {
            pub trait Example {
                fn foo(&self);
            }
        };

        let expected: ItemStruct = parse_quote! {
            #[cfg(test)]
            #[derive(Clone)]
            pub struct ExampleSpy {
                pub foo: autospy::SpyFunction<(), ()>
            }
        };

        assert_eq!(
            expected,
            generate_spy_struct(&input, &AssociatedSpyTypes::new())
        );
    }

    #[test]
    fn generated_spy_struct_handles_owned_return_values() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                fn foo(&self) -> String;
            }
        };

        let expected: ItemStruct = parse_quote! {
            #[cfg(test)]
            #[derive(Clone)]
            struct ExampleSpy {
                pub foo: autospy::SpyFunction<(), String>
            }
        };

        assert_eq!(
            expected,
            generate_spy_struct(&input, &AssociatedSpyTypes::new())
        );
    }

    #[test]
    fn generated_spy_struct_captures_reference_arguments_as_owned() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                fn foo(&self, argument: &str);
            }
        };

        let expected: ItemStruct = parse_quote! {
            #[cfg(test)]
            #[derive(Clone)]
            struct ExampleSpy {
                pub foo: autospy::SpyFunction< < str as ToOwned > :: Owned , () >
            }
        };

        assert_eq!(
            expected,
            generate_spy_struct(&input, &AssociatedSpyTypes::new())
        );
    }

    #[test]
    fn generated_spy_struct_captures_associated_type_arguments() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                #[autospy(String)]
                type Item;
                fn foo(&self, argument: Self::Item);
            }
        };

        let expected: ItemStruct = parse_quote! {
            #[cfg(test)]
            #[derive(Clone)]
            struct ExampleSpy {
                pub foo: autospy::SpyFunction< String , () >
            }
        };

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
        let input: ItemTrait = parse_quote! {
            trait Example {
                #[autospy(String)]
                type Item;
                fn foo(&self) -> Self::Item;
            }
        };

        let expected: ItemStruct = parse_quote! {
            #[cfg(test)]
            #[derive(Clone)]
            struct ExampleSpy {
                pub foo: autospy::SpyFunction< (), String >
            }
        };

        assert_eq!(
            expected,
            generate_spy_struct(
                &input,
                &associated_spy_types(quote! { Item }, quote! { String })
            )
        );
    }

    #[test]
    fn no_spy_function_created_if_function_marked_with_use_default() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                #[autospy(use_default)]
                fn foo(&self) -> u8 {
                    1
                }
            }
        };

        let expected: ItemStruct = parse_quote! {
            #[cfg(test)]
            #[derive(Clone)]
            struct ExampleSpy {}
        };

        assert_eq!(
            expected,
            generate_spy_struct(&input, &AssociatedSpyTypes::new())
        );
    }

    #[test]
    fn generated_spy_struct_is_generic_over_trait_generics() {
        let input: ItemTrait = parse_quote! {
            pub trait Example<T> {
                fn foo(&self);
            }
        };

        let expected: ItemStruct = parse_quote! {
            #[cfg(test)]
            #[derive(Clone)]
            pub struct ExampleSpy<T> {
                pub foo: autospy::SpyFunction<(), ()>
            }
        };

        assert_eq!(
            expected,
            generate_spy_struct(&input, &AssociatedSpyTypes::new())
        );
    }

    #[test]
    fn generated_spy_struct_is_generic_over_multiple_trait_generics() {
        let input: ItemTrait = parse_quote! {
            pub trait Example<T, R> {
                fn foo(&self);
            }
        };

        let expected: ItemStruct = parse_quote! {
            #[cfg(test)]
            #[derive(Clone)]
            pub struct ExampleSpy<T, R> {
                pub foo: autospy::SpyFunction<(), ()>
            }
        };

        assert_eq!(
            expected,
            generate_spy_struct(&input, &AssociatedSpyTypes::new())
        );
    }

    #[test]
    fn generated_spy_struct_is_generic_over_trait_generics_with_where_clause() {
        let input: ItemTrait = parse_quote! {
            pub trait Example<T, R> where T: Copy {
                fn foo(&self);
            }
        };

        let expected: ItemStruct = parse_quote! {
            #[cfg(test)]
            #[derive(Clone)]
            pub struct ExampleSpy<T, R> where T: Copy {
                pub foo: autospy::SpyFunction<(), ()>
            }
        };

        assert_eq!(
            expected,
            generate_spy_struct(&input, &AssociatedSpyTypes::new())
        );
    }

    #[test]
    fn generated_spy_struct_handles_trait_function_marked_as_supertrait() {
        let input: ItemTrait = parse_quote! {
            trait Example: Supertrait {
                fn foo(&self) -> String;
                #[cfg(test)]
                #[autospy(supertrait = "Supertrait")]
                fn bar(&self) -> bool;
            }
        };

        let expected: ItemStruct = parse_quote! {
            #[cfg(test)]
            #[derive(Clone)]
            struct ExampleSpy {
                pub foo: autospy::SpyFunction<(), String>,
                pub bar: autospy::SpyFunction<(), bool>
            }
        };

        assert_eq!(
            expected,
            generate_spy_struct(&input, &AssociatedSpyTypes::new())
        );
    }

    fn associated_spy_types(ident: TokenStream, r#type: TokenStream) -> AssociatedSpyTypes {
        [(parse_quote! { #ident }, parse_quote! { #r#type })]
            .into_iter()
            .collect()
    }
}
