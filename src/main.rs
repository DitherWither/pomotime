use leptos::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div>
            <h1>{ "Hello, World!" }</h1>
        </div>
    }
}

pub fn main() {
    mount_to_body(|cx| {
        view! {
            cx,
            <App />
        }
    })
}
