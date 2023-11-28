//! Contains the [`TickLoop`] struct.
//! The tick loop is the heart of Saunter; It runs all of your code at a set tick rate (TPS).
//! If your code takes longer than the tick interval to run,
//! the tick loop will run as fast as possible until it catches back up.

use log;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};

use crate::error::SaunterError;
use crate::snapshot::{Snapshot, Snapshots};
type Listener<T, E> =
    dyn FnMut(f32, Vec<E>, TickLoopControl, Instant) -> Result<T, SaunterError> + Send;

pub enum TickLoopState {
    Running,
    Stopped,
    Paused,
}

pub struct TickLoopControl {
    state: Arc<Mutex<TickLoopState>>,
}

impl TickLoopControl {
    pub fn stop(&mut self) {
        let mut state = self.state.lock().unwrap();
        *state = TickLoopState::Stopped;
    }

    pub fn pause(&mut self) {
        let mut state = self.state.lock().unwrap();
        *state = TickLoopState::Paused;
    }

    pub fn resume(&mut self) {
        let mut state = self.state.lock().unwrap();
        *state = TickLoopState::Running;
    }
}

/// The tick loop is the heart of Saunter.
/// The tick loop runs your code at a set tick rate and generates [`Snapshots`]
pub struct TickLoop<S: Snapshot, E: Send> {
    pub listener: Box<Listener<S, E>>,
    pub tick_length: Duration,
    reciever: Receiver<E>,
    state: Arc<Mutex<TickLoopState>>,
    snapshots: Arc<RwLock<Snapshots<S>>>,
}

impl<'a, S: Snapshot, E: Send> TickLoop<S, E> {
    /// Creates a new Loop struct.
    /// It is recommended to use [`init`](TickLoop::init) instead.
    pub fn new<F>(
        listener: F,
        tps: f32,
        reciever: Receiver<E>,
        state: Arc<Mutex<TickLoopState>>,
        snapshots: Arc<RwLock<Snapshots<S>>>,
    ) -> Self
    where
        F: FnMut(f32, Vec<E>, TickLoopControl, Instant) -> Result<S, SaunterError> + Send + 'static,
    {
        let tick_length = Duration::from_secs_f32(1.0 / tps);
        TickLoop {
            listener: Box::new(listener),
            tick_length,
            reciever,
            snapshots,
            state,
        }
    }

    /// Creates a new Loop struct and returns a [`Sender`] to send events to the loop.
    pub fn init<F>(
        listener: F,
        tps: f32,
    ) -> (
        Self,
        Sender<E>,
        TickLoopControl,
        Arc<RwLock<Snapshots<S>>>,
    )
    where
        F: FnMut(f32, Vec<E>, TickLoopControl, Instant) -> Result<S, SaunterError> + Send + 'static,
    {
        let (event_sender, event_reciever) = mpsc::channel::<E>();
        let snapshots = Arc::new(RwLock::new(Snapshots::new()));
        let state = Arc::new(Mutex::new(TickLoopState::Running));

        (
            Self::new(listener, tps, event_reciever, state.clone(), snapshots.clone()),
            event_sender,
            TickLoopControl {
                state: state.clone(),
            },
            snapshots,
        )
    }

    /// Starts the loop. This function will block the current thread. So the loop should be sent to a new thread, and start called on it there.
    pub fn start(&mut self) {
        let mut deficit = Duration::from_secs_f32(0.0);

        'a: loop {
            let tick_time = std::time::Instant::now();

            match *self.state.lock().unwrap() {
                TickLoopState::Stopped => break 'a,
                TickLoopState::Paused => {
                    spin_sleep::sleep(self.tick_length);
                    continue 'a;
                }
                TickLoopState::Running => {}
            }

            let events = self.reciever.try_iter().collect();

            if let Ok(snapshot) = (self.listener)(
                self.tick_length.as_secs_f32(),
                events,
                TickLoopControl {
                    state: self.state.clone(),
                },
                tick_time,
            ) {
                let mut tick_wlock = self.snapshots.write().unwrap();
                log::debug!("lock aquired {:?}", std::time::Instant::now());
                (*tick_wlock).update(snapshot);
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
