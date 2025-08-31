use crate::arguments::is_argument_marked_as_ignore;
use crate::associated_types::AssociatedSpyTypes;
use syn::visit_mut::VisitMut;
use syn::{FnArg, PatType, Signature, TypePath, parse_quote};

pub struct AssociatedTypeReplacer<'a> {
    pub associated_spy_types: &'a AssociatedSpyTypes,
}

impl VisitMut for AssociatedTypeReplacer<'_> {
    fn visit_type_path_mut(&mut self, type_path: &mut TypePath) {
        if let Some(replacement) = self.associated_type_replacement(type_path) {
            *type_path = replacement;
        }

        syn::visit_mut::visit_type_path_mut(self, type_path);
    }
}

impl AssociatedTypeReplacer<'_> {
    fn associated_type_replacement(&self, type_path: &mut TypePath) -> Option<TypePath> {
        if type_path.qself.is_some() || type_path.path.segments.len() != 2 {
            return None;
        }

        let first = &type_path.path.segments[0].ident;

        if first != "Self" {
            return None;
        }

        let second = &type_path.path.segments[1].ident;

        let (_, associated_type) = self
            .associated_spy_types
            .iter()
            .find(|(ident, _)| *ident == second)?;

        Some(associated_type.clone())
    }
}

// TODO: do this by visitor pattern too?
pub fn underscore_ignored_arguments_in_signature(signature: &mut Signature) {
    non_self_signature_arguments_mut(signature)
        .filter(|argument| is_argument_marked_as_ignore(argument))
        .for_each(rename_argument_to_underscore);
}

pub fn non_self_signature_arguments_mut(
    signature: &mut Signature,
) -> impl Iterator<Item = &mut PatType> {
    signature.inputs.iter_mut().filter_map(|input| match input {
        FnArg::Typed(argument) => Some(argument),
        _ => None,
    })
}

fn rename_argument_to_underscore(argument: &mut PatType) {
    argument.pat = parse_quote! { _ };
}
