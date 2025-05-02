use quote::{format_ident, quote};
use syn::{FnArg, ItemTrait, Pat, TraitItem, Type};

pub fn generate(item: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let _trait: ItemTrait = syn::parse2(item.clone()).unwrap();
    let trait_name = &_trait.ident;
    let spy_name = format_ident!("{}Spy", trait_name);

    let mut spy_fields = Vec::new();
    let mut trait_impls = Vec::new();

    for item in &_trait.items {
        if let TraitItem::Fn(method) = item {
            let method_name = &method.sig.ident;
            let arguments = &method.sig.inputs;

            if let Some(FnArg::Typed(pat_type)) = arguments.iter().nth(1) {
                let argument_name = if let Pat::Ident(ref pat_ident) = *pat_type.pat {
                    &pat_ident.ident
                } else {
                    continue;
                };

                let argument_type = *pat_type.ty.clone();

                let (dereference_type, count) = remove_reference(&argument_type);

                let dereferences: proc_macro2::TokenStream = "* "
                    .repeat(count.saturating_sub(1) as usize)
                    .parse()
                    .expect("impossible to fail");

                spy_fields.push(quote! {
                    pub #method_name: autospy::SpyFunction<<#dereference_type as ToOwned>::Owned>
                });

                trait_impls.push(quote! {
                    fn #method_name(&self, #argument_name: #argument_type) {
                        self.#method_name.spy((#dereferences #argument_name).to_owned())
                    }
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
            #(#trait_impls)*
        }
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
    fn single_owned_argument_non_public_sync_trait() {
        assert_eq!(
            quote! {
                trait TestTrait {
                    fn function(&self, argument: String);
                }

                #[derive(Default, Clone)]
                struct TestTraitSpy {
                    pub function: autospy::SpyFunction<<String as ToOwned>::Owned>
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
                    pub function: autospy::SpyFunction<<str as ToOwned>::Owned>
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
                    pub function: autospy::SpyFunction<<str as ToOwned>::Owned>
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
}
