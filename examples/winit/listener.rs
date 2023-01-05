use saunter::error::SaunterError;
use saunter::listener::Listener;
use std::time::Instant;

use crate::tick::PrintTick;
pub struct PrinterListener {
    pub val: f32,
}
impl Listener for PrinterListener {
    fn tick(
        &mut self,
        _dt: f32,
        events: &mut Vec<saunter::event::Event<winit::event::Event<'static, ()>>>,
        time: Instant,
    ) -> Result<PrintTick, SaunterError> {
        self.val = 1.0 - self.val;

        for event in events {
            if let saunter::event::Event::Other(event) = event {
                if let winit::event::Event::WindowEvent { event, .. } = event {
                    log::info!("Tick {:?}", event);
                }
            }
        }

        Ok(PrintTick::new(time, self.val))
    }

    type TickType = PrintTick;
    type EventType = winit::event::Event<'static, ()>;
}
