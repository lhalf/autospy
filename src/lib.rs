extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemTrait, TraitItem, parse_macro_input};

#[proc_macro_attribute]
pub fn autospy(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Convert the original input for later use
    let input_copy = proc_macro2::TokenStream::from(item.clone());

    let input = parse_macro_input!(item as ItemTrait);
    let trait_name = &input.ident;
    let spy_name = format_ident!("{}Spy", trait_name);

    let mut spy_fields = Vec::new();
    let mut trait_impls = Vec::new();

    for item in &input.items {
        if let TraitItem::Fn(method) = item {
            let method_name = &method.sig.ident;
            let inputs = &method.sig.inputs;

            let mut arg_names = Vec::new();
            let mut params = Vec::new();

            for arg in inputs.iter().skip(1) {
                if let syn::FnArg::Typed(pat_type) = arg {
                    if let syn::Pat::Ident(ref pat_ident) = *pat_type.pat {
                        let arg_ident = &pat_ident.ident;
                        arg_names.push(quote! { #arg_ident });
                        params.push(quote! { #arg_ident: String }); // assumes all args are String
                    }
                }
            }

            spy_fields.push(quote! {
                pub #method_name: Vec<String>
            });

            trait_impls.push(quote! {
                fn #method_name(&mut self, #(#params),*) {
                    #(self.#method_name.push(#arg_names);)*
                }
            });
        }
    }

    let expanded = quote! {
        #input_copy

        #[derive(Default)]
        pub struct #spy_name {
            #(#spy_fields),*
        }

        impl #trait_name for #spy_name {
            #(#trait_impls)*
        }
    };

    TokenStream::from(expanded)
}
