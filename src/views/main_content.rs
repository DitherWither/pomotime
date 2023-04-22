// SPDX-License-Identifier: GPL-3.0-or-later
//! The main content view
//!
//! This view contains the main content for the app.
//! It contains the timer and the controls for the timer.

use crate::{timer::Timer, views::*};
use chrono::Duration;
use leptos::*;

#[component]
pub fn MainContent(cx: Scope) -> impl IntoView {
    let (timer, set_timer) = create_signal(cx, Timer::new(Duration::seconds(30)));

    view! {
        cx,
        <section class="main-content">
            <TimerDurationChanger timer=timer set_timer=set_timer/>
            <TimerStatusDisplay timer=timer/>
            <TimerStatusChanger timer=timer set_timer=set_timer/>
        </section>
    }
}
