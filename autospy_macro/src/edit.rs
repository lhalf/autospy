use crate::arguments::is_argument_marked_as_ignore;
use crate::associated_types::{AssociatedSpyTypes, AssociatedType};
use syn::visit_mut::VisitMut;
use syn::{FnArg, PatType, Signature, Type, parse_quote};

pub struct AssociatedTypeReplacer<'a> {
    pub associated_spy_types: &'a AssociatedSpyTypes,
}

impl VisitMut for AssociatedTypeReplacer<'_> {
    fn visit_type_mut(&mut self, r#type: &mut Type) {
        if let Some(replacement) = self.associated_type_replacement(r#type) {
            *r#type = replacement;
        }

        syn::visit_mut::visit_type_mut(self, r#type);
    }
}

impl AssociatedTypeReplacer<'_> {
    fn associated_type_replacement(&self, r#type: &mut Type) -> Option<Type> {
        let Type::Path(type_path) = r#type else {
            return None;
        };

        if type_path.qself.is_some() || type_path.path.segments.len() != 2 {
            return None;
        }

        if type_path.path.segments[0].ident != "Self" {
            return None;
        }

        let (_, AssociatedType { r#type, .. }) = self
            .associated_spy_types
            .iter()
            .find(|(ident, _)| **ident == type_path.path.segments[1].ident)?;

        Some(r#type.clone())
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
        FnArg::Receiver(_) => None,
    })
}

fn rename_argument_to_underscore(argument: &mut PatType) {
    argument.pat = parse_quote! { _ };
}
