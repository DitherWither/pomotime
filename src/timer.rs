//! This module contains the timer and related functions.
//!
//! This module contains the timer struct, and the functions that are used to start the timer.

use chrono::Duration;

use leptos::*;
use tracing::info;

/// A timer that counts down from a given duration.
///
/// The reason we use a struct here is because we want to be able to
/// check the total time, and whether or not the timer is running.
#[derive(Clone)]
pub struct Timer {
    /// The amount of time left on the timer.
    pub time_left: Duration,

    /// The total amount of time on the timer.
    ///
    /// This is in seconds.
    pub total_time: Duration,

    /// Whether or not the timer is running.
    ///
    /// This is used to stop the timer from counting down.
    pub is_timer_running: bool,
}

impl Timer {
    /// Creates a new timer with the given duration.
    ///
    /// This will create a new timer with the given duration.
    ///
    /// # Arguments
    ///
    /// * `duration` - The duration of the timer.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::time::Duration;
    ///
    /// use crate::timer::Timer;
    ///
    /// let timer = Timer::new(Duration::seconds(10));
    /// ```
    pub fn new(duration: Duration) -> Self {
        info!("Creating new timer with duration: {:?}", duration);
        Self {
            time_left: duration,
            total_time: duration,
            is_timer_running: true,
        }
    }
}

/// Starts the timer.
///
/// This will start the timer, and will recursively call itself every second asynchronously
/// until the timer is done.
///
/// # Arguments
///
/// * `timer` - The getter for the timer.
/// * `timer_setter` - The setter for the timer.
pub fn start_timer(timer: ReadSignal<Timer>, timer_setter: WriteSignal<Timer>) {
    // If the timer is not running, then we don't need to do anything.
    // Just return and stop the timer.
    //
    // We don't need to explicitly stop the timer, because we set the timeout
    // in the end of the function.
    if !timer.get().is_timer_running {
        info!("Timer is not running. Stopping timer.");
        return;
    }

    // Subtract one from the time left.
    timer_setter.update(move |timer| {
        timer.time_left = timer
            .time_left
            .checked_sub(&Duration::seconds(1))
            .expect("1 second to be subtracted from the timer");
    });

    // If the time left is less than or equal to zero, then we need to stop the timer.
    if timer.get().time_left.is_zero() {
        info!("Timer is less than or is 0. Stopping timer.");
        timer_setter.update(|timer| {
            timer.is_timer_running = false;
        });
        return;
    };

    // Otherwise, we need to set a timeout to call this function again.
    set_timeout(
        move || start_timer(timer, timer_setter),
        std::time::Duration::from_secs(1), // Leptos uses std::time::Duration, instead of chrono::Duration.
    )
}
