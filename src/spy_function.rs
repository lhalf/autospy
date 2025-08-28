use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

pub struct SpyFunction<A, R> {
    /// The captured arguments the function was called with.
    pub arguments: Arguments<A>,
    /// The return values of the function.
    pub returns: Returns<R>,
}

impl<A, R> Clone for SpyFunction<A, R> {
    fn clone(&self) -> Self {
        Self {
            arguments: self.arguments.clone(),
            returns: self.returns.clone(),
        }
    }
}

impl<A, R> Default for SpyFunction<A, R> {
    fn default() -> Self {
        Self {
            arguments: Arguments::default(),
            returns: Returns::default(),
        }
    }
}

impl<A, R> SpyFunction<A, R> {
    /// Captures the arguments into [`arguments`](Self::arguments) and tries to return the next value from [`returns`](Self::returns).
    /// Will panic if not enough return values have been specified for the number of times the spy is called.
    pub fn spy(&self, arguments: A) -> R {
        self.arguments.push(arguments);
        self.returns
            .next()
            .expect("spy function called more times than expected")
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

    /// Returns all captured arguments from the spy function.
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
    /// Adds a single return value to the back of the queue for the spy function.
    pub fn push_back(&self, value: R) -> &Self {
        self.0.lock().expect("mutex poisoned").push_back(value);
        self
    }

    fn next(&self) -> Option<R> {
        self.0.lock().expect("mutex poisoned").pop_front()
    }

    /// Clears the configured return values for the spy function.
    pub fn clear(&self) {
        self.0.lock().expect("mutex poisoned").clear()
    }
}

impl<R: Clone> Returns<R> {
    /// Adds the specified number of return values to the back of the queue for the spy function.
    pub fn push_back_n(&self, value: R, count: usize) -> &Self {
        std::iter::repeat_n(value, count).fold(self, |acc, value| acc.push_back(value))
    }
}
