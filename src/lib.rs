// vim: tw=80
//! A test spy object library.
//!
//! autospy is a macro to create spy versions of almost any trait.
//! They can be used in unit tests as a stand-in for the real object.
//!
//! # Usage
//!
//! To use autospy simply use the [`#[autospy]`](attr.autospy.html).
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
mod spy_function;

pub use spy_function::SpyFunction;

pub use autospy_macro::autospy;
