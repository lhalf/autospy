//! A test spy object library.
//!
//! [`#[autospy]`](attr.autospy.html) is a macro to create a spy from almost any trait.
//! The spy can be used in unit tests as a stand-in for the real trait.
//!
//! # Usage
//!
//! - Attribute your trait using `#[autospy]`
//! - Set return values using [`set()`](Returns::set)
//! - Get captured arguments using [`get()`](Arguments::get)
//!
//! ```rust
//! #[autospy::autospy]
//! trait MyTrait {
//!     fn foo(&self, argument: u32) -> u32;
//! }
//!
//! fn call_with_ten(x: impl MyTrait) -> u32 {
//!     x.foo(10)
//! }
//!
//! let spy = MyTraitSpy::default();
//! spy.foo.returns.set([20]);
//!
//! assert_eq!(20, call_with_ten(spy.clone()));
//! assert_eq!(vec![10], spy.foo.arguments.get());
//! ```
//!
//! <div class="warning">
//!   The generated spy object and trait impl are
//!   <a href="https://doc.rust-lang.org/book/ch11-03-test-organization.html#the-tests-module-and-cfgtest"><code>#[cfg(test)]</code></a>
//!   by default.
//!   To disable this see <a href="#features">features</a>.
//!
//!   It is recommended to use
//!   <code>#[cfg_attr(test, autospy)]</code>, as well for all attributes discussed below,
//!   to make it transparent autospy is only expanded under test.
//! </div>
//!
//! ## Multiple arguments
//!
//! Methods with multiple arguments are captured in a tuple.
//!
//! ```rust
//! #[autospy::autospy]
//! trait MyTrait {
//!     fn foo(&self, arg1: u32, arg2: String);
//! }
//!
//! fn use_trait(x: impl MyTrait)  {
//!     x.foo(10, "hello!".to_string())
//! }
//!
//! let spy = MyTraitSpy::default();
//! spy.foo.returns.set([()]);
//!
//! use_trait(spy.clone());
//!
//! assert_eq!(vec![(10, "hello!".to_string())], spy.foo.arguments.get());
//! ```
//!
//! ## Reference arguments
//!
//! `#[autospy]` will automatically convert reference arguments into owned types when captured.
//!
//! ```rust
//! #[autospy::autospy]
//! trait MyTrait {
//!     fn foo(&self, argument: &str);
//! }
//!
//! fn use_trait(x: impl MyTrait) {
//!     x.foo("hello!")
//! }
//!
//! let spy = MyTraitSpy::default();
//! spy.foo.returns.set([()]);
//!
//! use_trait(spy.clone());
//!
//! assert_eq!(vec!["hello!"], spy.foo.arguments.get());
//! ```
//!
//! ## Ignore arguments
//!
//! Arguments can be ignored using `#[autospy(ignore)]` if you do not wish to capture them in the spy.
//!
//! ```rust
//! #[autospy::autospy]
//! trait MyTrait {
//!     fn foo(&self, #[autospy(ignore)] ignored: &str, argument: &str);
//! }
//!
//! fn use_trait(x: impl MyTrait) {
//!     x.foo("ignored!", "capture me!")
//! }
//!
//! let spy = MyTraitSpy::default();
//! spy.foo.returns.set([()]);
//!
//! use_trait(spy.clone());
//!
//! assert_eq!(vec!["capture me!"], spy.foo.arguments.get());
//! ```
//!
//! Arguments called `_` will also be implicitly ignored.
//!
//! ```rust
//! #[autospy::autospy]
//! trait MyTrait {
//!     fn foo(&self, _: &str, argument: &str);
//! }
//!
//! fn use_trait(x: impl MyTrait) {
//!     x.foo("ignored!", "capture me!")
//! }
//!
//! let spy = MyTraitSpy::default();
//! spy.foo.returns.set([()]);
//!
//! use_trait(spy.clone());
//!
//! assert_eq!(vec!["capture me!"], spy.foo.arguments.get());
//! ```
//!
//! ## Associated types
//!
//! An `#[autospy(TYPE)]` attribute can be applied to associated types to tell the spy how to capture them.
//!
//! ```rust
//! #[autospy::autospy]
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
//! let spy = MyTraitSpy::default();
//! spy.foo.returns.set([()]);
//!
//! use_trait(spy.clone());
//!
//! assert_eq!(vec!["hello!"], spy.foo.arguments.get());
//! ```
//!
//! ## External traits
//!
//! External traits can be turned into a spy using `#[autospy(external)]`, you will need to include the signatures for the external trait functions you want the spy to implement.
//!
//! ```rust
//! use std::io::Read;
//!
//! #[autospy::autospy(external)]
//! trait Read {
//!     fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>;
//! }
//!
//! fn use_trait(mut x: impl Read) -> std::io::Result<usize> {
//!     let mut buf = [];
//!     x.read(&mut buf)
//! }
//!
//! let spy = ReadSpy::default();
//! spy.read.returns.set([Err(std::io::Error::other("read fails!"))]);
//!
//! assert!(use_trait(spy).is_err());
//! ```
//!
//! ## Returns attribute
//!
//! Trait functions that return generics can have the return type specified using the `#[autospy(returns = "TYPE")]` attribute.
//!
//! ```rust
//! #[autospy::autospy]
//! trait MyTrait {
//!     #[autospy(returns = "String")]
//!     fn foo(&self) -> impl ToString;
//! }
//!
//! fn use_trait(x: impl MyTrait) -> String {
//!     x.foo().to_string()
//! }
//!
//! let spy = MyTraitSpy::default();
//! spy.foo.returns.set(["a string!".to_string()]);
//!
//! assert_eq!("a string!", use_trait(spy));
//! ```
//!
//! ## Static trait arguments
//!
//! Trait functions that have generic arguments and are [`'static`](https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html) will automatically be captured in a [`Box`].
//!
//! ```rust
//! #[autospy::autospy]
//! trait MyTrait {
//!     fn foo(&self, argument: impl ToString + 'static);
//! }
//!
//! fn use_trait(x: impl MyTrait) {
//!     x.foo("hello!")
//! }
//!
//! let spy = MyTraitSpy::default();
//! spy.foo.returns.set([()]);
//!
//! use_trait(spy.clone());
//!
//! assert_eq!("hello!", spy.foo.arguments.get()[0].to_string())
//! ```
//!
//! ## Generic traits
//!
//! The spy will have the same generics as the trait definition.
//!
//! ```rust
//! #[autospy::autospy]
//! trait MyTrait<A: Copy, R> {
//!     fn foo(&self, argument: A) -> R;
//! }
//!
//! fn use_trait(x: impl MyTrait<u32, String>) -> String {
//!     x.foo(10)
//! }
//!
//! let spy = MyTraitSpy::<u32, String>::default();
//! spy.foo.returns.set(["hello!".to_string()]);
//!
//! assert_eq!("hello!", use_trait(spy.clone()));
//!
//! assert_eq!(vec![10], spy.foo.arguments.get())
//! ```
//!
//! ## Async traits
//!
//! Async functions in traits are stable as of [Rust 1.75](https://blog.rust-lang.org/2023/12/28/Rust-1.75.0/); however, this did not include support for using traits containing async functions as `dyn Trait`. They can be used via the [`async_trait`](https://docs.rs/async-trait/latest/async_trait/) crate. `#[autospy]` is compatible with the `#[async_trait]` macro.
//! <div class="warning">
//! <code>#[autospy]</code> must come before <code>#[async_trait]</code>.
//! </div>
//!
//! ```rust
//! use pollster::FutureExt as _;
//!
//! #[autospy::autospy]
//! #[async_trait::async_trait]
//! trait MyTrait {
//!     async fn foo(&self, argument: &str);
//! }
//!
//! async fn use_async_trait(x: impl MyTrait) {
//!     x.foo("hello async!").await
//! }
//!
//! let spy = MyTraitSpy::default();
//! spy.foo.returns.set([()]);
//!
//! use_async_trait(spy.clone()).block_on();
//!
//! assert_eq!("hello async!", spy.foo.arguments.get()[0])
//! ```
//!
//! If you are using an async trait your spy might not be used immediately, for instance it might be spawned in a task.
//! You can use the [`recv()`](Arguments::recv) method on arguments to instruct the spy to wait asynchronously until the spy is used.
//! `recv()` is enabled by the default feature [**async**](#features) and, as an `async` function, will need to be called from within an async test.
//!
//! ```rust
//! use std::time::Duration;
//!
//! #[autospy::autospy]
//! #[async_trait::async_trait]
//! trait MyTrait: Send + 'static {
//!     async fn foo(&self, argument: &str);
//! }
//!
//! async fn use_async_trait(x: impl MyTrait) {
//!     tokio::task::spawn(async move {
//!         tokio::time::sleep(Duration::from_millis(100)).await;
//!         x.foo("async used after some time!").await;
//!     });
//! }
//!
//! tokio::runtime::Runtime::new().unwrap().block_on(async {
//!     let spy = MyTraitSpy::default();
//!     spy.foo.returns.set([()]);
//!
//!     use_async_trait(spy.clone()).await;
//!     // spy not used yet
//!     assert!(spy.foo.arguments.get().is_empty());
//!     // spy used after 100ms
//!     assert_eq!("async used after some time!", spy.foo.arguments.recv().await[0])
//! })
//! ```
//!
//! ## Into attribute
//!
//! If you wish to capture an argument as a different type, and it implements [`From`] you can use the `#[autospy(into = "TYPE")]` attribute on the argument.
//!
//! ```rust
//! use std::net::Ipv4Addr;
//!
//! #[autospy::autospy]
//! trait MyTrait {
//!     fn foo(&self, #[autospy(into = "Ipv4Addr")] ip: [u8; 4]);
//! }
//!
//! fn use_trait(x: impl MyTrait) {
//!     x.foo([192, 168, 0, 1])
//! }
//!
//! let spy = MyTraitSpy::default();
//! spy.foo.returns.set([()]);
//!
//! use_trait(spy.clone());
//!
//! assert_eq!(vec![Ipv4Addr::new(192, 168, 0, 1)], spy.foo.arguments.get())
//! ```
//!
//! ## Into with attribute
//!
//! If you wish to capture an argument as a different type, and it doesn't implement [`From`] you can use the `#[autospy(into = "TYPE", with = "FUNCTION")]` attribute on the argument.
//!
//! ```rust
//! use std::string::FromUtf8Error;
//!
//! #[autospy::autospy]
//! trait MyTrait {
//!     fn foo(&self, #[autospy(into = "Result<String, FromUtf8Error>", with = "String::from_utf8")] bytes: Vec<u8>);
//! }
//!
//! fn use_trait(x: impl MyTrait) {
//!     x.foo(b"hello!".to_vec())
//! }
//!
//! let spy = MyTraitSpy::default();
//! spy.foo.returns.set([()]);
//!
//! use_trait(spy.clone());
//!
//! assert_eq!(vec![Ok("hello!".to_string())], spy.foo.arguments.get())
//! ```
//!
//! ## Associated consts
//!
//! An `#[autospy(VALUE)]` attribute can be applied to associated consts to set them in the spy. Alternatively, if no attribute is provided and the type has a [`Default`] that will be used.
//!
//! ```rust
//! #[autospy::autospy]
//! trait MyTrait {
//!     #[autospy(100)]
//!     const VALUE: u64;
//!     const DEFAULT: u64;
//!     fn foo(&self);
//! }
//!
//! assert_eq!(100, MyTraitSpy::VALUE);
//! assert_eq!(0, MyTraitSpy::DEFAULT);
//! ```
//!
//! ## Default trait implementations
//!
//! If your trait has a default implementation for a method, an `#[autospy(use_default)]` attribute can be used on the method to tell the spy to use the default. Therefore, no spy values will be recorded for this function.
//!
//! ```rust
//! #[autospy::autospy]
//! trait MyTrait {
//!     #[autospy(20)]
//!     const VALUE: u64;
//!     #[autospy(use_default)]
//!     fn foo(&self) -> u64 {
//!         Self::VALUE + 100
//!     }
//! }
//!
//! fn use_trait(x: impl MyTrait) -> u64 {
//!     x.foo()
//! }
//!
//! assert_eq!(120, use_trait(MyTraitSpy::default()));
//! ```
//!

//! # Examples
//!
//! For additional examples please see the [examples](https://github.com/lhalf/autospy/tree/main/examples).

//! # Features
//!
//! - **test** - makes the generated spy object and trait impl `#[cfg(test)]` - enabled by default.
//! - **async** - enables additional async support features on the spy, if you are not using async traits you can safely disable this - enabled by default.

mod spy_function;

/// The captured arguments of a spy function.
#[allow(unused_imports)]
pub use spy_function::Arguments;
/// The return values of a spy function.
#[allow(unused_imports)]
pub use spy_function::Returns;
/// Captures arguments and holds return values.
pub use spy_function::SpyFunction;

/// Automatically generate spy objects for traits.
///
/// For more details, see [usage](crate#usage).
pub use autospy_macro::autospy;
