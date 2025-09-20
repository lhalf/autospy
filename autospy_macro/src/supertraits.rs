use syn::{ItemTrait, TraitItem, TraitItemMacro};

pub fn autospy_supertraits(item_trait: &ItemTrait) -> impl Iterator<Item = ItemTrait> {
    item_trait
        .items
        .iter()
        .filter_map(autospy_supertrait_macro)
        .filter_map(to_supertrait)
}

pub fn autospy_supertrait_macro(trait_item: &TraitItem) -> Option<&TraitItemMacro> {
    match trait_item {
        TraitItem::Macro(r#macro) if is_autospy_supertrait_macro(r#macro) => Some(r#macro),
        _ => None,
    }
}

fn is_autospy_supertrait_macro(trait_item_macro: &TraitItemMacro) -> bool {
    let path = &trait_item_macro.mac.path;
    match path.segments.len() {
        1 => path.segments[0].ident == "supertrait",
        2 => path.segments[0].ident == "autospy" && path.segments[1].ident == "supertrait",
        _ => false,
    }
}

fn to_supertrait(trait_item_macro: &TraitItemMacro) -> Option<ItemTrait> {
    syn::parse2::<ItemTrait>(trait_item_macro.mac.tokens.clone()).ok()
}

#[cfg(test)]
mod tests {
    use super::autospy_supertraits;
    use syn::{ItemTrait, parse_quote};

    #[test]
    fn no_autospy_supertrait_macros() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                fn foo(&self);
            }
        };

        assert!(autospy_supertraits(&input).collect::<Vec<_>>().is_empty());
    }

    #[test]
    fn invalid_name_non_autospy_trait_macros() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                fn foo(&self);
                not_recognised! {}
            }
        };

        assert!(autospy_supertraits(&input).collect::<Vec<_>>().is_empty());
    }

    #[test]
    fn invalid_name_autospy_trait_macros() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                fn foo(&self);
                autospy::not_recognised! {}
            }
        };

        assert!(autospy_supertraits(&input).collect::<Vec<_>>().is_empty());
    }

    #[test]
    fn invalid_trait_inside_valid_autospy_supertrait_macro() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                fn foo(&self);
                autospy::supertrait! {
                    fn not_a_valid_trait(&self);
                }
            }
        };

        assert!(autospy_supertraits(&input).collect::<Vec<_>>().is_empty());
    }

    #[test]
    fn single_autospy_supertrait_macro() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                fn foo(&self);
                autospy::supertrait! {
                    trait Supertrait {
                        fn bar(&self);
                    }
                }
            }
        };

        assert_eq!(1, autospy_supertraits(&input).collect::<Vec<_>>().len());
    }

    #[test]
    fn single_supertrait_macro() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                fn foo(&self);
                supertrait! {
                    trait Supertrait {
                        fn bar(&self);
                    }
                }
            }
        };

        assert_eq!(1, autospy_supertraits(&input).collect::<Vec<_>>().len());
    }

    #[test]
    fn multiple_autospy_supertrait_macros() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                fn foo(&self);
                autospy::supertrait! {
                    trait Supertrait {
                        fn bar(&self);
                    }
                }
                autospy::supertrait! {
                    trait Megatrait {
                        fn baz(&self);
                    }
                }
            }
        };

        assert_eq!(2, autospy_supertraits(&input).collect::<Vec<_>>().len());
    }
}
