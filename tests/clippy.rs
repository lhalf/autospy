#![deny(
    missing_docs,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]

//! expanded macro does not trigger any of the above clippy lints
#[autospy::autospy]
#[allow(dead_code)]
trait MyTrait {
    fn function(&self);
}
