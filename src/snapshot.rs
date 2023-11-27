//! Contains the [`Tick`](crate::tick::Tick) trait and [`Ticks`](crate::tick::Ticks) struct.

use crate::{error::SaunterError, interpolate::Interpolate};
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    mem,
    time::Instant,
};

// A snapshot of the state of the game engine, or current scene at the time of creation.
pub trait Snapshot: Interpolate + Clone + Debug {
    /// Returns the time that the tick was created.
    fn get_time(&self) -> &Instant;
}

/// A struct that holds the last tick and the most recent tick. It is used to lerp between the two ticks.
pub struct Snapshots<T: Snapshot> {
    pub last_snapshot: Option<T>,
    pub new_snapshot: T,
}

impl<T: Snapshot> Snapshots<T> {
    /// Creates a new Ticks struct with the provided first tick.
    /// The first tick should be the default state of the game engine or scene.
    pub fn new(first: T) -> Self {
        Snapshots {
            last_snapshot: None,
            new_snapshot: first,
        }
    }

    /// Interpolates between the last tick and the new tick.
    pub fn interpolate_ticks(
        &self,
        t: f32,
        interpolation: impl Fn(f32) -> f32,
    ) -> Result<T, SaunterError> {
        if let Some(last) = &self.last_snapshot {
            Ok(<T as Interpolate>::interpolate(
                last,
                &self.new_snapshot,
                t,
                interpolation,
            ))
        } else {
            Err(SaunterError::TickError(SnapshotError::TooFewSnapshots))
        }
    }

    /// Drops last tick and replaces it with new tick, and then replaces new tick with the new new tick.
    pub fn update(&mut self, new_snapshot: T) {
        self.last_snapshot = Some(mem::replace(&mut self.new_snapshot, new_snapshot));
    }
}

/// Contains errors that can occur when using ticks.
#[derive(Debug)]
pub enum SnapshotError {
    TooFewSnapshots,
    CouldNotCreateSnapshot,
}
impl Error for SnapshotError {}

impl Display for SnapshotError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SnapshotError::TooFewSnapshots => write!(f, "Not enough ticks to lerp"),
            SnapshotError::CouldNotCreateSnapshot => write!(f, "Could not create a tick"),
        }
    }
}
