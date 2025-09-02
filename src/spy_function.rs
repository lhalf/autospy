use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

pub struct SpyFunction<A, R> {
    /// The captured arguments the function was called with.
    pub arguments: Arguments<A>,
    /// The return values of the function.
    pub returns: Returns<R>,
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
        if !std::thread::panicking() && self.returns.next().is_some() {
            panic!(
                "function '{}' had unused return values when dropped",
                self.name
            )
        }
    }
}

impl<A, R> SpyFunction<A, R> {
    /// Captures the arguments into [`arguments`](Self::arguments) and tries to return the next value from [`returns`](Self::returns).
    /// <div class="warning">
    /// Panics if not enough return values have been specified for the number of times the function is called.
    /// </div>
    #[track_caller]
    pub fn spy(&self, arguments: A) -> R {
        self.arguments.push(arguments);
        match self.returns.next() {
            Some(return_value) => return_value,
            None => {
                let called_count = self.arguments.take_all().len();
                panic!(
                    "function '{}' had {} return values specified, but was called {} time(s)",
                    self.name,
                    called_count - 1,
                    called_count
                )
            }
        }
    }
}

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

impl<A> Arguments<A> {
    fn push(&self, arguments: A) {
        self.captured
            .lock()
            .expect("mutex poisoned")
            .push(arguments);
        #[cfg(feature = "async")]
        let _ = self.sender.send_blocking(());
    }

    /// Returns all captured arguments.
    pub fn take_all(&self) -> Vec<A> {
        std::mem::take(&mut *self.captured.lock().expect("mutex poisoned"))
    }

    /// Asynchronously returns all captured arguments when the spy is used.
    /// Enabled by default via the **async** feature.
    #[cfg(feature = "async")]
    pub async fn recv(&self) -> Vec<A> {
        self.receiver.recv().await.unwrap();
        self.take_all()
    }
}

/// # Panics
/// Panics if not enough return values have been specified for the number of times the function is called.
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
/// Panics if too many return values have been specified for the number of times the function is called.
/// ```should_panic
/// #[autospy::autospy]
/// trait MyTrait {
///     fn foo(&self);
/// }
///
/// let spy = MyTraitSpy::default();
///
/// spy.foo.returns.extend([(), ()]);
///
/// spy.foo()
/// // panics because the spy is dropped with unused return values
/// ```
pub struct Returns<R>(Arc<Mutex<VecDeque<R>>>);

impl<R> Clone for Returns<R> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<R> Default for Returns<R> {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(VecDeque::new())))
    }
}

impl<R> Returns<R> {
    /// Appends a value to the back of the spy return values.
    ///
    /// # Examples
    /// ```rust
    /// #[autospy::autospy]
    /// trait MyTrait {
    ///     fn foo(&self) -> u8;
    /// }
    ///
    /// let spy = MyTraitSpy::default();
    /// spy.foo.returns.push_back(42);
    ///
    /// assert_eq!(42, spy.foo())
    /// ```
    pub fn push_back(&self, value: R) -> &Self {
        self.0.lock().expect("mutex poisoned").push_back(value);
        self
    }

    /// Extends the spy return values with the specified values.
    ///
    /// # Examples
    /// ```rust
    /// #[autospy::autospy]
    /// trait MyTrait {
    ///     fn foo(&self) -> u8;
    /// }
    ///
    /// let spy = MyTraitSpy::default();
    /// spy.foo.returns.push_back(0);
    /// spy.foo.returns.extend([1,2]);
    ///
    /// assert_eq!(0, spy.foo());
    /// assert_eq!(1, spy.foo());
    /// assert_eq!(2, spy.foo());
    /// ```
    pub fn extend<I: IntoIterator<Item = R>>(&self, values: I) {
        self.0.lock().expect("mutex poisoned").extend(values);
    }

    /// Clears the spy return values.
    ///
    /// # Examples
    /// ```rust
    /// #[autospy::autospy]
    /// trait MyTrait {
    ///     fn foo(&self) -> u8;
    /// }
    ///
    /// let spy = MyTraitSpy::default();
    /// spy.foo.returns.push_back(42);
    /// spy.foo.returns.clear();
    /// spy.foo.returns.push_back(0);
    ///
    /// assert_eq!(0, spy.foo())
    /// ```
    pub fn clear(&self) {
        self.0.lock().expect("mutex poisoned").clear()
    }

    fn next(&self) -> Option<R> {
        self.0.lock().expect("mutex poisoned").pop_front()
    }
}

impl<R: Clone> Returns<R> {
    /// Adds the specified number of return values to the back of the queue for the spy function.
    ///
    /// # Examples
    /// ```rust
    /// #[autospy::autospy]
    /// trait MyTrait {
    ///     fn foo(&self) -> String;
    /// }
    ///
    /// let spy = MyTraitSpy::default();
    /// spy.foo.returns.push_back_n("ho".to_string(), 3);
    ///
    /// assert_eq!("ho", spy.foo());
    /// assert_eq!("ho", spy.foo());
    /// assert_eq!("ho", spy.foo());
    /// ```
    pub fn push_back_n(&self, value: R, count: usize) -> &Self {
        std::iter::repeat_n(value, count).fold(self, |acc, value| acc.push_back(value))
    }
}
