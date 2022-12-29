use winit::event::Event;
use crate::error::SaunterError;

pub trait Listener: Send + Sync {
    type TickType;
    fn tick(&mut self, dt: f32, events: &mut Vec<Event<'_, ()>>) -> Result<Self::TickType, SaunterError>;
}
