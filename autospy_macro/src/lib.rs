extern crate proc_macro;

mod associated_types;
mod attribute;
mod edit;
mod generate;
mod generate_spy_struct;
mod generate_spy_trait;
mod inspect;
mod strip_attributes;

use generate::generate;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn autospy(attributes: TokenStream, item: TokenStream) -> TokenStream {
    if attributes.is_empty() {
        TokenStream::from(generate(proc_macro2::TokenStream::from(item)))
    } else {
        item
    }
}
