//! This module contains the view for `TimerStatusDisplay`.
//!
//! This view displays the status of a timer.
//! It displays the time left, the total time, and whether or not the timer is running.
//! 
//! # Example
//! 
//! Inside a view:
//! 
//! ```html
//! <TimerStatusDisplay timer=timer/>
//! ```

use chrono::Duration;
use leptos::*;

use crate::timer::Timer;

/// A component that displays the status of a timer.
///
/// This displays the time left, the total time, and whether or not the timer is running.
///
/// # Arguments
///
/// * `timer` - The getter for the timer.
#[component]
pub fn TimerStatusDisplay(cx: Scope, timer: ReadSignal<Timer>) -> impl IntoView {
    view! {
        cx,
        <div>
            <p> "Time Left: " { move || { duration_to_string(timer().time_left) }}</p>
            <p> "Total Time: " { move || { duration_to_string(timer().total_time) }}</p> 
            <p> "Is Timer Running: " { move || { timer().is_timer_running.to_string() }}</p>
        </div>
    }
}

fn duration_to_string(duration: Duration) -> String {
    let minutes = duration.num_minutes();
    let seconds = duration.num_seconds() - minutes * 60;

    format!("{}:{:02}", minutes, seconds)
}