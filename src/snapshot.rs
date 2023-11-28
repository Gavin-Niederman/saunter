//! [`Snapshot`] and [`Snapshots`] implementations.
//! Every time your [listener](crate::tickloop::TickLoop::init) runs, it will generate a snapshot. A snapshot is a representation of the state of your game at a given tick.
//! The snapshot is then put into a Snapshots, which is used to interpolate between snapshots outside of the tick loop.
//! In general, it is advised to put as little data as possible into your snapshot, as it is moved around in memory quite a bit.

use crate::{error::SaunterError, interpolate::Interpolate};
use std::{fmt::Debug, mem, time::Instant};

// A snapshot of the state of the game engine. It also must store the time of creation in some way.
pub trait Snapshot: Interpolate + Debug {
    /// Returns the time that the snapshot was created.
    fn get_time(&self) -> &Instant;
}

/// A struct that holds the last snapshot and the most recent snapshot. It is used to interpolate between the two snapshots.
pub struct Snapshots<T: Snapshot> {
    pub last_snapshot: Option<T>,
    pub new_snapshot: Option<T>,
}

impl<T: Snapshot> Snapshots<T> {
    /// Creates a new snapshots struct with the provided first snapshot.
    pub fn new() -> Self {
        Snapshots {
            last_snapshot: None,
            new_snapshot: None,
        }
    }

    /// Interpolates between the last tick and the new tick.
    pub fn interpolate_ticks(
        &self,
        t: f32,
        interpolation: impl Fn(f32) -> f32,
    ) -> Result<T, SaunterError> {
        match (&self.last_snapshot.as_ref(), &self.new_snapshot.as_ref()) {
            (Some(ref last), Some(ref new)) => {
                Ok(<T as Interpolate>::interpolate(last, new, t, interpolation))
            }
            _ => Err(SaunterError::TooFewSnapshots),
        }
    }

    /// Drops last tick and replaces it with new tick, and then replaces new tick with the new new tick.
    pub fn update(&mut self, new_snapshot: T) {
        self.last_snapshot = mem::replace(&mut self.new_snapshot, Some(new_snapshot));
    }
}
