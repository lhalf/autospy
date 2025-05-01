use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemTrait, TraitItem, parse_macro_input};

pub fn generate(item: TokenStream) -> TokenStream {
    let _trait = parse_macro_input!(item as ItemTrait);
    let spy_name = format_ident!("{}Spy", _trait.ident);

    let mut spy_fields = vec![];
    let mut method_impls = vec![];

    for item in &_trait.items {
        if let TraitItem::Fn(method) = item {
            let method_name = &method.sig.ident;
            let output = &method.sig.output;

            let inputs: Vec<_> = method
                .sig
                .inputs
                .iter()
                .filter_map(|arg| match arg {
                    syn::FnArg::Typed(pat_type) => Some(pat_type),
                    _ => None,
                })
                .collect();

            let input_idents: Vec<_> = inputs.iter().map(|p| &p.pat).collect();
            let input_types: Vec<_> = inputs.iter().map(|p| &p.ty).collect();

            let args_tuple_type = if input_types.len() == 1 {
                quote! { #(#input_types)* }
            } else {
                quote! { (#(#input_types),*) }
            };

            let args_tuple_value = if input_idents.len() == 1 {
                quote! { #(#input_idents)* }
            } else {
                quote! { (#(#input_idents),*) }
            };

            spy_fields.push(quote! {
                pub #method_name: spygen::spy_function::SpyFunction<#args_tuple_type, #output>,
            });

            let asyncness = &method.sig.asyncness;
            let awaitness = if asyncness.is_some() {
                quote! {.await}
            } else {
                quote! {}
            };

            method_impls.push(quote! {
                #asyncness fn #method_name(&self, #(#inputs),*) #output {
                    self.#method_name.spy(#args_tuple_value) #awaitness
                }
            });
        }
    }

    let expanded = quote! {
        #_trait

        #[cfg(test)]
        pub mod test_doubles {
            use super::*;
            use spygen::spy_function::SpyFunction;

            #[derive(Default, Clone)]
            #_trait.vis struct #spy_name {
                #(#spy_fields)*
            }

            #[async_trait::async_trait]
            impl #_trait.ident for #spy_name {
                #(#method_impls)*
            }
        }
    };

    TokenStream::from(expanded)
}
