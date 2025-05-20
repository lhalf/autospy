use crate::inspect;
use crate::inspect::AssociatedType;
use syn::visit_mut::VisitMut;
use syn::{Attribute, FnArg, ItemTrait, PatType, Signature, TraitItem, parse_quote};

pub struct AssociatedTypeReplacer {
    pub associated_type: AssociatedType,
}

impl VisitMut for AssociatedTypeReplacer {
    fn visit_type_path_mut(&mut self, type_path: &mut syn::TypePath) {
        if type_path.qself.is_none() && type_path.path.segments.len() == 2 {
            let segments = &type_path.path.segments;
            if segments[0].ident == "Self"
                && segments[1].ident == self.associated_type.name.to_string()
            {
                *type_path = syn::parse2(self.associated_type._type.clone())
                    .expect("invalid associated type");
                return;
            }
        }

        syn::visit_mut::visit_type_path_mut(self, type_path);
    }
}

pub fn strip_attributes_from_trait(mut item_trait: ItemTrait) -> ItemTrait {
    item_trait
        .items
        .iter_mut()
        .for_each(strip_attributes_from_item);
    item_trait
}

pub fn underscore_ignored_arguments_in_signature(signature: &mut Signature) {
    non_self_signature_arguments_mut(signature)
        .filter(|argument| inspect::is_argument_marked_as_ignore(argument))
        .for_each(rename_argument_to_underscore);
}

pub fn strip_attributes_from_signature(signature: &mut Signature) {
    for argument in non_self_signature_arguments_mut(signature) {
        strip_autospy_attributes(&mut argument.attrs);
    }
}

fn strip_attributes_from_item(item: &mut TraitItem) {
    match item {
        TraitItem::Fn(function) => {
            strip_autospy_attributes(&mut function.attrs);
            strip_attributes_from_signature(&mut function.sig);
        }
        TraitItem::Type(_type) => strip_autospy_attributes(&mut _type.attrs),
        _ => todo!(),
    }
}

fn strip_autospy_attributes(attributes: &mut Vec<Attribute>) {
    attributes.retain(|attribute| !inspect::is_autospy_attribute(attribute));
}

fn rename_argument_to_underscore(argument: &mut PatType) {
    argument.pat = parse_quote! { _ };
}

fn non_self_signature_arguments_mut(
    signature: &mut Signature,
) -> impl Iterator<Item = &mut PatType> {
    signature.inputs.iter_mut().filter_map(|input| match input {
        FnArg::Typed(argument) => Some(argument),
        _ => None,
    })
}
