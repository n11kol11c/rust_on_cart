pub type Callback<T> = Box<dyn Fn(T) + Send + Sync>;

pub struct CallbackHandler<T> {
    callbacks: Vec<Callback<T>>,
}

impl<T> CallbackHandler<T> {
    pub fn new() -> Self {
        Self { callbacks: vec![] }
    }

    pub fn register(&mut self, callback: Callback<T>) {
        self.callbacks.push(callback);
    }

    pub fn trigger(&self, value: T)
    where
        T: Clone,
    {
        for cb in &self.callbacks {
            cb(value.clone());
        }
    }
}
