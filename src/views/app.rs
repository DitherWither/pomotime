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

use leptos::*;
use tracing::info;

use crate::views::*;

/// The main app view
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    info!("App started.");

    view! {
        cx,
        <Header />
        <MainContent />
        <Footer />
    }
}
