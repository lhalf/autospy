use crate::associated_types::AssociatedType;
use crate::inspect;
use syn::visit_mut::VisitMut;
use syn::{FnArg, PatType, Signature, parse_quote};

pub struct AssociatedTypeReplacer {
    pub associated_type: AssociatedType,
}

impl VisitMut for AssociatedTypeReplacer {
    fn visit_type_path_mut(&mut self, type_path: &mut syn::TypePath) {
        if type_path.qself.is_none() && type_path.path.segments.len() == 2 {
            let segments = &type_path.path.segments;
            if segments[0].ident == "Self" && segments[1].ident == self.associated_type.name {
                *type_path = syn::parse2(self.associated_type.r#type.clone())
                    .expect("invalid associated type");
                return;
            }
        }

        syn::visit_mut::visit_type_path_mut(self, type_path);
    }
}

pub fn underscore_ignored_arguments_in_signature(signature: &mut Signature) {
    non_self_signature_arguments_mut(signature)
        .filter(|argument| inspect::is_argument_marked_as_ignore(argument))
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
