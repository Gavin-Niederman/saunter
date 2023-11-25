//! Contains the [`Loop`](crate::tickloop::Loop) struct.

use log;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crate::error::SaunterError;
use crate::event::Event;
use crate::snapshot::{Snapshot, Snapshots};
type Listener<T, E> = dyn FnMut(f32, Vec<Event<E>>, Instant) -> Result<T, SaunterError> + Send;

/// The Loop struct is the heart of saunter. It calls [`tick`](crate::listener::Listener::tick) on the [`Listener`](crate::listener::Listener) passed to it and updates the [`Ticks`](crate::tick::Ticks) struct passed to it.
pub struct Loop<T: Snapshot, E: Send + Clone> {
    pub listener: Box<Listener<T, E>>,
    pub tick_length: Duration,
    pub tps: f32,
    pub events: Vec<Event<E>>,
    reciever: Receiver<Event<E>>,
}

impl<'a, T: Snapshot, E: Send + Clone> Loop<T, E> {
    /// Creates a new Loop struct.
    /// It is recommended to use [`init`](crate::tickloop::Loop::init) instead.
    pub fn new<F>(listener: F, tps: f32, reciever: Receiver<Event<E>>) -> Self
    where
        F: FnMut(f32, Vec<Event<E>>, Instant) -> Result<T, SaunterError> + Send + 'static,
    {
        let tick_length = Duration::from_secs_f32(1.0 / tps);
        Loop {
            listener: Box::new(listener),
            tick_length,
            tps,
            events: Vec::new(),
            reciever,
        }
    }

    /// Creates a new Loop struct and returns a [`Sender`](std::sync::mpsc::Sender) to send events to the loop.
    pub fn init<F>(
        listener: F,
        first_tick: T,
        tps: f32,
    ) -> (
        Self,
        Sender<Event<E>>,
        &'static mut Arc<RwLock<Snapshots<T>>>,
    )
    where
        F: FnMut(f32, Vec<Event<E>>, Instant) -> Result<T, SaunterError> + Send + 'static,
    {
        let (event_sender, event_reciever) = mpsc::channel::<Event<E>>();
        let ticks = Box::leak(Box::new(Arc::new(RwLock::new(Snapshots::new(first_tick)))));

        (
            Self::new(listener, tps, event_reciever),
            event_sender,
            ticks,
        )
    }

    /// Starts the loop. This function will block the current thread. So the loop should be sent to a new thread, and start called on it there.
    pub fn start(&mut self, ticks: Arc<RwLock<Snapshots<T>>>) {
        let mut deficit = Duration::from_secs_f32(0.0);

        'a: loop {
            let tick_time = std::time::Instant::now();

            self.events = self.reciever.try_iter().collect();
            for event in self.events.iter() {
                if let Event::Close = event {
                    break 'a;
                }
            }

            if let Ok(tick) = (self.listener)(
                self.tick_length.as_secs_f32(),
                self.events.clone(),
                tick_time,
            ) {
                let mut tick_wlock = ticks.write().unwrap();
                log::debug!("lock aquired {:?}", std::time::Instant::now());
                (*tick_wlock).update(tick);
                // Drop the write lock so the read lock can be acquired.
            }
            log::debug!("lock dropped {:?}", std::time::Instant::now());

            let elapsed = tick_time.elapsed();
            if elapsed < self.tick_length {
                let mut sleep_dur = self.tick_length - elapsed;
                // Automatically catch the loop back up when there is a deficit.
                log::debug!("deficit: {:?}", deficit);
                if deficit < sleep_dur {
                    sleep_dur -= deficit;
                    deficit = Duration::from_secs_f32(0.0);
                } else {
                    deficit -= sleep_dur;
                    sleep_dur *= 0;
                }

                spin_sleep::sleep(self.tick_length - elapsed);
            } else {
                let current_tick_deficit = elapsed - self.tick_length;
                deficit += current_tick_deficit;
                log::debug!("tick took too long");
            }

            log::debug!("actual tick length {:?}", tick_time.elapsed());
        }
    }
}
