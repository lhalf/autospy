use std::sync::MutexGuard;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

pub struct SpyFunction<A, R> {
    /// The captured arguments the function was called with.
    pub arguments: Arguments<A>,
    /// The return values of the function.
    pub returns: Returns<A, R>,
    name: &'static str,
}

impl<A, R> Clone for SpyFunction<A, R> {
    fn clone(&self) -> Self {
        Self {
            arguments: self.arguments.clone(),
            returns: self.returns.clone(),
            name: self.name,
        }
    }
}

impl<A, R> From<&'static str> for SpyFunction<A, R> {
    fn from(name: &'static str) -> Self {
        Self {
            arguments: Arguments::default(),
            returns: Returns::default(),
            name,
        }
    }
}

impl<A, R> Drop for SpyFunction<A, R> {
    fn drop(&mut self) {
        if !std::thread::panicking()
            && self.returns.is_last_reference()
            && self.returns.queue_len() != 0
        {
            panic!(
                "function '{}' had {} unused return values when dropped",
                self.name,
                self.returns.queue_len()
            )
        }
    }
}

impl<A, R> SpyFunction<A, R> {
    /// Captures the arguments into [`arguments`](Self::arguments) and tries to return the next value from [`returns`](Self::returns).
    /// <div class="warning">
    /// Panics if not enough return values have been set for the number of times the function is called.
    /// </div>
    #[track_caller]
    pub fn spy(&self, arguments: A) -> R {
        let return_value = self.returns.next(&arguments);

        self.arguments.push(arguments);

        return_value.unwrap_or_else(|_| {
            let set_count = self.returns.set_count.load(Ordering::Relaxed);
            panic!(
                "function '{}' had {} return values set, but was called {} time(s)",
                self.name,
                set_count,
                set_count.saturating_add(1)
            )
        })
    }
}

///
/// Arguments implements [`PartialEq`] for `[A]`, `&[A]` and `Vec<A>`.
///
/// # Examples
/// ```rust
/// #[autospy::autospy]
/// trait MyTrait {
///     fn foo(&self, bar: u8);
/// }
///
/// fn use_trait(trait_object: impl MyTrait) {
///     trait_object.foo(10)
/// }
///
/// let spy = MyTraitSpy::default();
/// spy.foo.returns.set([()]);
///
/// use_trait(spy.clone());
///
/// // all valid PartialEq implementations
/// assert_eq!([10], spy.foo.arguments);
/// assert_eq!([10].as_slice(), spy.foo.arguments);
/// assert_eq!(vec![10], spy.foo.arguments);
/// ```
#[derive(Debug)]
pub struct Arguments<A> {
    captured: Arc<Mutex<Vec<A>>>,
    #[cfg(feature = "async")]
    sender: async_channel::Sender<()>,
    #[cfg(feature = "async")]
    receiver: async_channel::Receiver<()>,
}

impl<A> Clone for Arguments<A> {
    fn clone(&self) -> Self {
        Self {
            captured: self.captured.clone(),
            #[cfg(feature = "async")]
            sender: self.sender.clone(),
            #[cfg(feature = "async")]
            receiver: self.receiver.clone(),
        }
    }
}

impl<A> Default for Arguments<A> {
    fn default() -> Self {
        #[cfg(feature = "async")]
        let (sender, receiver) = async_channel::unbounded();
        Self {
            captured: Arc::new(Mutex::new(Vec::new())),
            #[cfg(feature = "async")]
            sender,
            #[cfg(feature = "async")]
            receiver,
        }
    }
}

impl<A, B: PartialEq<A>> PartialEq<[B]> for Arguments<A> {
    fn eq(&self, other: &[B]) -> bool {
        other == self.get().as_slice()
    }
}

impl<A: PartialEq<B>, B> PartialEq<Arguments<A>> for [B] {
    fn eq(&self, other: &Arguments<A>) -> bool {
        other.get().as_slice() == self
    }
}

impl<A, B: PartialEq<A>, const N: usize> PartialEq<[B; N]> for Arguments<A> {
    fn eq(&self, other: &[B; N]) -> bool {
        other == self.get().as_slice()
    }
}

impl<A: PartialEq<B>, B, const N: usize> PartialEq<Arguments<A>> for [B; N] {
    fn eq(&self, other: &Arguments<A>) -> bool {
        other.get().as_slice() == self
    }
}

impl<A, B: PartialEq<A>, const N: usize> PartialEq<&[B; N]> for Arguments<A> {
    fn eq(&self, other: &&[B; N]) -> bool {
        *other == self.get().as_slice()
    }
}

impl<A: PartialEq<B>, B, const N: usize> PartialEq<Arguments<A>> for &[B; N] {
    fn eq(&self, other: &Arguments<A>) -> bool {
        other.get().as_slice() == *self
    }
}

impl<A, B: PartialEq<A>> PartialEq<&[B]> for Arguments<A> {
    fn eq(&self, other: &&[B]) -> bool {
        *other == self.get().as_slice()
    }
}

impl<A: PartialEq<B>, B> PartialEq<Arguments<A>> for &[B] {
    fn eq(&self, other: &Arguments<A>) -> bool {
        other.get().as_slice() == *self
    }
}

impl<A: PartialEq<B>, B> PartialEq<Vec<B>> for Arguments<A> {
    fn eq(&self, other: &Vec<B>) -> bool {
        *self.get() == *other
    }
}

impl<A, B: PartialEq<A>> PartialEq<Arguments<A>> for Vec<B> {
    fn eq(&self, other: &Arguments<A>) -> bool {
        *self == *other.get()
    }
}

impl<A> Arguments<A> {
    fn push(&self, arguments: A) {
        self.captured
            .lock()
            .expect("mutex poisoned")
            .push(arguments);
        #[cfg(feature = "async")]
        let _ = self.sender.send_blocking(());
    }

    /// Gets the captured arguments. This returns a [`MutexGuard`] which must be dereferenced.
    ///
    /// # Examples
    /// ```rust
    /// #[autospy::autospy]
    /// trait MyTrait {
    ///     fn foo(&self, bar: u8);
    /// }
    ///
    /// fn use_trait(trait_object: impl MyTrait) {
    ///     trait_object.foo(10)
    /// }
    ///
    /// let spy = MyTraitSpy::default();
    /// spy.foo.returns.set([()]);
    ///
    /// use_trait(spy.clone());
    ///
    /// // get will not clear the captured spy arguments
    /// assert_eq!(vec![10], *spy.foo.arguments.get());
    /// assert_eq!(vec![10], *spy.foo.arguments.get());
    /// ```
    pub fn get(&self) -> MutexGuard<'_, Vec<A>> {
        self.captured.lock().expect("mutex poisoned")
    }

    /// Takes the captured arguments.
    ///
    /// # Examples
    /// ```rust
    /// #[autospy::autospy]
    /// trait MyTrait {
    ///     fn foo(&self, bar: u8);
    /// }
    ///
    /// fn use_trait(trait_object: impl MyTrait) {
    ///     trait_object.foo(10)
    /// }
    ///
    /// let spy = MyTraitSpy::default();
    /// spy.foo.returns.set([()]);
    ///
    /// use_trait(spy.clone());
    ///
    /// // take will clear the captured spy arguments
    /// assert_eq!(vec![10], spy.foo.arguments.take());
    /// assert!(spy.foo.arguments.take().is_empty());
    /// ```
    pub fn take(&self) -> Vec<A> {
        std::mem::take(&mut *self.captured.lock().expect("mutex poisoned"))
    }

    /// Asynchronously takes all captured arguments when the spy is used.
    /// Enabled by default via the **async** feature.
    ///
    /// # Examples
    /// ```rust
    /// use std::time::Duration;
    ///
    /// #[autospy::autospy]
    /// #[async_trait::async_trait]
    /// trait MyTrait: Send + 'static {
    ///     async fn foo(&self, argument: &str);
    /// }
    ///
    /// async fn use_async_trait(x: impl MyTrait) {
    ///     tokio::task::spawn(async move {
    ///         tokio::time::sleep(Duration::from_millis(100)).await;
    ///         x.foo("async used after some time!").await;
    ///     });
    /// }
    ///
    /// tokio::runtime::Runtime::new().unwrap().block_on(async {
    ///     let spy = MyTraitSpy::default();
    ///     spy.foo.returns.set([()]);
    ///
    ///     use_async_trait(spy.clone()).await;
    ///     // spy not used yet
    ///     assert!(spy.foo.arguments.take().is_empty());
    ///     // spy used after 100ms
    ///     assert_eq!("async used after some time!", spy.foo.arguments.recv().await[0])
    /// })
    /// ```
    #[cfg(feature = "async")]
    pub async fn recv(&self) -> Vec<A> {
        self.receiver.recv().await.unwrap();
        self.take()
    }
}

/// # Panics
/// Panics if not enough return values have been set for the number of times the function is called.
/// ```should_panic
/// #[autospy::autospy]
/// trait MyTrait {
///     fn foo(&self);
/// }
///
/// let spy = MyTraitSpy::default();
///
/// spy.foo()  // panics because we haven't set a return value
/// ```
///
/// Panics if too many return values have been set for the number of times the function is called.
/// ```should_panic
/// #[autospy::autospy]
/// trait MyTrait {
///     fn foo(&self);
/// }
///
/// let spy = MyTraitSpy::default();
///
/// spy.foo.returns.set([(), ()]);
///
/// spy.foo()
/// // panics because the spy is dropped with unused return values
/// ```
///
/// Will never panic if a return function is set.
/// ```
/// #[autospy::autospy]
/// trait MyTrait {
///     fn foo(&self);
/// }
///
/// let spy = MyTraitSpy::default();
///
/// spy.foo.returns.set_fn(|_| ());
///
/// spy.foo() // will always return ()
/// ```
pub struct Returns<A, R> {
    queue: Arc<Mutex<ReturnQueue<A, R>>>,
    set_count: Arc<AtomicUsize>,
}

impl<A, R> Clone for Returns<A, R> {
    fn clone(&self) -> Self {
        Self {
            queue: Arc::clone(&self.queue),
            set_count: Arc::clone(&self.set_count),
        }
    }
}

impl<A, R> Default for Returns<A, R> {
    fn default() -> Self {
        Self {
            queue: Arc::new(Mutex::new(ReturnQueue::Finite(VecDeque::new()))),
            set_count: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl<A, R> Returns<A, R> {
    /// Set the spy return values.
    ///
    /// # Examples
    /// ```rust
    /// #[autospy::autospy]
    /// trait MyTrait {
    ///     fn foo(&self) -> u8;
    /// }
    ///
    /// let spy = MyTraitSpy::default();
    /// spy.foo.returns.set([0,1,2]);
    ///
    /// // return values are returned in order from left to right
    /// assert_eq!(0, spy.foo());
    /// assert_eq!(1, spy.foo());
    /// assert_eq!(2, spy.foo());
    /// ```
    pub fn set<I: IntoIterator<Item = R>>(&self, values: I) {
        let queue: ReturnQueue<_, _> = values.into_iter().collect();
        self.set_count.fetch_add(queue.len(), Ordering::Relaxed);
        *self.queue.lock().expect("mutex poisoned") = queue;
    }

    /// Set a return function for the spy that can use the function [arguments](Arguments). When set, the spy will always return using this function.
    ///
    /// # Examples
    /// ```rust
    /// #[autospy::autospy]
    /// trait MyTrait {
    ///     fn foo(&self, bar: &str) -> usize;
    /// }
    ///
    /// let spy = MyTraitSpy::default();
    /// spy.foo.returns.set_fn(|bar| bar.len());
    ///
    /// assert_eq!(3, spy.foo("baz"));
    /// ```
    pub fn set_fn(&self, getter: impl FnMut(&A) -> R + Send + 'static) {
        *self.queue.lock().expect("mutex poisoned") = ReturnQueue::Infinite(Box::new(getter));
    }

    fn next(&self, arguments: &A) -> Result<R, CalledTooManyTimesError> {
        self.queue.lock().expect("mutex poisoned").next(arguments)
    }

    fn is_last_reference(&mut self) -> bool {
        Arc::get_mut(&mut self.queue).is_some()
    }

    fn queue_len(&self) -> usize {
        self.queue.lock().expect("mutex poisoned").len()
    }
}

type GetReturn<A, R> = Box<dyn FnMut(&A) -> R + Send + 'static>;

enum ReturnQueue<A, R> {
    Finite(VecDeque<R>),
    Infinite(GetReturn<A, R>),
}

impl<A, R> FromIterator<R> for ReturnQueue<A, R> {
    fn from_iter<T: IntoIterator<Item = R>>(values: T) -> Self {
        Self::Finite(values.into_iter().collect())
    }
}

impl<A, R> ReturnQueue<A, R> {
    fn next(&mut self, arguments: &A) -> Result<R, CalledTooManyTimesError> {
        match self {
            Self::Finite(queue) => queue.pop_front().ok_or(CalledTooManyTimesError),
            Self::Infinite(getter) => Ok(getter(arguments)),
        }
    }

    fn len(&self) -> usize {
        match self {
            ReturnQueue::Finite(queue) => queue.len(),
            ReturnQueue::Infinite(_) => 0,
        }
    }
}

struct CalledTooManyTimesError;
