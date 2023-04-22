# PomoTime

PomoTime is a simple pomodoro timer that helps you focus on your work.

It is made with leptos in rust.

It is hosted at https://ditherwither.github.io/pomotime/

## File structure
 - `src/` contains the rust source code
   - `src/views/` contain the leptos components that make up the app
     - `src/views/timer` contains components for the timer
 - `assets/` contains the static assets used in the app
 - `styles/` contains the css/sass styles used in the app
 - `svg-src` contains the unminified svg files used in the app. Should be minified before use. Also loads fonts from google fonts instead of locally so that it works without our server. These are currently minified using a web tool manually. The minimized files are in `assets/`

## Building

### Requirements

 - rust toolchain (Install using [rustup](https://rustup.rs/))
 - `wasm32-unknown-unknown` target (Install using `rustup target add wasm32-unknown-unknown`)
 - `trunk` (to install run `cargo install trunk`)

### Building

 - Run `trunk build --release` to build the app for production. The output will be in `dist/`
 - Run `trunk serve` to serve the app on `localhost:8080`

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE](LICENSE) file for details.

## Libraries used

 - [leptos](https://crates.io/crates/leptos) for the frontend
 - [chrono](https://crates.io/crates/chrono) for time
 - [tracing](https://crates.io/crates/tracing) and [tracing-wasm](https://crates.io/crates/tracing-wasm) for logging and tracing
 - [console_error_panic_hook](https://crates.io/crates/console_error_panic_hook) for logging panics.