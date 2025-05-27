// vim: tw=80
//! A test spy object library.
//!
//! autospy is a macro to create spy versions of almost any trait.
//! They can be used in unit tests as a stand-in for the real object.
//!
//! # Usage
//!
//! To use autospy simply attribute your trait using [`#[autospy]`](attr.autospy.html).
//!
//! **Note:** If you're using the spy object for tests it is recommended to use `#[cfg_attr(test, autospy)]`.
//! ```rust
//! use autospy::autospy;
//!
//! #[autospy]
//! trait MyTrait {
//!     fn foo(&self, argument: u32) -> u32;
//! }
//!
//! fn call_with_ten(x: &dyn MyTrait) -> u32 {
//!     x.foo(10)
//! }
//!
//! let spy = MyTraitSpy::default();
//! spy.foo.returns.push_back(20);
//!
//! assert_eq!(20, call_with_ten(&spy.clone()));
//! assert_eq!(vec![10], spy.foo.arguments.take_all());
//! ```
//!
//! ## References
//!
//! `autospy` will automatically convert reference arguments into owned types.
//!
//! ```rust
//! use autospy::autospy;
//!
//! #[autospy]
//! trait MyTrait {
//!     fn foo(&self, argument: &str);
//! }
//!
//! fn use_trait(x: &dyn MyTrait) {
//!     x.foo("hello!")
//! }
//!
//! let spy = MyTraitSpy::default();
//! spy.foo.returns.push_back(());
//!
//! use_trait(&spy.clone());
//!
//! assert_eq!(vec!["hello!"], spy.foo.arguments.take_all());
//! ```
//!
//! ## Associated types
//!
//! An `#[autospy(TYPE)]` attribute can be applied to associated types to tell `autospy` how to capture them.
//!
//! ```rust
//! use autospy::autospy;
//!
//! #[autospy]
//! trait MyTrait {
//!     #[autospy(String)]
//!     type Item;
//!     fn foo(&self, argument: Self::Item);
//! }
//!
//! fn use_trait(x: &dyn MyTrait<Item=String>) {
//!     x.foo("hello!".to_string())
//! }
//!
//! let spy = MyTraitSpy::default();
//! spy.foo.returns.push_back(());
//!
//! use_trait(&spy.clone());
//!
//! assert_eq!(vec!["hello!"], spy.foo.arguments.take_all());
//! ```
//!
//! ## Ignore arguments
//!
//! Arguments can be ignored using `#[autospy(ignore)]` if you do not wish to capture them in the spy.
//!
//! ```rust
//! use autospy::autospy;
//!
//! #[autospy]
//! trait MyTrait {
//!     fn foo(&self, #[autospy(ignore)] ignored: &str, argument: &str);
//! }
//!
//! fn use_trait(x: &dyn MyTrait) {
//!     x.foo("ignored!", "capture me!")
//! }
//!
//! let spy = MyTraitSpy::default();
//! spy.foo.returns.push_back(());
//!
//! use_trait(&spy.clone());
//!
//! assert_eq!(vec!["capture me!"], spy.foo.arguments.take_all());
//! ```
mod spy_function;

pub use spy_function::SpyFunction;

pub use autospy_macro::autospy;
