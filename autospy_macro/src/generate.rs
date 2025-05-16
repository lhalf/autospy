use quote::{ToTokens, format_ident, quote};
use syn::{ItemTrait, ReturnType, TraitItemFn, Type, TypeImplTrait};

use crate::{edit, inspect};
use proc_macro2::TokenStream;

pub fn generate(item: TokenStream) -> TokenStream {
    let item_trait: ItemTrait = syn::parse2(item.clone()).unwrap();
    let visibility = &item_trait.vis;
    let trait_name = &item_trait.ident;
    let spy_name = format_ident!("{}Spy", trait_name);
    let spy_fields = trait_spy_fields(&item_trait);
    let spy_function_definitions = trait_spy_function_definitions(&item_trait);
    let stripped_item_trait = edit::strip_attributes_from_trait(item_trait.clone());

    quote! {
        #stripped_item_trait

        #[derive(Default, Clone)]
        #visibility struct #spy_name {
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
        tuple_or_single(inspect::spyable_arguments(function).map(argument_spy_type));
    let return_type = function_return_type(function);

    quote! {
        pub #function_name: autospy::SpyFunction<#spy_argument_type, #return_type>
    }
}

fn function_return_type(function: &TraitItemFn) -> TokenStream {
    if let Some(specified_return_type) = inspect::get_return_attribute_type(&function.attrs) {
        return specified_return_type;
    }

    match &function.sig.output {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_arrow, return_type) => return_type.to_token_stream(),
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
    edit::strip_attributes_from_signature(&mut signature);

    quote! {
        #signature {
            self.#function_name.spy(#spy_arguments)
        }
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
}
