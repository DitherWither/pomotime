//! The main app view
//! 
//! This view contains the main app view.
//! It contains the timer and the controls for the timer.
//! 
//! # Example
//! 
//! ```rust
//! use leptos::*;
//! 
//! use crate::views::*;
//! 
//! mount_to_body(|cx| {
//!    view! {
//!       cx,
//!      <App />
//!   }
//! });
//! ```

use std::time::Duration;

use leptos::*;
use tracing::info;

use crate::views::*;
use crate::timer::Timer;

/// The main app view
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (timer, set_timer) = create_signal(cx, Timer::new(Duration::from_secs(30)));

    info!("App started.");

    view! {
        cx,
        <div>
            <h1> "PomoTime" </h1>
            <h2> "WIP, Most stuff won't work" </h2>

            <TimerDurationChanger timer=timer set_timer=set_timer/>
            <TimerStatusDisplay timer=timer/>
            <TimerStatusChanger timer=timer set_timer=set_timer/>

        </div>
    }
}