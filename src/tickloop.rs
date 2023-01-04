use log;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use winit::event::Event;

use crate::listener;
use crate::tick::{Tick, Ticks};

pub struct Loop<'a, T: Tick> {
    pub listener: Box<dyn listener::Listener<TickType = T>>,
    pub tick_length: Duration,
    pub tps: u32,
    pub events: Vec<Event<'a, ()>>,
    reciever: Receiver<Event<'a, ()>>,
}

impl<'a, T: Tick> Loop<'a, T> {
    pub fn new(
        listener: Box<dyn listener::Listener<TickType = T>>,
        tps: u32,
        reciever: Receiver<Event<'a, ()>>,
    ) -> Self {
        let tick_length = Duration::from_secs_f32(1.0 / tps as f32);
        Loop {
            listener,
            tick_length,
            tps,
            events: Vec::new(),
            reciever,
        }
    }

    pub fn start(&mut self, ticks: Arc<RwLock<Ticks<T>>>) {
        let mut loop_helper = spin_sleep::LoopHelper::builder()
            .report_interval_s(0.25)
            .build_with_target_rate(self.tps as f64);
        loop {
            let tick_time = std::time::Instant::now();
            loop_helper.loop_start();


            self.events = self.reciever.try_iter().collect();
            for event in self.events.iter() {
                if let Event::WindowEvent { event, .. } = event {
                    if let winit::event::WindowEvent::CloseRequested = event {
                        break;
                    }
                }
            }
            if let Ok(tick) =
                self.listener
                    .tick(self.tick_length.as_secs_f32(), &mut self.events, tick_time)
            {
                let mut tick_wlock = ticks.write().unwrap();
                log::debug!("lock aquired {:?}", std::time::Instant::now());
                (*tick_wlock).update(tick);
                // Drop the write lock so the read lock can be acquired.
            }
            log::debug!("lock dropped {:?}", std::time::Instant::now());

            loop_helper.loop_sleep();

            log::debug!("actual tick length {:?}", tick_time.elapsed());
        }
    }
}
