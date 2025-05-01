use std::sync::{Arc, Mutex};

pub struct SpyFunction<A> {
    pub arguments: Arguments<A>,
}

impl<A> Clone for SpyFunction<A> {
    fn clone(&self) -> Self {
        Self {
            arguments: self.arguments.clone(),
        }
    }
}

impl<A> Default for SpyFunction<A> {
    fn default() -> Self {
        Self {
            arguments: Arguments::default(),
        }
    }
}

impl<A> SpyFunction<A> {
    pub fn spy(&self, arguments: A) {
        self.arguments.push(arguments);
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
        self.0.lock().unwrap().push(arguments);
    }

    pub fn take_all(&self) -> Vec<A> {
        std::mem::take(&mut *self.0.lock().unwrap())
    }
}
