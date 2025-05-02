extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, FnArg, ItemTrait, Pat, TraitItem};

#[proc_macro_attribute]
pub fn autospy(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let original_item = proc_macro2::TokenStream::from(item.clone());

    let input_trait = parse_macro_input!(item as ItemTrait);
    let trait_name = &input_trait.ident;
    let spy_name = format_ident!("{}Spy", trait_name);

    let mut spy_fields = Vec::new();
    let mut trait_impls = Vec::new();

    for item in &input_trait.items {
        if let TraitItem::Fn(method) = item {
            let method_name = &method.sig.ident;
            let inputs = &method.sig.inputs;

            // Skip the first input (`&self`)
            if let Some(FnArg::Typed(pat_type)) = inputs.iter().skip(1).next() {
                // Extract the argument name and type
                let arg_name = if let Pat::Ident(ref pat_ident) = *pat_type.pat {
                    &pat_ident.ident
                } else {
                    continue;
                };

                // Field for SpyFunction
                spy_fields.push(quote! {
                    pub #method_name: SpyFunction<#*pat_type.ty>
                });

                // Trait method implementation
                trait_impls.push(quote! {
                    fn #method_name(&self, #arg_name: #*pat_type.ty) {
                        self.#method_name.spy(#arg_name);
                    }
                });
            }
        }
    }

    let expanded = quote! {
        #original_item

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
