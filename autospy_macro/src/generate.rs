use crate::associated_types::get_associated_types;
use crate::generate_spy_struct::generate_spy_struct;
use crate::generate_spy_trait::generate_spy_trait;
use crate::strip_attributes::strip_attributes;
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemTrait;

pub fn generate(item: TokenStream) -> TokenStream {
    let item_trait: ItemTrait = syn::parse2(item).expect("invalid trait definition");
    let associated_type = get_associated_types(&item_trait);
    let stripped_item_trait = strip_attributes(item_trait.clone());
    let spy_struct = generate_spy_struct(&item_trait, &associated_type);
    let spy_trait = generate_spy_trait(&item_trait, &associated_type);

    quote! {
        #stripped_item_trait
        #spy_struct
        #spy_trait
    }
}

pub fn tuple_or_single(mut items: impl Iterator<Item = TokenStream>) -> TokenStream {
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
    fn arguments_marked_with_into_attribute_are_captured_as_that_type() {
        insta::assert_snapshot!(generate_pretty(quote! {
            trait MyTrait {
                fn function(&self, #[autospy(into="IpAddr")] ip: [u8; 4]);
            }
        }));
    }

    #[test]
    fn arguments_marked_with_with_attribute_are_captured_with_that_expression() {
        insta::assert_snapshot!(generate_pretty(quote! {
            trait MyTrait {
                fn function(
                    &self,
                    #[autospy(into="Result<String,Utf8Error>", with="String::from_utf8")] bytes: Vec<u8>,
                );
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
                #[autospy(returns = "String")]
                fn function(&self) -> impl ToString;
            }
        }))
    }

    #[test]
    fn functions_marked_with_multiple_attributes_retain_non_autospy_attributes() {
        insta::assert_snapshot!(generate_pretty(quote! {
            trait TestTrait {
                #[some_attribute]
                #[autospy(returns = "String")]
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

    #[test]
    fn traits_with_multiple_associated_types() {
        insta::assert_snapshot!(generate_pretty(quote! {
            trait TestTrait {
                #[autospy(String)]
                type Argument;
                #[autospy(String)]
                type Return;
                fn function(&self, argument: Self::Argument) -> Self::Return;
            }
        }))
    }

    #[test]
    fn async_trait() {
        insta::assert_snapshot!(generate_pretty(quote! {
            #[async_trait]
            trait TestTrait {
                async fn function(&self);
            }
        }))
    }

    #[test]
    fn async_trait_with_other_trait_requirements() {
        insta::assert_snapshot!(generate_pretty(quote! {
            #[async_trait]
            trait TestTrait: Send + Sync + 'static {
                async fn function(&self);
            }
        }))
    }
    #[test]
    fn traits_with_associated_consts() {
        insta::assert_snapshot!(generate_pretty(quote! {
            trait TestTrait {
                #[autospy("example")]
                const VALUE: &'static str;
                fn function(&self);
            }
        }))
    }
    #[test]
    fn traits_with_default_definitions() {
        insta::assert_snapshot!(generate_pretty(quote! {
            trait TestTrait {
                #[autospy(use_default)]
                fn function(&self) -> u8 {
                    1
                }
            }
        }))
    }
}
