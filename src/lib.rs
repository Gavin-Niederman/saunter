//! # Saunter
//!
//! A library for tick based game engines.
//!
//! ## How to use Saunter
//!
//! The core of saunter is the loop. The loop calls the tick function on the provided listener. 
//! Your listener should store the state of your game engine or the current scene in your engine.
//!
//! ```rust
//! struct ExampleListener {
//!     val: f32,
//! }
//! impl Listener for ExampleListener {
//!     // The type of the tick that tick function returns
//!     type TickType = ExampleTick; 
//!     // The type of event that is passed to tick in the Event::Other(EventType) variant
//!     type EventType = ExampleEvent;
//!
//!     fn tick(
//!         &mut self,
//!         dt: f32,
//!         events: &mut Vec<Event<Self::EventType>>,
//!         time: Instant,
//!     ) -> Result<Self::TickType, SaunterError> {
//!
//!         self.val = 1.0 - self.val;
//!         log::info!("{}", self.val);
//!
//!         Ok(Self::TickType {val: self.val})
//!     }
//! }
//! ```
//!
//! You may have noticed the type Tick being used a lot here.
//! Ticks store a snapshot of the state of the listener. The tick function returns the tick that was just processed.
//! In a game engine a tick would only ever be used for rendering. 
//! no game logic should happen in the render thread. 
//! Because we only need to send things used for rendering in the tick you should try your best to limit what gets sent in a tick for optimization.
//!
//! ```rust
//! struct ExampleTick {
//!     // Ticks store the time they were made so that they can be interpolated
//!     pub time: Instant,
//!
//!     pub val: f32,
//! }
//! impl Tick for ExampleTick {
//!     // This is called by ticks when b is the most recent tick and self is the last to interpolate between the two
//!     fn lerp(&self, b: &Self, t: f32) -> Result<Self, MathError> {
//!         Ok( Self {
//!             // Saunter provides lerp functions.
//!             time: math::lerp_instant(&self.time, &b.time, t)?,
//!             val: math::lerp(self.val, b.val, t)?,
//!         })
//!     }
//!
//!     fn get_time(&self) -> &Instant {
//!         &self.time
//!     } 
//! }
//! ```
//!
//! Now we can create the loop! The easiest way to create a loop is using `Loop::init()`
//!
//! ```rust
//! let (mut tick_loop, event_sender, ticks) = Loop::init(
//!     // The listener that the loop will call tick() on
//!     Box::new(listener),
//!     // The state of the engine or scene when the program is started. AKA the first tick
//!     first_tick, 
//!     // The number of ticks to occur per second (TPS), does not have to be an integer
//!     tps,
//! );
//! ```
//!
//! Now that you have a loop all that is left is to send it events. To do this you can use the event_sender that was returned by `Loop::init()`. The event sender is a `Sender<Event<EventType>>` where EventType is the type of event you specified in your listener. **EventType is wrapped in a ``saunter::event::Event``**. This is to guarentee that you can send a close event to the loop. To send an event you can use the `send()` method on the event sender.
//!
//! ### Wait? What is a Ticks?
//!
//! The ticks type is used to store the most recent and last tick. 
//! It has a lerp funtion that returns a new tick interpolated by the amount specified between the two ticks for use in rendering, or whatever you choose.
//! The ticks returned by ``Loop::init()`` is not actually a ticks, it is a ``Arc<RwLock<Ticks<...>>>``, 
//! This is because it is constantly being updated by the tickloop. For this reason, **only ever have a read lock on ticks**.
//! To help with optimization, it is best practise to immediatly drop the read lock when you are done with it.

pub mod error;
pub mod event;
pub mod listener;
pub mod math;
pub mod tick;
pub mod tickloop;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lerp() {
        assert_eq!(math::lerp(0.0, 100.0, 0.75).unwrap(), 75.0);

        let vals = vec![(0.0, 1.0), (0.5, 1.0), (0.0, 2.0)];
        assert_eq!(math::lerp_vec(vals, 0.5).unwrap(), vec![0.5, 0.75, 1.0]);
    }
}
