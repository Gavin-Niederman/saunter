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
        _dt: f32,
        _events: &mut Vec<saunter::event::Event<Self::EventType>>,
        time: std::time::Instant,
    ) -> Result<Self::TickType, saunter::error::SaunterError> {
        self.val = 1.0 - self.val;
        log::info!("{}", self.val);

        Ok(NoWindowTick { val: self.val, time })
    }
}
