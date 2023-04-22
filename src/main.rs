// SPDX-License-Identifier: GPL-3.0-or-later
use leptos::*;

use pomotime::views::*;

pub fn main() {
    // Set up panic messages and tracing.
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    mount_to_body(|cx| {
        view! {
            cx,
            <App />
        }
    })
}
