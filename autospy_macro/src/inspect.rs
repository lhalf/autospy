use proc_macro2::TokenStream;
use quote::quote;
use syn::visit::Visit;
use syn::{ItemTrait, ReturnType, TraitItem, TraitItemConst, TraitItemFn, Type, TypeReference};

pub fn associated_consts(item_trait: &ItemTrait) -> impl Iterator<Item = &TraitItemConst> {
    item_trait.items.iter().filter_map(|item| match item {
        TraitItem::Const(associated_const) => Some(associated_const),
        _ => None,
    })
}

pub fn trait_functions(item_trait: &ItemTrait) -> impl Iterator<Item = &TraitItemFn> {
    item_trait.items.iter().filter_map(|item| match item {
        TraitItem::Fn(function) => Some(function),
        _ => None,
    })
}

pub fn owned_trait_functions(item_trait: ItemTrait) -> impl Iterator<Item = TraitItemFn> {
    item_trait.items.into_iter().filter_map(|item| match item {
        TraitItem::Fn(function) => Some(function),
        _ => None,
    })
}

pub fn cfg() -> TokenStream {
    if cfg!(feature = "test") {
        quote! { #[cfg(test)] }
    } else {
        TokenStream::new()
    }
}

pub fn has_function_returning_elided_lifetime_reference(item_trait: &ItemTrait) -> bool {
    trait_functions(item_trait).any(function_return_has_elided_lifetime_reference)
}

fn function_return_has_elided_lifetime_reference(function: &TraitItemFn) -> bool {
    matches!(
        &function.sig.output,
        ReturnType::Type(_, return_type)
            if matches!(return_type.as_ref(), Type::Reference(type_ref) if type_ref.lifetime.is_none())
    )
}

pub fn has_function_returning_type_containing_elided_lifetime_reference(
    item_trait: &ItemTrait,
) -> bool {
    trait_functions(item_trait).any(function_return_type_contains_elided_lifetime_reference)
}

fn function_return_type_contains_elided_lifetime_reference(function: &TraitItemFn) -> bool {
    if let ReturnType::Type(_, return_type) = &function.sig.output {
        let mut visitor = LifetimeHunter {
            found_elided: false,
        };
        visitor.visit_type(return_type);
        visitor.found_elided
    } else {
        false
    }
}

struct LifetimeHunter {
    found_elided: bool,
}

impl<'ast> Visit<'ast> for LifetimeHunter {
    fn visit_type_reference(&mut self, type_ref: &'ast TypeReference) {
        if type_ref.lifetime.is_none() {
            self.found_elided = true;
        }

        syn::visit::visit_type_reference(self, type_ref);
    }
}
