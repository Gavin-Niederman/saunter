pub enum Event<T: Send> {
    Close,
    Other(T),
}
