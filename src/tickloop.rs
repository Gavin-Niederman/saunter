use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

use winit::event::Event;

use crate::listener;

pub struct Loop<'a> {
    pub listener: Box<dyn listener::Listener>,
    pub tick_length: Duration,
    pub events: Vec<Event<'a, ()>>,
    reciever: Receiver<Event<'a, ()>>,
}

impl<'a> Loop<'a> {
    pub fn new(
        listener: Box<dyn listener::Listener>,
        tps: u32,
        reciever: Receiver<Event<'a, ()>>,
    ) -> Self {
        let tick_length = Duration::from_secs_f32(1.0 / tps as f32);
        Loop {
            listener,
            tick_length,
            events: Vec::new(),
            reciever,
        }
    }

    pub fn start(&mut self) {
        loop {
            self.events = self.reciever.try_iter().collect();
            for event in self.events.iter() {
                if let Event::WindowEvent { event, .. } = event {
                    if let winit::event::WindowEvent::CloseRequested = event {
                        break;
                    }
                }
            }
            self.listener
                .tick(self.tick_length.as_secs_f32(), &mut self.events);

            thread::sleep(self.tick_length);
        }
    }
}
