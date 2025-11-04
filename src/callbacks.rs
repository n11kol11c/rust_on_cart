pub type CallBack<T><dyn Fn(T) + Send + Sync>;

pub struct CallbackHandler<T> {
    callbacks: Vec<Callback<T>>,
}
