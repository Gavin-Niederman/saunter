//! Listener trait

use std::time::Instant;

use crate::{error::SaunterError, event::Event, tick::Snapshot};

/// A Listener is a struct that implements this trait. It is passed to the loop and the loop calls the [`tick`](crate::listener::Listener::tick) function on it.
/// The Listener should either store the state of the game engine or the current scene in the game engine.
/// [`TickType`](crate::listener::Listener::TickType) is the type of the tick that is returned when tick is called on the listner.
/// [`EventType`](crate::listener::Listener::EventType) is the type of the event passed to the listener in the [`Event::Other(EventType)`](crate::event::Event::Other) variant.
pub trait Listener: Send {
    type Tick: Snapshot;
    type Event: Send + Clone;
    fn tick(
        &mut self,
        dt: f32,
        events: Vec<Event<Self::Event>>,
        time: Instant,
    ) -> Result<Self::Tick, SaunterError>;
}
