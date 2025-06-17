use std::collections::BTreeMap;
use crate::attribute;
use syn::{Ident, ItemTrait, TraitItem, TraitItemType, TypePath};

pub type AssociatedSpyTypes = BTreeMap<Ident, TypePath>;

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

    use quote::{ToTokens, format_ident};
    use syn::{ItemTrait, parse_quote};

    #[test]
    fn empty_trait_has_no_associated_types() {
        let input: ItemTrait = parse_quote! {
            trait Example {}
        };

        assert_eq!(AssociatedSpyTypes::new(), get_associated_types(&input));
    }

    #[test]
    fn single_associated_type() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                #[autospy(String)]
                type Hello;
            }
        };

        let expected = to_associated_spy_types([("Hello", "String")]);

        assert_eq!(expected, get_associated_types(&input));
    }

    #[test]
    fn multiple_associated_types() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                #[autospy(String)]
                type Hello;
                #[autospy(bool)]
                type Nope;
            }
        };

        let expected = to_associated_spy_types([("Hello", "String"), ("Nope", "bool")]);

        assert_eq!(expected, get_associated_types(&input));
    }

    #[test]
    fn multiple_associated_types_in_between_trait_functions() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                fn again();
                #[autospy(String)]
                type Hello;
                fn hello();
                #[autospy(bool)]
                type Nope;
            }
        };

        let expected = to_associated_spy_types([("Hello", "String"), ("Nope", "bool")]);

        assert_eq!(expected, get_associated_types(&input));
    }

    #[test]
    fn other_attributes_dont_affect_associated_types() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                #[another_attribute]
                #[autospy(String)]
                #[some_attribute]
                type Hello;
            }
        };

        let expected = to_associated_spy_types([("Hello", "String")]);

        assert_eq!(expected, get_associated_types(&input));
    }

    // TODO: is this the behaviour we want?
    #[test]
    fn associated_type_uses_the_first_found_attribute() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                #[another_attribute]
                #[autospy(String)]
                #[autospy(bool)]
                type Hello;
            }
        };

        let expected = to_associated_spy_types([("Hello", "String")]);

        assert_eq!(expected, get_associated_types(&input));
    }

    fn to_associated_spy_types(
        items: impl IntoIterator<Item = (&'static str, &'static str)>,
    ) -> AssociatedSpyTypes {
        items
            .into_iter()
            .map(|(ident, r#type)| {
                (
                    format_ident!("{ident}"),
                    syn::parse2(format_ident!("{type}").to_token_stream()).unwrap(),
                )
            })
            .collect()
    }
}
