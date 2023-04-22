//! This module contains the view for `TimerStatusChanger`.
//!
//! This view allows the user to change the status of the timer
//! between running, stopped, and reset.
//!
//! # Example
//!
//! Inside a view:
//!
//! ```html
//! <TimerStatusChanger timer=timer set_timer=timer/>
//! ```

use leptos::*;
use tracing::info;

use crate::timer::{start_timer, Timer};

/// A component that allows the user to change the status of a timer.
///
/// This allows the user to start, stop, and reset the timer.
///
/// # Arguments
///
/// * `timer` - The getter for the timer.
/// * `set_timer` - The setter for the timer.
#[component]
pub fn TimerStatusChanger(
    cx: Scope,
    timer: ReadSignal<Timer>,
    set_timer: WriteSignal<Timer>,
) -> impl IntoView {
    view! {
        cx,
        <div class="hcenter btn-row">
            <button
                    class="margin-1"
                    on:click=move |_| {
                        info!("Start button clicked, Starting timer.");
                        start_timer(timer, set_timer);
                    }>"Start Timer"</button>

                <button
                    class="margin-1"
                    on:click=move |_| {
                        set_timer.update(|timer| {
                            info!("Stop button clicked, setting the timer to stop.");
                            timer.is_timer_running = false;
                        });
                    }>"Stop Timer"</button>

                <button
                    class="margin-1 red"
                    on:click=move |_| {
                        info!("Reset button clicked, Resetting timer.");
                        set_timer.update(|timer| {
                            timer.time_left = timer.total_time;
                            timer.is_timer_running = true;
                        });
                    }>"Reset Timer"</button>
            </div>
    }
}
