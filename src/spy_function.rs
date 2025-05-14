use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

pub struct SpyFunction<A, R> {
    pub arguments: Arguments<A>,
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
    pub fn push_back(&self, value: R) {
        self.0.lock().expect("mutex poisoned").push_back(value);
    }

    pub fn next(&self) -> Option<R> {
        self.0.lock().expect("mutex poisoned").pop_front()
    }
}
