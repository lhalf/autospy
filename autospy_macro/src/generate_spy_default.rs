use crate::associated_types::{AssociatedSpyTypes, AssociatedType};
use crate::generics::generics_idents;
use crate::inspect::cfg;
use crate::{attribute, inspect, supertraits};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Generics, ItemTrait, TraitItemFn};

pub fn generate_spy_default(
    item_trait: &ItemTrait,
    associated_spy_types: &AssociatedSpyTypes,
) -> TokenStream {
    let cfg = cfg();

    let generics = &item_trait.generics;
    let generics_idents = generic_idents(item_trait, associated_spy_types);
    let generics_where_clause = &generics.where_clause;

    let spy_name = format_ident!("{}Spy", &item_trait.ident);
    let spy_defaults = generate_spy_defaults(item_trait);

    quote! {
        #cfg
        impl #generics Default for #spy_name #generics_idents #generics_where_clause {
            fn default() -> Self {
                Self {
                    #(#spy_defaults),*
                }
            }
        }
    }
}

fn generic_idents(item_trait: &ItemTrait, associated_spy_types: &AssociatedSpyTypes) -> Generics {
    generics_idents(
        &item_trait.generics,
        inspect::has_function_returning_type_containing_elided_lifetime_reference(item_trait)
            || associated_spy_types
                .values()
                .any(AssociatedType::has_lifetime),
    )
}

fn generate_spy_defaults(item_trait: &ItemTrait) -> impl Iterator<Item = TokenStream> {
    inspect::trait_functions(item_trait)
        .cloned()
        .chain(
            supertraits::autospy_supertraits(item_trait).flat_map(inspect::owned_trait_functions),
        )
        .map(|function| function_as_default(&function))
}

fn function_as_default(function: &TraitItemFn) -> TokenStream {
    if attribute::has_use_default_attribute(&function.attrs) {
        return TokenStream::new();
    }

    let function_ident = &function.sig.ident;
    let function_name = function_ident.to_string();
    quote! { #function_ident: autospy::SpyFunction::from(#function_name) }
}

#[cfg(test)]
mod tests {
    use crate::associated_types::{AssociatedSpyTypes, AssociatedType};
    use crate::generate_spy_default::generate_spy_default;
    use quote::quote;
    use syn::{ItemTrait, parse_quote};

    #[test]
    fn no_functions_in_trait() {
        let input: ItemTrait = parse_quote! {
            trait Example {}
        };

        let expected = quote! {
            #[cfg(test)]
            impl Default for ExampleSpy {
                fn default() -> Self {
                    Self {

                    }
                }
            }
        };

        let actual = generate_spy_default(&input, &AssociatedSpyTypes::new());

        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn single_function_in_trait() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                fn foo(&self);
            }
        };

        let expected = quote! {
            #[cfg(test)]
            impl Default for ExampleSpy {
                fn default() -> Self {
                    Self {
                        foo: autospy::SpyFunction::from("foo")
                    }
                }
            }
        };

        let actual = generate_spy_default(&input, &AssociatedSpyTypes::new());

        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn multiple_functions_in_trait() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                fn foo(&self);
                fn bar(&self);
            }
        };

        let expected = quote! {
            #[cfg(test)]
            impl Default for ExampleSpy {
                fn default() -> Self {
                    Self {
                        foo: autospy::SpyFunction::from("foo"),
                        bar: autospy::SpyFunction::from("bar")
                    }
                }
            }
        };

        let actual = generate_spy_default(&input, &AssociatedSpyTypes::new());

        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn use_default_on_function_in_trait() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                #[autospy(use_default)]
                fn foo(&self) -> u8 {
                    1
                }
            }
        };

        let expected = quote! {
            #[cfg(test)]
            impl Default for ExampleSpy {
                fn default() -> Self {
                    Self {}
                }
            }
        };

        let actual = generate_spy_default(&input, &AssociatedSpyTypes::new());

        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn basic_generic_trait() {
        let input: ItemTrait = parse_quote! {
            trait Example<T> {
                fn foo(&self);
            }
        };

        let expected = quote! {
            #[cfg(test)]
            impl<T> Default for ExampleSpy<T> {
                fn default() -> Self {
                    Self {
                        foo: autospy::SpyFunction::from("foo")
                    }
                }
            }
        };

        let actual = generate_spy_default(&input, &AssociatedSpyTypes::new());

        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn multiple_generics_trait() {
        let input: ItemTrait = parse_quote! {
            trait Example<W, O, T> {
                fn foo(&self);
            }
        };

        let expected = quote! {
            #[cfg(test)]
            impl<W, O, T> Default for ExampleSpy<W, O, T> {
                fn default() -> Self {
                    Self {
                        foo: autospy::SpyFunction::from("foo")
                    }
                }
            }
        };

        let actual = generate_spy_default(&input, &AssociatedSpyTypes::new());

        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn generic_trait_with_bounds() {
        let input: ItemTrait = parse_quote! {
            trait Example<T: Copy> {
                fn foo(&self);
            }
        };

        let expected = quote! {
            #[cfg(test)]
            impl<T: Copy> Default for ExampleSpy<T> {
                fn default() -> Self {
                    Self {
                        foo: autospy::SpyFunction::from("foo")
                    }
                }
            }
        };

        let actual = generate_spy_default(&input, &AssociatedSpyTypes::new());

        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn generic_trait_with_where_clause() {
        let input: ItemTrait = parse_quote! {
            trait Example<T> where T: Copy {
                fn foo(&self);
            }
        };

        let expected = quote! {
            #[cfg(test)]
            impl<T> Default for ExampleSpy<T> where T: Copy {
                fn default() -> Self {
                    Self {
                        foo: autospy::SpyFunction::from("foo")
                    }
                }
            }
        };

        let actual = generate_spy_default(&input, &AssociatedSpyTypes::new());

        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn supertrait_macro_in_trait() {
        let input: ItemTrait = parse_quote! {
            trait Example: Supertrait {
                fn foo(&self);
                autospy::supertrait! {
                    trait Supertrait {
                        fn bar(&self) -> bool;
                    }
                }
            }
        };

        let expected = quote! {
            #[cfg(test)]
            impl Default for ExampleSpy {
                fn default() -> Self {
                    Self {
                        foo: autospy::SpyFunction::from("foo"),
                        bar: autospy::SpyFunction::from("bar")
                    }
                }
            }
        };

        let actual = generate_spy_default(&input, &AssociatedSpyTypes::new());

        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn trait_with_lifetime() {
        let input: ItemTrait = parse_quote! {
            trait Example<'a> {
                fn foo(&self) -> &'a str;
            }
        };

        let expected = quote! {
            #[cfg(test)]
            impl<'a> Default for ExampleSpy<'a> {
                fn default() -> Self {
                    Self {
                        foo: autospy::SpyFunction::from("foo")
                    }
                }
            }
        };

        let actual = generate_spy_default(&input, &AssociatedSpyTypes::new());

        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn trait_with_elided_lifetime_reference_return() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                fn foo(&self) -> &str;
            }
        };

        let expected = quote! {
            #[cfg(test)]
            impl Default for ExampleSpy<'_> {
                fn default() -> Self {
                    Self {
                        foo: autospy::SpyFunction::from("foo")
                    }
                }
            }
        };

        let actual = generate_spy_default(&input, &AssociatedSpyTypes::new());

        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn trait_function_returning_type_containing_elided_lifetime_reference() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                fn foo(&self) -> Result<&str, ()>;
            }
        };

        let expected = quote! {
            #[cfg(test)]
            impl Default for ExampleSpy<'_> {
                fn default() -> Self {
                    Self {
                        foo: autospy::SpyFunction::from("foo")
                    }
                }
            }
        };

        let actual = generate_spy_default(&input, &AssociatedSpyTypes::new());

        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn trait_with_lifetime_and_elided_lifetime_reference_return() {
        let input: ItemTrait = parse_quote! {
            trait Example<'a> {
                fn foo(&self) -> &'a str;
                fn bar(&self) -> &str;
            }
        };

        let expected = quote! {
            #[cfg(test)]
            impl<'a> Default for ExampleSpy<'a, '_> {
                fn default() -> Self {
                    Self {
                        foo: autospy::SpyFunction::from("foo"),
                        bar: autospy::SpyFunction::from("bar")
                    }
                }
            }
        };

        let actual = generate_spy_default(&input, &AssociatedSpyTypes::new());

        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn trait_with_elided_lifetime_and_associated_type_with_lifetime_makes_default_impl_have_elided_lifetime()
     {
        let input: ItemTrait = parse_quote! {
            trait Example {
                #[autospy(&'a str)]
                type Example;
            }
        };

        let expected = quote! {
            #[cfg(test)]
            impl Default for ExampleSpy<'_> {
                fn default() -> Self {
                    Self {}
                }
            }
        };

        let mut associated_types = AssociatedSpyTypes::new();

        associated_types.insert(
            parse_quote! { Example },
            AssociatedType {
                r#type: parse_quote! { &'a str },
                generics: parse_quote! {},
            },
        );

        let actual = generate_spy_default(&input, &associated_types);

        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn trait_with_elided_lifetime_reference_return_and_associated_type_with_lifetime_makes_only_one_elided_lifetime()
     {
        let input: ItemTrait = parse_quote! {
            trait Example {
                #[autospy(&'a str)]
                type Example;
                fn foo(&self) -> &str;
            }
        };

        let expected = quote! {
            #[cfg(test)]
            impl Default for ExampleSpy<'_> {
                fn default() -> Self {
                    Self {
                        foo: autospy::SpyFunction::from("foo")
                    }
                }
            }
        };

        let mut associated_types = AssociatedSpyTypes::new();

        associated_types.insert(
            parse_quote! { Example },
            AssociatedType {
                r#type: parse_quote! { &'a str },
                generics: parse_quote! {},
            },
        );

        let actual = generate_spy_default(&input, &associated_types);

        assert_eq!(actual.to_string(), expected.to_string());
    }
}
