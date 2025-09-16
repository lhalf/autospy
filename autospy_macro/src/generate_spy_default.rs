use crate::generics::generics_idents;
use crate::inspect::cfg;
use crate::{attribute, inspect};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemTrait, TraitItemFn};

pub fn generate_spy_default(item_trait: &ItemTrait) -> TokenStream {
    let cfg = cfg();

    let generics = &item_trait.generics;
    let generics_idents = generics_idents(generics);
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

fn generate_spy_defaults(item_trait: &ItemTrait) -> impl Iterator<Item = TokenStream> {
    inspect::trait_functions(item_trait).map(function_as_default)
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
    use crate::generate_spy_default::generate_spy_default;
    use quote::{ToTokens, quote};
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

        let actual = generate_spy_default(&input);

        assert_eq!(actual.to_token_stream().to_string(), expected.to_string());
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

        let actual = generate_spy_default(&input);

        assert_eq!(actual.to_token_stream().to_string(), expected.to_string());
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

        let actual = generate_spy_default(&input);

        assert_eq!(actual.to_token_stream().to_string(), expected.to_string());
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

        let actual = generate_spy_default(&input);

        assert_eq!(actual.to_token_stream().to_string(), expected.to_string());
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

        let actual = generate_spy_default(&input);

        assert_eq!(actual.to_token_stream().to_string(), expected.to_string());
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

        let actual = generate_spy_default(&input);

        assert_eq!(actual.to_token_stream().to_string(), expected.to_string());
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

        let actual = generate_spy_default(&input);

        assert_eq!(actual.to_token_stream().to_string(), expected.to_string());
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

        let actual = generate_spy_default(&input);

        assert_eq!(actual.to_token_stream().to_string(), expected.to_string());
    }

    #[test]
    fn supertrait_function_in_trait() {
        let input: ItemTrait = parse_quote! {
            trait Example: Supertrait {
                fn foo(&self);
                #[cfg(test)]
                #[autospy(supertrait = "SuperTrait")]
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

        let actual = generate_spy_default(&input);

        assert_eq!(actual.to_token_stream().to_string(), expected.to_string());
    }
}
