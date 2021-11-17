pub use std::time::{Duration, Instant};
use bevy_ecs::system::ResMut;

/// Tracks elapsed time since the last update and since the App has started
/// Updates on [`CoreState::First`]
#[derive(Debug)]
pub struct Time {
    startup: Instant,
    startup_delta: Duration,
    last_update: Option<Instant>,
    delta: Duration,
}

impl Default for Time {
    fn default() -> Time {
        Time {
            startup: Instant::now(),
            startup_delta: Duration::from_secs(0),
            last_update: None,
            delta: Duration::from_secs(0),
        }
    }
}

impl Time {
    pub fn update(&mut self) {
        self.update_with_instant(Instant::now());
    }

    pub(crate) fn update_with_instant(&mut self, instant: Instant) {
        if let Some(last_update) = self.last_update {
            self.delta = instant - last_update;
        }
        self.startup_delta = instant - self.startup;
        self.last_update = Some(instant);
    }

    /// The delta between the current tick and last tick as a [`Duration`]
    pub fn delta(&self) -> Duration {
        self.delta
    }

    /// The delta between the current and last tick as [`f32`] seconds
    pub fn delta_seconds(&self) -> f32 {
        self.delta.as_secs_f32()
    }

    /// The delta between the current and last tick as [`f64`] seconds
    pub fn delta_seconds_f64(&self) -> f64 {
        self.delta.as_secs_f64()
    }

    /// The time since startup in seconds
    pub fn delta_since_startup(&self) -> Duration {
        self.startup_delta
    }

    /// The [`Instant`] the app was started
    pub fn startup(&self) -> Instant {
        self.startup
    }

    /// The ['Instant'] when [`Time::update`] was last called, if it exists
    pub fn last_update(&self) -> Option<Instant> {
        self.last_update
    }
}

pub(crate) fn update_time(mut time: ResMut<Time>) {
    time.update();
}
