extern crate proc_macro;

mod arguments;
mod associated_types;
mod attribute;
mod edit;
mod generate;
mod generate_spy_default;
mod generate_spy_struct;
mod generate_spy_trait;
mod generics;
mod inspect;
mod strip_attributes;
mod supertraits;

use generate::generate;
use proc_macro::TokenStream;
use syn::parse_quote;

#[proc_macro_attribute]
pub fn autospy(attributes: TokenStream, item: TokenStream) -> TokenStream {
    let item = proc_macro2::TokenStream::from(item);
    let external_trait = match attributes.to_string().as_str() {
        "" => false,
        "external" => true,
        _ => panic!("invalid attribute"),
    };
    TokenStream::from(generate(parse_quote! { #item }, external_trait))
}
