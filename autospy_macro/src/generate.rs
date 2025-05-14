use quote::{format_ident, quote};
use syn::{ItemTrait, TraitItemFn};

use crate::inspect;
use proc_macro2::TokenStream;

pub fn generate(item: TokenStream) -> TokenStream {
    let item_trait: ItemTrait = syn::parse2(item.clone()).unwrap();
    let trait_name = &item_trait.ident;
    let spy_name = format_ident!("{}Spy", trait_name);
    let spy_fields = trait_spy_fields(&item_trait);
    let spy_function_definitions = trait_spy_function_definitions(&item_trait);

    quote! {
        #item

        #[derive(Default, Clone)]
        struct #spy_name {
            #(#spy_fields),*
        }

        impl #trait_name for #spy_name {
            #(#spy_function_definitions)*
        }
    }
}

fn trait_spy_fields(item_trait: &ItemTrait) -> impl Iterator<Item = TokenStream> {
    inspect::trait_functions(item_trait).map(function_as_spy_field)
}

fn function_as_spy_field(function: &TraitItemFn) -> TokenStream {
    let function_name = &function.sig.ident;
    let spy_argument_type =
        tuple_or_single(inspect::spyable_arguments(function).map(argument_owned_type));
    quote! {
        pub #function_name: autospy::SpyFunction<#spy_argument_type, ()>
    }
}

fn trait_spy_function_definitions(item_trait: &ItemTrait) -> impl Iterator<Item = TokenStream> {
    inspect::trait_functions(item_trait).map(function_as_spy_function)
}

fn function_as_spy_function(function: &TraitItemFn) -> TokenStream {
    let function_signature = &function.sig;
    let function_name = &function.sig.ident;
    let spy_arguments =
        tuple_or_single(inspect::spyable_arguments(function).map(argument_to_owned_expression));
    quote! {
        #function_signature {
            self.#function_name.spy(#spy_arguments)
        }
    }
}

fn argument_owned_type(argument: inspect::SpyableArgument) -> proc_macro2::TokenStream {
    let dereferenced_type = &argument.dereferenced_type;
    quote! { <#dereferenced_type as ToOwned>::Owned }
}

fn argument_to_owned_expression(argument: inspect::SpyableArgument) -> proc_macro2::TokenStream {
    let argument_name = &argument.name;
    let dereferences: proc_macro2::TokenStream = "* "
        .repeat(argument.dereference_count.saturating_sub(1) as usize)
        .parse()
        .expect("always valid token stream");
    quote! { (#dereferences #argument_name).to_owned() }
}

fn tuple_or_single(mut items: impl Iterator<Item = TokenStream>) -> proc_macro2::TokenStream {
    match (items.next(), items.next(), items) {
        (None, _, _) => quote! { () },
        (Some(first), None, _) => quote! { #first },
        (Some(first), Some(second), remainder) => quote! { ( #first , #second #(, #remainder)* ) },
    }
}

#[cfg(test)]
mod tests {
    use crate::generate::generate;
    use quote::quote;

    #[test]
    fn no_arguments_non_public_sync_trait() {
        assert_eq!(
            quote! {
                trait TestTrait {
                    fn function(&self);
                }

                #[derive(Default, Clone)]
                struct TestTraitSpy {
                    pub function: autospy::SpyFunction< (), ()>
                }

                impl TestTrait for TestTraitSpy {
                    fn function(&self) {
                        self.function.spy(())
                    }
                }
            }
            .to_string(),
            generate(quote! {
                trait TestTrait {
                    fn function(&self);
                }
            })
            .to_string()
        )
    }

    #[test]
    fn single_owned_argument_non_public_sync_trait() {
        assert_eq!(
            quote! {
                trait TestTrait {
                    fn function(&self, argument: String);
                }

                #[derive(Default, Clone)]
                struct TestTraitSpy {
                    pub function: autospy::SpyFunction< <String as ToOwned>::Owned, ()>
                }

                impl TestTrait for TestTraitSpy {
                    fn function(&self, argument: String) {
                        self.function.spy((argument).to_owned())
                    }
                }
            }
            .to_string(),
            generate(quote! {
                trait TestTrait {
                    fn function(&self, argument: String);
                }
            })
            .to_string()
        )
    }

    #[test]
    fn single_borrowed_argument_non_public_sync_trait() {
        assert_eq!(
            quote! {
                trait TestTrait {
                    fn function(&self, argument: &str);
                }

                #[derive(Default, Clone)]
                struct TestTraitSpy {
                    pub function: autospy::SpyFunction< <str as ToOwned>::Owned, ()>
                }

                impl TestTrait for TestTraitSpy {
                    fn function(&self, argument: &str) {
                        self.function.spy((argument).to_owned())
                    }
                }
            }
            .to_string(),
            generate(quote! {
                trait TestTrait {
                    fn function(&self, argument: &str);
                }
            })
            .to_string()
        )
    }

    #[test]
    fn single_multiple_referenced_argument_non_public_sync_trait() {
        assert_eq!(
            quote! {
                trait TestTrait {
                    fn function(&self, argument: &&&str);
                }

                #[derive(Default, Clone)]
                struct TestTraitSpy {
                    pub function: autospy::SpyFunction< <str as ToOwned>::Owned, ()>
                }

                impl TestTrait for TestTraitSpy {
                    fn function(&self, argument: & & &str) {
                        self.function.spy((** argument).to_owned())
                    }
                }
            }
            .to_string(),
            generate(quote! {
                trait TestTrait {
                    fn function(&self, argument: &&&str);
                }
            })
            .to_string()
        )
    }

    #[test]
    fn multiple_owned_arguments_non_public_sync_trait() {
        assert_eq!(quote!{
            trait TestTrait {
                fn function(&self, argument1: String, argument2: String);
            }

            #[derive(Default, Clone)]
            struct TestTraitSpy {
                pub function: autospy::SpyFunction<(<String as ToOwned>::Owned, <String as ToOwned>::Owned), ()>
            }

            impl TestTrait for TestTraitSpy {
                fn function(&self, argument1: String, argument2: String) {
                    self.function.spy(((argument1).to_owned(), (argument2).to_owned()))
                }
            }
         }.to_string(), generate(quote!{
             trait TestTrait {
                 fn function(&self, argument1: String, argument2: String);
             }
         }).to_string())
    }

    #[test]
    fn multiple_borrowed_arguments_non_public_sync_trait() {
        assert_eq!(quote!{
            trait TestTrait {
                fn function(&self, argument1: &str, argument2: &str);
            }

            #[derive(Default, Clone)]
            struct TestTraitSpy {
                pub function: autospy::SpyFunction<(<str as ToOwned>::Owned, <str as ToOwned>::Owned), ()>
            }

            impl TestTrait for TestTraitSpy {
                fn function(&self, argument1: &str, argument2: &str) {
                    self.function.spy(((argument1).to_owned(), (argument2).to_owned()))
                }
            }
         }.to_string(), generate(quote!{
             trait TestTrait {
                 fn function(&self, argument1: &str, argument2: &str);
             }
         }).to_string())
    }

    #[test]
    fn multiple_multiple_reference_arguments_non_public_sync_trait() {
        assert_eq!(quote!{
            trait TestTrait {
                fn function(&self, argument1: &&&&str, argument2: &&&str);
            }

            #[derive(Default, Clone)]
            struct TestTraitSpy {
                pub function: autospy::SpyFunction<(<str as ToOwned>::Owned, <str as ToOwned>::Owned), ()>
            }

            impl TestTrait for TestTraitSpy {
                fn function(&self, argument1: & & & &str, argument2: & & &str) {
                    self.function.spy(((*** argument1).to_owned(), (** argument2).to_owned()))
                }
            }
         }.to_string(), generate(quote!{
             trait TestTrait {
                 fn function(&self, argument1: &&&&str, argument2: &&&str);
             }
         }).to_string())
    }
}
