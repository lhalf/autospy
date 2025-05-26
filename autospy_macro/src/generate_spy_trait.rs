use crate::associated_types::AssociatedSpyTypes;
use crate::strip_attributes::strip_attributes_from_signature;
use crate::{edit, generate, inspect};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemTrait, TraitItemFn, Type};

pub fn generate_spy_trait(
    item_trait: &ItemTrait,
    associated_spy_types: &AssociatedSpyTypes,
) -> TokenStream {
    let trait_name = &item_trait.ident;
    let spy_name = format_ident!("{}Spy", trait_name);
    let associated_type_definitions = associated_type_definitions(associated_spy_types);
    let spy_function_definitions = trait_spy_function_definitions(item_trait);

    quote! {
        impl #trait_name for #spy_name {
            #(#associated_type_definitions)*
            #(#spy_function_definitions)*
        }
    }
}

fn associated_type_definitions(
    associated_spy_types: &AssociatedSpyTypes,
) -> impl Iterator<Item = TokenStream> {
    associated_spy_types
        .iter()
        .map(|(name, r#type)| quote! { type #name = #r#type; })
}

fn trait_spy_function_definitions(item_trait: &ItemTrait) -> impl Iterator<Item = TokenStream> {
    inspect::trait_functions(item_trait).map(function_as_spy_function)
}

fn function_as_spy_function(function: &TraitItemFn) -> TokenStream {
    let function_name = &function.sig.ident;
    let spy_arguments = generate::tuple_or_single(
        inspect::spyable_arguments(function).map(argument_to_spy_expression),
    );

    let mut signature = function.sig.clone();
    edit::underscore_ignored_arguments_in_signature(&mut signature);
    strip_attributes_from_signature(&mut signature);

    quote! {
        #signature {
            self.#function_name.spy(#spy_arguments)
        }
    }
}

fn argument_to_spy_expression(argument: inspect::SpyableArgument) -> TokenStream {
    let argument_name = &argument.name;

    if let Some(with_expression) = argument.with_expression {
        return quote! { #with_expression ( #argument_name ) };
    }

    if argument.into_type.is_some() {
        return quote! { #argument_name.into() };
    }

    if let Type::ImplTrait(_) = argument.dereferenced_type {
        return quote! { Box::new(#argument_name) };
    }

    if argument.dereference_count <= 1 {
        return quote! { #argument_name.to_owned() };
    }

    let dereferences = dereference_tokens(&argument);
    quote! { (#dereferences #argument_name).to_owned() }
}

fn dereference_tokens(argument: &inspect::SpyableArgument) -> TokenStream {
    "*".repeat((argument.dereference_count - 1) as usize)
        .parse()
        .expect("always valid token stream")
}

#[cfg(test)]
mod tests {
    use crate::associated_types::AssociatedSpyTypes;

    use super::generate_spy_trait;
    use quote::{ToTokens, quote};
    use syn::ItemTrait;

    #[test]
    fn empty_generated_trait() {
        let input: ItemTrait = syn::parse2(quote! {
            trait Example {}
        })
        .unwrap();

        let expected = quote! {
            impl Example for ExampleSpy {}
        };

        let actual = generate_spy_trait(&input, &AssociatedSpyTypes::new());

        assert_eq!(actual.to_token_stream().to_string(), expected.to_string());
    }

    #[test]
    fn empty_generated_trait_impl() {
        let input: ItemTrait = syn::parse2(quote! {
            trait Example {}
        })
        .unwrap();

        let expected = quote! {
            impl Example for ExampleSpy {}
        };

        let actual = generate_spy_trait(&input, &AssociatedSpyTypes::new());

        assert_eq!(actual.to_token_stream().to_string(), expected.to_string());
    }

    #[test]
    fn ignored_arguments_are_underscored_and_not_captured_in_trait_impl() {
        let input: ItemTrait = syn::parse2(quote! {
            trait Example {
                fn function(&self, #[autospy(ignore)] ignored: &str, captured: &str);
            }
        })
        .unwrap();

        let expected = quote! {
            impl Example for ExampleSpy {
                fn function(&self, _: &str, captured: &str) {
                    self.function.spy(captured.to_owned())
                }
            }
        };

        let actual = generate_spy_trait(&input, &AssociatedSpyTypes::new());

        assert_eq!(actual.to_token_stream().to_string(), expected.to_string());
    }

    #[test]
    fn functions_with_static_impl_arguments_are_boxed() {
        let input: ItemTrait = syn::parse2(quote! {
            trait Example {
                fn function(&self, argument: impl ToString + 'static);
            }
        })
        .unwrap();

        let expected = quote! {
            impl Example for ExampleSpy {
                fn function(&self, argument: impl ToString + 'static) {
                    self.function.spy(Box::new(argument))
                }
            }
        };

        let actual = generate_spy_trait(&input, &AssociatedSpyTypes::new());

        assert_eq!(actual.to_token_stream().to_string(), expected.to_string());
    }

    #[test]
    fn arguments_with_into_attribute_are_captured() {
        let input: ItemTrait = syn::parse2(quote! {
            trait Example {
                fn function(&self, #[autospy(into="IpAddr")] ip: [u8; 4]);
            }
        })
        .unwrap();

        let expected = quote! {
            impl Example for ExampleSpy {
                fn function(&self, ip: [u8; 4]) {
                    self.function.spy(ip.into())
                }
            }
        };

        let actual = generate_spy_trait(&input, &AssociatedSpyTypes::new());

        assert_eq!(actual.to_token_stream().to_string(), expected.to_string());
    }
}
