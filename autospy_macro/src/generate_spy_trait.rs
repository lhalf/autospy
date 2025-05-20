use crate::inspect::AssociatedType;
use crate::strip_attributes::strip_attributes_from_signature;
use crate::{edit, generate, inspect};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemTrait, TraitItemFn, Type};

pub fn generate_spy_trait(
    item_trait: &ItemTrait,
    associated_type: &Option<AssociatedType>,
) -> TokenStream {
    let trait_name = &item_trait.ident;
    let spy_name = format_ident!("{}Spy", trait_name);
    let associated_type_definitions = associated_type_definitions(associated_type);
    let spy_function_definitions = trait_spy_function_definitions(item_trait);

    quote! {
        impl #trait_name for #spy_name {
            #associated_type_definitions
            #(#spy_function_definitions)*
        }
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
