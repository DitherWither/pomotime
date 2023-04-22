// SPDX-License-Identifier: GPL-3.0-or-later
//! The header view
//!
//! This view contains the header for the app.
//! And it contains the title of the app.
use leptos::*;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <header>
            <div class="navbar">
                <h1> "PomoTime" </h1>
                <span class="hidden-mobile">"A simple pomodoro timer that helps you focus on your work"</span>
            </div>
        </header>
    }
}
