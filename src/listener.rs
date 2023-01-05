use std::time::Instant;

use crate::{error::SaunterError, event::Event, tick::Tick};

pub trait Listener: Send + Sync {
    type TickType: Tick;
    type EventType: Send;
    fn tick(
        &mut self,
        dt: f32,
        events: &mut Vec<Event<Self::EventType>>,
        time: Instant,
    ) -> Result<Self::TickType, SaunterError>;
}
