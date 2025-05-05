use quote::{format_ident, quote};
use syn::{FnArg, ItemTrait, Pat, TraitItem, Type};

pub fn generate(item: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let _trait: ItemTrait = syn::parse2(item.clone()).unwrap();
    let trait_name = &_trait.ident;
    let spy_name = format_ident!("{}Spy", trait_name);

    let mut spy_fields = Vec::new();
    let mut spy_trait_impls = Vec::new();

    for item in &_trait.items {
        if let TraitItem::Fn(method) = item {
            let method_name = &method.sig.ident;
            let arguments = &method.sig.inputs;

            let (owned_types, owned_values) = extract_argument_spy_types(arguments);

            if !owned_types.is_empty() {
                let spy_argument_type = tuple_or_single(&owned_types);
                let values_to_spy = tuple_or_single(&owned_values);

                spy_fields.push(quote! {
                    pub #method_name: autospy::SpyFunction<#spy_argument_type>
                });

                spy_trait_impls.push(quote! {
                    fn #method_name(#arguments) {
                        self.#method_name.spy(#values_to_spy)
                    }
                });
            } else {
                spy_trait_impls.push(quote! {
                    fn #method_name(&self) {}
                });
            }
        }
    }

    quote! {
        #item

        #[derive(Default, Clone)]
        struct #spy_name {
            #(#spy_fields),*
        }

        impl #trait_name for #spy_name {
            #(#spy_trait_impls)*
        }
    }
}

fn extract_argument_spy_types(arguments: &syn::punctuated::Punctuated<FnArg, syn::token::Comma>)
    -> (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>)
{
    let mut owned_types = Vec::new();
    let mut owned_values = Vec::new();

    for argument in arguments {
        if let FnArg::Typed(pat_type) = argument {
            let argument_name = match *pat_type.pat {
                Pat::Ident(ref pat_ident) => &pat_ident.ident,
                _ => continue,
            };

            let argument_type = *pat_type.ty.clone();
            let (dereferenced_type, count) = remove_reference(&argument_type);

            let dereferences: proc_macro2::TokenStream = "* "
                .repeat(count.saturating_sub(1) as usize)
                .parse()
                .expect("impossible to fail");

            owned_types.push(quote! { <#dereferenced_type as ToOwned>::Owned });
            owned_values.push(quote! { (#dereferences #argument_name).to_owned() });
        }
    }

    (owned_types, owned_values)
}

fn tuple_or_single(inputs: &[proc_macro2::TokenStream]) -> proc_macro2::TokenStream {
    if inputs.len() == 1 {
        quote! { #(#inputs)* }
    } else {
        quote! { ( #(#inputs),* ) }
    }
}

fn remove_reference(argument_type: &Type) -> (Type, u8) {
    match argument_type {
        Type::Reference(referenced_argument) => {
            let (dereferenced_argument, count) = remove_reference(&referenced_argument.elem);
            (dereferenced_argument, count + 1)
        }
        argument_type => (argument_type.clone(), 0),
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
                struct TestTraitSpy {}

                impl TestTrait for TestTraitSpy {
                    fn function(&self) {}
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
                    pub function: autospy::SpyFunction< <String as ToOwned>::Owned>
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
                    pub function: autospy::SpyFunction< <str as ToOwned>::Owned>
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
                    pub function: autospy::SpyFunction< <str as ToOwned>::Owned>
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
                pub function: autospy::SpyFunction<(<String as ToOwned>::Owned, <String as ToOwned>::Owned)>
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
                pub function: autospy::SpyFunction<(<str as ToOwned>::Owned, <str as ToOwned>::Owned)>
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
                pub function: autospy::SpyFunction<(<str as ToOwned>::Owned, <str as ToOwned>::Owned)>
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
