//! Event types.

/// An event that can be sent to a listener. T is the eventtype (E) that you give to the loop when you make it.
/// In future there will be more variants of this enum.
pub enum Event<T: Send> {
    Close,
    Other(T),
}
