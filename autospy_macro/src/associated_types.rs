use crate::attribute;
use syn::{Ident, ItemTrait, TraitItem, TraitItemType, TypePath};

// Vec rather than HashMap so that ordering is preserved.
// Probably more efficent anyway because never very many of them. But this has not been performance tested.
pub type AssociatedSpyTypes = Vec<(Ident, TypePath)>;

pub fn get_associated_types(item_trait: &ItemTrait) -> AssociatedSpyTypes {
    item_trait
        .items
        .iter()
        .filter_map(associated_types)
        .filter_map(associated_type_name_and_spy_type)
        .collect()
}

fn associated_types(item: &TraitItem) -> Option<&TraitItemType> {
    match item {
        TraitItem::Type(trait_type) => Some(trait_type),
        _ => None,
    }
}

fn associated_type_name_and_spy_type(trait_item: &TraitItemType) -> Option<(Ident, TypePath)> {
    Some((
        trait_item.ident.clone(),
        attribute::associated_type(&trait_item.attrs)?,
    ))
}

#[cfg(test)]
mod tests {
    use crate::associated_types::{AssociatedSpyTypes, get_associated_types};

    use quote::quote;
    use syn::ItemTrait;

    #[test]
    fn empty_trait_has_no_associated_types() {
        let input: ItemTrait = syn::parse2(quote! {
            trait Example {}
        })
        .unwrap();

        assert_eq!(AssociatedSpyTypes::new(), get_associated_types(&input));
    }

    #[test]
    fn single_associated_type() {
        let input: ItemTrait = syn::parse2(quote! {
            trait Example {
                #[autospy(String)]
                type Hello;
            }
        })
        .unwrap();

        let expected: AssociatedSpyTypes = vec![(
            syn::parse2(quote! {
                Hello
            })
            .unwrap(),
            syn::parse2(quote! {
                String
            })
            .unwrap(),
        )];

        assert_eq!(expected, get_associated_types(&input));
    }

    #[test]
    fn multiple_associated_types() {
        let input: ItemTrait = syn::parse2(quote! {
            trait Example {
                #[autospy(String)]
                type Hello;
                #[autospy(bool)]
                type Nope;
            }
        })
        .unwrap();

        let expected: AssociatedSpyTypes = vec![
            (
                syn::parse2(quote! {
                    Hello
                })
                .unwrap(),
                syn::parse2(quote! {
                    String
                })
                .unwrap(),
            ),
            (
                syn::parse2(quote! {
                    Nope
                })
                .unwrap(),
                syn::parse2(quote! {
                    bool
                })
                .unwrap(),
            ),
        ];

        assert_eq!(expected, get_associated_types(&input));
    }

    #[test]
    fn multiple_associated_types_in_between_trait_functions() {
        let input: ItemTrait = syn::parse2(quote! {
            trait Example {
                fn again();
                #[autospy(String)]
                type Hello;
                fn hello();
                #[autospy(bool)]
                type Nope;
            }
        })
        .unwrap();

        let expected: AssociatedSpyTypes = vec![
            (
                syn::parse2(quote! {
                    Hello
                })
                .unwrap(),
                syn::parse2(quote! {
                    String
                })
                .unwrap(),
            ),
            (
                syn::parse2(quote! {
                    Nope
                })
                .unwrap(),
                syn::parse2(quote! {
                    bool
                })
                .unwrap(),
            ),
        ];

        assert_eq!(expected, get_associated_types(&input));
    }

    #[test]
    fn other_attributes_dont_affect_associated_types() {
        let input: ItemTrait = syn::parse2(quote! {
            trait Example {
                #[another_attribute]
                #[autospy(String)]
                #[some_attribute]
                type Hello;
            }
        })
        .unwrap();

        let expected: AssociatedSpyTypes = vec![(
            syn::parse2(quote! {
                Hello
            })
            .unwrap(),
            syn::parse2(quote! {
                String
            })
            .unwrap(),
        )];

        assert_eq!(expected, get_associated_types(&input));
    }
}
