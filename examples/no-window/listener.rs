use saunter::listener::Listener;

use crate::tick::NoWindowTick;

pub struct NoWindowListener {
    pub val: f32,
}
impl Listener for NoWindowListener {
    type TickType = NoWindowTick;

    type EventType = ();

    fn tick(
        &mut self,
        dt: f32,
        events: &mut Vec<saunter::event::Event<Self::EventType>>,
        time: std::time::Instant,
    ) -> Result<Self::TickType, saunter::error::SaunterError> {
        todo!()
    }
}