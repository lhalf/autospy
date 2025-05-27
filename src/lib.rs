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
//! **Note:** The spy object generated is [`#[cfg(test)]`](https://doc.rust-lang.org/book/ch11-03-test-organization.html#the-tests-module-and-cfgtest), so you do not need `#[cfg_attr(test, autospy)]`.
//! ```rust
//! use autospy::autospy;
//!
//! #[autospy]
//! trait MyTrait {
//!     fn foo(&self, argument: u32) -> u32;
//! }
//!
//! fn call_with_ten(x: impl MyTrait) -> u32 {
//!     x.foo(10)
//! }
//!
//! #[cfg(test)]
//! mod tests {
//!     use super::*;
//!
//!     #[test]
//!     fn test() {
//!         let spy = MyTraitSpy::default();
//!         spy.foo.returns.push_back(20);
//!
//!         assert_eq!(20, call_with_ten(spy.clone()));
//!         assert_eq!(vec![10], spy.foo.arguments.take_all());
//!     }
//! }
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
//! fn use_trait(x: impl MyTrait) {
//!     x.foo("hello!")
//! }
//!
//! #[cfg(test)]
//! mod tests {
//!     use super::*;
//!
//!     #[test]
//!     fn test() {
//!         let spy = MyTraitSpy::default();
//!         spy.foo.returns.push_back(());
//!
//!         use_trait(spy.clone());
//!
//!         assert_eq!(vec!["hello!"], spy.foo.arguments.take_all());
//!     }
//! }
//! ```
//!
//! ## Associated types
//!
//! An `#[autospy(TYPE)]` attribute can be applied to associated types to tell the spy how to capture them.
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
//! fn use_trait(x: impl MyTrait<Item=String>) {
//!     x.foo("hello!".to_string())
//! }
//!
//! #[cfg(test)]
//! mod tests {
//!     use super::*;
//!
//!     #[test]
//!     fn test() {
//!         let spy = MyTraitSpy::default();
//!         spy.foo.returns.push_back(());
//!
//!         use_trait(spy.clone());
//!
//!         assert_eq!(vec!["hello!"], spy.foo.arguments.take_all());
//!     }
//! }
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
//! fn use_trait(x: impl MyTrait) {
//!     x.foo("ignored!", "capture me!")
//! }
//!
//! #[cfg(test)]
//! mod tests {
//!     use super::*;
//!
//!     #[test]
//!     fn test() {
//!         let spy = MyTraitSpy::default();
//!         spy.foo.returns.push_back(());
//!
//!         use_trait(spy.clone());
//!
//!         assert_eq!(vec!["capture me!"], spy.foo.arguments.take_all());
//!     }
//! }
//! ```
//!
//! ## Returns attribute
//!
//! Trait functions that return generics can have the type specified using the `#[autospy(returns = "TYPE")]` attribute.
//!
//! ```rust
//! use autospy::autospy;
//!
//! #[autospy]
//! trait MyTrait {
//!     #[autospy(returns = "String")]
//!     fn foo(&self) -> impl ToString;
//! }
//!
//! fn use_trait(x: impl MyTrait) -> String {
//!     x.foo().to_string()
//! }
//!
//! #[cfg(test)]
//! mod tests {
//!     use super::*;
//!
//!     #[test]
//!     fn test() {
//!         let spy = MyTraitSpy::default();
//!         spy.foo.returns.push_back("a string!".to_string());
//!
//!         assert_eq!("a string!", use_trait(spy));
//!     }
//! }
//! ```
//!
//! ## Static trait arguments
//!
//! Trait functions that have generic arguments and are [`'static`](https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html) will automatically be captured in a [`Box`](https://doc.rust-lang.org/std/boxed/struct.Box.html).
//!
//! ```rust
//! use autospy::autospy;
//!
//! #[autospy]
//! trait MyTrait {
//!     fn foo(&self, argument: impl ToString + 'static);
//! }
//!
//! fn use_trait(x: impl MyTrait) {
//!     x.foo("hello!")
//! }
//!
//! #[cfg(test)]
//! mod tests {
//!     use super::*;
//!
//!     #[test]
//!     fn test() {
//!         let spy = MyTraitSpy::default();
//!         spy.foo.returns.push_back(());
//!
//!         use_trait(spy.clone());
//!
//!         assert_eq!("hello!", spy.foo.arguments.take_all()[0].to_string())
//!     }
//! }
//! ```

mod spy_function;

pub use spy_function::SpyFunction;

pub use autospy_macro::autospy;
