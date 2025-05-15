// vim: tw=80
//! A test spy object library.
//!
//! autospy is a macro to create spy versions of almost any trait.
//! They can be used in unit tests as a stand-in for the real object.
//!
//! # Usage
//!
//! To use autospy simply use the [`#[autospy]`](attr.autospy.html).

mod spy_function;

pub use spy_function::SpyFunction;

pub use autospy_macro::autospy;
