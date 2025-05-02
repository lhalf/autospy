extern crate proc_macro;

mod generate;

use generate::generate;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn autospy(_attr: TokenStream, item: TokenStream) -> TokenStream {
    TokenStream::from(generate(proc_macro2::TokenStream::from(item)))
}
