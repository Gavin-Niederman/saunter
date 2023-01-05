use log;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::time::Duration;

use crate::event::Event;
use crate::listener::{self, Listener};
use crate::tick::{Tick, Ticks};

pub struct Loop<T: Tick, E: Send> {
    pub listener: Box<dyn listener::Listener<TickType = T, EventType = E>>,
    pub tick_length: Duration,
    pub tps: f32,
    pub events: Vec<Event<E>>,
    reciever: Receiver<Event<E>>,
}

impl<'a, T: Tick, E: Send> Loop<T, E> {
    pub fn new(
        listener: Box<dyn listener::Listener<TickType = T, EventType = E>>,
        tps: f32,
        reciever: Receiver<Event<E>>,
    ) -> Self {
        let tick_length = Duration::from_secs_f32(1.0 / tps);
        Loop {
            listener,
            tick_length,
            tps,
            events: Vec::new(),
            reciever,
        }
    }

    pub fn init(
        listener: Box<dyn Listener<TickType = T, EventType = E>>,
        first_tick: T,
        tps: f32,
    ) -> (Self, Sender<Event<E>>, &'static mut Arc<RwLock<Ticks<T>>>) {
        let (event_sender, event_reciever) = mpsc::channel::<Event<E>>();
        let ticks = Box::leak(Box::new(Arc::new(RwLock::new(Ticks::new(first_tick)))));

        (
            Self::new(listener, tps, event_reciever),
            event_sender,
            ticks,
        )
    }

    pub fn start(&mut self, ticks: Arc<RwLock<Ticks<T>>>) {
        let mut loop_helper = spin_sleep::LoopHelper::builder()
            .report_interval_s(0.25)
            .build_with_target_rate(self.tps);
        loop {
            let tick_time = std::time::Instant::now();
            loop_helper.loop_start();

            self.events = self.reciever.try_iter().collect();
            for event in self.events.iter() {
                if let Event::Close = event {
                    break;
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
