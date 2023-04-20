use std::time::Duration;

use leptos::*;

// TODO: Seperate stuff into different files.
// TODO: Implement the duration changer.
// TODO: Display time in MM:SS format.
// TODO: Improve styling


/// The main app view
#[component]
fn App(cx: Scope) -> impl IntoView {
    let (timer, set_timer) = create_signal(cx, Timer::new(Duration::from_secs(10)));

    view! {
        cx,
        <div>
            <h1> "PomoTime" </h1>
            <h2> "WIP, Most stuff won't work" </h2>

            <TimerDurationChanger set_timer=set_timer/>
            <TimerStatusDisplay timer=timer/>
            <TimerStatusChanger timer=timer set_timer=set_timer/>

        </div>
    }
}

/// A timer that counts down from a given duration.
///
/// The reason we use a struct here is because we want to be able to
/// check the total time, and whether or not the timer is running.
#[derive(Clone)]
struct Timer {
    /// The amount of time left on the timer.
    ///
    /// This is in seconds.
    time_left: i32,

    /// The total amount of time on the timer.
    ///
    /// This is in seconds.
    total_time: i32,

    /// Whether or not the timer is running.
    ///
    /// This is used to stop the timer from counting down.
    is_timer_running: bool,
}

impl Timer {
    /// Creates a new timer with the given duration.
    fn new(duration: Duration) -> Self {
        Self {
            time_left: duration.as_secs() as i32,
            total_time: duration.as_secs() as i32,
            is_timer_running: true,
        }
    }
}


/// A component that displays the status of a timer.
/// 
/// This displays the time left, the total time, and whether or not the timer is running.
/// 
/// # Arguments
/// 
/// * `timer` - The getter for the timer.
#[component]
fn TimerStatusDisplay(cx: Scope, timer: ReadSignal<Timer>) -> impl IntoView {
    view! {
        cx,
        <div>
            <p> "Time Left:" { move || { timer().time_left.to_string() }}</p>
            <p> "Total Time:" { move || { timer().total_time.to_string() }}</p>
            <p> "Is Timer Running:" { move || { timer().is_timer_running.to_string() }}</p>
        </div>
    }
}

/// A component that allows the user to change the status of a timer.
/// 
/// This allows the user to start, stop, and reset the timer.
/// 
/// # Arguments
/// 
/// * `timer` - The getter for the timer.
/// * `set_timer` - The setter for the timer.
#[component]
fn TimerStatusChanger(
    cx: Scope,
    timer: ReadSignal<Timer>,
    set_timer: WriteSignal<Timer>,
) -> impl IntoView {
    view! {
        cx,
        <button
                on:click=move |_| {
                    start_timer(timer, set_timer);
                }>"Start Timer"</button>

            <button
                on:click=move |_| {
                    set_timer.update(|timer| {
                        timer.is_timer_running = false;
                    });
                }>"Stop Timer"</button>

            <button
                on:click=move |_| {
                    set_timer.update(|timer| {
                        timer.time_left = timer.total_time;
                        timer.is_timer_running = true;
                    });
                }>"Reset Timer"</button>
    }
}

/// Should allow the user to decide between a short break, long break, and work.
/// 
/// This isn't actually implemented yet. It just shows a message saying that it isn't implemented.
/// 
/// # Arguments
/// 
/// * `set_timer` - The setter for the timer.
#[component]
fn TimerDurationChanger(cx: Scope, set_timer: WriteSignal<Timer>) -> impl IntoView {
    view! {
        cx,
        <div>
            <h3>"Change Timer Duration: TODO"</h3>
        </div>
    }
}

/// Starts the timer.
/// 
/// This will start the timer, and will recursively call itself every second asynchronously
/// until the timer is done.
fn start_timer(timer: ReadSignal<Timer>, timer_setter: WriteSignal<Timer>) {

    // If the timer is not running, then we don't need to do anything.
    // Just return and stop the timer.
    //
    // We don't need to explicitly stop the timer, because we set the timeout 
    // in the end of the function.
    if !timer.get().is_timer_running {
        return;
    }


    // Subtract one from the time left.
    timer_setter.update(move |timer| {
        timer.time_left -= 1;
    });

    // If the time left is less than or equal to zero, then we need to stop the timer.
    if timer.get().time_left <= 0 {
        timer_setter.update(|timer| {
            timer.is_timer_running = false;
        });
        return;
    };

    // Otherwise, we need to set a timeout to call this function again.
    set_timeout(
        move || start_timer(timer, timer_setter),
        Duration::from_secs(1),
    )
}


pub fn main() {
    mount_to_body(|cx| {
        view! {
            cx,
            <App />
        }
    })
}
