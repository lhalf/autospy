extern crate proc_macro;

mod edit;
mod generate;
mod inspect;

use generate::generate;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn autospy(attributes: TokenStream, item: TokenStream) -> TokenStream {
    let attributes = proc_macro2::TokenStream::from(attributes);
    let item = proc_macro2::TokenStream::from(item);
    TokenStream::from(generate(attributes, item))
}
