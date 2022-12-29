use crate::error::SaunterError;
use winit::event::Event;

pub trait Listener: Send + Sync {
    type TickType;
    fn tick(
        &mut self,
        dt: f32,
        events: &mut Vec<Event<'_, ()>>,
    ) -> Result<Self::TickType, SaunterError>;
}
