use saunter::error::SaunterError;
use saunter::listener::Listener;
use std::time::Instant;

use crate::tick::WinitTick;
pub struct WinitListener {
    pub val: f32,
}
impl Listener for WinitListener {
    fn tick(
        &mut self,
        _dt: f32,
        events: &mut Vec<saunter::event::Event<winit::event::Event<'static, ()>>>,
        time: Instant,
    ) -> Result<WinitTick, SaunterError> {
        self.val = 1.0 - self.val;

        for event in events {
            if let saunter::event::Event::Other(event) = event {
                if let winit::event::Event::WindowEvent { event, .. } = event {
                    log::info!("Tick {:?}", event);
                }
            }
        }

        Ok(WinitTick::new(time, self.val))
    }

    type TickType = WinitTick;
    type EventType = winit::event::Event<'static, ()>;
}
