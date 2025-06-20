use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

pub struct SpyFunction<A, R> {
    /// The spied arguments of the function.
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

pub struct Arguments<A>(Arc<Mutex<Vec<A>>>);

impl<A> Clone for Arguments<A> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<A> Default for Arguments<A> {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Vec::new())))
    }
}

impl<A> Arguments<A> {
    pub fn push(&self, arguments: A) {
        self.0.lock().expect("mutex poisoned").push(arguments);
    }

    pub fn take_all(&self) -> Vec<A> {
        std::mem::take(&mut *self.0.lock().expect("mutex poisoned"))
    }

    #[cfg(feature = "async")]
    pub async fn take_all_with_timeout(&self, timeout: std::time::Duration) -> Result<Vec<A>, ()> {
        let retry_interval = std::time::Duration::from_millis(10);
        let attempts = (timeout.as_millis() / retry_interval.as_millis()) as usize;
        let strategy = tokio_retry2::strategy::FixedInterval::new(retry_interval).take(attempts);

        tokio_retry2::Retry::spawn(strategy, async || match self.take_all() {
            arguments if arguments.is_empty() => Err(tokio_retry2::RetryError::transient(())),
            arguments => Ok(arguments),
        })
        .await
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
    pub fn push_back(&self, value: R) -> &Self {
        self.0.lock().expect("mutex poisoned").push_back(value);
        self
    }

    pub fn next(&self) -> Option<R> {
        self.0.lock().expect("mutex poisoned").pop_front()
    }

    pub fn clear(&self) {
        self.0.lock().expect("mutex poisoned").clear()
    }
}

impl<R: Clone> Returns<R> {
    pub fn push_back_n(&self, value: R, count: usize) -> &Self {
        std::iter::repeat_n(value, count).fold(self, |acc, value| acc.push_back(value))
    }
}
