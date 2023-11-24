use saunter::listener::Listener;

use crate::tick::NoWindowTick;

pub struct NoWindowListener {
    pub val: f32,
}
impl Listener for NoWindowListener {
    type Tick = NoWindowTick;

    type Event = ();

    fn tick(
        &mut self,
        _dt: f32,
        _events: Vec<saunter::event::Event<Self::Event>>,
        time: std::time::Instant,
    ) -> Result<Self::Tick, saunter::error::SaunterError> {
        self.val = 1.0 - self.val;
        log::info!("ticked {}", self.val);

        Ok(NoWindowTick { val: self.val, time })
    }
}
