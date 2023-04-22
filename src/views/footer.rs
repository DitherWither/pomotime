// SPDX-License-Identifier: GPL-3.0-or-later
//! The footer view
//!
//! This view contains the footer for the app.
//! And it contains the license information for the app and a link to the source code.

use leptos::*;

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <footer>
            <div class="footer">
                <p>
                    "Made with "
                    <span class="heart">"❤"</span>
                    " by "
                    <a href="https://github.com/DitherWither">"DitherWither"</a>
                </p>
                <p>
                    "This web application is open source, and licensed under the GNU General Public License. Source code and License details available in the "
                    <a href="https://github.com/DitherWither/pomotime/">"GitHub Repo."</a>
                </p>
            </div>
        </footer>
    }
}
