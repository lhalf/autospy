use crate::{edit, inspect};
use syn::{Attribute, ItemTrait, Signature, TraitItem};

pub fn strip_attributes(mut item_trait: ItemTrait) -> ItemTrait {
    item_trait
        .items
        .iter_mut()
        .for_each(strip_attributes_from_item);
    item_trait
}

pub fn strip_attributes_from_signature(signature: &mut Signature) {
    for argument in edit::non_self_signature_arguments_mut(signature) {
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
        _ => (),
    }
}

fn strip_autospy_attributes(attributes: &mut Vec<Attribute>) {
    attributes.retain(|attribute| !inspect::is_autospy_attribute(attribute));
}

#[cfg(test)]
mod tests {
    use crate::strip_attributes::strip_attributes;
    use quote::quote;
    use syn::ItemTrait;

    #[test]
    fn autospy_attributes_are_stripped_from_arguments() {
        let input: ItemTrait = syn::parse2(quote! {
            trait Example {
                fn foo(&self, #[autospy(ignore)] ignored: &str);
            }
        })
        .unwrap();

        let expected: ItemTrait = syn::parse2(quote! {
            trait Example {
                fn foo(&self, ignored: &str);
            }
        })
        .unwrap();

        assert_eq!(expected, strip_attributes(input));
    }

    #[test]
    fn non_autospy_attributes_are_retained_on_arguments() {
        let input: ItemTrait = syn::parse2(quote! {
            trait Example {
                fn foo(&self, #[some_attribute] #[autospy(ignore)] ignored: &str);
            }
        })
        .unwrap();

        let expected: ItemTrait = syn::parse2(quote! {
            trait Example {
                fn foo(&self, #[some_attribute] ignored: &str);
            }
        })
        .unwrap();

        assert_eq!(expected, strip_attributes(input));
    }

    #[test]
    fn autospy_attributes_are_stripped_on_associated_types() {
        let input: ItemTrait = syn::parse2(quote! {
            trait Example {
                #[autospy(String)]
                type Item;
                fn foo(&self, argument: Self::Item);
            }
        })
        .unwrap();

        let expected: ItemTrait = syn::parse2(quote! {
            trait Example {
                type Item;
                fn foo(&self, argument: Self::Item);
            }
        })
        .unwrap();

        assert_eq!(expected, strip_attributes(input));
    }

    #[test]
    fn non_autospy_attributes_are_retained_on_associated_types() {
        let input: ItemTrait = syn::parse2(quote! {
            trait Example {
                #[some_attribute]
                #[autospy(String)]
                type Item;
                fn foo(&self, argument: Self::Item);
            }
        })
        .unwrap();

        let expected: ItemTrait = syn::parse2(quote! {
            trait Example {
                #[some_attribute]
                type Item;
                fn foo(&self, argument: Self::Item);
            }
        })
        .unwrap();

        assert_eq!(expected, strip_attributes(input));
    }
}
