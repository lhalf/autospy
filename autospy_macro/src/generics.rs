use syn::{GenericParam, Generics};

pub fn generics_idents(generics: &Generics) -> Generics {
    let mut generics_idents = generics.clone();

    for param in generics_idents.params.iter_mut() {
        if let GenericParam::Type(ty_param) = param {
            ty_param.bounds.clear();
            ty_param.colon_token = None;
            ty_param.eq_token = None;
            ty_param.default = None;
        }
    }

    generics_idents
}

#[cfg(test)]
mod tests {
    use super::generics_idents;
    use syn::{Generics, parse_quote};

    #[test]
    fn single_bound() {
        let input: Generics = parse_quote! {
            <T: Copy>
        };

        let expected: Generics = parse_quote! {
            <T>
        };

        assert_eq!(expected, generics_idents(&input));
    }

    #[test]
    fn multiple_bounds() {
        let input: Generics = parse_quote! {
            <T: Clone + Send + 'static>
        };

        let expected: Generics = parse_quote! {
            <T>
        };

        assert_eq!(expected, generics_idents(&input));
    }

    #[test]
    fn multiple_with_bounds() {
        let input: Generics = parse_quote! {
            <T: Clone, U: Default + Send>
        };

        let expected: Generics = parse_quote! {
            <T, U>
        };

        assert_eq!(expected, generics_idents(&input));
    }

    #[test]
    fn with_defaults() {
        let input: Generics = parse_quote! {
            <T = i32, U: Clone = String>
        };

        let expected: Generics = parse_quote! {
            <T, U>
        };

        assert_eq!(expected, generics_idents(&input));
    }

    #[test]
    fn with_lifetimes_and_consts() {
        let input: Generics = parse_quote! {
            <'a, const N: usize, T: Copy>
        };

        let expected: Generics = parse_quote! {
            <'a, const N: usize, T>
        };

        assert_eq!(expected, generics_idents(&input));
    }

    #[test]
    fn empty_generics() {
        let input: Generics = parse_quote! {
            <>
        };

        let expected: Generics = parse_quote! {
            <>
        };

        assert_eq!(expected, generics_idents(&input));
    }
}
