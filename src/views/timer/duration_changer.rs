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

use chrono::Duration;

use leptos::{ev::Event, *};
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
    let (_current_selection, set_current_selection) =
        create_signal(cx, timer.get().total_time.to_string());
    let on_input = move |event| click_event_handler(event, timer, set_timer, set_current_selection);

    view! {
        cx,
        <div>
            <h3>"Change Timer Duration: "</h3>

            <InputElement
                _value="30"
                _id="work-radio-button"
                _on_input=on_input
                _label="Work"
                _checked=true
            />

            <InputElement
                _value="5"
                _id="short-break-radio-button"
                _on_input=on_input
                _label="Short Break"
                _checked=false
            />

            <InputElement
                _value="15"
                _id="long-break-radio-button"
                _on_input=on_input
                _label="Long Break"
                _checked=false
            />
        </div>
    }
}

/// Displays a radio button and a label.
/// and sets a click event handler for the radio button.
///
/// # Arguments
///
/// * `_value` - The value of the radio button.
/// * `_id` - The id of the radio button.
/// * `_on_input` - The event handler for the radio button.
/// * `_label` - The label for the radio button.
/// * `_checked` - Whether or not the radio button is checked.
#[component]
fn input_element<T>(
    cx: Scope,
    _value: &'static str,
    _id: &'static str,
    _on_input: T,
    _label: &'static str,
    _checked: bool,
) -> impl IntoView
where
    T: FnMut(leptos::ev::Event) + 'static,
{
    view! {
        cx,
        <input
            type="radio"
            value=_value
            name="timer-duration-changer"
            id=_id
            on:input=_on_input
            checked=_checked
        />
        <label
            for=_id
        >
            { _label }
        </label>
    }
}

/// Handles the click event for the `TimerDurationChanger` component.
///
/// # Arguments
///
/// * `event` - The event that triggered this function.
/// * `timer` - The getter for the timer.
/// * `set_timer` - The setter for the timer.
/// * `set_current_selection` - The setter for the current selection.
fn click_event_handler(
    event: Event,
    timer: ReadSignal<Timer>,
    set_timer: WriteSignal<Timer>,
    set_current_selection: WriteSignal<String>,
) {
    let duration_string = event_target_value(&event);

    info!("Changing timer duration to: {}", duration_string);

    let duration = duration_string
        .parse::<i32>()
        .expect("duration to be parsable to i32");

    set_current_selection(duration_string);

    let duration = Duration::minutes(duration as i64);

    if duration != timer.get().total_time {
        info!("Setting timer to new duration: {}", duration);
        set_timer.update(|timer| {
            *timer = Timer::new(duration);
        });
    } else {
        info!("Duration is the same as the current duration. Doing nothing.");
    }
}
