//! This module contains the view `TimerDurationChanger`.
//! 
//! This view allows the user to change the duration of the timer.
//! 
//! # Example
//! 
//! Inside a view:
//! ```html
//! <TimerDurationChanger timer=timer set_timer=timer/>
//! ```
//! 
//! 

use std::time::Duration;

use leptos::*;
use tracing::info;

use crate::timer::Timer;

/// Should allow the user to decide between a short break, long break, and work.
///
/// This isn't actually implemented yet. It just shows a message saying that it isn't implemented.
///
/// # Arguments
///
/// * `set_timer` - The setter for the timer.
#[component]
pub fn TimerDurationChanger(
    cx: Scope,
    timer: ReadSignal<Timer>,
    set_timer: WriteSignal<Timer>,
) -> impl IntoView {
    let (_a, set_a) = create_signal(cx, timer.get().total_time.to_string());
    let on_input = move |event| {
        let duration_string = event_target_value(&event);

        info!("Changing timer duration to: {}", duration_string);

        let duration = duration_string
            .parse::<i32>()
            .expect("duration to be parsable to i32");

        set_a(duration_string);

        if duration != timer.get().total_time {
            info!("Setting timer to new duration: {}", duration);
            set_timer.update(|timer| {
                // TODO: Change the durations to be minutes instead of seconds.
                *timer = Timer::new(Duration::from_secs(duration as u64));
            });
        } else {
            info!("Duration is the same as the current duration. Doing nothing.");
        }
    };
    

    view! {
        cx,
        <div>
            <h3>"Change Timer Duration: "</h3>

            // TODO: Seperate into a different component.
            <input 
                type="radio"
                value="30" 
                name="timer-duration-changer" 
                id="work-radio-button" 
                on:input=on_input
                checked
            />
            <label 
                for="work-radio-button"
            >
                "Work"
            </label>

            
            <input 
                type="radio" 
                value="5" 
                name="timer-duration-changer" 
                id="short-break-radio-button" 
                on:input=on_input
            />

            <label 
                for="short-break-radio-button"
            >
                "Short Break"
            </label>

            <input 
                type="radio" 
                value="15" 
                name="timer-duration-changer" 
                id="long-break-radio-button" 
                on:input=on_input
            />

            <label 
                for="long-break-radio-button"
            >
                "Long Break"
            </label>
        </div>
    }
}
