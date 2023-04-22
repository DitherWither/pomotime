// SPDX-License-Identifier: GPL-3.0-or-later
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

use chrono::Duration;

use leptos::*;
use tracing::info;

use crate::timer::Timer;
use crate::views::*;

/// The main app view
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (timer, set_timer) = create_signal(cx, Timer::new(Duration::seconds(30)));

    info!("App started.");

    view! {
        cx,
        <div>
            <div class="navbar">
                <h1> "PomoTime" </h1>
                <span class="hidden-mobile">"A simple pomodoro timer that helps you focus on your work"</span>
            </div>

            <TimerDurationChanger timer=timer set_timer=set_timer/>
            <TimerStatusDisplay timer=timer/>
            <TimerStatusChanger timer=timer set_timer=set_timer/>

        </div>
    }
}
