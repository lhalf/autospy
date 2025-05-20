use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemTrait, TraitItemFn, Type};

use crate::generate_spy_struct::generate_spy_struct;
use crate::inspect::AssociatedType;
use crate::strip_attributes::{strip_attributes, strip_attributes_from_signature};
use crate::{edit, inspect};

pub fn generate(item: TokenStream) -> TokenStream {
    let item_trait: ItemTrait = syn::parse2(item.clone()).expect("invalid trait definition");
    let associated_type = inspect::associated_type(&item_trait);
    let trait_name = &item_trait.ident;
    let spy_name = format_ident!("{}Spy", trait_name);
    let spy_struct = generate_spy_struct(&item_trait, &associated_type);
    let associated_type_definitions = associated_type_definitions(&associated_type);
    let spy_function_definitions = trait_spy_function_definitions(&item_trait);
    let stripped_item_trait = strip_attributes(item_trait.clone());

    quote! {
        #stripped_item_trait

        #spy_struct

        impl #trait_name for #spy_name {
            #associated_type_definitions
            #(#spy_function_definitions)*
        }
    }
}

fn trait_spy_function_definitions(item_trait: &ItemTrait) -> impl Iterator<Item = TokenStream> {
    inspect::trait_functions(item_trait).map(function_as_spy_function)
}

fn function_as_spy_function(function: &TraitItemFn) -> TokenStream {
    let function_name = &function.sig.ident;
    let spy_arguments =
        tuple_or_single(inspect::spyable_arguments(function).map(argument_to_spy_expression));

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

fn tuple_or_single(mut items: impl Iterator<Item = TokenStream>) -> TokenStream {
    match (items.next(), items.next(), items) {
        (None, _, _) => quote! { () },
        (Some(first), None, _) => quote! { #first },
        (Some(first), Some(second), remainder) => quote! { ( #first , #second #(, #remainder)* ) },
    }
}

fn associated_type_definitions(associated_type: &Option<AssociatedType>) -> TokenStream {
    match associated_type {
        Some(associated_type) => {
            let name = associated_type.name.clone();
            let _type = associated_type._type.clone();
            quote! { type #name = #_type; }
        }
        None => TokenStream::new(),
    }
}

#[cfg(test)]
mod tests {
    use crate::generate::generate;
    use proc_macro2::TokenStream;
    use quote::quote;

    fn generate_pretty(tokens: TokenStream) -> String {
        let expanded = generate(tokens).to_string();
        prettyplease::unparse(&syn::parse_file(&expanded).unwrap())
    }

    #[test]
    fn arguments_marked_with_into_attribute_are_captured() {
        insta::assert_snapshot!(generate_pretty(quote! {
            #[autospy::autospy]
            trait MyTrait {
                fn function(&self, #[autospy(into=IpAddr)] ip: [u8; 4]);
            }
        }));
    }

    #[test]
    fn arguments_marked_with_ignore_attribute_are_not_captured() {
        insta::assert_snapshot!(generate_pretty(quote! {
            trait TestTrait {
                fn function(&self, #[autospy(ignore)] ignored: &str, captured: &str);
            }
        }));
    }

    #[test]
    fn arguments_marked_with_multiple_attributes_retain_non_autospy_attributes() {
        insta::assert_snapshot!(generate_pretty(quote! {
            trait TestTrait {
                fn function(&self, #[some_attribute] #[autospy(ignore)] ignored: &str, captured: &str);
            }
        }));
    }

    #[test]
    fn multiple_ignored_arguments_are_not_captured() {
        insta::assert_snapshot!(generate_pretty(quote! {
            trait TestTrait {
                fn function(&self, #[autospy(ignore)] ignored1: &str, #[autospy(ignore)] ignored2: &str, captured1: &str, captured2: &str);
            }
        }));
    }

    #[test]
    fn method_with_no_arguments() {
        insta::assert_snapshot!(generate_pretty(quote! {
            trait TestTrait {
                fn function(&self);
            }
        }));
    }

    #[test]
    fn multiple_methods_with_no_arguments() {
        insta::assert_snapshot!(generate_pretty(quote! {
            trait TestTrait {
                fn function1(&self);
                fn function2(&self);
            }
        }));
    }

    #[test]
    fn trait_with_non_pub_crate_visibility() {
        insta::assert_snapshot!(generate_pretty(quote! {
            trait TestTrait {
                fn function(&self);
            }
        }));
    }

    #[test]
    fn trait_with_pub_visibility() {
        insta::assert_snapshot!(generate_pretty(quote! {
            pub trait TestTrait {
                fn function(&self);
            }
        }))
    }

    #[test]
    fn trait_with_pub_crate_visibility() {
        insta::assert_snapshot!(generate_pretty(quote! {
            pub(crate) trait TestTrait {
                fn function(&self);
            }
        }))
    }

    #[test]
    fn method_with_non_void_return_type() {
        insta::assert_snapshot!(generate_pretty(quote! {
            trait TestTrait {
                fn function(&self) -> bool;
            }
        }))
    }

    #[test]
    fn single_owned_argument() {
        insta::assert_snapshot!(generate_pretty(quote! {
            trait TestTrait {
                fn function(&self, argument: String);
            }
        }))
    }

    #[test]
    fn borrowed_argument_is_converted_to_owned_type() {
        insta::assert_snapshot!(generate_pretty(quote! {
            trait TestTrait {
                fn function(&self, argument: &str);
            }
        }))
    }

    #[test]
    fn multiple_nested_references_in_argument_converted_to_owned_type() {
        insta::assert_snapshot!(generate_pretty(quote! {
            trait TestTrait {
                fn function(&self, argument: &&&str);
            }
        }))
    }

    #[test]
    fn multiple_owned_arguments() {
        insta::assert_snapshot!(generate_pretty(quote! {
            trait TestTrait {
                fn function(&self, argument1: String, argument2: String);
            }
        }))
    }

    #[test]
    fn multiple_borrowed_arguments_converted_to_owned() {
        insta::assert_snapshot!(generate_pretty(quote! {
            trait TestTrait {
                fn function(&self, argument1: &str, argument2: &str);
            }
        }))
    }

    #[test]
    fn multiple_arguments_that_are_nested_references_converted_to_owned() {
        insta::assert_snapshot!(generate_pretty(quote! {
            trait TestTrait {
                fn function(&self, argument1: &&&&str, argument2: &&&str);
            }
        }))
    }

    #[test]
    fn single_static_impl_argument_converted_to_boxed_dyn() {
        insta::assert_snapshot!(generate_pretty(quote! {
            trait TestTrait {
                fn function(&self, argument: impl ToString + 'static);
            }
        }))
    }

    #[test]
    fn multiple_impl_bounds_static_argument_converted_to_boxed_dyn() {
        insta::assert_snapshot!(generate_pretty(quote! {
            trait TestTrait {
                fn function(&self, argument: impl ToString + Debug + 'static);
            }
        }))
    }

    #[test]
    fn functions_marked_with_return_attribute_have_their_return_types_changed() {
        insta::assert_snapshot!(generate_pretty(quote! {
            trait TestTrait {
                #[autospy(returns = String)]
                fn function(&self) -> impl ToString;
            }
        }))
    }

    #[test]
    fn functions_marked_with_multiple_attributes_retain_non_autospy_attributes() {
        insta::assert_snapshot!(generate_pretty(quote! {
            trait TestTrait {
                #[some_attribute]
                #[autospy(returns = String)]
                fn function(&self) -> impl ToString;
            }
        }))
    }

    #[test]
    fn traits_with_single_associated_type_attribute_return_expected_type() {
        insta::assert_snapshot!(generate_pretty(quote! {
            trait TestTrait {
                #[autospy(String)] type Item;
                fn function(&self) -> Self::Item;
            }
        }))
    }

    #[test]
    fn traits_with_single_associated_type_attribute_capture_expected_type() {
        insta::assert_snapshot!(generate_pretty(quote! {
            trait TestTrait {
                #[autospy(String)] type Item;
                fn function(&self, argument: Self::Item);
            }
        }))
    }
}
