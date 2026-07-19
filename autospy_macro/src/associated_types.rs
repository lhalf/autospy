use crate::attribute;
use crate::supertraits;
use std::collections::BTreeMap;
use syn::{Generics, Ident, ItemTrait, Lifetime, TraitItem, TraitItemType, Type};

pub type AssociatedSpyTypes = BTreeMap<Ident, AssociatedType>;

#[derive(Debug, PartialEq, Eq)]
pub struct AssociatedType {
    pub r#type: Type,
    pub generics: Generics,
}

impl AssociatedType {
    pub const fn lifetime(&self) -> Option<&Lifetime> {
        match &self.r#type {
            Type::Reference(reference) => reference.lifetime.as_ref(),
            _ => None,
        }
    }

    pub const fn has_lifetime(&self) -> bool {
        self.lifetime().is_some()
    }
}

pub fn get_all_associated_types(item_trait: &ItemTrait) -> AssociatedSpyTypes {
    supertraits::autospy_supertraits(item_trait)
        .flat_map(|supertrait| get_associated_types(&supertrait))
        .chain(get_associated_types(item_trait))
        .collect()
}

pub fn get_associated_types(item_trait: &ItemTrait) -> AssociatedSpyTypes {
    item_trait
        .items
        .iter()
        .filter_map(associated_types)
        .filter_map(associated_type_name_and_spy_type)
        .collect()
}

const fn associated_types(item: &TraitItem) -> Option<&TraitItemType> {
    match item {
        TraitItem::Type(trait_type) => Some(trait_type),
        _ => None,
    }
}

fn associated_type_name_and_spy_type(
    trait_item: &TraitItemType,
) -> Option<(Ident, AssociatedType)> {
    Some((
        trait_item.ident.clone(),
        AssociatedType {
            r#type: attribute::associated_type(&trait_item.attrs)?,
            generics: trait_item.generics.clone(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use crate::associated_types::{
        AssociatedSpyTypes, AssociatedType, get_all_associated_types, get_associated_types,
    };

    use quote::format_ident;
    use syn::{ItemTrait, TraitItemType, Type, parse_quote};

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

        let expected = to_associated_spy_types([("Hello", parse_quote! { String })]);

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
                #[autospy(&str)]
                type Reference;
                #[autospy(&'static str)]
                type StaticReference;
            }
        };

        let expected = to_associated_spy_types([
            ("Hello", parse_quote! { String }),
            ("Nope", parse_quote! { bool }),
            ("Reference", parse_quote! { &str }),
            ("StaticReference", parse_quote! { &'static str }),
        ]);

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

        let expected = to_associated_spy_types([
            ("Hello", parse_quote! { String }),
            ("Nope", parse_quote! { bool }),
        ]);

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

        let expected = to_associated_spy_types([("Hello", parse_quote! { String })]);

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

        let expected = to_associated_spy_types([("Hello", parse_quote! { String })]);

        assert_eq!(expected, get_associated_types(&input));
    }

    #[test]
    fn associated_type_with_generic_associated_type_captures_generics() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                #[autospy(String)]
                type Hello<'a> where Self: 'a;
            }
        };

        let mut expected = AssociatedSpyTypes::new();

        // couldn't get parse_quote! to work directly on a syn::Generics
        let associated_type: TraitItemType = parse_quote! { type Hello<'a> where Self: 'a; };

        expected.insert(
            parse_quote! { Hello },
            AssociatedType {
                r#type: parse_quote! { String },
                generics: associated_type.generics,
            },
        );

        assert_eq!(expected, get_associated_types(&input));
    }

    #[test]
    fn get_all_associated_types_ignores_supertrait_blocks_when_getting_top_level_types() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                #[autospy(String)]
                type Hello;
                autospy::supertrait! {
                    trait Supertrait {
                        #[autospy(bool)]
                        type Nope;
                    }
                }
            }
        };

        let expected = to_associated_spy_types([("Hello", parse_quote! { String })]);

        assert_eq!(expected, get_associated_types(&input));
    }

    #[test]
    fn get_all_associated_types_includes_top_level_and_supertrait_types() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                #[autospy(String)]
                type Hello;
                autospy::supertrait! {
                    trait Supertrait {
                        #[autospy(bool)]
                        type Nope;
                    }
                }
            }
        };

        let expected = to_associated_spy_types([
            ("Hello", parse_quote! { String }),
            ("Nope", parse_quote! { bool }),
        ]);

        assert_eq!(expected, get_all_associated_types(&input));
    }

    #[test]
    fn get_all_associated_types_includes_types_from_multiple_supertraits() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                autospy::supertrait! {
                    trait ReadNorFlash {
                        #[autospy(String)]
                        type Reader;
                    }
                }
                autospy::supertrait! {
                    trait ErrorType {
                        #[autospy(bool)]
                        type Error;
                    }
                }
            }
        };

        let expected = to_associated_spy_types([
            ("Reader", parse_quote! { String }),
            ("Error", parse_quote! { bool }),
        ]);

        assert_eq!(expected, get_all_associated_types(&input));
    }

    #[test]
    fn get_all_associated_types_ignores_supertrait_types_without_attribute() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                autospy::supertrait! {
                    trait Supertrait {
                        type NotCaptured;
                        #[autospy(bool)]
                        type Captured;
                    }
                }
            }
        };

        let expected = to_associated_spy_types([("Captured", parse_quote! { bool })]);

        assert_eq!(expected, get_all_associated_types(&input));
    }

    #[test]
    fn get_all_associated_types_of_trait_without_supertraits_matches_top_level_types() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                #[autospy(String)]
                type Hello;
            }
        };

        assert_eq!(
            get_associated_types(&input),
            get_all_associated_types(&input)
        );
    }

    fn to_associated_spy_types(
        items: impl IntoIterator<Item = (&'static str, Type)>,
    ) -> AssociatedSpyTypes {
        items
            .into_iter()
            .map(|(ident, r#type)| {
                (
                    format_ident!("{ident}"),
                    AssociatedType {
                        r#type,
                        generics: parse_quote! {},
                    },
                )
            })
            .collect()
    }
}
