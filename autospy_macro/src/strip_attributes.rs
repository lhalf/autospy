use crate::supertraits::autospy_supertrait_macro;
use crate::{attribute, edit};
use syn::{Attribute, ItemTrait, Signature, TraitItem};

pub fn strip_attributes(mut item_trait: ItemTrait) -> ItemTrait {
    item_trait
        .items
        .retain(|item| autospy_supertrait_macro(item).is_none());
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

pub fn strip_autospy_attributes(attributes: &mut Vec<Attribute>) {
    attributes.retain(|attribute| !attribute::is_autospy_attribute(attribute));
}

fn strip_attributes_from_item(item: &mut TraitItem) {
    match item {
        TraitItem::Fn(function) => {
            strip_autospy_attributes(&mut function.attrs);
            strip_attributes_from_signature(&mut function.sig);
        }
        TraitItem::Type(_type) => strip_autospy_attributes(&mut _type.attrs),
        TraitItem::Const(_const) => strip_autospy_attributes(&mut _const.attrs),
        _ => (),
    }
}

#[cfg(test)]
mod tests {
    use crate::strip_attributes::strip_attributes;
    use syn::{ItemTrait, parse_quote};

    #[test]
    fn non_autospy_trait_attributes_are_retained() {
        let input: ItemTrait = parse_quote! {
            #[some_attribute]
            trait Example {
                fn foo(&self);
            }
        };

        let expected: ItemTrait = parse_quote! {
            #[some_attribute]
            trait Example {
                fn foo(&self);
            }
        };

        assert_eq!(expected, strip_attributes(input));
    }

    #[test]
    fn autospy_attributes_are_stripped_from_arguments() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                fn foo(&self, #[autospy(ignore)] ignored: &str);
            }
        };

        let expected: ItemTrait = parse_quote! {
            trait Example {
                fn foo(&self, ignored: &str);
            }
        };

        assert_eq!(expected, strip_attributes(input));
    }

    #[test]
    fn cfg_attr_test_autospy_attributes_are_stripped_from_arguments() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                fn foo(&self, #[cfg_attr(test, autospy(ignore))] ignored: &str);
            }
        };

        let expected: ItemTrait = parse_quote! {
            trait Example {
                fn foo(&self, ignored: &str);
            }
        };

        assert_eq!(expected, strip_attributes(input));
    }

    #[test]
    fn non_autospy_attributes_are_retained_on_arguments() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                fn foo(&self, #[some_attribute] #[autospy(ignore)] ignored: &str);
            }
        };

        let expected: ItemTrait = parse_quote! {
            trait Example {
                fn foo(&self, #[some_attribute] ignored: &str);
            }
        };

        assert_eq!(expected, strip_attributes(input));
    }

    #[test]
    fn autospy_attributes_are_stripped_on_associated_types() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                #[autospy(String)]
                type Item;
                fn foo(&self, argument: Self::Item);
            }
        };

        let expected: ItemTrait = parse_quote! {
            trait Example {
                type Item;
                fn foo(&self, argument: Self::Item);
            }
        };

        assert_eq!(expected, strip_attributes(input));
    }

    #[test]
    fn autospy_attributes_are_stripped_on_associated_consts() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                #[autospy(100)]
                const VALUE: u64;
                fn foo(&self);
            }
        };

        let expected: ItemTrait = parse_quote! {
            trait Example {
                const VALUE: u64;
                fn foo(&self);
            }
        };

        assert_eq!(expected, strip_attributes(input));
    }

    #[test]
    fn cfg_attr_test_autospy_attributes_are_stripped_on_associated_types() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                #[cfg_attr(test, autospy(String))]
                type Item;
                fn foo(&self, argument: Self::Item);
            }
        };

        let expected: ItemTrait = parse_quote! {
            trait Example {
                type Item;
                fn foo(&self, argument: Self::Item);
            }
        };

        assert_eq!(expected, strip_attributes(input));
    }

    #[test]
    fn non_autospy_attributes_are_retained_on_associated_types() {
        let input: ItemTrait = parse_quote! {
            trait Example {
                #[some_attribute]
                #[autospy(String)]
                type Item;
                fn foo(&self, argument: Self::Item);
            }
        };

        let expected: ItemTrait = parse_quote! {
            trait Example {
                #[some_attribute]
                type Item;
                fn foo(&self, argument: Self::Item);
            }
        };

        assert_eq!(expected, strip_attributes(input));
    }

    #[test]
    fn supertrait_macros_are_stripped() {
        let input: ItemTrait = parse_quote! {
            trait Example: Supertrait {
                fn foo(&self);
                #[cfg(test)]
                autospy::supertrait! {
                    trait Supertrait {
                       fn bar(&self);
                    }
                }
            }
        };

        let expected: ItemTrait = parse_quote! {
            trait Example: Supertrait {
                fn foo(&self);
            }
        };

        assert_eq!(expected, strip_attributes(input));
    }
}
