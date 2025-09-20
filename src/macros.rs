/// Allows the spy to use supertraits.
///
/// If using autospy as a dev dependency you **MUST** mark the supertrait macro as `#[cfg(test)]`.
///
/// # Examples
/// ```rust
/// use std::io::Read;
///
/// #[autospy::autospy]
/// trait MyTrait: Read {
///     fn foo(&self) -> u64;
///     autospy::supertrait! {
///         trait Read {
///             fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>;
///         }   
///     }
/// }
///
/// fn use_trait(mut x: impl MyTrait) -> (u64, std::io::Result<usize>) {
///     let mut buf = [];
///     (x.foo(), x.read(&mut buf))
/// }
///
/// let spy = MyTraitSpy::default();
/// spy.foo.returns.set([1]);
/// spy.read.returns.set([Ok(0)]);
///
/// let result =  use_trait(spy);
/// assert_eq!(1, result.0);
/// assert_eq!(0, result.1.unwrap())
/// ```
#[macro_export]
macro_rules! supertrait {
    ($($tt:tt)*) => {};
}
